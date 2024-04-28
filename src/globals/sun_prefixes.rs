use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub(crate) static ref SUN_PREFIXES: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![
        "0922", "0923", "0924", "0925", "0931", "0932", "0933", "0934", "0940", "0941", "0942",
        "0943", "0944", "0973", "0974",
    ]));
}

#[cfg(test)]
mod tests {
    use crate::mutate::append_sun_prefixes;

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
    fn test_append_sun_prefix() {
        // Clear existing prefixes for a clean test environment
        let mut prefixes = SUN_PREFIXES.lock().unwrap();
        prefixes.clear();
        prefixes.extend(vec!["0922", "0923"]); // Reset to a known state
        drop(prefixes); // Explicitly drop to release the lock

        // Append a new prefix and test
        append_sun_prefixes(&["0943"]);
        let prefixes = SUN_PREFIXES.lock().unwrap();
        assert!(
            prefixes.contains(&"0943"),
            "Prefix 0943 should be in the list"
        );
    }

    #[test]
    fn test_thread_safety_on_sun_prefix() {
        test_thread_safety_on_prefixes(SUN_PREFIXES.clone(), "Sun");
    }
}
