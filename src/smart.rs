use regex::Regex;

use crate::{errors::MobileNetworkError, pattern::generate_pattern, validate::Validate};

#[derive(Debug, Clone)]
pub struct Smart(regex::Regex);

impl Smart {
    pub(crate) fn new(prefixes: &[&str]) -> Result<Self, MobileNetworkError> {
        let pattern = generate_pattern(prefixes)?;
        let regex =
            Regex::new(&pattern).map_err(|e| MobileNetworkError::RegexError(e.to_string()))?;
        Ok(Self(regex))
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
