use std::fmt;

#[derive(Clone,PartialEq,Eq,Debug, PartialOrd, Hash, Ord, Copy)]
pub enum TyLit {
    Bool,
    Int,
}

impl TyLit {
    pub fn from_lit(lit: &str) -> Option<TyLit> {
        match lit {
            "#B" => Some(TyLit::Bool),
            "#Int" => Some(TyLit::Int),
            _ => None,
        }
    }

    pub fn to_lit(&self) -> &str {
        match *self {
            TyLit::Int => "#Int",
            TyLit::Bool => "#B",
        }
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Ty {
    Base(TyLit),
    Arrow(Vec<Ty>, Box<Ty>),
    Bottom,
}

impl From<TyLit> for Ty {
    fn from(t: TyLit) -> Ty {
        Ty::Base(t)
    }
}

impl Ty {
    pub fn unparse(&self) -> String {
        match self {
            &Ty::Base(TyLit::Bool) => "#B".into(),
            &Ty::Base(TyLit::Int) => "#Int".into(),
            &Ty::Arrow(ref t1, ref t2) => {
                let mut ty_str = String::new();

                for ty in t1 {
                    ty_str.push_str(&format!("{}, ", ty.unparse()));
                }
                ty_str.pop();
                ty_str.pop();

                format!("({} -> {})", ty_str, t2.unparse())
            }
            &Ty::Bottom => "_|_".into(),
        }
    }

    pub fn arrow(t1: &[Ty], t2: Ty) -> Ty {
        Ty::Arrow(Vec::from(t1), Box::new(t2))
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.unparse())
    }
}
