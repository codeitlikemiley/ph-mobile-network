use regex::Regex;

use crate::{errors::MobileNetworkError, validate::Validate};

pub(crate) const GLOBE_PREFIXES: &[&str] = &[
    "0817", "0904", "0905", "0906", "0915", "0916", "0917", "0926", "0927", "0935", "0936", "0937",
    "0945", "0954", "0955", "0956", "0965", "0966", "0967", "0975", "0976", "0977", "0978", "0979",
    "0995", "0996", "0997", "09173", "09175", "09176", "09178", "09253", "09255", "09256", "09257",
    "09258",
];

pub struct Globe(regex::Regex);

impl Globe {
    pub(crate) fn new() -> Result<Self, MobileNetworkError> {
        
        let pattern = format!("^({})\\d{{7}}$", GLOBE_PREFIXES.join("|"));
        Regex::new(&pattern)
            .map(Self)
            .map_err(|e| MobileNetworkError::RegexError(e.to_string()))
    }
}

impl Validate for Globe {
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
