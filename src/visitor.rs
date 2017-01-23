use toplevel::{Term, Ty};
use vars::{VarAbs};

pub trait Visitor<'a, VA: VarAbs>: Sized {
    // type O;

    fn visit_var(&mut self, _: &'a VA::Var) { }
    fn visit_true(&mut self) { }
    fn visit_false(&mut self) { }
    fn visit_not(&mut self) { }
    fn visit_ty(&mut self, _: &'a Ty) { }
    fn visit_abs(&mut self, var: &'a VA::Var, ty: &'a Ty, body: &'a Term<VA>) {
        walk_abs(self, var, ty, body);
    }
    fn visit_term(&mut self, term: &'a Term<VA>) {
        walk_term(self, term);
    }
    fn visit_app(&mut self, f: &'a Term<VA>, x: &'a Term<VA>) {
        walk_app(self, f, x);
    }
    fn visit_if(&mut self, cond: &'a Term<VA>, b1: &'a Term<VA>, b2: &'a Term<VA>) {
        walk_if(self, cond, b1, b2);
    }
}

pub fn walk_if<'a, V, VA>(v: &mut V, cond: &'a Term<VA>, b1: &'a Term<VA>, b2: &'a Term<VA>)
    where VA: VarAbs,
          V: Visitor<'a, VA> {
    v.visit_term(cond);
    v.visit_term(b1);
    v.visit_term(b2);
}

pub fn walk_app<'a, V, VA>(v: &mut V, f: &'a Term<VA>, x: &'a Term<VA>)
    where VA: VarAbs,
          V: Visitor<'a, VA> {
    v.visit_term(f);
    v.visit_term(x);
}

pub fn walk_abs<'a, V, VA>(v: &mut V, var: &'a VA::Var, ty: &'a Ty, body: &'a Term<VA>)
    where VA: VarAbs,
          V: Visitor<'a, VA> {
    v.visit_var(var);
    v.visit_ty(ty);
    v.visit_term(body);
}

pub fn walk_term<'a, V, VA>(v: &mut V, term: &'a Term<VA>)
    where VA: VarAbs,
          V: Visitor<'a, VA> {
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

pub struct FVVisitor<VA: VarAbs> {
    vs: VA,
}

impl<VA> FVVisitor<VA>
    where VA: VarAbs {

    pub fn new() -> FVVisitor<VA> {
        FVVisitor { vs: VA::new_va() }
    }

    pub fn open_terms(&mut self, t: &Term<VA>) -> VA {
        walk_term(self, t);
        let ret = self.vs.clone();
        self.vs.clear();
        ret
    }

    pub fn is_closed(&mut self, t: &Term<VA>) -> bool {
        walk_term(self, t);
        let ret = self.vs.is_empty();
        self.vs.clear();
        ret
    }

    pub fn check(&mut self, t: &Term<VA>) -> Result<(), String> {
        if !self.open_terms(t).is_empty() {
            Err(format!("Open Terms found in {}!", t.unparse()))
        } else {
            Ok(())
        }
    }
}

impl<'a, VA> Visitor<'a, VA> for FVVisitor<VA>
    where VA: VarAbs {
    fn visit_var(&mut self, v: &'a VA::Var) {
        self.vs.extend(v.clone());
    }

    fn visit_abs(&mut self, var: &'a VA::Var, ty: &'a Ty, body: &'a Term<VA>) {
        walk_abs(self, var, ty, body);
        self.vs.remove(var.clone());
    }
}

#[cfg(test)]
mod test {
    use super::FVVisitor;
    use toplevel::{Term};
    use parser::{parse_Term};
    use vars::{VarAbs, BasicVar};

    fn get(s: &str) -> Term<BasicVar> {
        parse_Term(s).unwrap()
    }

    // fn check_all(code: &str, fvs: &[&str]) {
    //     let ot =
    // }

    #[test]
    fn test_fv() {
        let mut fv = FVVisitor::new();

        {
            let mut check_all = |code: &str, fvs: &[&str]| {
                let ot = fv.open_terms(&get(code));
                for v in fvs.iter() {
                    assert!(ot.has((*v).into()));
                }
            };

            check_all("x", &["x"]);
            check_all("(x x)", &["x"]);
            check_all("(x y)", &["x", "y"]);
            check_all("#T", &[]);
            check_all("(if #T x y)", &["x", "y"]);
            check_all("((lam x: #B.x) y)", &["y"])
        }

        // assert_eq!(fv.open_terms(&get("x")), hashset!{"x".into()});
        // assert_eq!(fv.open_terms(&get("(x x)")), hashset!{"x".into()});
        // assert_eq!(fv.open_terms(&get("(x y)")), hashset!{"x".into(), "y".into()});
        // assert_eq!(fv.open_terms(&get("#T")), hashset!{});
        // assert_eq!(fv.open_terms(&get("(if #T x y)")), hashset!{"x".into(), "y".into()});
        // assert_eq!(fv.open_terms(&get("((lam x: #B.x) y)")), hashset!{"y".into()});
        assert!(fv.is_closed(&get("(lam x: #B.x)")));
        assert!(!fv.is_closed(&get("(lam x: #B.(y x))")));
    }
}
