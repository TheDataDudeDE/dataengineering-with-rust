/*

// Mutex that protects the data vector, and then we spawn three threads 
//that each acquire a lock on the mutex and modify an element of the vector.
*/


use std::sync::{RwLock, Arc};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let mut handles = vec![];

    for i in 0..3 {
        let data = data.clone();
        let handle = thread::spawn(move || {
            let mut data = data.write().unwrap();
            data[i] += 1;
        });
        handles.push(handle)
    };

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data);
}
