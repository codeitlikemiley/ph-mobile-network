use crate::globals::{
    dito_prefixes::DITO_PREFIXES, globe_prefixes::GLOBE_PREFIXES, smart_prefixes::SMART_PREFIXES,
    sun_prefixes::SUN_PREFIXES, tnt_prefixes::TNT_PREFIXES,
};

pub fn append_globe_prefixes(prefixes: &[&'static str]) {
    let mut globe_prefixes = GLOBE_PREFIXES.lock().unwrap();
    globe_prefixes.extend_from_slice(prefixes);
}

pub fn append_dito_prefixes(prefixes: &[&'static str]) {
    let mut dito_prefixes = DITO_PREFIXES.lock().unwrap();
    dito_prefixes.extend_from_slice(prefixes);
}

pub fn append_smart_prefixes(prefixes: &[&'static str]) {
    let mut smart_prefixes = SMART_PREFIXES.lock().unwrap();
    smart_prefixes.extend_from_slice(prefixes);
}

pub fn append_sun_prefixes(prefixes: &[&'static str]) {
    let mut sun_prefixes = SUN_PREFIXES.lock().unwrap();
    sun_prefixes.extend_from_slice(prefixes);
}

pub fn append_tnt_prefixes(prefixes: &[&'static str]) {
    let mut tnt_prefixes = TNT_PREFIXES.lock().unwrap();
    tnt_prefixes.extend_from_slice(prefixes);
}

pub fn reset_globe_prefixes() {
    let mut globe_prefixes = GLOBE_PREFIXES.lock().unwrap();
    globe_prefixes.clear();
}

pub fn reset_dito_prefixes() {
    let mut dito_prefixes = DITO_PREFIXES.lock().unwrap();
    dito_prefixes.clear();
}

pub fn reset_smart_prefixes() {
    let mut smart_prefixes = SMART_PREFIXES.lock().unwrap();
    smart_prefixes.clear();
}

pub fn reset_sun_prefixes() {
    let mut sun_prefixes = SUN_PREFIXES.lock().unwrap();
    sun_prefixes.clear();
}

pub fn reset_tnt_prefixes() {
    let mut tnt_prefixes = TNT_PREFIXES.lock().unwrap();
    tnt_prefixes.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        reset_globe_prefixes();
        reset_dito_prefixes();
        reset_smart_prefixes();
        reset_sun_prefixes();
        reset_tnt_prefixes();
    }

    #[test]
    fn reset_dito_prefixes_should_return_empty() {
        reset_dito_prefixes();
        let prefixes = DITO_PREFIXES.lock().unwrap();
        assert!(prefixes.is_empty());
    }

    #[test]
    fn reset_globe_prefixes_should_return_empty() {
        reset_globe_prefixes();
        let prefixes = GLOBE_PREFIXES.lock().unwrap();
        assert!(prefixes.is_empty());
    }

    #[test]
    fn reset_smart_prefixes_should_return_empty() {
        reset_smart_prefixes();
        let prefixes = SMART_PREFIXES.lock().unwrap();
        assert!(prefixes.is_empty());
    }

    #[test]
    fn reset_sun_prefixes_should_return_empty() {
        reset_sun_prefixes();
        let prefixes = SUN_PREFIXES.lock().unwrap();
        assert!(prefixes.is_empty());
    }

    #[test]
    fn reset_tnt_prefixes_should_return_empty() {
        setup();
        let prefixes = TNT_PREFIXES.lock().unwrap();
        assert!(prefixes.is_empty());
    }

    #[test]
    fn append_globe_prefixes_should_contain_prefixes() {
        setup();
        append_globe_prefixes(&["0917", "0925"]);
        let prefixes = GLOBE_PREFIXES.lock().unwrap();
        assert!(prefixes.len() == 2);
        assert!(prefixes.contains(&"0917") && prefixes.contains(&"0925"));
    }

    #[test]
    fn append_dito_prefixes_should_contain_prefixes() {
        setup();
        append_dito_prefixes(&["0897", "0898"]);
        let prefixes = DITO_PREFIXES.lock().unwrap();
        assert!(prefixes.len() == 2);
        assert!(prefixes.contains(&"0897") && prefixes.contains(&"0898"));
    }

    #[test]
    fn append_smart_prefixes_should_contain_prefixes() {
        setup();
        append_smart_prefixes(&["0912", "0918"]);
        let prefixes = SMART_PREFIXES.lock().unwrap();
        assert!(prefixes.len() == 2);
        assert!(prefixes.contains(&"0912") && prefixes.contains(&"0918"));
    }

    #[test]
    fn append_sun_prefixes_should_contain_prefixes() {
        setup();
        append_sun_prefixes(&["0922", "0933"]);
        let prefixes = SUN_PREFIXES.lock().unwrap();
        assert!(prefixes.len() == 2);
        assert!(prefixes.contains(&"0922") && prefixes.contains(&"0933"));
    }

    #[test]
    fn append_tnt_prefixes_should_contain_prefixes() {
        setup();
        append_tnt_prefixes(&["0910", "0911"]);
        let prefixes = TNT_PREFIXES.lock().unwrap();
        assert!(prefixes.len() == 2);
        assert!(prefixes.contains(&"0910") && prefixes.contains(&"0911"));
    }
}
