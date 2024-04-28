use regex::Regex;

use crate::{errors::MobileNetworkError, pattern::generate_pattern, validate::Validate};

pub struct Sun(regex::Regex);

impl Sun {
    pub(crate) fn new(prefixes: &[&str]) -> Result<Self, MobileNetworkError> {
        let pattern = generate_pattern(prefixes)?;
        let regex =
            Regex::new(&pattern).map_err(|e| MobileNetworkError::RegexError(e.to_string()))?;
        Ok(Self(regex))
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

        if !self.0.is_match(number) {
            return Err(MobileNetworkError::UnrecognizedPrefix(number.to_owned()));
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_ok_pattern_initialization() {
        let prefixes = ["0917", "0925"];
        let result = Sun::new(&prefixes);
        // no error on initialization
        assert!(result.is_ok());
    }

    #[test]
    fn is_valid() {
        let prefixes = ["0917", "0925"];
        let result = Sun::new(&prefixes).unwrap();
        // exact length and prefix  and is a valid number
        assert!(result.validate("09171234567").unwrap());
    }

    #[test]
    fn is_err_for_non_numeric() {
        let prefixes = ["0917", "0925"];
        let result = Sun::new(&prefixes).unwrap();
        // contains letters
        assert!(result.validate("0917abc4567").is_err());
    }

    #[test]
    fn is_err_on_invalid_length() {
        let prefixes = ["0917", "0925"];
        let result = Sun::new(&prefixes).unwrap();
        // 9 digits
        assert!(result.validate("0917123456").is_err());
        // 12 digits
        assert!(result.validate("091712345678").is_err());
    }

    #[test]
    fn is_err_on_unrecognized_prefix() {
        let prefixes = ["0917", "0925"];

        let result = Sun::new(&prefixes).unwrap();

        let result = result.validate("09991234567");

        assert!(result.is_err());
    }
}
