use std::collections::HashSet;

use ast::term::Ty;
use toplevel::Term;
// use vars::{VarAbs};

pub trait Visitor<'a>: Sized {
    // type O;

    fn visit_var(&mut self, _: &'a str) {}
    fn visit_true(&mut self) {}
    fn visit_false(&mut self) {}
    fn visit_not(&mut self) {}
    fn visit_ty(&mut self, _: &'a Ty) {}
    fn visit_abs(&mut self, var: &'a str, ty: &'a Ty, body: &'a Term) {
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

pub fn walk_if<'a, V>(v: &mut V, cond: &'a Term, b1: &'a Term, b2: &'a Term)
    where V: Visitor<'a>
{
    v.visit_term(cond);
    v.visit_term(b1);
    v.visit_term(b2);
}

pub fn walk_app<'a, V>(v: &mut V, f: &'a Term, x: &'a Term)
    where V: Visitor<'a>
{
    v.visit_term(f);
    v.visit_term(x);
}

pub fn walk_abs<'a, V>(v: &mut V, var: &'a str, ty: &'a Ty, body: &'a Term)
    where V: Visitor<'a>
{
    v.visit_var(var);
    v.visit_ty(ty);
    v.visit_term(body);
}

pub fn walk_term<'a, V>(v: &mut V, term: &'a Term)
    where V: Visitor<'a>
{
    match term {
        &Term::Var(ref var) => {
            v.visit_var(var);
        }
        &Term::Abs(ref var, ref ty, ref body) => {
            v.visit_abs(var, ty, body);
        }
        &Term::App(ref f, ref x) => {
            v.visit_app(f, x);
        }
        &Term::If(ref cond, ref b1, ref b2) => {
            v.visit_if(cond, b1, b2);
        }
        &Term::True => {
            v.visit_true();
        }
        &Term::False => {
            v.visit_false();
        }
        &Term::Not => {
            v.visit_not();
        }
    }
}

pub struct FVVisitor<'a> {
    vs: HashSet<&'a str>,
    term: &'a Term,
}

impl<'a> FVVisitor<'a> {
    pub fn new(term: &'a Term) -> FVVisitor<'a> {
        let mut fv = FVVisitor {
            vs: HashSet::new(),
            term: term,
        };

        walk_term(&mut fv, term);
        fv
    }

    pub fn open_terms(&'a self) -> HashSet<&'a str> {
        self.vs.clone()
    }

    pub fn is_closed(&'a self) -> bool {
        self.vs.is_empty()
    }

    pub fn check(&self) -> Result<(), String> {
        if !self.is_closed() {
            Err(format!("Open Terms found in {}!", self.term.unparse()))
        } else {
            Ok(())
        }
    }
}

impl<'a> Visitor<'a> for FVVisitor<'a> {
    fn visit_var(&mut self, v: &'a str) {
        self.vs.insert(v);
    }

    fn visit_abs(&mut self, var: &'a str, ty: &'a Ty, body: &'a Term) {
        walk_abs(self, var, ty, body);
        self.vs.remove(var);
    }
}

#[cfg(test)]
mod test {
    use super::FVVisitor;
    use toplevel::Term;
    use parser::parse_Term;
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
