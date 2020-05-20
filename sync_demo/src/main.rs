/////////////////////////////////////////////////////////////
// sync_demo::main.rs - BlockingQueue                      //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 19 May 2020  //
/////////////////////////////////////////////////////////////
/*
   This is a prototype for a blocking queue.  There is
   another alternative, based on Rust channels which are,
   essentially blocking queues.

   I will build a prototype from channels, and then make
   a decision about which to use for a final design.
*/
#![allow(clippy::mutex_atomic)]
use std::io::*;
use std::{time};
use std::sync::atomic::*;

fn demo()  {

    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;
    use std::collections::*;
    
    /*---------------------------------------------
        shared queue and condition variable 
        - condition variable makes thread wait for
          queue to have at least one entry
        - It is the reason queue blocks dequeuer
          when empty.
        - Arcs are thread-safe pointers, so both
          shared1 and its clone, shared2, refer to 
          the same locks and queue.
    */
    let shared1 = Arc::new((
        Mutex::new(false),  
        Condvar::new(),
        Mutex::new(VecDeque::<String>::new()), 
    ));
    let shared2 = shared1.clone();
    
    /*---------------------------------------------
        Atomic used to poll for active thread
    */
    let thread_started = Arc::new(AtomicBool::new(false));
    let checker = thread_started.clone();

    /*---------------------------------------------
        Start child thread
        - Dequeues messages sent by main thread.
    */
    let handle = thread::spawn(move|| {

        print!("\n  thread started");
        thread_started.store(true, Ordering::SeqCst);
        let _time_delay = time::Duration::from_millis(55);
        let (lock, cvar, lq) = &*shared2;
        loop {
            //thread::sleep(_time_delay);
            let mut _started = lock.lock().unwrap();
            
            /*-- block on empty queue --*/
            _started = cvar.wait(_started).unwrap();

            let mut q = lq.lock().unwrap();
            if q.len() == 0 {
                continue;  // spurious return
            }
            /*-- dequeue and display message --*/
            let item = q.pop_front().unwrap();
            print!("\n  dequeued {:?} on child thread", item);
            let _ = std::io::stdout().flush();
            
            /*-- client shuts down dequeuer with quit msg --*/
            if item == "quit" {
                break;
            }
        }
    
        print!("\n  thread finishing");
    });

    /*---------------------------------------------
        Main thread thread enqueues messages
        for child thread.
    */
    /*-- wait for child thread to start ---------*/
    let _time_delay = time::Duration::from_micros(10);

    while !checker.load(Ordering::SeqCst) {
        thread::sleep(_time_delay);
    }
    /*-- start sending messages --*/
    let (lock, cvar, lq) = &*shared1;
    let mut not_processed = 0;

    for i in 0..5 {
        let mut value:String;
        if i < 4 {
          value = String::from("msg #");
          value.push_str(&i.to_string());
        }
        else {
            value = "quit".to_string();
        }
        print!("\n  enqueue  {:?} on main thread", &value);
        {
            let _notify_lock = lock.lock().unwrap();
            let mut q = lq.lock().unwrap();
            q.push_back(value);
            not_processed = q.len();
        }
        cvar.notify_one();    
    }
    /*---------------------------------------------
        Make sure all queued items are processed:
        - Needed because notifies that are issued
          before thread starts are dropped.
    */
    for _i in 0..not_processed {
        cvar.notify_one();
    }

    print!("\n  waiting for child thread to finish");
    let _ = handle.join();
}

fn main() {

    print!("\n  Demonstrating queue shared between threads");
    print!("\n ============================================");

    demo();
    print!("\n\n  That's all Folks!\n");
}