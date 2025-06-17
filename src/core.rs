use crate::error::EnvarError;
use crate::list_envar::ListEnvar;
use crate::list_envar::ListEnvarConfig;
use crate::ErrorReason;
use std::borrow::Cow;

enum EnvarStore<T> {
    OnStartup(std::sync::OnceLock<T>),
    OnDemand(std::sync::Mutex<(Option<String>, Option<T>)>),
}

pub enum EnvarDef<T> {
    Unset,
    Default(T),
}

impl<T> EnvarDef<T> {
    pub fn to_option(self) -> Option<T> {
        match self {
            EnvarDef::Default(value) => Some(value),
            EnvarDef::Unset => None,
        }
    }
}

pub struct Envar<T, F = fn() -> EnvarDef<T>> {
    _name: &'static str,
    _default_factory: F,
    /// used when loaded on startup
    store: EnvarStore<T>,
}

impl<T, F> Envar<T, F>
where
    T: Clone + 'static,
    EnvarParser<T>: EnvarParse<T>,
    F: Fn() -> EnvarDef<T>,
{
    pub const fn on_demand(name: &'static str, default_factory: F) -> Self {
        Self {
            _name: name,
            _default_factory: default_factory,
            store: EnvarStore::OnDemand(std::sync::Mutex::new((None, None))),
        }
    }

    pub const fn on_startup(name: &'static str, default_factory: F) -> Self {
        Self {
            _name: name,
            _default_factory: default_factory,
            store: EnvarStore::OnStartup(std::sync::OnceLock::new()),
        }
    }

    pub fn name(&self) -> &'static str {
        self._name
    }

    pub fn value(&self) -> Result<T, EnvarError> {
        match &self.store {
            EnvarStore::OnStartup(once_loaded) => {
                // check if once lock is initialized
                if let Some(value) = once_loaded.get() {
                    return Ok(value.clone());
                }

                if let Ok(value) = std::env::var(self._name) {
                    match EnvarParser::<T>::parse(Cow::Borrowed(self._name), value.as_str()) {
                        Ok(value) => {
                            // preemption is possible, we make sure to maintain consistency
                            Ok(once_loaded.get_or_init(move || value).clone())
                        }
                        Err(EnvarError::TryDefault(varname)) => {
                            if let EnvarDef::Default(default) = (self._default_factory)() {
                                Ok(once_loaded.get_or_init(move || default).clone())
                            } else {
                                Err(EnvarError::NotSet(varname))
                            }
                        }
                        Err(e) => Err(e),
                    }
                } else {
                    if let Some(value) = once_loaded.get() {
                        return Ok(value.clone());
                    }
                    if let EnvarDef::Default(default) = (self._default_factory)() {
                        let _ = once_loaded.set(default.clone());
                        return Ok(default);
                    } else {
                        Err(EnvarError::NotSet(Cow::Borrowed(self._name)))
                    }
                }
            }
            EnvarStore::OnDemand(mutex) => {
                let mut entry = mutex.lock().unwrap();
                let env_value = std::env::var(self._name).ok();

                let reset_value =
                    |env_value: Option<String>, entry: &mut (Option<String>, Option<T>)| {
                        let value = match env_value.as_ref() {
                            None => (self._default_factory)().to_option(),
                            Some(value) => match EnvarParser::<T>::parse(
                                Cow::Borrowed(self._name),
                                value.as_str(),
                            ) {
                                Ok(value) => Some(value),
                                Err(EnvarError::TryDefault(varname)) => {
                                    if let EnvarDef::Default(default) = (self._default_factory)() {
                                        return Ok(default);
                                    } else {
                                        return Err(EnvarError::NotSet(varname));
                                    }
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            },
                        };

                        let value = match value {
                            None => return Err(EnvarError::NotSet(Cow::Borrowed(self._name))),
                            Some(value) => value,
                        };

                        entry.0 = env_value;
                        entry.1 = Some(value.clone());

                        return Ok(value);
                    };

                if entry.0.as_ref() == env_value.as_ref() {
                    if let Some(value) = entry.1.clone() {
                        return Ok(value);
                    }
                }

                return reset_value(env_value, &mut entry);
            }
        }
    }
}

macro_rules! impl_via_parse {
    ($($t:ty),*) => {
        $(
        impl EnvarParse<$t> for $crate::EnvarParser<$t> {
            fn parse(varname: Cow<'static, str>, s: &str) -> Result<$t, EnvarError> {
                Ok(s.parse::<$t>().map_err(|e| EnvarError::ParseError {
                    varname,
                    typename: stringify!($t),
                    value: s.to_string(),
                    reason: ErrorReason::new(move || format!("{}", e)),
                })?)
            }
        }
        )*
    };
}

impl_via_parse!(usize, u64, u32, u16, u8, isize, i64, i32, i16, i8, f64, f32);

impl EnvarParse<String> for EnvarParser<String> {
    fn parse(_varname: Cow<'static, str>, value: &str) -> Result<String, EnvarError> {
        return Ok(value.to_string());
    }
}

impl EnvarParse<bool> for EnvarParser<bool> {
    fn parse(varname: Cow<'static, str>, value: &str) -> Result<bool, EnvarError> {
        let value = value.trim();
        if value.is_empty() {
            return Ok(false);
        }

        for true_alternative in crate::special_constants::TRUE_ALTERNATIVES {
            if true_alternative.eq_ignore_ascii_case(value) {
                return Ok(true);
            }
        }

        for false_alternative in crate::special_constants::FALSE_ALTERNATIVES {
            if false_alternative.eq_ignore_ascii_case(value) {
                return Ok(false);
            }
        }

        return Err(EnvarError::ParseError {
            varname,
            typename: "bool",
            value: value.to_string(),
            reason: ErrorReason::new({
                let owned_value = value.to_string();
                move || owned_value
            }),
        });
    }
}

impl<T, C> EnvarParse<ListEnvar<T, C>> for EnvarParser<ListEnvar<T, C>>
where
    C: ListEnvarConfig,
    EnvarParser<T>: EnvarParse<T>,
{
    fn parse(varname: Cow<'static, str>, value: &str) -> Result<ListEnvar<T, C>, EnvarError> {
        let mut list: Vec<T> = vec![];

        for item in value.split(C::SEP) {
            if C::FILTER_EMPTY_STR && item.is_empty() {
                continue;
            }
            let trimmed = item.trim();
            if C::FILTER_WHITESPACE && trimmed.is_empty() {
                continue;
            }
            let parsed = EnvarParser::<T>::parse(varname.clone(), trimmed);
            match parsed {
                Ok(value) => list.push(value),
                Err(e) => return Err(e),
            }
        }

        Ok(ListEnvar::new(list))
    }
}

impl<T> EnvarParse<Option<T>> for EnvarParser<Option<T>>
where
    EnvarParser<T>: EnvarParse<T>,
{
    fn parse(varname: Cow<'static, str>, value: &str) -> Result<Option<T>, EnvarError> {
        let value = value.trim();
        if value.is_empty() {
            return Err(EnvarError::TryDefault(varname));
        }
        let parsed = EnvarParser::<T>::parse(varname, value);
        Ok(Some(parsed?))
    }
}

pub struct EnvarParser<T: ?Sized>(std::marker::PhantomData<T>);

pub trait EnvarParse<T> {
    fn parse(varname: Cow<'static, str>, value: &str) -> Result<T, EnvarError>;
}
