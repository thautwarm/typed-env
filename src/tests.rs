use crate::{Envar, EnvarDef, EnvarError, ListEnvar, ListEnvarConfig};
use std::sync::Mutex;

static SINGLE_THREAD_ASSURANCE: Mutex<()> = Mutex::new(());

// Helper function to clear environment variable
fn clear_env_var(name: &str) {
    unsafe { std::env::remove_var(name) };
}

// Helper function to set environment variable
fn set_env_var(name: &str, value: &str) {
    unsafe { std::env::set_var(name, value) };
}

// Helper to get lock and handle poisoning
fn get_test_lock() -> std::sync::MutexGuard<'static, ()> {
    SINGLE_THREAD_ASSURANCE.lock().unwrap()
}

#[test]
fn test_parse_i32() {
    let _lock = get_test_lock();

    static VAR: Envar<i32> = Envar::on_demand("T1_TEST_I32", || EnvarDef::Unset);

    let value = VAR.value();
    assert!(value.is_err());
    assert!(
        matches!(value.err().unwrap(), EnvarError::NotSet(varname) if varname == "T1_TEST_I32")
    );

    let _ = unsafe { std::env::set_var("T1_TEST_I32", "123") };

    static VAR2: Envar<i32> = Envar::on_demand("T1_TEST_I32", || EnvarDef::Default(123));

    let value = VAR2.value();
    assert!(value.is_ok());
    assert_eq!(value.unwrap(), 123);
}

// Test all integer types
#[test]
fn test_all_integer_types() {
    let _lock = get_test_lock();

    // Test i8
    clear_env_var("TEST_I8");
    static VAR_I8: Envar<i8> = Envar::on_demand("TEST_I8", || EnvarDef::Unset);
    set_env_var("TEST_I8", "127");
    assert_eq!(VAR_I8.value().unwrap(), 127i8);

    // Test i16
    clear_env_var("TEST_I16");
    static VAR_I16: Envar<i16> = Envar::on_demand("TEST_I16", || EnvarDef::Unset);
    set_env_var("TEST_I16", "32767");
    assert_eq!(VAR_I16.value().unwrap(), 32767i16);

    // Test i32
    clear_env_var("TEST_I32");
    static VAR_I32: Envar<i32> = Envar::on_demand("TEST_I32", || EnvarDef::Unset);
    set_env_var("TEST_I32", "2147483647");
    assert_eq!(VAR_I32.value().unwrap(), 2147483647i32);

    // Test i64
    clear_env_var("TEST_I64");
    static VAR_I64: Envar<i64> = Envar::on_demand("TEST_I64", || EnvarDef::Unset);
    set_env_var("TEST_I64", "9223372036854775807");
    assert_eq!(VAR_I64.value().unwrap(), 9223372036854775807i64);

    // Test isize
    clear_env_var("TEST_ISIZE");
    static VAR_ISIZE: Envar<isize> = Envar::on_demand("TEST_ISIZE", || EnvarDef::Unset);
    set_env_var("TEST_ISIZE", "1000");
    assert_eq!(VAR_ISIZE.value().unwrap(), 1000isize);

    // Test u8
    clear_env_var("TEST_U8");
    static VAR_U8: Envar<u8> = Envar::on_demand("TEST_U8", || EnvarDef::Unset);
    set_env_var("TEST_U8", "255");
    assert_eq!(VAR_U8.value().unwrap(), 255u8);

    // Test u16
    clear_env_var("TEST_U16");
    static VAR_U16: Envar<u16> = Envar::on_demand("TEST_U16", || EnvarDef::Unset);
    set_env_var("TEST_U16", "65535");
    assert_eq!(VAR_U16.value().unwrap(), 65535u16);

    // Test u32
    clear_env_var("TEST_U32");
    static VAR_U32: Envar<u32> = Envar::on_demand("TEST_U32", || EnvarDef::Unset);
    set_env_var("TEST_U32", "4294967295");
    assert_eq!(VAR_U32.value().unwrap(), 4294967295u32);

    // Test u64
    clear_env_var("TEST_U64");
    static VAR_U64: Envar<u64> = Envar::on_demand("TEST_U64", || EnvarDef::Unset);
    set_env_var("TEST_U64", "18446744073709551615");
    assert_eq!(VAR_U64.value().unwrap(), 18446744073709551615u64);

    // Test usize
    clear_env_var("TEST_USIZE");
    static VAR_USIZE: Envar<usize> = Envar::on_demand("TEST_USIZE", || EnvarDef::Unset);
    set_env_var("TEST_USIZE", "1000");
    assert_eq!(VAR_USIZE.value().unwrap(), 1000usize);
}

