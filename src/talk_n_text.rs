use regex::Regex;

use crate::{errors::MobileNetworkError, validate::Validate};

pub(crate) const TALK_N_TEXT_PREFIXES: &[&str] = &[
    "0907", "0909", "0910", "0912", "0918", "0930", "0938", "0946", "0948", "0950", "0963", "0989",
    "0998",
];

pub struct TNT(regex::Regex);

impl TNT {
    pub(crate) fn new() -> Result<Self, MobileNetworkError> {
        let pattern = format!("^({})\\d{{7}}$", TALK_N_TEXT_PREFIXES.join("|"));
        Regex::new(&pattern)
            .map(Self)
            .map_err(|e| MobileNetworkError::RegexError(e.to_string()))
    }
}

impl Validate for TNT {
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
