use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MobileNetworkError {
    InvalidLength,
    NonNumeric,
    UnrecognizedPrefix,
    RegexError(String), // To handle regex compilation errors
    LockError(String), // To handle mutex lock errors
}

impl fmt::Display for MobileNetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MobileNetworkError::InvalidLength => write!(
                f,
                "Invalid phone number length. Expected exactly 11 digits."
            ),
            MobileNetworkError::NonNumeric => {
                write!(f, "Mobile number contains non-numeric characters.")
            }
            MobileNetworkError::UnrecognizedPrefix => {
                write!(f, "Mobile number prefix is not recognized.")
            }
            MobileNetworkError::RegexError(text) => write!(f, "Invalid Regex error {}", text),
            MobileNetworkError::LockError(text) => write!(f, "Mutex lock error {}", text),
        }
    }
}

impl Error for MobileNetworkError {}
