use super::ty::Ty;

#[derive(Clone,PartialEq,Eq, Debug)]
pub enum Term {
    Var(u32),
    Abs(Vec<Ty>, Box<Term>),
    App(Box<Term>, Vec<Term>),
    True,
    False,
    Int(i64),
    Not,
    If(Box<Term>, Box<Term>, Box<Term>),
    Stuck,
}

impl Term {
    pub fn is_val(&self) -> bool {
        match self {
            &Term::True => true,
            &Term::False => true,
            &Term::Not => true,
            &Term::Abs(..) => true,
            &Term::Var(_) => true,
            &Term::Int(_) => true,
            _ => false,
        }
    }

    pub fn unparse(&self) -> String {
        match self {
            &Term::True => "#T".into(),
            &Term::False => "#F".into(),
            &Term::Not => "!".into(),
            &Term::Int(n) => n.to_string(),
            &Term::Var(ref x) => x.to_string(),
            &Term::App(ref t1, ref t2) => {
                let mut arg_str = String::new();
                for arg in t2 {
                    arg_str.push_str(&format!("{} ", arg.unparse()));
                }
                arg_str.pop();
                format!("({} {})", t1.unparse(), arg_str)
            }
            &Term::Abs(ref args, ref b) => {
                let mut arg_str: String = String::new();
                for (_, type_) in args.iter().enumerate() {
                    arg_str.push_str(&format!(": {}, ", type_.unparse()));
                }
                arg_str.pop();
                arg_str.pop();
                // arg_str.
                format!("(/lam {}. {})", arg_str, b.unparse())
            }
            &Term::If(ref cond, ref b1, ref b2) => {
                format!("(If {} {} {})", cond.unparse(), b1.unparse(), b2.unparse())
            }
            &Term::Stuck => format!("#STUCK#"),
        }
    }

    // TODO
    pub fn get_vars(&self) -> Vec<u32> {
        let mut ret = Vec::new();

        match self {
            &Term::True => {}
            &Term::False => {}
            &Term::Not => {}
            &Term::Int(_) => {}
            &Term::Var(x) => {
                ret.push(x);
            }
            &Term::App(ref f, ref args) => {
                for term in args {
                    ret.extend(term.get_vars());
                }
                ret.append(&mut f.get_vars());
            }
            &Term::Abs(_, ref body) => {
                ret.append(&mut body.get_vars());
            }
            &Term::If(ref cond, ref b1, ref b2) => {
                let (cv, b1v, b2v) = (cond.get_vars(), b1.get_vars(), b2.get_vars());

                ret.extend(cv.iter().chain(b1v.iter()).chain(b2v.iter()));
            }
            &Term::Stuck => {}
        };

        ret
    }

    pub fn app(f: Term, x: &[Term]) -> Term {
        Term::App(Box::new(f), Vec::from(x))
    }

    pub fn if_(cond: Term, then: Term, else_: Term) -> Term {
        Term::If(Box::new(cond), Box::new(then), Box::new(else_))
    }

    pub fn abs(ty: &[Ty], body: Term) -> Term {
        Term::Abs(Vec::from(ty), Box::new(body))
    }
}