#[test]
fn test_float_types() {
    let _lock = get_test_lock();

    // Test f32
    clear_env_var("TEST_F32");
    static VAR_F32: Envar<f32> = Envar::on_demand("TEST_F32", || EnvarDef::Unset);
    set_env_var("TEST_F32", "3.14159");
    let result = VAR_F32.value().unwrap();
    assert!((result - 3.14159f32).abs() < f32::EPSILON);

    // Test f64
    clear_env_var("TEST_F64");
    static VAR_F64: Envar<f64> = Envar::on_demand("TEST_F64", || EnvarDef::Unset);
    set_env_var("TEST_F64", "3.141592653589793");
    let result = VAR_F64.value().unwrap();
    assert!((result - 3.141592653589793f64).abs() < f64::EPSILON);
}

#[test]
fn test_string_type() {
    let _lock = get_test_lock();

    clear_env_var("TEST_STRING");
    static VAR_STRING: Envar<String> = Envar::on_demand("TEST_STRING", || EnvarDef::Unset);
    set_env_var("TEST_STRING", "Hello, World!");
    assert_eq!(VAR_STRING.value().unwrap(), "Hello, World!");

    // Test with empty string
    set_env_var("TEST_STRING", "");
    assert_eq!(VAR_STRING.value().unwrap(), "");

    // Test with whitespace
    set_env_var("TEST_STRING", "  spaces around  ");
    assert_eq!(VAR_STRING.value().unwrap(), "  spaces around  ");
}

#[test]
fn test_bool_type() {
    let _lock = get_test_lock();

    clear_env_var("TEST_BOOL");
    static VAR_BOOL: Envar<bool> = Envar::on_demand("TEST_BOOL", || EnvarDef::Unset);

    // Test true alternatives
    let true_values = [
        "true", "1", "yes", "y", "on", "enabled", "TRUE", "YES", "ON",
    ];
    for value in &true_values {
        set_env_var("TEST_BOOL", value);
        assert_eq!(
            VAR_BOOL.value().unwrap(),
            true,
            "Failed for value: {}",
            value
        );
    }

    // Test false alternatives
    let false_values = [
        "false", "0", "no", "n", "off", "disabled", "FALSE", "NO", "OFF",
    ];
    for value in &false_values {
        set_env_var("TEST_BOOL", value);
        assert_eq!(
            VAR_BOOL.value().unwrap(),
            false,
            "Failed for value: {}",
            value
        );
    }

    // Test empty string (should be false)
    set_env_var("TEST_BOOL", "");
    assert_eq!(VAR_BOOL.value().unwrap(), false);

    // Test whitespace only (should be false)
    set_env_var("TEST_BOOL", "   ");
    assert_eq!(VAR_BOOL.value().unwrap(), false);

    // Test invalid bool value
    set_env_var("TEST_BOOL", "invalid");
    let result = VAR_BOOL.value();
    assert!(result.is_err());
    match result.err().unwrap() {
        EnvarError::ParseError { typename, .. } => {
            assert_eq!(typename, "bool");
        }
        _ => panic!("Expected ParseError"),
    }
}

#[test]
fn test_on_startup_vs_on_demand() {
    let _lock = get_test_lock();

    // Test on_startup
    clear_env_var("TEST_STARTUP");
    set_env_var("TEST_STARTUP", "42");
    static VAR_STARTUP: Envar<i32> = Envar::on_startup("TEST_STARTUP", || EnvarDef::Unset);
    assert_eq!(VAR_STARTUP.value().unwrap(), 42);

    // Change env var value - on_startup should still return original value
    set_env_var("TEST_STARTUP", "100");
    assert_eq!(VAR_STARTUP.value().unwrap(), 42); // Should still be 42

    // Test on_demand
    clear_env_var("TEST_DEMAND");
    set_env_var("TEST_DEMAND", "42");
    static VAR_DEMAND: Envar<i32> = Envar::on_demand("TEST_DEMAND", || EnvarDef::Unset);
    assert_eq!(VAR_DEMAND.value().unwrap(), 42);

    // Change env var value - on_demand should return new value
    set_env_var("TEST_DEMAND", "100");
    assert_eq!(VAR_DEMAND.value().unwrap(), 100); // Should be 100
}

#[test]
fn test_default_values() {
    let _lock = get_test_lock();

    // Test with default set
    clear_env_var("TEST_DEFAULT_SET");
    static VAR_DEFAULT_SET: Envar<i32> =
        Envar::on_demand("TEST_DEFAULT_SET", || EnvarDef::Default(999));
    assert_eq!(VAR_DEFAULT_SET.value().unwrap(), 999);

    // Test with env var set - should override default
    set_env_var("TEST_DEFAULT_SET", "123");
    assert_eq!(VAR_DEFAULT_SET.value().unwrap(), 123);

    // Test with default unset
    clear_env_var("TEST_DEFAULT_UNSET");
    static VAR_DEFAULT_UNSET: Envar<i32> =
        Envar::on_demand("TEST_DEFAULT_UNSET", || EnvarDef::Unset);
    let result = VAR_DEFAULT_UNSET.value();
    assert!(result.is_err());
    match result.err().unwrap() {
        EnvarError::NotSet(varname) => {
            assert_eq!(varname, "TEST_DEFAULT_UNSET");
        }
        _ => panic!("Expected NotSet error"),
    }
}

