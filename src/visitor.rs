use toplevel::{Term, Ty};
use vars as v;
use vars::{Var, VarSet};

pub trait Visitor<'a>: Sized {
    // type State;

    fn visit_var(&mut self, v: &'a v::Var) { }
    fn visit_true(&mut self) { }
    fn visit_false(&mut self) { }
    fn visit_not(&mut self) { }
    fn visit_ty(&mut self, ty: &'a Ty) { }
    fn visit_abs(&mut self, var: &'a v::Var, ty: &'a Ty, body: &'a Term) {
        walk_abs(self, var, ty, body);
    }
    fn visit_term(&mut self, term: &'a Term) {
        walk_term(self, term);
    }
    fn visit_app(&mut self, f: &'a Term, x: &'a Term) {
        walk_app(self, f, x);
    }
    fn visit_if(&mut self, cond: &'a Term, b1: &'a Term, b2: &'a Term) {
        walk_if(self, cond, b1, b2);
    }
}

pub fn walk_if<'a, V: Visitor<'a>>(v: &mut V, cond: &'a Term, b1: &'a Term, b2: &'a Term) {
    v.visit_term(cond);
    v.visit_term(b1);
    v.visit_term(b2);
}

pub fn walk_app<'a, V: Visitor<'a>>(v: &mut V, f: &'a Term, x: &'a Term) {
    v.visit_term(f);
    v.visit_term(x);
}

pub fn walk_abs<'a, V: Visitor<'a>>(v: &mut V, var: &'a v::Var, ty: &'a Ty, body: &'a Term) {
    v.visit_var(var);
    v.visit_ty(ty);
    v.visit_term(body);
}

pub fn walk_term<'a, V: Visitor<'a>>(v: &mut V, term: &'a Term) {
    match term {
        &Term::Var(ref var) => { v.visit_var(var); }
        &Term::Abs(ref var, ref ty, ref body) => { v.visit_abs(var, ty, body); }
        &Term::App(ref f, ref x) => { v.visit_app(f, x); }
        &Term::If(ref cond, ref b1, ref b2) => { v.visit_if(cond, b1, b2); }
        &Term::True => { v.visit_true(); }
        &Term::False => { v.visit_false(); }
        &Term::Not => { v.visit_not(); }
    }
}

pub struct FVVisitor {
    vs: VarSet,
}

impl FVVisitor {
    pub fn new() -> FVVisitor {
        FVVisitor { vs: VarSet::new() }
    }

    // pub fn fvs(&self) -> &VarSet { &self.vs }

    pub fn open_terms(&mut self, t: &Term) -> VarSet {
        walk_term(self, t);
        let ret = self.vs.clone();
        self.vs.clear();
        ret
    }

    pub fn is_closed(&mut self, t: &Term) -> bool {
        walk_term(self, t);
        let ret = self.vs.is_empty();
        self.vs.clear();
        ret
    }

    pub fn check(&mut self, t: &Term) -> Result<(), String> {
        if !self.open_terms(t).is_empty() {
            Err(format!("Open Terms found in {}!", t.unparse()))
        } else {
            Ok(())
        }
    }
}

impl<'a> Visitor<'a> for FVVisitor {
    fn visit_var(&mut self, v: &'a v::Var) {
        self.vs.insert(v.clone());
    }

    fn visit_abs(&mut self, var: &'a v::Var, ty: &'a Ty, body: &'a Term) {
        self.vs.remove(var);
        walk_abs(self, var, ty, body);
    }
}

#[cfg(test)]
mod test {
    use super::{Visitor, FVVisitor};
    use toplevel::{Term, Ty};

    #[test]
    fn test_fv() {
        let mut fv = FVVisitor::new();

        assert_eq!(fv.open_terms(&Term::Var("x".into())), hashset!{String::from("x")});
    }
}
