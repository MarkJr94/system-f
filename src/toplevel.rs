use vars::{VarAbs, BasicVar};

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Ty {
    Bool,
    Arrow(Box<Ty>, Box<Ty>),
    Bottom,
}

pub fn arrow(alpha: Ty, beta: Ty) -> Ty {
    Ty::Arrow(Box::new(alpha), Box::new(beta))
}

#[derive(Clone,PartialEq,Eq, Debug)]
pub enum Term<V: VarAbs> {
    Var(V::Var),
    Abs(V::Var, Ty, Box<Term<V>>),
    App(Box<Term<V>>, Box<Term<V>>),
    True,
    False,
    Not,
    If(Box<Term<V>>, Box<Term<V>>, Box<Term<V>>),
}

pub type BasicTerm = Term<BasicVar>;

impl Ty {
    pub fn unparse(&self) -> String {
        match self {
            &Ty::Bool => "B".into(),
            &Ty::Arrow(ref t1,ref t2) => format!("({} -> {})", t1.unparse(),  t2.unparse()),
            &Ty::Bottom => "_|_".into(),
        }
    }
}

impl<V: VarAbs> Term<V> {
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
            &Term::Abs(ref x, ref t, ref b) =>
                format!("(\\l {}: {}. {})", x, t.unparse(), b.unparse()),
            &Term::If(ref cond, ref b1, ref b2) => format!("(If {} {} {})", cond.unparse(), b1.unparse(), b2.unparse()),
        }
    }
}
