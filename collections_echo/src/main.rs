/////////////////////////////////////////////////////////////
// collections_echo - demonstrate collections library      //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 14 May 2020  //
/////////////////////////////////////////////////////////////

use std::collections::*;
use std::fmt::*;

/*-- displays type name and value --*/
fn show_type<T: Debug>(t:&T) {
    let name = std::any::type_name::<T>();
    print!("\n  type: {:?}", name);
    print!("\n  {:?}", &t);
}
/*-- echo function shows the type and value of its arg --*/
/*---------------------------------------------------------
   Illustrates how to accept and return iterable
   generic type
*/
fn echo<C: Debug + Clone + IntoIterator>(c:&C) -> &C 
   where C::Item: Debug, {
    show_type(c);
    print!("\n  items are: ");
    let iter = c.clone().into_iter();  // iter consumes
    for item in iter {
        print!("{:?} ", item);
    }
    c
}
/*-- demonstration --*/
fn main() {
    let putline = || { print!("\n"); };

    print!("\n  collections_echo");
    print!("\n ==================");

    let v = vec![1,2,3];
    let r = echo(&v);
    print!("\n\n  echo fn returned:");
    show_type(&r);
    putline();

    let mut vd = VecDeque::<f64>::new();
    vd.push_back(1.0);
    vd.push_front(2.5);
    vd.push_back(-1.5);
    let r = echo(&vd);
    print!("\n\n  echo fn returned:");
    show_type(&r);
    putline();

    let mut hm = HashMap::<i32, &str>::new();
    hm.insert(0,"zero");
    hm.insert(1,"one");
    hm.insert(2,"two");
    let r = echo(&hm);
    print!("\n\n  echo fn returned:");
    show_type(&r);
    putline();

    println!("\n  That's all Folks!\n");
}
