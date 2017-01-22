#![recursion_limit = "200"]

#[macro_use]
extern crate maplit;
#[macro_use]
extern crate pest;

mod toplevel;
mod vars;
mod visitor;
mod parser;
mod parens;

use std::io;
use std::io::{stdin, stdout};
use std::io::prelude::*;

fn actually() -> io::Result<()> {
    let mut input = String::new();
    let (mut i, mut o) = (stdin(), stdout());

    loop {
        try!(i.read_line(&mut input));

        {
            let thing = parens::parse_Term(&input);
            if let Ok(t) = thing {
                println!("{}", t.unparse());
            } else {
                break;
            }
        }
        input.clear();
    }

    Ok(())
}

fn main() {
    actually();
}
