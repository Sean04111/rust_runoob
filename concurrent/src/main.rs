// this is the parallel in rust =D

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicUsize, Ordering};

fn main(){
    // arc_data();
    // condvar_sync();
    // poisoned_mutex();
    // mutex_data();
    // atomic();
    CAS();
}


fn standard_thread(){
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("print in thread: {}",i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 0..5{
        println!("print in main thread : {}",i);
        thread::sleep(Duration::from_secs(1));
    }

    // join() is used to wait for sub thread to return

    handle.join().unwrap();
}

// atomic reference counting
fn arc_data(){
    // Arc stands for  Atomic Reference  Counting
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..5{
        // if an automatic reference counter need to be used in another thread
        // it needs to be cloned , not just use it !
        let counter = Arc::clone(&counter);

        // move here is used to move the ownership of variables
        let handle = thread::spawn(move || {
            // lock here is like mutex.lock()
            // and it doesn't need unlock cause Mutex will auto unlock when over the closure
            let mut num = counter.lock().unwrap();
            println!("thread counter reference counting add ,now is {} at thread {:?}", *num, thread::current().id());
            *num += 1;

        });

        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("final reference count is {}", *counter.lock().unwrap());
}

// cond variable for thread sync
fn condvar_sync(){
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let pair_clone = Arc::clone(&pair);
    thread::spawn(move || {
        let (lock, cvar) = &(*pair_clone);
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &(*pair);
    let mut started = lock.lock().unwrap();
    while !*started{
        started = cvar.wait(started).unwrap();
    }

    println!("finish thread sync!");
}

// so if a mutex was locked and not unlock , it will get into a poisoned state
fn poisoned_mutex (){
    let shared_data = Arc::new(Mutex::new(0));

    let shared_data_clone = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        let mut data = shared_data_clone.lock().unwrap();
        *data += 1;
        panic!("hell no");
    });

    handle.join().unwrap();

    match shared_data.lock(){
        Ok(_) => println!("lock ok"),
        Err(poisoned) => {
            println!("lock is poisoned");

            let value = poisoned.into_inner();
            println!("value is {}", value);
        }
    };
}


// use mutex to protect complex data

#[derive(Debug)]
struct SharedData {
    count:usize,
    message: String,
}

impl SharedData{
    fn new(count:usize, message: &str) -> SharedData{
        SharedData{
            count,
            message:message.to_owned(),
        }
    }

    fn increment_count(&mut self){
        self.count += 1;
    }

    fn update_message(&mut self, new_msg: &str) {
        self.message = new_msg.to_owned();
    }
}

fn mutex_data(){
    let shared_data = Arc::new(Mutex::new(SharedData::new(0,"msg")));


    let mut handlers = vec![];

    for i in 0..5{
        let cloned_shared_data = Arc::clone(&shared_data);
        let handler = thread::spawn(move ||{
            let mut data = cloned_shared_data.lock().unwrap();
            data.increment_count();
            data.update_message("goodbye");
        });

        handlers.push(handler);
    }

    for handle in handlers{
        handle.join().unwrap();
    }

    let final_data = shared_data.lock().unwrap();
    println!("{:#?}", final_data);
}

// atomics
fn atomic(){
    let counter = Arc::new(AtomicUsize::new(0));

    let mut handlers = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move ||{
           counter.fetch_add(1,Ordering::Relaxed);
        });
        handlers.push(handler);
    }

    for handler in handlers{
        handler.join().unwrap();
    }

    println!("final counter is {}", counter.load(Ordering::Relaxed));
}

fn CAS(){
    let value = AtomicUsize::new(0);
    let mut current = value.load(Ordering::Relaxed);
    while value.compare_exchange(current,10, Ordering::Relaxed,Ordering::Relaxed).is_err(){
        current = value.load(Ordering::Relaxed);
    }
    println!("success");
}