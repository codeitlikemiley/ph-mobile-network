use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

pub(crate) mod dito;
pub mod errors;
pub(crate) mod pattern;
pub(crate) mod globe;
pub mod mobile_network;
pub(crate) mod smart;
pub(crate) mod sun;
pub(crate) mod talk_n_text;
pub mod validate;

lazy_static! {
    pub(crate) static ref GLOBE_PREFIXES: Arc<Mutex<Vec<&'static str>>> =
        Arc::new(Mutex::new(vec![
            "0817", "0904", "0905", "0906", "0915", "0916", "0917", "0926", "0927", "0935", "0936",
            "0937", "0945", "0954", "0955", "0956", "0965", "0966", "0967", "0975", "0976", "0977",
            "0978", "0979", "0995", "0996", "0997", "09173", "09175", "09176", "09178", "09253",
            "09255", "09256", "09257", "09258",
        ]));
}

pub fn append_globe_prefix(prefix: &'static str) {
    let mut prefixes = GLOBE_PREFIXES.lock().unwrap();
    prefixes.push(prefix);
}

lazy_static! {
    pub(crate) static ref DITO_PREFIXES: Arc<Mutex<Vec<&'static str>>> =
        Arc::new(Mutex::new(vec![
            "0895", "0896", "0897", "0898", "0991", "0992", "0993", "0994",
        ]));
}

pub fn append_dito_prefix(prefix: &'static str) {
    let mut prefixes = DITO_PREFIXES.lock().unwrap();
    prefixes.push(prefix);
}

lazy_static! {
    pub(crate) static ref SMART_PREFIXES: Arc<Mutex<Vec<&'static str>>> =
        Arc::new(Mutex::new(vec![
            "0813", "0908", "0911", "0913", "0914", "0919", "0920", "0921", "0928", "0929", "0939",
            "0946", "0947", "0949", "0951", "0961", "0963", "0968", "0969", "0970", "0981", "0998",
            "0999", "0960",
        ]));
}

pub fn append_smart_prefix(prefix: &'static str) {
    let mut prefixes = SMART_PREFIXES.lock().unwrap();
    prefixes.push(prefix);
}

lazy_static! {
    pub(crate) static ref SUN_PREFIXES: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![
        "0922", "0923", "0924", "0925", "0931", "0932", "0933", "0934", "0940", "0941", "0942",
        "0943", "0944", "0973", "0974",
    ]));
}

pub fn append_sun_prefix(prefix: &'static str) {
    let mut prefixes = SUN_PREFIXES.lock().unwrap();
    prefixes.push(prefix);
}

lazy_static! {
    pub(crate) static ref TNT_PREFIXES: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![
        "0907", "0909", "0910", "0912", "0918", "0930", "0938", "0946", "0948", "0950", "0963",
        "0989", "0998",
    ]));
}

pub fn append_tnt_prefix(prefix: &'static str) {
    let mut prefixes = TNT_PREFIXES.lock().unwrap();
    prefixes.push(prefix);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn test_thread_safety_on_prefixes(prefixes: Arc<Mutex<Vec<&'static str>>>, network_name: &str) {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let prefix = format!("099{}", i);
                let static_prefix: &'static str = Box::leak(prefix.into_boxed_str());
                let prefixes_clone = Arc::clone(&prefixes); // Clone the Arc for each thread
                thread::spawn(move || {
                    let mut locked_prefixes = prefixes_clone.lock().unwrap();
                    locked_prefixes.push(static_prefix);
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        // Verify that all appended prefixes are present
        let locked_prefixes = prefixes.lock().unwrap();
        for i in 0..10 {
            let expected_prefix = format!("099{}", i);
            assert!(
                locked_prefixes.contains(&expected_prefix.as_str()),
                "Prefix {} should be in the list for {} network",
                expected_prefix,
                network_name
            );
        }
    }

    #[test]
    fn test_append_dito_prefix() {
        // Clear existing prefixes for a clean test environment
        let mut prefixes = DITO_PREFIXES.lock().unwrap();
        prefixes.clear();
        prefixes.extend(vec!["0895", "0896"]); // Reset to a known state
        drop(prefixes); // Explicitly drop to release the lock

        // Append a new prefix and test
        append_dito_prefix("0991");
        let prefixes = DITO_PREFIXES.lock().unwrap();
        assert!(
            prefixes.contains(&"0991"),
            "Prefix 0991 should be in the list"
        );
    }

    #[test]
    fn test_append_globe_prefix() {
        // Clear existing prefixes for a clean test environment
        let mut prefixes = GLOBE_PREFIXES.lock().unwrap();
        prefixes.clear();
        prefixes.extend(vec!["0817", "0905"]); // Reset to a known state
        drop(prefixes); // Explicitly drop to release the lock

        // Append a new prefix and test
        append_globe_prefix("0912");
        let prefixes = GLOBE_PREFIXES.lock().unwrap();
        assert!(
            prefixes.contains(&"0912"),
            "Prefix 0912 should be in the list"
        );
    }

    #[test]
    fn test_append_smart_prefix() {
        // Clear existing prefixes for a clean test environment
        let mut prefixes = SMART_PREFIXES.lock().unwrap();
        prefixes.clear();
        prefixes.extend(vec!["0908", "0919"]); // Reset to a known state
        drop(prefixes); // Explicitly drop to release the lock

        // Append a new prefix and test
        append_smart_prefix("0999");
        let prefixes = SMART_PREFIXES.lock().unwrap();
        assert!(
            prefixes.contains(&"0999"),
            "Prefix 0999 should be in the list"
        );
    }

    #[test]
    fn test_append_sun_prefix() {
        // Clear existing prefixes for a clean test environment
        let mut prefixes = SUN_PREFIXES.lock().unwrap();
        prefixes.clear();
        prefixes.extend(vec!["0922", "0923"]); // Reset to a known state
        drop(prefixes); // Explicitly drop to release the lock

        // Append a new prefix and test
        append_sun_prefix("0943");
        let prefixes = SUN_PREFIXES.lock().unwrap();
        assert!(
            prefixes.contains(&"0943"),
            "Prefix 0943 should be in the list"
        );
    }

    #[test]
    fn test_append_tnt_prefix() {
        // Clear existing prefixes for a clean test environment
        let mut prefixes = TNT_PREFIXES.lock().unwrap();
        prefixes.clear();
        prefixes.extend(vec!["0907", "0912"]); // Reset to a known state
        drop(prefixes); // Explicitly drop to release the lock

        // Append a new prefix and test
        append_tnt_prefix("0989");
        let prefixes = TNT_PREFIXES.lock().unwrap();
        assert!(
            prefixes.contains(&"0989"),
            "Prefix 0989 should be in the list"
        );
    }

    #[test]
    fn test_thread_safety_on_globe_prefix() {
        test_thread_safety_on_prefixes(GLOBE_PREFIXES.clone(), "Globe");
    }

    #[test]
    fn test_thread_safety_on_dito_prefix() {
        test_thread_safety_on_prefixes(DITO_PREFIXES.clone(), "Dito");
    }

    #[test]
    fn test_thread_safety_on_smart_prefix() {
        test_thread_safety_on_prefixes(SMART_PREFIXES.clone(), "Smart");
    }

    #[test]
    fn test_thread_safety_on_sun_prefix() {
        test_thread_safety_on_prefixes(SUN_PREFIXES.clone(), "Sun");
    }

    #[test]
    fn test_thread_safety_on_tnt_prefix() {
        test_thread_safety_on_prefixes(TNT_PREFIXES.clone(), "TNT");
    }
}
