use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
};

fn without_mutex() -> usize {
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let atomic_counter = Arc::clone(&atomic_counter);
        handles.push(thread::spawn(move || {
            atomic_counter.fetch_add(1, Ordering::Relaxed);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    atomic_counter.load(Ordering::Relaxed)
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(*counter.lock().unwrap(), 10);
    let counter = without_mutex();
    assert_eq!(counter, 10)
}
