use crate::errors::MobileNetworkError;

pub trait Validate {
     fn validate(&self, number: &str) -> Result<bool, MobileNetworkError>;
 }