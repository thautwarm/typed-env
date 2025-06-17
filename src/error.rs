use crate::ErrorReason;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvarError {
    #[error("Cannot parse environment variable {varname} (value = {value:?}) as {typename}")]
    ParseError {
        varname: Cow<'static, str>,
        typename: &'static str,
        value: String,
        reason: ErrorReason,
    },

    #[error("Environment variable {0} is not set")]
    NotSet(Cow<'static, str>),

    // This is a special case:
    // Sometimes even if the environment variable is set, we
    // might prefer to use the default value (if any).
    // For example, if the environment variable is set to an empty string,
    // we might prefer to use the default value.
    #[error("Environment variable {0} is not set and default factory returned None")]
    TryDefault(Cow<'static, str>),
}
