/////////////////////////////////////////////////////////////
// fs_echo::main.rs - demonstrate file operations          //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 15 May 2020  //
/////////////////////////////////////////////////////////////

use std::fs::*;
use std::io::*;

/*-----------------------------------------------------------
   Note:
   The try operator ? has been used in several places in 
   both using_File and using_OpenOptions to avoid a lot
   of explicit error handling.

   The try operator bubbles errors up to the caller, so
   if there are several places where errors may occur, 
   they can all be handled in one place by the caller.

   What is surprising, but useful, is that the Result<T,E>
   types are not all the same.  In some results the type T
   is File, or (), or usize.  However, the error types are
   all io::Error, and that is what ? returns to the caller,
   so it just works!

   If there are no errors, ? simply unwraps the result to 
   be used, and everything is significantly simpler.
*/
#[allow(non_snake_case)]
fn using_File(file_name:&str, msg:&str) -> std::io::Result<()> {
    let mut wfile = File::create(file_name)?;
    print!("\n  writing message {:?} to file {:?}", msg, file_name);
    wfile.write_all(msg.as_bytes())?;
    let mut rfile = File::open(file_name)?;
    let mut rcv_msg = String::new();
    let _ = rfile.read_to_string(&mut rcv_msg);
    print!("\n  message {:?} read from {:?}", rcv_msg, file_name);
    Ok(())
}
fn open_file_for_write(path:&str) -> std::io::Result<File> {
    let wfile = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)?;
    Ok(wfile)
}
fn open_file_for_read(path:&str) -> std::io::Result<File> {
    let rfile = OpenOptions::new()
                .read(true)
                .open(path)?;
    Ok(rfile)
}
#[allow(non_snake_case)]
fn using_OpenOptions(file_name:&str, msg:&str) -> std::io::Result<()> {
    let mut wfile = open_file_for_write(file_name)?;
    print!("\n  writing message {:?} to file {:?}", msg, file_name);
    wfile.write_all(msg.as_bytes())?;

    let mut rfile = open_file_for_read(file_name)?;
    let mut rcv_msg = String::new();
    let _ = rfile.read_to_string(&mut rcv_msg);
    print!("\n  message {:?} read from {:?}", rcv_msg, file_name);
    Ok(())
}
fn main() -> std::io::Result<()> {
    let putline = || print!("\n");

    let file_name = "test.txt";
    let msg = "msg from writer";
    print!("\n  std::fs echo demonstration");
    print!("\n ============================");
    putline();

    print!("\n  demo using File structure");
    print!("\n ---------------------------");
    using_File(file_name, msg)?;
    putline();

    print!("\n  demo using OpenOptions structure");
    print!("\n ----------------------------------");
    using_OpenOptions(file_name, msg)?;

    println!("\n\n  That's all Folks!\n");
    Ok(())
}
