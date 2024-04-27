use std::fmt::Debug;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test1() {
    let (tx1,rx) = mpsc::channel();
    let tx2 = tx1.clone();
    let handle1 = thread::spawn(move || {
        let vec = vec![1, 2, 3];
        for item in vec {
            tx1.send(item).expect("something is wrong in tx1");
            thread::sleep(Duration::from_secs(1));
        }
    });
    let handle2 = thread::spawn(move || {
        let vec = vec![1, 2, 3];
        for item in vec {
            tx2.send(item).expect("something is wrong in tx2");
            thread::sleep(Duration::from_secs(1));
        }
    });
    handle1.join();
    handle2.join();
    for re in rx {
        println!("got {re}")
    }
}

#[test]
fn test2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}