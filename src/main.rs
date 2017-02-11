#![feature(conservative_impl_trait)]
#![recursion_limit = "400"]
#![allow(dead_code)]

#[macro_use]
extern crate maplit;
extern crate pom;

mod toplevel;
mod vars;
// mod visitor;
mod parser;
mod ast;
// mod typecheck;
// mod debrujin;

use std::io;
use std::io::{stdin};

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
