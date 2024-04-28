use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub(crate) static ref GLOBE_PREFIXES: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(vec![
        "0817", "0904", "0905", "0906", "0915", "0916", "0917", "0926", "0927", "0935", "0936",
        "0937", "0945", "0954", "0955", "0956", "0965", "0966", "0967", "0975", "0976", "0977",
        "0978", "0979", "0995", "0996", "0997", "09173", "09175", "09176", "09178", "09253",
        "09255", "09256", "09257", "09258",
    ]));
}

#[cfg(test)]
mod tests {
    use crate::mutate::append_globe_prefixes;

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
    fn test_append_globe_prefix() {
        // Clear existing prefixes for a clean test environment
        let mut prefixes = GLOBE_PREFIXES.lock().unwrap();
        prefixes.clear();
        prefixes.extend(vec!["0817", "0905"]); // Reset to a known state
        drop(prefixes); // Explicitly drop to release the lock

        // Append a new prefix and test
        append_globe_prefixes(&["0912"]);
        let prefixes = GLOBE_PREFIXES.lock().unwrap();
        assert!(
            prefixes.contains(&"0912"),
            "Prefix 0912 should be in the list"
        );
    }

    #[test]
    fn test_thread_safety_on_globe_prefix() {
        test_thread_safety_on_prefixes(GLOBE_PREFIXES.clone(), "Globe");
    }
}
