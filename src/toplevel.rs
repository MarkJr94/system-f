use vars as v;
use vars::{Var, VarSet};

use std::borrow::Cow;

#[derive(Clone,PartialEq,Eq, Debug)]
pub enum Ty {
    Bool,
    Arrow(Box<Ty>, Box<Ty>),
}

#[derive(Clone,PartialEq,Eq, Debug)]
pub enum Term {
    Var(v::Var),
    Abs(v::Var, Ty, Box<Term>),
    App(Box<Term>, Box<Term>),
    True,
    False,
    Not,
    If(Box<Term>, Box<Term>, Box<Term>),
}

impl Ty {
    pub fn unparse(&self) -> String {
        match self {
            &Ty::Bool => "B".into(),
            &Ty::Arrow(ref t1,ref t2) => format!("({} -> {})", t1.unparse(),  t2.unparse()),
        }
    }
}

impl Term {
    pub fn is_val(&self) -> bool {
        match self {
            &Term::True => true,
            &Term::False => true,
            &Term::Not => true,
            &Term::Abs(..) => true,
            _ => false,
        }
    }

    pub fn unparse(&self) -> String {
        match self {
            &Term::True => "T".into(),
            &Term::False => "F".into(),
            &Term::Not => "!".into(),
            &Term::Var(ref x) => x.clone(),
            &Term::App(ref t1, ref t2) => format!("({} {})", t1.unparse(), t2.unparse()),
            &Term::Abs(ref x, ref t, ref b) =>
                format!("(\\l {}: {}. {})", x, t.unparse(), b.unparse()),
            &Term::If(ref cond, ref b1, ref b2) => format!("(If {} {} {})", cond.unparse(), b1.unparse(), b2.unparse()),
        }
    }
}
