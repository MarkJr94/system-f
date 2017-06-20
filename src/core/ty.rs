#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Ty {
    Bool,
    Arrow(Box<Ty>, Box<Ty>),
    Bottom,
}

impl Ty {
    pub fn unparse(&self) -> String {
        match self {
            &Ty::Bool => "B".into(),
            &Ty::Arrow(ref t1, ref t2) => format!("({} -> {})", t1.unparse(), t2.unparse()),
            &Ty::Bottom => "_|_".into(),
        }
    }
}

pub fn arrow(t1: Ty, t2: Ty) -> Ty {
    Ty::Arrow(Box::new(t1), Box::new(t2))
}
