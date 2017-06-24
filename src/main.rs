#![feature(conservative_impl_trait)]
#![allow(dead_code)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate maplit;
extern crate pom;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

mod toplevel;
mod visitor;
mod core;
mod eval;
mod errors;
mod lispy;
mod typecheck;
mod debrujin;

use std::io;
use std::io::stdin;

use slog::{Drain, Logger};

fn root_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = Logger::root(drain, o!("context" => "main"));

    _log
}

fn inner(input: &str, logger: &Logger) -> errors::Result<(core::Term, core::Term, core::Ty)> {
    let code = lispy::get_code(input.as_bytes())?;
    {
        let fv = visitor::FVVisitor::new(&code);
        let _ = fv.check()?;
    }
    let ast = {
        let mut rv = debrujin::RenameVisitor::new();
        rv.rename_term(&code)?
    };
    let ty = {
        let mut typechecker = typecheck::TypeCheckVisitor::new();
        typechecker.type_of(&ast)?
    };

    let v = {
        let mut evaluator = eval::Evaluator::new(logger);
        evaluator.eval(&ast)?
    };

    Ok((ast, v, ty))
}

fn actually(logger: &Logger) -> io::Result<()> {
    let mut input = String::new();
    let i = stdin();

    loop {
        try!(i.read_line(&mut input));

        {
            let thing = inner(&input, logger);
            match thing {
                Ok((ast, val, ty)) => {
                    println!("{:?} => {:?}: {}", ast, val, ty);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        input.clear();
    }
}

fn main() {
    let logger = root_logger();

    actually(&logger).unwrap()
}
