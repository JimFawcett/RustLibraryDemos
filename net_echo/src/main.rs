/////////////////////////////////////////////////////////////
// net_echo::main.rs - demonstrate Tcp communication       //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 16 May 2020  //
/////////////////////////////////////////////////////////////
/*
   This demo uses TcpListener and TcpStream which use 
   sockets internally.  So, like sockets, they are byte
   oriented.

   In order to send messages, the application needs a 
   protocol for defining when to stop reading messages.
   For this demo, newlines are used as message terminators,
   which works well for a demonstration.

   An application will probably need something more flexible
   like HTTP style messages.
*/
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener, Shutdown::Both};
use std::thread;
use std::str;
use std::io::{BufReader};

/*-- demo handler receives one message and replys --*/
fn handle_client(mut stream: &TcpStream) -> std::io::Result<()> {
    print!("\n  entered client handler");
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut rcv_msg = String::new();
    reader.read_line(&mut rcv_msg)?;
    rcv_msg.pop();  // remove '\n' sentinal
    print!("\n  client handler received: ");
    print!("{:?}",rcv_msg);
    let _ = std::io::stdout().flush();

    rcv_msg.push_str(" recieved!\n");
    let _ = std::io::stdout().flush();
    stream.write_all(&rcv_msg.as_bytes())?;
    stream.shutdown(Both)?;
    Ok(())
}
/*-- demo listener accepts only one connection --*/
fn start_listener(end_point :&str) -> std::io::Result<()> {
    print!("\n  starting listener on {:?}", end_point);
    let _ = std::io::stdout().flush();
    let tcpl = TcpListener::bind(end_point)?;
    for stream in tcpl.incoming() {
        print!("\n  listener accepted connection");
        let _ = handle_client(&stream?)?;
        /*-- for this demo, just accept one connection --*/
        break;
    }
    Ok(())
}
/*-- demonstration --*/
fn main() -> std::io::Result<()> {

    print!("\n  net_echo demonstration");
    print!("\n ========================");

    let rcvr_endpoint = "127.0.0.1:8080";
    let mut msg = "msg from connector".to_string();

    /*-- run listener on child thread --*/
    let handle = thread::spawn( 
        move || { let _ = start_listener(rcvr_endpoint); }
    );

    /*-- send message --*/
    print!("\n  connecting to {:?}", rcvr_endpoint);
    let mut stream = TcpStream::connect(rcvr_endpoint)?;
    print!("\n  sending message {:?}", msg);
    msg.push('\n');  // message end sentinal
    stream.write_all(&msg.as_bytes())?;

    /*-- read reply message --*/
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut rcv_msg = String::new();
    reader.read_line(&mut rcv_msg)?;
    rcv_msg.pop();  // remove '\n' sentinal
    print!("\n  connector received reply {:?}", rcv_msg);
    let _ = std::io::stdout().flush();

    let _ = handle.join();  // wait for child to shutdown

    println!("\n\n  That's all Folks!\n\n");
    Ok(())
}
