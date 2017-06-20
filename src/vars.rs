use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

pub trait VarAbs: Clone {
    type Var: Display + Clone + PartialEq + Hash + Eq;

    // fn new_va() -> Self;
    fn extend(&mut self, Self::Var);
    fn lookup(&self, Self::Var) -> Option<Self::Var>;
    fn has(&self, v: Self::Var) -> bool {
        self.lookup(v).is_some()
    }
    fn remove(&mut self, Self::Var) -> bool;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BasicVar {
    vs: HashSet<String>,
}

impl VarAbs for BasicVar {
    type Var = String;

    // fn new_va() -> BasicVar {
    //     BasicVar { vs: HashSet::new() }
    // }

    fn extend(&mut self, v: String) {
        self.vs.insert(v);
    }

    fn lookup(&self, v: String) -> Option<String> {
        self.vs.get(&v).cloned()
    }

    fn remove(&mut self, v: String) -> bool {
        self.vs.remove(&v)
    }

    fn is_empty(&self) -> bool {
        self.vs.is_empty()
    }

    fn clear(&mut self) {
        self.vs.clear();
    }
}
