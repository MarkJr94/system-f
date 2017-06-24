pub use core::Ty;

#[derive(Clone,PartialEq,Eq, Debug)]
pub enum Term {
    Var(String),
    Abs(Vec<(String, Ty)>, Box<Term>),
    App(Box<Term>, Vec<Term>),
    Int(i64),
    True,
    False,
    Not,
    If(Box<Term>, Box<Term>, Box<Term>),
    Bottom,
}

impl Term {
    pub fn is_val(&self) -> bool {
        match self {
            &Term::True => true,
            &Term::False => true,
            &Term::Not => true,
            &Term::Abs(..) => true,
            &Term::Var(..) => true,
            &Term::Int(..) => true,
            _ => false,
        }
    }

    pub fn unparse(&self) -> String {
        match self {
            &Term::True => "T".into(),
            &Term::False => "F".into(),
            &Term::Not => "!".into(),
            &Term::Var(ref x) => x.to_string(),
            &Term::Int(n) => n.to_string(),
            &Term::App(ref t1, ref t2) => {
                let mut arg_str = String::new();
                for arg in t2 {
                    arg_str.push_str(&format!("{} ", arg.unparse()));
                }
                format!("({} {})", t1.unparse(), arg_str)
            }
            &Term::Abs(ref args, ref b) => {
                let mut arg_str: String = String::new();
                for &(ref name, ref type_) in args {
                    arg_str.push_str(&format!("{}: {},", name, type_.unparse()));
                }
                // arg_str.
                format!("(lam {}. {})", arg_str, b.unparse())
            }
            &Term::If(ref cond, ref b1, ref b2) => {
                format!("(if {} {} {})", cond.unparse(), b1.unparse(), b2.unparse())
            }
            &Term::Bottom => "_|_".into(),
        }
    }

    pub fn app(f: Term, x: &[Term]) -> Term {
        Term::App(Box::new(f), Vec::from(x))
    }

    pub fn abs(args: &[(&str, Ty)], body: Term) -> Term {
        let args = args.iter()
            .map(|&(ref name, ref ty)| ((*name).into(), ty.clone()))
            .collect();
        Term::Abs(args, Box::new(body))
    }

    pub fn abss(args: &[(String, Ty)], body: Term) -> Term {
        let args = args.iter()
            .map(|&(ref name, ref ty)| ((name).clone(), ty.clone()))
            .collect();
        Term::Abs(args, Box::new(body))
    }

    pub fn if_(cond: Term, pass: Term, fail: Term) -> Term {
        Term::If(Box::new(cond), Box::new(pass), Box::new(fail))
    }

    pub fn var<S: Into<String>>(s: S) -> Term {
        Term::Var(s.into())
    }
}
