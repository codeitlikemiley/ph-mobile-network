use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub(crate) static ref SMART_PREFIXES: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![
        "0813", "0908", "0911", "0913", "0914", "0919", "0920", "0921", "0928", "0929", "0939",
        "0946", "0947", "0949", "0951", "0961", "0963", "0968", "0969", "0970", "0981", "0998",
        "0999", "0960",
    ]));
}

#[cfg(test)]
mod tests {
    use crate::mutate::append_smart_prefixes;

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
    fn test_append_smart_prefix() {
        // Clear existing prefixes for a clean test environment
        let mut prefixes = SMART_PREFIXES.lock().unwrap();
        prefixes.clear();
        prefixes.extend(vec!["0908", "0919"]); // Reset to a known state
        drop(prefixes); // Explicitly drop to release the lock

        // Append a new prefix and test
        append_smart_prefixes(&["0999"]);
        let prefixes = SMART_PREFIXES.lock().unwrap();
        assert!(
            prefixes.contains(&"0999"),
            "Prefix 0999 should be in the list"
        );
    }

    #[test]
    fn test_thread_safety_on_smart_prefix() {
        test_thread_safety_on_prefixes(SMART_PREFIXES.clone(), "Smart");
    }
}
