#[cfg(feature = "wasm-js")]
use wasm_bindgen::JsValue;
#[cfg(feature = "napi-types")]
use napi::{JsError, Error as NapiError};

#[derive(strum_macros::Display, Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubsetSumError {
    ExecutionTimeout,
}

#[cfg(feature = "wasm-js")]
impl From<SubsetSumError> for JsValue {
    fn from(s: SubsetSumError) -> Self {
        Self::from(s.to_string())
    }
}

#[cfg(feature = "napi-types")]
impl From<SubsetSumError> for JsError {
    fn from(error: SubsetSumError) -> Self {
        JsError::from(NapiError::from_reason(format!("SubsetSumError: {}", error)))
    }
}

