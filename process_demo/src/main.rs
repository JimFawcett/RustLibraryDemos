/////////////////////////////////////////////////////////////
// process_demo::main.rs - start child process             //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 16 May 2020  //
/////////////////////////////////////////////////////////////

#![allow(unused_imports)]
use std::process::*;
use std::io::*;

fn main() {

    let putline = || print!("\n");

    print!("\n  spawning child process");
    print!("\n ========================\n");
    putline();

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd.exe")
                .args(&[
                    "/C", "type",
                    "cargo.toml"
                ])
                .output()
                .expect("failed to execute process")
    }
    else {
        Command::new("sh")
                 .arg("-C")
                 .arg("echo.hello")
                 .output()
                 .expect("failed to execute process")
    };

    std::io::stdout().write_all(&output.stdout).unwrap();

    println!("\n\n  That's all Folks!\n\n");
}
