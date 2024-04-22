use std::thread;
use std::time::Duration;

fn main() {
    println!(" ");
    println!("Tutorial 20 - Concurrency + Threads ");
    println!("------------------------------------");

    // Create a thread with spawn
    
    thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);

            // Forces thread to sleep and allow another thread to execute
            thread::sleep(Duration::from_millis(1));
        }
    });

    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread (thread1) : {}", i);

            // Forces thread to sleep and allow another thread to execute
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread1.join().unwrap();
}
