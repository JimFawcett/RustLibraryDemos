// sync_demo::main.rs - BlockingQueue

//use std::ops::{DerefMut, Deref};
//use std::io::*;
//use std::mem::*;
use std::sync::*;
use std::thread;
use std::collections::{VecDeque};
//use std::cell::*;
//type Queue<T> = Arc<Mutex<VecDeque<T>>>;

// #[derive(Debug)]
// pub struct BqEntry<T: ?Sized + Clone> {
//     entry: RefCell<T>,
// }
// impl<T: ?Sized + Clone> Clone for BqEntry<T> {
//     fn clone(&self) -> Self {
//         let c;
//         unsafe { c = self.entry.get_mut(); }
//         BqEntry::new(c.clone())
//     }
// }
// impl<T: ?Sized + Clone> BqEntry<T> {
//     pub fn new(t: T) -> BqEntry<T> {
//         let e = BqEntry {
//             entry: RefCell::new(t),
//         };
//         e
//     }
//     pub fn into_inner(self) -> T where T: Sized, {
//         self.entry.into_inner()
//     }
//     pub fn get_mut(&mut self) -> &mut T {
//         &mut *self.entry.get_mut()
//     }
// }
//type BQentry<T> = UnsafeCell<T>;
//type BQueue<T> = Arc<Mutex<VecDeque<BqEntry<T>>>>;
type BQueue<T> = Arc<Mutex<VecDeque<T>>>;


#[derive(Debug)]
pub struct BlockingQueue<T: ?Sized + Clone> {
  queue: BQueue<T>,
  cv: Condvar,
  //mtx: Mutex<VecDeque<T>>,
}
unsafe impl<T: Clone> Sync for BlockingQueue<T> {}
unsafe impl<T: Clone> Send for BlockingQueue<T> {}

impl<T: Clone > BlockingQueue<T> {
    pub fn new() -> BlockingQueue<T> {
        BlockingQueue {
            cv: Condvar::new(),
            //queue: BQueue::<T>::new(),
            queue: Arc::new(Mutex::new(VecDeque::<T>::new())),
        }
    }
    pub fn en_queue(&mut self, t: T) -> LockResult<()> {
        //let entry = BqEntry::new(t);
        if let Ok(mut q) = self.queue.lock() {
            q.push_back(t);
        }
        Ok(())
    }
    pub fn de_queue(&mut self) -> Option<T> {
        loop {
            if let Ok(mut q) = self.queue.lock() {
                if q.len() > 0 {
                    if let Some(val) = q.front() {
                        let t = val.clone();
                        q.pop_front();
                        self.cv.notify_one();
                        return Some(t);
                    }
                    else {
                        return None;
                    }
                }
                else {
                    let _rslt = self.cv.wait(q);
                    if let Err(_) = _rslt {
                        return None
                    }
                }
            }
        }
        
    }
    pub fn len(&self) -> usize {
        let mut len: usize = 0;
        if let Ok(q) = self.queue.lock() {
            len = q.len();
        }
        len
    }
}

fn main() {

    print!("\n  Creating BlockingQueue<String>");
    let mut bq = BlockingQueue::<String>::new();
    
    print!("\n  en_queuing {:?}", "zero");
    let _ = bq.en_queue("zero".to_string());
    print!("\n  en_queuing {:?}", "one");
    let _ = bq.en_queue("one".to_string());
    print!("\n  en_queuing {:?}", "two");
    let _ = bq.en_queue("two".to_string());
    print!("\n  en_queuing {:?}", "three");
    let _ = bq.en_queue("three".to_string());
    print!("\n  en_queuing {:?}", "four");
    let _ = bq.en_queue("four".to_string());
    print!("\n  en_queuing {:?}", "quit");
    let _ = bq.en_queue("quit".to_string());
    print!("\n  queue size is {}", bq.len());

    while let Some(item) = bq.de_queue() {
        print!("\n  dequeuing {:?}", item);
        if item == "quit" {
            break;  // comment break to see queue block
        }
    }


    // let sbq = Arc::new(Mutex::new(BlockingQueue::<String>::new()));
    // let sbq1 = Arc::clone(&sbq);
    // let sbq2 = Arc::clone(&sbq);
    
    // unsafe {
    //     let mut bq = BlockingQueue::<String>::new();
    //     let mut rbq = &bq; 
    //     type Q = BlockingQueue<String>;
    //     let handle = thread::spawn(move || {
    //     loop {
    //         let item = rbq.de_queue().unwrap();
    //         print!("\n  dequeued {:?}", item);
    //         if item == "quit".to_string() {
    //             break;
    //         }
    //     }
        
    // //         // let mut oq: Q;
    // //         // {
    // //         //     if let Ok(mut q) = sbq2.lock() { oq = *q; }
    // //         //         print!("\n  attempting to de_queue");
    // //         //         let _ = std::io::stdout().flush();
    // //         //         let item = oq.de_queue().unwrap();
    // //         //         print!("\n  de_queue returned");
    // //         //         print!("\n  {:?}", item);
    // //         //         if item == "quit".to_string() {
    // //         //             break;
    // //         //         }
    // //             // }
    // //             // else {
    // //             //     print!("\n  couldn't de_queue");
    // //             //     break;
    // //             // }
    // //         // }
    
    // });
    
        // print!("\n  en_queuing {:?}", "zero");
        // bq.en_queue("zero".to_string());

        // print!("\n  en_queuing {:?}", "zero");
        // bq.en_queue("zero".to_string());

        // print!("\n  en_queuing {:?}", "zero");
        // bq.en_queue("zero".to_string());

        // print!("\n  en_queuing {:?}", "zero");
        // bq.en_queue("zero".to_string());

        // print!("\n  en_queuing {:?}", "zero");
        // bq.en_queue("zero".to_string());

        // print!("\n  en_queuing {:?}", "zero");
        // bq.en_queue("zero".to_string());
        // let _ = handle.join();
    // }
    // // if let Ok(mut q) = sbq1.lock() {
    // //     let _ = q.en_queue("zero".to_string());
    // // }
    // // print!("\n  en_queuing {:?}", "one");
    // // if let Ok(mut q) = sbq1.lock() {
    // //     let _ = q.en_queue("one".to_string());
    // // }
    // // print!("\n  en_queuing {:?}", "two");
    // // if let Ok(mut q) = sbq1.lock() {
    // //     let _ = q.en_queue("two".to_string());
    // // }
    // // print!("\n  en_queuing {:?}", "three");
    // // if let Ok(mut q) = sbq1.lock() {
    // //     let _ = q.en_queue("three".to_string());
    // // }
    // // print!("\n  en_queuing {:?}", "four");
    // // if let Ok(mut q) = sbq1.lock() {
    // //     let _ = q.en_queue("four".to_string());
    // // }
    // // print!("\n  en_queuing {:?}", "quit");
    // // if let Ok(mut q) = sbq1.lock() {
    // //     let _ = q.en_queue("quit".to_string());
    // // }

    // // print!("\n  en_queuing {:?}", "one");
    // // let _ = bq.en_queue("one".to_string());
    // // print!("\n  en_queuing {:?}", "two");
    // // let _ = bq.en_queue("two".to_string());
    // // print!("\n  en_queuing {:?}", "three");
    // // let _ = bq.en_queue("three".to_string());
    // // print!("\n  en_queuing {:?}", "four");
    // // let _ = bq.en_queue("four".to_string());
    // // print!("\n  en_queuing {:?}", "quit");
    // // let _ = bq.en_queue("quit".to_string());
    // // print!("\n  queue size is {}", bq.len());

    
    println!("\n\n  That's all Folks!\n");
}
