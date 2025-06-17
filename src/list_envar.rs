use std::marker::PhantomData;
use std::sync::Arc;

pub struct ListEnvar<E, C> {
    _marker: PhantomData<C>,
    _vec: Arc<Vec<E>>,
}

impl<E: Clone, C> Clone for ListEnvar<E, C> {
    fn clone(&self) -> Self {
        Self {
            _marker: PhantomData,
            _vec: self._vec.clone(),
        }
    }
}

/// Configuration for the `ListEnvar` type
pub trait ListEnvarConfig {
    /// The separator to use when parsing the list
    const SEP: &'static str;

    /// Whether to filter empty strings
    const FILTER_EMPTY_STR: bool;

    /// Whether to filter whitespace
    const FILTER_WHITESPACE: bool;
}

impl<T, C: ListEnvarConfig> ListEnvar<T, C> {
    pub(crate) fn new(vec: Vec<T>) -> Self {
        Self {
            _marker: PhantomData,
            _vec: Arc::new(vec),
        }
    }
}

impl<T, C: ListEnvarConfig> std::ops::Deref for ListEnvar<T, C> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self._vec
    }
}

impl<T: std::fmt::Debug, C: ListEnvarConfig> std::fmt::Debug for ListEnvar<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListEnvar {{ _vec: {:?} }}", self._vec)
    }
}

impl<T: std::fmt::Display, C: ListEnvarConfig> std::fmt::Display for ListEnvar<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self._vec.iter().enumerate() {
            if i > 0 {
                write!(f, "{}", C::SEP)?;
            }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}
