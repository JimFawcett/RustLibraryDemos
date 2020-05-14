/////////////////////////////////////////////////////////////
// io_echo::main.rs - std::io library demonstration        //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 14 May 2020  //
/////////////////////////////////////////////////////////////

use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    print!("\n  Echo std::io");
    print!("\n ==============");

    const QUIT:char = 'q';
    print!("\n  Enter single \'q\' character to terminate program");
    loop {
        print!("\n  enter some text: ");
        let _ = std::io::stdout().flush();
        let mut input_str = String::new();
        let size = std::io::stdin().read_line(&mut input_str)?;

        /////////////////////////////////////////////////////////////
        // line below needs ctrl z to terminate read
        // let size = std::io::stdin().read_to_string(&mut input_str)?

        print!("\n  read {} bytes", size);
        let rslt = input_str.chars().nth(0);
        if rslt == Some(QUIT) && size == 3 {
            break;
        }
        let out_str = format!("\n  {:?}", input_str);
        std::io::stdout().write_all(out_str.as_ref())?;  // convert to &[u8]
        std::io::stdout().flush()?;
    }
    println!("\n  That's all Folks!");
    Ok(())
}
