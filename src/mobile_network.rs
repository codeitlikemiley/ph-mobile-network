use std::fmt;

use crate::{
    dito::Dito,
    errors::MobileNetworkError,
    globals::{
        dito_prefixes::DITO_PREFIXES, globe_prefixes::GLOBE_PREFIXES,
        smart_prefixes::SMART_PREFIXES, sun_prefixes::SUN_PREFIXES, tnt_prefixes::TNT_PREFIXES,
    },
    globe::Globe,
    smart::Smart,
    sun::Sun,
    talk_n_text::TNT,
    validate::Validate,
};

pub enum MobileNetwork {
    Globe(Globe),
    Smart(Smart),
    Sun(Sun),
    TNT(TNT),
    Dito(Dito),
    Invalid(String),
}

impl MobileNetwork {
    pub fn get(number: &str) -> Result<Self, MobileNetworkError> {
        let prefix = &number[..number.len().min(4)];

        let globe_prefixes = GLOBE_PREFIXES.try_lock().map_err(|_| {
            MobileNetworkError::MutexError("Failed to lock GLOBE_PREFIXES".to_string())
        })?;

        let smart_prefixes = SMART_PREFIXES.try_lock().map_err(|_| {
            MobileNetworkError::MutexError("Failed to lock SMART_PREFIXES".to_string())
        })?;

        let sun_prefixes = SUN_PREFIXES.try_lock().map_err(|_| {
            MobileNetworkError::MutexError("Failed to lock SUN_PREFIXES".to_string())
        })?;

        let talk_n_text_prefixes = TNT_PREFIXES.try_lock().map_err(|_| {
            MobileNetworkError::MutexError("Failed to lock TALK_N_TEXT_PREFIXES".to_string())
        })?;

        let dito_prefixes = DITO_PREFIXES.try_lock().map_err(|_| {
            MobileNetworkError::MutexError("Failed to lock DITO_PREFIXES".to_string())
        })?;

        if globe_prefixes.contains(&prefix) {
            Globe::new(&globe_prefixes).map(MobileNetwork::Globe)
        } else if smart_prefixes.contains(&prefix) {
            Smart::new(&smart_prefixes).map(MobileNetwork::Smart)
        } else if sun_prefixes.contains(&prefix) {
            Sun::new(&sun_prefixes).map(MobileNetwork::Sun)
        } else if talk_n_text_prefixes.contains(&prefix) {
            TNT::new(&talk_n_text_prefixes).map(MobileNetwork::TNT)
        } else if dito_prefixes.contains(&prefix) {
            Dito::new(&dito_prefixes).map(MobileNetwork::Dito)
        } else {
            Err(MobileNetworkError::UnrecognizedPrefix(prefix.to_string()))
        }
    }
}

impl Validate for MobileNetwork {
    fn validate(&self, number: &str) -> Result<bool, crate::errors::MobileNetworkError> {
        match self {
            MobileNetwork::Globe(globe) => globe.validate(number),
            MobileNetwork::Smart(smart) => smart.validate(number),
            MobileNetwork::Sun(sun) => sun.validate(number),
            MobileNetwork::TNT(tnt) => tnt.validate(number),
            MobileNetwork::Dito(dito) => dito.validate(number),
            MobileNetwork::Invalid(text) => {
                Err(MobileNetworkError::UnrecognizedPrefix(text.clone()))
            }
        }
    }
}

impl fmt::Display for MobileNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MobileNetwork::Globe(_) => write!(f, "globe"),
            MobileNetwork::Smart(_) => write!(f, "smart"),
            MobileNetwork::Sun(_) => write!(f, "sun"),
            MobileNetwork::TNT(_) => write!(f, "tnt"),
            MobileNetwork::Dito(_) => write!(f, "dito"),
            MobileNetwork::Invalid(reason) => write!(f, "invalid ({})", reason),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mutate::*;

    use super::*;

    // Testing setup function that prepares test data and environment
    fn setup() {
        // Clear and set up each prefix list before each test
        reset_dito_prefixes();
        reset_globe_prefixes();
        reset_smart_prefixes();
        reset_sun_prefixes();
        reset_tnt_prefixes();

        append_dito_prefixes(&["0897", "0898"]);
        append_globe_prefixes(&["0917", "0918"]);
        append_smart_prefixes(&["0919", "0920"]);
        append_sun_prefixes(&["0922", "0923"]);
        append_tnt_prefixes(&["0930", "0938"]);
    }

    #[test]
    fn display_formats_are_correct() {
        setup();
        let globe = MobileNetwork::get("09171234567").unwrap();
        let smart = MobileNetwork::get("09191234567").unwrap();
        let sun = MobileNetwork::get("09221234567").unwrap();
        let tnt = MobileNetwork::get("09301234567").unwrap();
        let dito = MobileNetwork::get("08971234567").unwrap();
        let invalid = MobileNetwork::Invalid("test".to_string());

        assert_eq!(format!("{}", globe), "globe");
        assert_eq!(format!("{}", smart), "smart");
        assert_eq!(format!("{}", sun), "sun");
        assert_eq!(format!("{}", tnt), "tnt");
        assert_eq!(format!("{}", dito), "dito");
        assert_eq!(format!("{}", invalid), "invalid (test)");
    }

    #[test]
    fn validate_all_network() {
        setup();
        let globe = MobileNetwork::get("09171234567").unwrap();

        assert!(globe.validate("09171234567").is_ok());

        let smart = MobileNetwork::get("09191234567").unwrap();
        assert!(smart.validate("09191234567").is_ok());

        let sun = MobileNetwork::get("09221234567").unwrap();
        assert!(sun.validate("09221234567").is_ok());

        let tnt = MobileNetwork::get("09301234567").unwrap();
        assert!(tnt.validate("09301234567").is_ok());

        let dito = MobileNetwork::get("08971234567").unwrap();
        assert!(dito.validate("08971234567").is_ok());

        let invalid = MobileNetwork::Invalid("test".to_string());
        assert!(invalid.validate("any_number").is_err());
    }

    #[test]
    fn test_get_globe_network() {
        setup();
        let result = MobileNetwork::get("09171234567");
        assert!(matches!(result, Ok(MobileNetwork::Globe(_))));
    }

    #[test]
    fn test_get_smart_network() {
        setup();
        let result = MobileNetwork::get("09191234567");
        assert!(matches!(result, Ok(MobileNetwork::Smart(_))));
    }

    #[test]
    fn test_get_sun_network() {
        setup();
        let result = MobileNetwork::get("09221234567");
        assert!(matches!(result, Ok(MobileNetwork::Sun(_))));
    }

    #[test]
    fn test_get_tnt_network() {
        setup();
        let result = MobileNetwork::get("09301234567");
        assert!(matches!(result, Ok(MobileNetwork::TNT(_))));
    }

    #[test]
    fn test_get_dito_network() {
        setup();
        let result = MobileNetwork::get("08971234567");
        assert!(matches!(result, Ok(MobileNetwork::Dito(_))));
    }

    #[test]
    fn test_get_invalid_network() {
        setup();
        let result = MobileNetwork::get("09871234567");
        assert!(matches!(
            result,
            Err(MobileNetworkError::UnrecognizedPrefix(_))
        ));
    }
}
