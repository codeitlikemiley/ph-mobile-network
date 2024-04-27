use crate::{DITO_PREFIXES, GLOBE_PREFIXES, SMART_PREFIXES, SUN_PREFIXES, TNT_PREFIXES};

pub fn append_globe_prefix(prefix: &'static str) {
    let mut prefixes = GLOBE_PREFIXES.lock().unwrap();
    prefixes.push(prefix);
}

pub fn append_dito_prefix(prefix: &'static str) {
    let mut prefixes = DITO_PREFIXES.lock().unwrap();
    prefixes.push(prefix);
}

pub fn append_smart_prefix(prefix: &'static str) {
    let mut prefixes = SMART_PREFIXES.lock().unwrap();
    prefixes.push(prefix);
}

pub fn append_sun_prefix(prefix: &'static str) {
    let mut prefixes = SUN_PREFIXES.lock().unwrap();
    prefixes.push(prefix);
}

pub fn append_tnt_prefix(prefix: &'static str) {
    let mut prefixes = TNT_PREFIXES.lock().unwrap();
    prefixes.push(prefix);
}

