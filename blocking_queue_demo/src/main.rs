/////////////////////////////////////////////////////////////
// sync_demo::main.rs - BlockingQueue                      //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 19 May 2020  //
/////////////////////////////////////////////////////////////
/*
   This is a BlockingQueue abstraction.  To be shared between
   threads, without using unsafe code, any abstraction must
   be composed only of Mutexes and Condvars or a struct or
   tuple with only those members.

   That means that the blocking queue must hold its native
   queue in a Mutex, as shown below.
   
   There is another alternative, based on Rust channels, which 
   are essentially blocking queues.

   In another demo I will build a prototype from channels, and 
   then discuss the advantages and disadvantages of each.
*/
use std::io::*;
use std::sync::*;
use std::collections::*;
use std::thread;

#[derive(Debug)]
struct BlockingQueue<T> {
    q: Mutex<VecDeque<T>>,
    cv: Condvar,
}
impl<T> BlockingQueue<T> {
    fn new() -> Self {
        Self {
            q: Mutex::new(VecDeque::new()),
            cv: Condvar::new(),
        }
    }
    fn en_q(&self, t:T) {
        let mut lq = self.q.lock().unwrap();
        lq.push_back(t);
        self.cv.notify_one();
    }
    fn de_q(&self) -> T {
        let mut lq = self.q.lock().unwrap();
        while lq.len() == 0 {
            lq = self.cv.wait(lq).unwrap();
        }
        lq.pop_front().unwrap()
    }
    fn len(&self) -> usize {
        self.q.lock().unwrap().len()
    }
}

/*-- simple test of BlockingQueue --*/
fn test() {

    let share = Arc::new(BlockingQueue::<String>::new());
    let share1 = Arc::clone(&share);
    let share2 = Arc::clone(&share);

    let flush = || { let _ = std::io::stdout().flush(); };

    /*-- child thread dequeues messages --*/
    let handle = thread::spawn(move || {
        print!("\n  child thread started");
        flush();
        loop {
            let t = share1.de_q();
            print!("\n  dequeued {} on child thread", t);
            flush();
            if &t == "quit" {
                break;
            }
        }
        print!("\n  thread shutting down");
        flush();
    });

    /*-- main thread enqueues messages --*/
    for i in 0..5 {
        let msg = format!("msg #{}", i.to_string());
        print!("\n  enqueued {:?} on main thread", msg);
        flush();
        share2.en_q(msg);
    }
    /*-- shut down child thread --*/
    print!("\n  enqueued {:?} on main thread", "quit");
    flush();
    share2.en_q("quit".to_string());

    /*-- child thread must complete before exiting --*/
    print!("\n  waiting for child thread to stop");
    flush();
    let _ = handle.join();

    print!("\n  queue length = {}", share2.len());
}

fn main() {

    print!("\n  Demonstrate queue shared between threads");
    print!("\n ==========================================");

    //demo();
    test();
    print!("\n\n  That's all Folks!\n");
}