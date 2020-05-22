/////////////////////////////////////////////////////////////
// mpsc_demo::main.rs - msg passing uses blocking queue    //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 22 May 2020  //
/////////////////////////////////////////////////////////////

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn send_proc(s:&str, tx : &mpsc::Sender<String>) {
    let max = 5;
    for i in 0..max {
        let msg = format!("msg #{} from {}", i.to_string(), s);
        print!("\n  sending  {:?}", msg);
        tx.send(msg).unwrap();
        thread::yield_now();   // others ready to run?
    }
}
fn recv_proc(rx: &mpsc::Receiver<String>) {
    for msg in rx {
        print!("\n  received {:?}", msg);
        if msg == "quit" {
            break;
        }
    }
}
fn demo() {
    /*-- setup channel --*/
    let (tx, rx) = mpsc::channel::<String>();
    let tx2 = mpsc::Sender::clone(&tx);
    let tx_quit = mpsc::Sender::clone(&tx);

    /*-- start receive thread --*/
    let rcv_handle = thread::spawn(
        move || { recv_proc(&rx); 
    });

    /*-- wait for receive thread to start --*/
    thread::sleep(Duration::from_millis(50));

    /*-- start send threads --*/
    let toms_handle = thread::spawn(
        move || { send_proc("Tom", &tx); 
    });
    let jerrys_handle = thread::spawn(
        move || { send_proc("Jerry", &tx2); 
    });
    /*-- wait for send threads to complete --*/
    let _ = toms_handle.join();
    let _ = jerrys_handle.join();

    /*-- now safe to stop receiver --*/
    let _ = tx_quit.send("quit".to_string());
    let _ = rcv_handle.join();
}
fn main() {
    print!("\n  Message passing demo");
    print!("\n ======================");

    demo();
    print!("\n\n That's all Folks!\n");
}
