// arc1.rs
// Make this code compile by filling in a value for `shared_numbers` where the
// TODO comment is and create an initial binding for `child_numbers`
// somewhere. Try not to create any copies of the `numbers` Vec!
// Execute `rustlings hint arc1` for hints :)

// Definitions
//
// std::sync::Arc
// pub struct Arc<T: ?Sized> { /* fields omitted */ }
// A thread-safe reference-counting pointer. 'Arc' stands for 'Atomically Reference Counted'.
// The type Arc<T> provides shared ownership of a value of type T, allocated in the heap. Invoking clone on Arc produces a new Arc instance, which points to the same allocation on the heap as the source Arc, while increasing a reference count. When the last Arc pointer to a given allocation is destroyed, the value stored in that allocation (often referred to as "inner value") is also dropped.
//
// std::sync::Mutex
// pub struct Mutex<T: ?Sized> { /* fields omitted */ }
// A mutual exclusion primitive useful for protecting shared data

// This mutex will block threads waiting for the lock to become available. The mutex can also be statically initialized or created via a new constructor. Each mutex has a type parameter which represents the data that it is protecting. The data can only be accessed through the RAII guards returned from lock and try_lock, which guarantees that the data is only ever accessed when the mutex is locked.

//

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    // main solution - relies on Arc type
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers); // TODO
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let shared_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            let mut child_numbers = shared_numbers;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 8;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
    another();
}

// Another solution based on Mutex and Arc
use std::sync::Mutex;
fn another() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(Mutex::new(numbers)); // TODO
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let shared_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            let mut child_numbers = shared_numbers.lock().unwrap();
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 8;
            }
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
