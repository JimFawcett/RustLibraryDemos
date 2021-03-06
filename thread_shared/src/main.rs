/////////////////////////////////////////////////////////////
// thread_shared::main.rs - shared string demo             //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 10 May 2020  //
/////////////////////////////////////////////////////////////

#![allow(dead_code)]

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn db_show<T:std::fmt::Debug>(t:T, msg:&str, p:bool) {
    print!("\n  --{}", msg);
    if p {
        let name = std::any::type_name::<T>();
        print!("\n  --TypeId: {},\n  --size: {}", name, std::mem::size_of::<T>());
    }
    print!("\n  --{:?}", t);
} 
fn thread_id() -> thread::ThreadId {
    thread::current().id()
}
fn thread_id_value(id:thread::ThreadId) -> char {
    let sid = format!("{:?}", id);
    sid.chars().nth(9).unwrap()
}
fn test2() -> std::io::Result<()> {
    print!("\n  Two child threads sharing string");
    print!("\n ==================================");

    let thrd1_id = thread_id_value(thread::current().id());
    print!("\n  main thread id      = {:?}", thrd1_id);
    let dur = Duration::from_millis(2);  // sleep time
    let mut s = String::new();            // shared string
    s.push(thrd1_id);  // main thread gets first edit

    let data = Arc::new(Mutex::new(s));   // thread safe shared wrapper
    
    /*-------------------------------------------------------
        Modify String s on child thread #1
    */
    let shared1 = Arc::clone(&data);      // clones pointer to data ref
    let _handle1 = thread::spawn(move || { 
        // shared1 clone moved without moving data
        let sid:String = format!("{:?}",thread::current().id());
        // sid = "ThreadId(2)"
        if let Some(sid) = sid.chars().nth(9) {
            // ThreadId(2) => 2
            for _i in 0..15 {
                {
                    if let Ok(mut temp) = shared1.lock() {
                        temp.push(sid);  // append thread id
                    }
                }  // unlocked here
                thread::sleep(dur);
            }
        }
    });
    
    /*-------------------------------------------------------
        Modify String s on child thread #2
    */
    let shared2 = Arc::clone(&data);
    // db_show(&shared2, "shared2", false);
    let _handle2 = thread::spawn(move || { 
        let sid:String = format!("{:?}",thread::current().id());
        if let Some(sid) = sid.chars().nth(9) {
            for _i in 0..15 {
                {
                    if let Ok(mut temp) = shared2.lock() {
                        temp.push(sid);
                    }
                }
                thread::sleep(dur);
            }
        }
    });
    /*-------------------------------------------------------
        main thread displaying child threads ids and then
        joining them, e.g., block until threads complete
    */
    let thrd2_id = thread_id_value(_handle1.thread().id());
    print!("\n  1st child thread id = {:?}", thrd2_id);
    let _ = _handle1.join();
    let thrd3_id = thread_id_value(_handle2.thread().id());
    print!("\n  2nd child thread id = {:?}", thrd3_id);
    let _ = _handle2.join();

    /*-------------------------------------------------------
        extract mutex from arc
        - commented line is simple but may panic
        - code used here manages error without panic
    */
    // let out = Arc::try_unwrap(data).expect("lock still owned");
    let rslt = Arc::try_unwrap(data);
    let out :Mutex<String>;
    match rslt {
        Ok(mtx) => out = mtx,
        Err(_) => {
            let error = std::io::Error::new(
                std::io::ErrorKind::Other, 
                "Arc access error"
            );
            return Err(error);
        },
    }
    /*-------------------------------------------------------
        extract string from mutex
        - commented line is simple but may panic
        - code used here manages error without panic
    */
    //let mut shared = out.into_inner().expect("can't lock mutex");
    let mut shared: String;
    let rslt = out.into_inner();
    match rslt {
        Ok(s) => shared = s,
        Err(_) => {
            let error = std::io::Error::new(
                std::io::ErrorKind::Other, 
                "Mutex access error"
            );
            return Err(error);
        } 
    }
    shared.push(thrd1_id);  // main thread has last edit

    /*-- display result of string modifications --*/
    print!("\n  final shared string value = {:?}",shared);
    Ok(())
}
fn main() -> std::io::Result<()> {

    test2()?;

    println!("\n\n  That's all Folks!\n\n");
    Ok(())
}
