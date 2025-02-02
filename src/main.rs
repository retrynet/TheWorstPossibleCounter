use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let num = Arc::new(AtomicUsize::new(1));
    let ptr = Arc::clone(&num);

    let inconsistency_counter = Arc::new(AtomicUsize::new(0));
    let inconsistency_counter_clone = Arc::clone(&inconsistency_counter);

    for _ in 0..1000 {
        let ptr = Arc::clone(&ptr);
        let inconsistency_counter_clone = Arc::clone(&inconsistency_counter_clone);
        thread::spawn(move || {
            loop {
                let old_value = ptr.fetch_add(1, Ordering::SeqCst);
                let new_value = ptr.load(Ordering::SeqCst);
                if new_value <= old_value {
                    inconsistency_counter_clone.fetch_add(1, Ordering::SeqCst);
                }
            }
        });
    }

    loop {
        let current_value = num.load(Ordering::SeqCst);
        println!(
            "Address = {:p}, Value = {:?}, Inconsistencies = {}",
            num,
            current_value,
            inconsistency_counter.load(Ordering::SeqCst)
        );
    }
}
