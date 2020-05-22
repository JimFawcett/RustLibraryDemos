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
use std::cell::*;

use std::sync::{Arc, Mutex, Condvar, LockResult};
use std::thread;
use std::collections::*;

struct BlockingQueue<T:Clone> { lq:Mutex<VecDeque<T>>, cv:Condvar, }
//unsafe impl<T:Clone> Sync for BlockingQueue<T> {}
unsafe impl<T:Clone> Send for BlockingQueue<T> {}
impl<T:Clone> BlockingQueue<T> {
    pub fn new() -> BlockingQueue<T> {
        Self {
            lq:Mutex::new(VecDeque::new()),
            cv:Condvar::new(),
        }
    }
    // pub fn clone(&self) -> BlockingQueue<T> {
    //     Self {

    //     }
    // }
    pub fn enQ(&mut self, t:T) /*-> std::sync::LockResult<()>*/ {
        self.lq.lock().unwrap().push_back(t);
        // let result = self.q.lock();
        // match result {
        //     Ok(q) => { q.push_back(t); std::sync::LockResult::Ok(()) },
        //     Err(error) => Err(std::sync::PoisonError(error)),
        // }
    }
    pub fn deQ(&mut self) -> T {
        let mut q = self.lq.lock().unwrap();
                
        /*-- block on empty queue --*/
        while q.len() == 0 {  // may get spurious returns
            q = self.cv.wait(q).unwrap();
        }
        /*-- dequeue and display message --*/
        q.pop_front().unwrap()
    }   // lock released
    pub fn len(&self) -> usize {
        self.lq.lock().unwrap().len()
    }
}
pub struct Sender<T:Clone> {
    pub inner : Arc<UnsafeCell<BlockingQueue<T>>>,
}
unsafe impl<T:Clone> Send for Sender<T> {}
impl<T:Clone + Send> Sender<T> {
    pub fn new() -> Sender<T> {
        Sender { inner: Arc::new(UnsafeCell::new(BlockingQueue::new())), }
    }
    pub fn clone(&self) ->Sender<T> {
        Sender {
            inner: self.inner.clone()
        }
    }
}
pub struct Receiver<T:Clone> {
    inner: Arc<UnsafeCell<BlockingQueue<T>>>,
}
unsafe impl<T:Clone> Send for Receiver<T> {}
impl<T:Clone + Send> Receiver<T> {
    pub fn new() -> Receiver<T> {
        Receiver { inner: Arc::new(UnsafeCell::new(BlockingQueue::new())), }
    }
}



fn new_demo() {
    // let mut shared1 = Arc::new(UnsafeCell::new(BlockingQueue::<String>::new()));
    // let mut shared2 = shared1.clone();
    let data = Sender::<String>::new();
    let shared1:Sender<String>;
    shared1.inner = Arc::clone(&data.inner);

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
        //unsafe {
            loop {
                let msg:String;
                unsafe {
                    msg = shared1.inner.into_inner().deQ();
                }
                print!("\n  dequeued {:?} on child thread", msg);
                if msg == "quit".to_string() {
                    break;
                }
            }
        //}
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
    // unsafe {
    //     let bq = shared1;
    // }
    let mut not_processed = 0;

    let max = 5;
    for i in 0..max {
        let mut value:String;
        if i < max-1 {
          value = String::from("msg #");
          value.push_str(&i.to_string());
        }
        else {
            value = "quit".to_string();
        }
        print!("\n  enqueue  {:?} on main thread", &value);
        unsafe {
            shared2.inner.into_inner().enQ(value);
        }
        // bq.enQ(value);
    }
    /*---------------------------------------------
        Make sure all queued items are processed:
        - Needed because notifies that are issued
          before thread starts are dropped.
    */
    // for _i in 0..not_processed {
    //     cvar.notify_one();
    // }

    print!("\n  waiting for child thread to finish");
    //let _ = handle.join();

}
// fn demo()  {

//     // use std::sync::{Arc, Mutex, Condvar};
//     // use std::thread;
//     // use std::collections::*;
    
//     /*---------------------------------------------
//         shared queue and condition variable 
//         - condition variable makes thread wait for
//           queue to have at least one entry
//         - It is the reason queue blocks dequeuer
//           when empty.
//         - Arcs are thread-safe pointers, so both
//           shared1 and its clone, shared2, refer to 
//           the same locks and queue.
//     */
//     let shared1 = Arc::new((
//         Mutex::new(VecDeque::<String>::new()), 
//         Condvar::new(),
//     ));
//     let shared2 = shared1.clone();
    
//     /*---------------------------------------------
//         Atomic used to poll for active thread
//     */
//     let thread_started = Arc::new(AtomicBool::new(false));
//     let checker = thread_started.clone();

//     /*---------------------------------------------
//         Start child thread
//         - Dequeues messages sent by main thread.
//     */
//     let handle = thread::spawn(move|| {

//         print!("\n  thread started");
//         thread_started.store(true, Ordering::SeqCst);
//         let _time_delay = time::Duration::from_millis(55);
//         let (lq, cvar) = &*shared2;
//         loop {
//             //thread::sleep(_time_delay);
//             let item: String;
//             {
//                 /*-- acquire lock --*/
//                 let mut q = lq.lock().unwrap();
                
//                 /*-- block on empty queue --*/
//                 while q.len() == 0 {  // may get spurious returns
//                     q = cvar.wait(q).unwrap();
//                 }
//                 /*-- dequeue and display message --*/
//                 item = q.pop_front().unwrap();
//             }   // lock released
            
//             print!("\n  dequeued {:?} on child thread", item);
//             let _ = std::io::stdout().flush();
            
//             /*-- client shuts down dequeuer with quit msg --*/
//             if item == "quit" {
//                 break;
//             }
//         }
    
//         print!("\n  thread finishing");
//     });

//     /*---------------------------------------------
//         Main thread thread enqueues messages
//         for child thread.
//     */
//     /*-- wait for child thread to start ---------*/
//     let _time_delay = time::Duration::from_micros(10);

//     while !checker.load(Ordering::SeqCst) {
//         thread::sleep(_time_delay);
//     }
//     /*-- start sending messages --*/
//     let (lq, cvar) = &*shared1;
//     let mut not_processed = 0;

//     let max = 5;
//     for i in 0..max {
//         let mut value:String;
//         if i < max-1 {
//           value = String::from("msg #");
//           value.push_str(&i.to_string());
//         }
//         else {
//             value = "quit".to_string();
//         }
//         print!("\n  enqueue  {:?} on main thread", &value);
//         {
//             let mut q = lq.lock().unwrap();
//             q.push_back(value);
//             not_processed = q.len();
//         }
//         cvar.notify_one();    
//     }
//     /*---------------------------------------------
//         Make sure all queued items are processed:
//         - Needed because notifies that are issued
//           before thread starts are dropped.
//     */
//     for _i in 0..not_processed {
//         cvar.notify_one();
//     }

//     print!("\n  waiting for child thread to finish");
//     let _ = handle.join();
// }

fn main() {

    print!("\n  Demonstrate blocking queue shared between threads");
    print!("\n ===================================================");

    new_demo();
    print!("\n\n  That's all Folks!\n");
}