/////////////////////////////////////////////////////////////
// time_demo::main.rs - demonstration of std::time         //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 22 May 2020  //
/////////////////////////////////////////////////////////////

use std::time;
use std::thread;
use std::convert::TryFrom;

/*-- how accurate is thread sleep ? --*/
fn sleep_proc(ms: u64) {
   let dur = time::Duration::from_millis(ms); 
   thread::sleep(dur);
}
/*-- nonsense calculation to waste time --*/
fn calc_proc(count: u64) {
    let mut _value: f64 = 42.0;
    let mut mult:i32 = 42; 
    let rslt = i32::try_from(count);
    match rslt {
        Ok(counter) => {
            for i in 0..counter {
                mult = mult + i;
                let mult = (mult + i) % 33;
                let _value = _value + mult as f64;
            }       
        },
        Err(_) => print!("\n  can't compute this number")
    }
}
/*-- writing to console is slow --*/
fn print_proc(count: u64) {
    print!("\n  ");
    for _num in 0..count {
        for _out in 0..25 {
            print!(". ");
        }
        print!("\n  ");
    }
}
/*-- stop watch timer --*/
fn measure_time<F: FnOnce(u64) -> ()>(f: F, arg: u64) -> time::Duration {
    let start = time::Instant::now();
    f(arg);
    start.elapsed()
}
/*-- demo time durations --*/
fn main() {
    print!("\n  Demonstrate std::time");
    print!("\n =======================");

    let putline = || print!("\n");

    putline();
    let mut duration = measure_time(sleep_proc, 3);
    print!("\n  thread::sleep for 3 ms actually took {:?}", duration);
    putline();

    duration = measure_time(calc_proc, 1500);
    print!("\n  time to perform nonsense calc was {:?}", duration);
    putline();

    duration = measure_time(print_proc, 15);
    print!("\n  time to print array of dots was {:?}", duration);

    println!("\n\n  That's all Folks!\n");
}