#[test]
fn test_parse_errors() {
    let _lock = get_test_lock();

    // Test invalid integer
    clear_env_var("TEST_PARSE_ERROR");
    static VAR_INT: Envar<i32> = Envar::on_demand("TEST_PARSE_ERROR", || EnvarDef::Unset);
    set_env_var("TEST_PARSE_ERROR", "not_a_number");
    let result = VAR_INT.value();
    assert!(result.is_err());
    match result.err().unwrap() {
        EnvarError::ParseError {
            varname, typename, ..
        } => {
            assert_eq!(varname, "TEST_PARSE_ERROR");
            assert_eq!(typename, "i32");
        }
        _ => panic!("Expected ParseError"),
    }

    // Test integer overflow
    static VAR_U8: Envar<u8> = Envar::on_demand("TEST_PARSE_ERROR", || EnvarDef::Unset);
    set_env_var("TEST_PARSE_ERROR", "999");
    let result = VAR_U8.value();
    assert!(result.is_err());

    // Test invalid float
    static VAR_FLOAT: Envar<f32> = Envar::on_demand("TEST_PARSE_ERROR", || EnvarDef::Unset);
    set_env_var("TEST_PARSE_ERROR", "not_a_float");
    let result = VAR_FLOAT.value();
    assert!(result.is_err());
}

#[test]
fn test_envar_name() {
    let _lock = get_test_lock();

    static VAR: Envar<i32> = Envar::on_demand("MY_VAR_NAME", || EnvarDef::Unset);
    assert_eq!(VAR.name(), "MY_VAR_NAME");
}

// Define test configurations for ListEnvar
#[derive(Clone)]
struct CommaConfig;
impl ListEnvarConfig for CommaConfig {
    const SEP: &'static str = ",";
    const FILTER_EMPTY_STR: bool = true;
    const FILTER_WHITESPACE: bool = true;
}

#[derive(Clone)]
struct SemicolonConfig;
impl ListEnvarConfig for SemicolonConfig {
    const SEP: &'static str = ";";
    const FILTER_EMPTY_STR: bool = false;
    const FILTER_WHITESPACE: bool = false;
}

#[derive(Clone)]
struct NoFilterConfig;
impl ListEnvarConfig for NoFilterConfig {
    const SEP: &'static str = ",";
    const FILTER_EMPTY_STR: bool = false;
    const FILTER_WHITESPACE: bool = false;
}

#[test]
fn test_envar_list_basic() {
    let _lock = get_test_lock();

    clear_env_var("TEST_LIST");
    static VAR_LIST: Envar<ListEnvar<i32, CommaConfig>> =
        Envar::on_demand("TEST_LIST", || EnvarDef::Unset);

    set_env_var("TEST_LIST", "1,2,3,4,5");
    let result = VAR_LIST.value().unwrap();
    assert_eq!(result.len(), 5);
    assert_eq!(result[0], 1);
    assert_eq!(result[1], 2);
    assert_eq!(result[2], 3);
    assert_eq!(result[3], 4);
    assert_eq!(result[4], 5);
}

#[test]
fn test_envar_list_string() {
    let _lock = get_test_lock();

    clear_env_var("TEST_LIST_STRING");
    static VAR_LIST: Envar<ListEnvar<String, CommaConfig>> =
        Envar::on_demand("TEST_LIST_STRING", || EnvarDef::Unset);

    set_env_var("TEST_LIST_STRING", "hello,world,test");
    let result = VAR_LIST.value().unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "hello");
    assert_eq!(result[1], "world");
    assert_eq!(result[2], "test");
}

