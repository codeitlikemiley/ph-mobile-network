use std::fmt;

use crate::{
    dito::Dito, errors::MobileNetworkError, globals::{dito_prefixes::DITO_PREFIXES, globe_prefixes::GLOBE_PREFIXES, smart_prefixes::SMART_PREFIXES, sun_prefixes::SUN_PREFIXES, tnt_prefixes::TNT_PREFIXES}, globe::Globe, smart::Smart, sun::Sun, talk_n_text::TNT, validate::Validate
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
            MobileNetwork::Invalid(text) => Err(MobileNetworkError::UnrecognizedPrefix(text.clone())),
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
