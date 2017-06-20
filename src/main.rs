#![feature(conservative_impl_trait)]
#![feature(slice_patterns, advanced_slice_patterns)]
#![recursion_limit = "400"]
#![allow(dead_code)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate maplit;
extern crate pom;
#[macro_use]
extern crate nom;

mod toplevel;
mod vars;
// mod visitor;
mod core;
mod errors;
mod lispy;
// mod typecheck;
// mod debrujin;

use std::io;
use std::io::stdin;

fn actually() -> io::Result<()> {
    let mut input = String::new();
    let i = stdin();

    // loop {
    //     try!(i.read_line(&mut input));

    //     {
    //         let thing = parens::parse_Term(&input);
    //         if let Ok(t) = thing {
    //             println!("{}", t.unparse());
    //         } else {
    //             break;
    //         }
    //     }
    //     input.clear();
    // }

    Ok(())
}

fn main() {
    actually().unwrap()
}
