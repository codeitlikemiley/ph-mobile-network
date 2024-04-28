use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MobileNetworkError {
    InvalidLength,
    NonNumeric,
    UnrecognizedPrefix(String),
    RegexError(String), // To handle regex compilation errors
    MutexError(String), // To handle mutex lock errors
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
            MobileNetworkError::UnrecognizedPrefix(text) => {
                write!(f,  "Unrecognized mobile number prefix: {}", text)
            }
            MobileNetworkError::RegexError(text) => write!(f, "Regex error {}", text),
            MobileNetworkError::MutexError(text) => write!(f, "Mutex lock error {}", text),
        }
    }
}

impl Error for MobileNetworkError {}
