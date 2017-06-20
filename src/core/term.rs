use super::ty::Ty;

#[derive(Clone,PartialEq,Eq, Debug)]
pub enum Term {
    Var(u32),
    Abs(Ty, Box<Term>),
    App(Box<Term>, Box<Term>),
    True,
    False,
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
            &Term::Abs(ref t, ref b) => format!("(lam : {}. {})", t.unparse(), b.unparse()),
            &Term::If(ref cond, ref b1, ref b2) => {
                format!("(If {} {} {})", cond.unparse(), b1.unparse(), b2.unparse())
            }
            &Term::Stuck => format!("#STUCK#"),
        }
    }

    // TODO
    pub fn get_vars(&self) -> Vec<u32> {
        let cur = self;
        let mut ret = Vec::new();

        match self {
            &Term::True => {}
            &Term::False => {}
            &Term::Not => {}
            &Term::Var(x) => {
                ret.push(x);
            }
            &Term::App(ref f, ref x) => {
                ret.append(&mut x.get_vars());
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
}
