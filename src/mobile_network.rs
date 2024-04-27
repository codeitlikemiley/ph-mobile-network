use crate::{
    dito::{Dito, DITO_PREFIXES}, errors::MobileNetworkError, globe::{Globe, GLOBE_PREFIXES}, smart::{Smart, SMART_PREFIXES}, sun::{Sun, SUN_PREFIXES}, talk_n_text::{TALK_N_TEXT_PREFIXES, TNT}, validate::Validate
};

pub  enum MobileNetwork {
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
        match prefix {
            p if GLOBE_PREFIXES.contains(&p) => Globe::new().map(MobileNetwork::Globe),
            p if SMART_PREFIXES.contains(&p) => Smart::new().map(MobileNetwork::Smart),
            p if SUN_PREFIXES.contains(&p) => Sun::new().map(MobileNetwork::Sun),
            p if TALK_N_TEXT_PREFIXES.contains(&p) => TNT::new().map(MobileNetwork::TNT),
            p if DITO_PREFIXES.contains(&p) => Dito::new().map(MobileNetwork::Dito),
            _ => Err(MobileNetworkError::UnrecognizedPrefix),
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
            MobileNetwork::Invalid(text) => Err(MobileNetworkError::RegexError(text.to_owned())),
        }
    }
}
