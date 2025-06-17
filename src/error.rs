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
}
