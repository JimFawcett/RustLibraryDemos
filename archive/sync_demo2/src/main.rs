// sync_demo::main.rs - BlockingQueue

use std::ops::{DerefMut, Deref};
use std::ops::*;
//use std::io::*;
//use std::mem::*;
use std::sync::*;
use std::thread;
use std::collections::{VecDeque};
use std::cell::*;

//type BQueue<T> = Arc<Mutex<VecDeque<T>>>;

// #[derive(Debug)]
// pub struct BQueue<T: Clone> {
//     queue: Arc<Mutex<VecDeque<T>>>,
// }
// impl<T:Clone> Deref for BQueue<T> {

//     type Target = Arc<Mutex<VecDeque<T>>>;
    
//     fn deref(&self) -> &self::Arc<Mutex<VecDeque<T>>> {
//         &self.queue
//     }
// }
// impl<T:Clone> DerefMut for BQueue<T> {

//     fn deref_mut(&mut self) -> &mut Arc<Mutex<VecDeque<T>>> {
//         &mut self.queue
//     }
// }
// impl<T:Clone> BQueue<T> {
//     pub fn new() -> BQueue<T> {
//         BQueue {
//             queue: Arc::new(Mutex::new(VecDeque::new())),
//         }
//     }
// }

#[derive(Debug)]
struct BQueue<T> { pub queue: std::cell::UnsafeCell<Mutex<VecDeque<T>>>, }
unsafe impl<T:Clone> Sync for BQueue<T> {}
impl<T> BQueue<T> {
    pub fn new() -> BQueue<T> {
        BQueue {
            queue: std::cell::UnsafeCell::new(Mutex::new(VecDeque::new())),
        }
    }
    pub fn into_inner(&self) -> T {
        self.queue.into_inner().into_inner().unwrap()
    }
}

#[derive(Debug)]
pub struct BlockingQueue<T: ?Sized + Clone> {
    queue: BQueue<T>,
    cv: Condvar,
}
unsafe impl<T: Clone> Sync for BlockingQueue<T> {}
unsafe impl<T: Clone> Send for BlockingQueue<T> {}

// impl<T:Clone> Deref for BlockingQueue<T> {

//     // type Target = Arc<Mutex<VecDeque<T>>>;
//     type Target = BQueue<T>;   

//     fn deref(&self) -> &self::BQueue<T> {
//         &self.queue
//     }
// }
// impl<T:Clone> DerefMut for BlockingQueue<T> {

//     fn deref_mut(&mut self) -> &mut BQueue<T> {
//         &mut self.queue
//     }
// }
impl<T: Clone > BlockingQueue<T> {
    pub fn new() -> BlockingQueue<T> {
        BlockingQueue {
            cv: Condvar::new(),
            queue: BQueue::new(),
            //queue: BQueue::<T>::new(),
            //queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    pub fn en_queue(&mut self, t: T) -> LockResult<()> {
        if let Ok(mut q) = self.queue.into_inner().lock() {
            q.push_back(RefCell::new(t));
        }
        Ok(())
    }
    pub fn de_queue(&mut self) -> RefCell<T> {
        loop {
            if let Ok(mut q) = self.queue.lock() {
                if q.len() > 0 {
                    if let Some(val) = q.front() {
                        let t = val.clone();
                        q.pop_front();
                        self.cv.notify_one();
                        return t;
                    }
                    else {
                        panic!();
                    }
                }
                else {
                    let _rslt = self.cv.wait(q);
                    if let Err(_) = _rslt {
                        panic!();
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

// #[derive(Debug, Clone)]
// pub struct ArcBq<T:Clone> {
//     abq : Arc<BlockingQueue<T>>,
// }
// unsafe impl<T:Clone + ?Sized + Send> Sync for ArcBq<T> {} 
// unsafe impl<T:Clone + ?Sized + Send> Send for ArcBq<T> {}

// impl<T:Clone> Deref for ArcBq<T> {

//     type Target = Arc<BlockingQueue<T>>;   

//     fn deref(&self) -> &Self::Target {
//         &self.abq
//     }
// }
// impl<T:Clone> DerefMut for ArcBq<T> {

//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.abq
//     }
// }
// impl<T:Clone> ArcBq<T> {
//     pub fn new() -> ArcBq<T> {
//         ArcBq {
//             abq: Arc::new(BlockingQueue::new())
//         }
//     }
//     // pub fn get_mut(&mut self) -> &mut T {
//     //     &mut self.abq.get_mut().unwrap()
//     // }
// }

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

    loop {
        let item = bq.de_queue().into_inner();
        print!("\n  dequeuing {:?}", item);
        if item == "quit" {
            break;  // comment break to see queue block
        }
    }

    let bq0 = RefCell::new(BlockingQueue::<String>::new());
    let sbq1 = &bq0;
    let sbq2 = &bq0;
    //let bq0 = Arc::new(BlockingQueue::<String>::new());
    // let sbq1 = Arc::clone(&bq0);
    // let sbq2 = Arc::clone(&bq0);
    
    let handle = thread::spawn(move || {
        loop {
            let item = sbq1.into_inner().de_queue().into_inner();
            print!("\n  dequeued {:?}", item);
            if item == "quit" {
                break;
            }
        }    
    });
    
    print!("\n  en_queuing {:?}", "zero");
    sbq2.into_inner().en_queue("zero".to_string());

    print!("\n  en_queuing {:?}", "one");
    sbq2.into_inner().en_queue("one".to_string());

    print!("\n  en_queuing {:?}", "two");
    sbq2.into_inner().en_queue("two".to_string());

    print!("\n  en_queuing {:?}", "three");
    sbq2.into_inner().en_queue("three".to_string());

    print!("\n  en_queuing {:?}", "four");
    sbq2.into_inner().en_queue("four".to_string());

    print!("\n  en_queuing {:?}", "quit");
    sbq2.into_inner().en_queue("quit".to_string());

    handle.join();
    println!("\n\n  That's all Folks!\n");
}
