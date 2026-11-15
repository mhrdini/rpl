/* -------------------------------------------------------------------------- */
/* Concurrent + Parallel programming                                          */
/* -------------------------------------------------------------------------- */
// concurrent -> different parts of program execute independently
// parallel -> different parts of program execute at same time
// "do not communicate by sharing memory; instead, share memory by communicating"

/* -------------------------------------------------------------------------- */
/* Threads: Run code simultaneously                                           */
/* -------------------------------------------------------------------------- */
// - program code -> is run in process
// - OS -> manages multiple processes at once
// - within a program -> can have simultaneous threads that run independently
//   e.g. web server with multiple threads to serve multiple requests at once

// more threads -> better performance + higher complexity
// potential problems of threads:
// (1) race conditions -> threads accessing data/resources in an inconsistent
// order
// (2) deadlocks -> 2 threads waiting for each other, preventing either from
// continuing
// (3) bugs that are difficult to reproduce and therefore fix

// rust standard library uses 1:1 model of thread implementation -> a program
// uses 1 OS thread per 1 language thread
// other thread implementation models exist with different trade-offs

/* -------------------------------------------------------------------------- */
/* Some ways to achieve concurrency in Rust                                   */
/* -------------------------------------------------------------------------- */
// - Message-passing concurrency -> channels send message between threads
// - Shared-state concurrency -> multiple threads have access to some piece of
//   data
// - Sync and Send traits -> to extend user-defined types and types provided by
//   standard library with Rust's concurrency guarantees

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

/* -------------------------------------------------------------------------- */
/* Extensible concurrency with Send and Sync traits                           */
/* -------------------------------------------------------------------------- */
// - from std::marker

// - Send marker trait -> indicates that ownership of values of type can be
//   transferred between threads
//   - is: almost every Rust type
//   - not: Rc<T>

// - Sync marker trait -> indicates that it's safe for type to be referenced
//   from multiple threads
//   - is: almost every Rust type, Mutex<T>
//   - not: Rc<T>, RefCell<T> and all related Cell<T> type

// - any type T implements Sync if &T (immutable ref to T) implements Send

fn main() {
    /* ---------------------------------------------------------------------- */
    /* Creating a new thread with spawn                                       */
    /* ---------------------------------------------------------------------- */
    // - call thread::spawn and pass it a closure containing code to run in new
    //   thread
    // - call thread::sleep to force thread to stop its execution for specified
    //   duration, allowing a different thread to run

    // no guarantee on order, no guarantee that spawned thread gets to run at
    // all

    // solution: to guarantee run to completion:
    // - declare variable to save JoinHandle<T>,
    // - call join on JoinHandle<T> variable to block threads currently running
    //   until thread represented by the variable terminates

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    /* ---------------------------------------------------------------------- */
    /* Using move closures with threads                                       */
    /* ---------------------------------------------------------------------- */
    // - move keywords to force a closure take ownership of values it uses from
    //   environment, therefore transferring ownership of those values from one
    //   thread to another

    let v = vec![1, 2, 3]; // owned value in main thread

    // what's happening:
    // - closure uses v, so it will capture v and make it part of closure's
    //   environment
    // - thread::spawn runs closure in new thread,
    // - println! only needs a reference to v, so closure tries to borrow v
    // - but Rust can't tell how long
    //   the spawned thread will run, so it doesn't know whether the reference
    //   to v will always be valid for as long as the thread runs or not

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    // drop(v); // can't be done anymore here, because its ownership was moved into closure
    handle.join().unwrap();

    /* ---------------------------------------------------------------------- */
    /* Message-passing: Transfer data between threads for safe concurrency    */
    /* ---------------------------------------------------------------------- */
    // - for message-sending concurrency -> use channels
    // - channel -> a means to send data from one thread to another
    // - each channel has two halves: (tx, rx)
    //   (1) transmitter -> upstream, its methods called with data to be sent
    //   (2) receiver -> downstream, its methods called to check for arriving
    //   messages
    // - a channel is closed if either transmitter or receiver half is dropped

    /* ---------------------------------------------------------------------- */
    /* Using mpsc: Multiple producer, single consumer                         */
    /* ---------------------------------------------------------------------- */
    // in standard library, channels can have multiple sending ends that produce
    // values but only one receiving end that consumes values

    let (tx, rx) = mpsc::channel();

    // clone() to create another producer
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // move tx into spawned thread and have it send one string
        // move val by sending it
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle = thread::spawn(move || {
        // let received = rx.recv().unwrap();
        for received in rx {
            println!("Got: {received}");
        }
    });

    handle.join().unwrap();

    /* ---------------------------------------------------------------------- */
    /* Shared-state concurrency: Using mutexes                                */
    /* ---------------------------------------------------------------------- */
    // - using mutexes so multiple threads accessing same memory location at
    //   "same time"

    // - mutex -> mutual exclusion = allows only one ethread to access some data
    //   at any given time
    // - lock -> data structure part of mutex that keeps track of who currently
    //   has exclusive access to data
    // - therefore mutex guards data via locking system

    // 2 rules for using mutexes:
    // (1) thread must attempt to acquire the mutex lock before using data within mutex
    // (2) when thread is done with data that mutex guards, it must unlock the
    //     data so that other threads can acquire the lock

    /* ---------------------------------------------------------------------- */
    /* Mutex<T>                                                               */
    /* ---------------------------------------------------------------------- */
    // immutable reference, but we can get a mutable reference to the value
    // inside it -> interior mutability
    let m = Mutex::new(5);

    // use Mutex::lock method to acquire lock
    // - will fail if another thread holding lock panics
    // - if acquired, return value similar to mutable reference to data inside
    //   -> MutexGuard (implements Deref to point to inner data, and Drop that
    //   releases lock automatically when MutexGuard goes out of scope)
    //   -> wrapped in a LockResult (dealt with unwrap)
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");

    /* ---------------------------------------------------------------------- */
    /* Arc<T>: Atomic reference counting                                      */
    /* ---------------------------------------------------------------------- */
    // 1st problem: want to have multiple threads access the Mutex counter
    // 2nd problem: usually use Rc<T> to have multiple ownership, but Rc<T>
    // is not safe to share across threads
    // - why unsafe: by way of not having the concurrency primitives needed to
    //   make sure changes to the reference count can't be interrupted by
    //   another thread -> potential wrong counts -> bugs that could lead to
    //   memory leaks or a value being dropped before we're done with it

    // solution: Arc<T> from std::sync
    // - like a thread-safe Rc<T>, keeps track of # of references pointing to it
    // - but using ATOMICS (see std::sync::atomic module), which are THREAD-SAFE
    // - call Arc::clone to increment the strong count

    /* Interior mutability                                                    */
    // just as a RefCell<T> allows us to mutate contents inside Rc<T>,
    // we use Mutex<T> to mutate contents inside Arc<T>

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // create 10 threads
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
