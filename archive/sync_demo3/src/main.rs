// sync_demo::main.rs - BlockingQueue

#![allow(unused_imports)]
use std::sync::*;
use std::thread;
use std::collections::*;
use std::cell::{UnsafeCell};

#[derive(Debug)]
pub struct BQ<T:Clone> { 
    pub lock:Mutex<bool>, 
    pub cv:Condvar, 
    pub q:std::cell::UnsafeCell<VecDeque<T>>, 
}
unsafe impl<T:Clone> Sync for BQ<T> {}
// impl<T:Clone> Clone for BQ<T> {
//     fn clone(&self) -> BQ<T> {
//         BQ {
//             lock:Mutex::new(false),
//             cv: self.cv,
//             q: self.q
//         }
//     }
// }
impl<T:Clone> BQ<T> {
    pub fn new() -> BQ<T> {
        BQ {
            lock: Mutex::new(false),
            cv: Condvar::new(),
            q: UnsafeCell::new(VecDeque::<T>::new()),
        }
    }
}

pub fn test() -> std::thread::JoinHandle<()> {

  let tup1 = BQ::<String>::new();
  let tup2 = &'static tup1;

  let handle = thread::spawn(move || {
    let lock = tup2.lock;
    let cv = tup2.cv;
    let q = tup2.q;
    let mut _started = lock.lock().unwrap();
    // loop {
    //     if q.into_inner().len() > 0 {
    //         let item = q.into_inner().pop_front().unwrap();
    //         print!("\n  dequeued {:?}", item);
    //     }
    // }
    *_started = true;
    cv.notify_one();
  });

  let lock = tup1.lock;
  let cv = tup1.cv;
  let _q = tup1.q;
  let mut started = lock.lock().unwrap();
  while !*started {
      print!("\n  waiting for thread start");
      started = cv.wait(started).unwrap();
  }
  print!("\n  thread started");
  handle
}

fn main() {


    let _ = test().join();
    // print!("\n  Creating BlockingQueue<String>");
    // let mut bq = BlockingQueue::<String>::new();
    
    // print!("\n  en_queuing {:?}", "zero");
    // let _ = bq.en_queue("zero".to_string());
    // print!("\n  en_queuing {:?}", "one");
    // let _ = bq.en_queue("one".to_string());
    // print!("\n  en_queuing {:?}", "two");
    // let _ = bq.en_queue("two".to_string());
    // print!("\n  en_queuing {:?}", "three");
    // let _ = bq.en_queue("three".to_string());
    // print!("\n  en_queuing {:?}", "four");
    // let _ = bq.en_queue("four".to_string());
    // print!("\n  en_queuing {:?}", "quit");
    // let _ = bq.en_queue("quit".to_string());
    // print!("\n  queue size is {}", bq.len());

    // loop {
    //     let item = bq.de_queue().into_inner();
    //     print!("\n  dequeuing {:?}", item);
    //     if item == "quit" {
    //         break;  // comment break to see queue block
    //     }
    // }

    // let bq0 = Arc::new(BlockingQueue::<String>::new());
    // let sbq1 = &bq0;
    // let sbq2 = &bq0;
    // //let bq0 = Arc::new(BlockingQueue::<String>::new());
    // // let sbq1 = Arc::clone(&bq0);
    // // let sbq2 = Arc::clone(&bq0);
    
    // let handle = thread::spawn(move || {
    //     loop {
    //         let item = sbq1.de_queue().into_inner();
    //         print!("\n  dequeued {:?}", item);
    //         if item == "quit" {
    //             break;
    //         }
    //     }    
    // });
    
    // print!("\n  en_queuing {:?}", "zero");
    // sbq2.en_queue("zero".to_string());

    // print!("\n  en_queuing {:?}", "one");
    // sbq2.en_queue("one".to_string());

    // print!("\n  en_queuing {:?}", "two");
    // sbq2.en_queue("two".to_string());

    // print!("\n  en_queuing {:?}", "three");
    // sbq2.en_queue("three".to_string());

    // print!("\n  en_queuing {:?}", "four");
    // sbq2.en_queue("four".to_string());

    // print!("\n  en_queuing {:?}", "quit");
    // sbq2.en_queue("quit".to_string());

    // handle.join();
    println!("\n\n  That's all Folks!\n");
}
