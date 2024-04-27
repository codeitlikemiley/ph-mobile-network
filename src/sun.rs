use regex::Regex;

use crate::{errors::MobileNetworkError, validate::Validate};

pub(crate) const SUN_PREFIXES: &[&str] = &[
    "0922", "0923", "0924", "0925", "0931", "0932", "0933", "0934", "0940", "0941", "0942", "0943",
    "0944", "0973", "0974",
];

pub struct Sun(regex::Regex);

impl Sun {
    pub(crate) fn new() -> Result<Self, MobileNetworkError> {
        let pattern = format!("^({})\\d{{7}}$", SUN_PREFIXES.join("|"));
        Regex::new(&pattern)
            .map(Self)
            .map_err(|e| MobileNetworkError::RegexError(e.to_string()))
    }
}

impl Validate for Sun {
    fn validate(&self, number: &str) -> Result<bool, MobileNetworkError> {
        if !number.chars().all(char::is_numeric) {
            return Err(MobileNetworkError::NonNumeric);
        }
        if number.len() != 11 {
            return Err(MobileNetworkError::InvalidLength);
        }
        Ok(self.0.is_match(number))
    }
}