#[test]
fn test_envar_list_filtering() {
    let _lock = get_test_lock();

    // Test with filtering enabled
    clear_env_var("TEST_LIST_FILTER");
    static VAR_LIST_FILTER: Envar<ListEnvar<String, CommaConfig>> =
        Envar::on_demand("TEST_LIST_FILTER", || EnvarDef::Unset);

    set_env_var("TEST_LIST_FILTER", "hello,,world,  ,test");
    let result = VAR_LIST_FILTER.value().unwrap();
    assert_eq!(result.len(), 3); // Empty strings and whitespace-only filtered out
    assert_eq!(result[0], "hello");
    assert_eq!(result[1], "world");
    assert_eq!(result[2], "test");

    // Test with filtering disabled
    clear_env_var("TEST_LIST_NO_FILTER");
    static VAR_LIST_NO_FILTER: Envar<ListEnvar<String, NoFilterConfig>> =
        Envar::on_demand("TEST_LIST_NO_FILTER", || EnvarDef::Unset);

    set_env_var("TEST_LIST_NO_FILTER", "hello,,world,  ,test");
    let result = VAR_LIST_NO_FILTER.value().unwrap();
    assert_eq!(result.len(), 5); // All items kept including empty and whitespace
    assert_eq!(result[0], "hello");
    assert_eq!(result[1], "");
    assert_eq!(result[2], "world");
    assert_eq!(result[3], ""); // The whitespace gets trimmed during parsing, so it becomes ""
    assert_eq!(result[4], "test");
}

#[test]
fn test_envar_list_different_separator() {
    let _lock = get_test_lock();

    clear_env_var("TEST_LIST_SEMICOLON");
    static VAR_LIST: Envar<ListEnvar<i32, SemicolonConfig>> =
        Envar::on_demand("TEST_LIST_SEMICOLON", || EnvarDef::Unset);

    set_env_var("TEST_LIST_SEMICOLON", "10;20;30");
    let result = VAR_LIST.value().unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], 10);
    assert_eq!(result[1], 20);
    assert_eq!(result[2], 30);
}

#[test]
fn test_envar_list_empty() {
    let _lock = get_test_lock();

    clear_env_var("TEST_LIST_EMPTY");
    static VAR_LIST: Envar<ListEnvar<String, CommaConfig>> =
        Envar::on_demand("TEST_LIST_EMPTY", || EnvarDef::Unset);

    set_env_var("TEST_LIST_EMPTY", "");
    let result = VAR_LIST.value().unwrap();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_envar_list_parse_error() {
    let _lock = get_test_lock();

    clear_env_var("TEST_LIST_PARSE_ERROR");
    static VAR_LIST: Envar<ListEnvar<i32, CommaConfig>> =
        Envar::on_demand("TEST_LIST_PARSE_ERROR", || EnvarDef::Unset);

    set_env_var("TEST_LIST_PARSE_ERROR", "1,2,not_a_number,4");
    let result = VAR_LIST.value();
    assert!(result.is_err());
    match result.err().unwrap() {
        EnvarError::ParseError {
            varname, typename, ..
        } => {
            assert_eq!(varname, "TEST_LIST_PARSE_ERROR");
            assert_eq!(typename, "i32");
        }
        _ => panic!("Expected ParseError"),
    }
}

#[test]
fn test_envar_list_display() {
    let _lock = get_test_lock();

    clear_env_var("TEST_LIST_DISPLAY");
    static VAR_LIST: Envar<ListEnvar<String, CommaConfig>> =
        Envar::on_demand("TEST_LIST_DISPLAY", || EnvarDef::Unset);

    set_env_var("TEST_LIST_DISPLAY", "hello,world,test");
    let result = VAR_LIST.value().unwrap();
    assert_eq!(format!("{}", result), "hello,world,test");
}

#[test]
fn test_envar_list_debug() {
    let _lock = get_test_lock();

    clear_env_var("TEST_LIST_DEBUG");
    static VAR_LIST: Envar<ListEnvar<i32, CommaConfig>> =
        Envar::on_demand("TEST_LIST_DEBUG", || EnvarDef::Unset);

    set_env_var("TEST_LIST_DEBUG", "1,2,3");
    let result = VAR_LIST.value().unwrap();
    let debug_str = format!("{:?}", result);
    assert!(debug_str.contains("ListEnvar"));
    assert!(debug_str.contains("[1, 2, 3]"));
}

#[test]
fn test_envar_def_to_option() {
    let set_def = EnvarDef::Default(42);
    assert_eq!(set_def.to_option(), Some(42));

    let unset_def: EnvarDef<i32> = EnvarDef::Unset;
    assert_eq!(unset_def.to_option(), None);
}

#[test]
fn test_envar_option() {
    let _lock = get_test_lock();
    set_env_var("TEST_OPTION1", "");
    static VAR_OPTION: Envar<Option<i32>> =
        Envar::on_startup("TEST_OPTION1", || EnvarDef::Default(Some(42)));
    assert_eq!(VAR_OPTION.value().unwrap(), Some(42));

    clear_env_var("TEST_OPTION2");
    static VAR_OPTION2: Envar<Option<i32>> =
        Envar::on_demand("TEST_OPTION2", || EnvarDef::Default(Some(42)));
    assert!(std::env::var("TEST_OPTION2").is_err());
    assert_eq!(VAR_OPTION2.value().unwrap(), Some(42));
}
