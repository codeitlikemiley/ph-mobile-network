use regex::Regex;

use crate::{errors::MobileNetworkError, validate::Validate};

pub(crate) const SMART_PREFIXES: &[&str] = &[
    "0813", "0908", "0911", "0913", "0914", "0919", "0920", "0921", "0928", "0929", "0939", "0946",
    "0947", "0949", "0951", "0961", "0963", "0968", "0969", "0970", "0981", "0998", "0999", "0960",
];

pub struct Smart(regex::Regex);

impl Smart {
    pub(crate) fn new() -> Result<Self, MobileNetworkError> {
        
        let pattern = format!("^({})\\d{{7}}$", SMART_PREFIXES.join("|"));
        Regex::new(&pattern)
            .map(Self)
            .map_err(|e| MobileNetworkError::RegexError(e.to_string()))
    }
}

impl Validate for Smart {
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
