#![recursion_limit = "200"]

#[macro_use]
extern crate maplit;
#[macro_use]
extern crate pest;

mod toplevel;
mod vars;
mod visitor;
mod parser;

fn main() {
    println!("Hello, world!");
}
