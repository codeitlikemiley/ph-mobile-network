use regex::Regex;

use crate::{errors::MobileNetworkError, validate::Validate};

pub(crate) const DITO_PREFIXES: &[&str] = &[
    "0895", "0896", "0897", "0898", "0991", "0992", "0993", "0994",
];

pub struct Dito(regex::Regex);

impl Dito {
    pub(crate) fn new() -> Result<Self, MobileNetworkError> {
        
        let pattern = format!("^({})\\d{{7}}$", DITO_PREFIXES.join("|"));
        Regex::new(&pattern)
            .map(Self)
            .map_err(|e| MobileNetworkError::RegexError(e.to_string()))
    }
}

impl Validate for Dito {
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

