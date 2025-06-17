use typed_env::{Envar, EnvarDef, EnvarError, EnvarParse, EnvarParser, ErrorReason, ListEnvar, ListEnvarConfig};

struct CommaListConf;
struct ColonListConf;

impl ListEnvarConfig for CommaListConf {
    const SEP: &'static str = ",";

    /// whether to filter empty strings
    const FILTER_EMPTY_STR: bool = true;

    /// whether to filter whitespace
    const FILTER_WHITESPACE: bool = true;
}

impl ListEnvarConfig for ColonListConf {
    const SEP: &'static str = ":";

    /// whether to filter empty strings
    const FILTER_EMPTY_STR: bool = true;

    /// whether to filter whitespace
    const FILTER_WHITESPACE: bool = true;
}


// accept: FOO="1,2,3, 5"
static FOO: Envar<ListEnvar<i32, CommaListConf>> = Envar::on_demand("FOO", || EnvarDef::Unset);

// accept: BAR="1:on:YES:Y:n"
static BAR: Envar<ListEnvar<bool, ColonListConf>> = Envar::on_demand("BAR", || EnvarDef::Unset);

// accept: LEVEL="vvv"
static LEVEL: Envar<Level> = Envar::on_demand("LEVEL", || EnvarDef::Unset);

// > FOO="1,2,,3" BAR="1:on" LEVEL="vvvv" cargo run
//
// output:
//  foo: ListEnvar { _vec: [1, 2, 3] }
//  bar: ListEnvar { _vec: [true, true] }
//  level: Level(4)
fn main() {
    let foo_values = FOO.value().unwrap();
    println!("foo: {:?}", foo_values);

    let bar_values = BAR.value().unwrap();
    println!("bar: {:?}", bar_values);

    let level = LEVEL.value().unwrap();
    println!("level: {:?}", level);
}


#[derive(Clone, Debug)]
pub struct Level(pub usize);

impl EnvarParse<Level> for EnvarParser<Level> {
    fn parse(varname: std::borrow::Cow<'static, str>, value: &str) -> Result<Level, typed_env::EnvarError> {
        let value = value.trim();
        let mut count = 0;
        for c in value.chars() {
            if c == 'v' {
                count += 1;
            }
            else {
                return Err(EnvarError::ParseError {
                    varname,
                    typename: std::any::type_name::<Level>(),
                    value: value.to_string(),
                    reason: ErrorReason::new(move || format!("invalid character: {}", c)),
                });
            }
        }
        Ok(Level(count))
    }
}