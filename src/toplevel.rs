pub use core::Ty;
use vars::{VarAbs, BasicVar};

pub fn arrow(alpha: Ty, beta: Ty) -> Ty {
    Ty::Arrow(Box::new(alpha), Box::new(beta))
}

#[derive(Clone,PartialEq,Eq, Debug)]
pub enum Term {
    Var(String),
    Abs(String, Ty, Box<Term>),
    App(Box<Term>, Box<Term>),
    True,
    False,
    Not,
    If(Box<Term>, Box<Term>, Box<Term>),
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
            &Term::Var(ref x) => x.to_string(),
            &Term::App(ref t1, ref t2) => format!("({} {})", t1.unparse(), t2.unparse()),
            &Term::Abs(ref x, ref t, ref b) => {
                format!("(lam {}: {}. {})", x, t.unparse(), b.unparse())
            }
            &Term::If(ref cond, ref b1, ref b2) => {
                format!("(if {} {} {})", cond.unparse(), b1.unparse(), b2.unparse())
            }
        }
    }

    pub fn app(f: Term, x: Term) -> Term {
        Term::App(Box::new(f), Box::new(x))
    }

    pub fn abs<T: Into<String>>(var: T, ty: Ty, body: Term) -> Term {
        Term::Abs(var.into(), ty, Box::new(body))
    }

    pub fn if_(cond: Term, pass: Term, fail: Term) -> Term {
        Term::If(Box::new(cond), Box::new(pass), Box::new(fail))
    }

    pub fn var<S: Into<String>>(s: S) -> Term {
        Term::Var(s.into())
    }
}
