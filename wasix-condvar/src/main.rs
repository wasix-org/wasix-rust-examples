use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

fn main() {
    // Inside of our lock, spawn a new thread, and then wait for it to start.
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    // We enter a lock
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();

    println!("condvar-secondary thread spawn");
    {
        let pair = Arc::clone(&pair);
        thread::spawn(move || {
            {
                println!("condvar-secondary thread started");
                let (lock, cvar) = &*pair;
                println!("condvar-secondary thread sleep(1sec) start");
                thread::sleep(Duration::from_secs(1));
                println!("condvar-secondary thread sleep(1sec) end");
                let mut started = lock.lock().unwrap();
                *started = true;
                println!("condvar-secondary thread set condition");
                // We notify the condvar that the value has changed.
                cvar.notify_one();
                println!("condvar-secondary thread notify");
            }
            thread::sleep(Duration::from_millis(50));
            println!("condvar-secondary thread exit");
        });
    }
    thread::sleep(Duration::from_millis(100));

    // Wait for the thread to start up.
    println!("condvar-main loop");
    while !*started {
        println!("condvar-main wait");
        started = cvar.wait(started).unwrap();
        println!("condvar-main woken");
    }
    println!("condvar-main parent done");
    thread::sleep(Duration::from_millis(100));

    println!("all done");
}
