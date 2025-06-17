# typed-env

[![Crates.io](https://img.shields.io/crates/v/typed-env.svg)](https://crates.io/crates/typed-env)
[![Documentation](https://docs.rs/typed-env/badge.svg)](https://docs.rs/typed-env)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/thautwarm/typed-env/workflows/CI/badge.svg)](https://github.com/thautwarm/typed-env/actions)
[![codecov](https://codecov.io/gh/thautwarm/typed-env/branch/main/graph/badge.svg)](https://codecov.io/gh/thautwarm/typed-env)

Describe the requirements of environment variables in a type-safe and ergonomic way.

## Features

- **Type Safety**: Parse environment variables directly into Rust types
- **Flexible Loading**: Choose between on-demand and startup loading strategies
- **Globals**: `LazyLock`-like globals for easy access to environment variables.
- **Rich Types**: Support for primitives, booleans, strings, lists and [your custom types](#custom-types).
- **Default Values**: Support for factory-based fallback/default values.
- **Error Handling**: Comprehensive error types with helpful messages
- **Specialized Support**:
    - [List](#lists): Parse comma-separated (or custom delimiter) lists with filtering options.
    - [Boolean](#booleans): Flexible boolean parsing with multiple accepted formats: `true`, `1`, `yes`, `y`, `on`, `enabled` (case insensitive) for true, and `false`, `0`, `no`, `n`, `off`, `disabled` (case insensitive) for false.
    - [Custom Types](#custom-types): Parse custom types with custom parsing and error handling.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
typed-env = "0.1"
```

### Basic Usage

```rust
use typed_env::{Envar, EnvarDef};

// Define typed environment variables
static PORT: Envar<u16> = Envar::on_demand("PORT", || EnvarDef::Default(8080));
static DEBUG: Envar<bool> = Envar::on_demand("DEBUG", || EnvarDef::Default(false));
static DATABASE_URL: Envar<String> = Envar::on_demand("DATABASE_URL", || EnvarDef::Unset);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the values - they're parsed automatically
    let port = PORT.value()?;           // u16
    let debug = DEBUG.value()?;         // bool
    let db_url = DATABASE_URL.value()?; // String

    println!("Server running on port {} (debug: {})", port, debug);
    println!("Database: {}", db_url);
    Ok(())
}
```

## Loading Strategies

### On-Demand Loading
Variables are parsed when first accessed and cached thereafter. If the environment variable is changed, the new value will be loaded on the next access.

```rust
static API_KEY: Envar<String> = Envar::on_demand("API_KEY", || EnvarDef::Unset);

// Parsed and cached on first call
let key1 = API_KEY.value()?;
// Returns cached value
let key2 = API_KEY.value()?;
// ...
unsafe { std::env::set_var("API_KEY", "1234567890"); }
// ...
let key3 = API_KEY.value()?; // Loads the new value
// ...
```

### Startup Loading
Variables are parsed once at startup and never re-read:

```rust
static MAX_CONNECTIONS: Envar<u32> = Envar::on_startup("MAX_CONNECTIONS", || EnvarDef::Default(100));

// Always returns the same value, even if env var changes
let max_conn = MAX_CONNECTIONS.value()?;
```

**WARNING**: `Envar::on_startup` does not load the environment variables at the actual startup time, but at the time of the first access.

## Supported Types

### Primitives
All standard integer and float types are supported:

```rust
static TIMEOUT_MS: Envar<u64> = Envar::on_demand("TIMEOUT_MS", || EnvarDef::Default(5000));
static RATE_LIMIT: Envar<f64> = Envar::on_demand("RATE_LIMIT", || EnvarDef::Default(10.5));
```

### Booleans
Flexible boolean parsing with multiple accepted formats:

```rust
static ENABLE_LOGS: Envar<bool> = Envar::on_demand("ENABLE_LOGS", || EnvarDef::Default(true));
```

**Accepted values:**
- **True**: `true`, `1`, `yes`, `y`, `on`, `enabled` (case insensitive)
- **False**: `false`, `0`, `no`, `n`, `off`, `disabled` (case insensitive)
- **Empty string**: treated as `false`

### Lists
Parse delimited lists with configurable separators and filtering:

```rust
use typed_env::{ListEnvar, ListEnvarConfig};

// Define a custom list configuration
struct CommaList;
impl ListEnvarConfig for CommaList {
    const SEP: &'static str = ",";
    const FILTER_EMPTY_STR: bool = true;
    const FILTER_WHITESPACE: bool = true;
}

static ALLOWED_ORIGINS: Envar<ListEnvar<String, CommaList>> =
    Envar::on_demand("ALLOWED_ORIGINS", || EnvarDef::Unset);

// ALLOWED_ORIGINS="localhost,127.0.0.1,example.com"
let origins = ALLOWED_ORIGINS.value()?;
for origin in origins.iter() {
    println!("Allowed origin: {}", origin);
}
```

### Custom Types

```rust
use typed_env::{
    Envar,
    EnvarDef,
    EnvarError,
    EnvarParse,
    EnvarParser,
    ErrorReason
};

// accept: LEVEL="vvv"
static LEVEL: Envar<Level> = Envar::on_demand("LEVEL", || EnvarDef::Unset);

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
```

## Default Values

### Set Defaults
Provide a fallback value when the environment variable is not set:

```rust
static WORKER_THREADS: Envar<usize> = Envar::on_demand("WORKER_THREADS", || EnvarDef::Default(4));

// Returns 4 if WORKER_THREADS is not set
let threads = WORKER_THREADS.value()?;
```

### Unset (Required)
Require the environment variable to be present:

```rust
static SECRET_KEY: Envar<String> = Envar::on_demand("SECRET_KEY", || EnvarDef::Unset);

// Returns Err(EnvarError::NotSet) if SECRET_KEY is not set
let secret = SECRET_KEY.value()?;
```

## Error Handling

The library provides detailed error information:

```rust
use typed_env::EnvarError;

match DATABASE_URL.value() {
    Ok(url) => println!("Database URL: {}", url),
    Err(EnvarError::NotSet(name)) => {
        eprintln!("Required environment variable {} is not set", name);
    }
    Err(EnvarError::ParseError { varname, typename, value, .. }) => {
        eprintln!("Failed to parse {} as {}: {:?}", varname, typename, value);
    }
}
```

## API Reference

### Core Types

- **`Envar<T>`**: The main environment variable container
- **`EnvarDef<T>`**: Defines default behavior (`Default(value)` or `Unset`)
- **`ListEnvar<T, C>`**: Container for list-type environment variables
- **`ListEnvarConfig`**: Trait for configuring list parsing behavior

### Methods

- **`Envar::on_demand(name, default_factory)`**: Create an on-demand loaded variable
- **`Envar::on_startup(name, default_factory)`**: Create a startup-loaded variable
- **`envar.value()`**: Get the parsed value, returns `Result<T, EnvarError>`
- **`envar.name()`**: Get the environment variable name

### Error Types

- **`EnvarError::NotSet(name)`**: Environment variable is not set and no default provided
- **`EnvarError::ParseError { varname, typename, value, reason }`**: Failed to parse the value

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.