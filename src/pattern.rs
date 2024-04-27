use crate::errors::MobileNetworkError;


pub(crate) fn generate_pattern(prefixes: &[&str]) -> Result<String, MobileNetworkError> {
    let pattern = format!("^({})\\d{{7}}$", prefixes.join("|"));
    Ok(pattern)
}