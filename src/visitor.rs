use std::collections::HashSet;

use core::Ty;
use toplevel::Term;

pub trait Visitor<'a>: Sized {
    // type O;

    fn visit_var(&mut self, _: &'a str) {}
    fn visit_int(&mut self, _: i64) {}
    fn visit_true(&mut self) {}
    fn visit_false(&mut self) {}
    fn visit_not(&mut self) {}
    fn visit_ty(&mut self, _: &'a Ty) {}
    fn visit_bottom(&mut self) {}
    fn visit_abs(&mut self, args: &'a [(String, Ty)], body: &'a Term) {
        walk_abs(self, args, body);
    }
    fn visit_term(&mut self, term: &'a Term) {
        walk_term(self, term);
    }
    fn visit_app(&mut self, f: &'a Term, args: &'a [Term]) {
        walk_app(self, f, args);
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

pub fn walk_app<'a, V>(v: &mut V, f: &'a Term, args: &'a [Term])
    where V: Visitor<'a>
{
    v.visit_term(f);
    for x in args {
        v.visit_term(x);
    }
}

pub fn walk_abs<'a, V>(v: &mut V, args: &'a [(String, Ty)], body: &'a Term)
    where V: Visitor<'a>
{


    for &(ref var, ref ty) in args {
        v.visit_var(var);
        v.visit_ty(ty);
    }

    v.visit_term(body);
}

pub fn walk_term<'a, V>(v: &mut V, term: &'a Term)
    where V: Visitor<'a>
{
    match term {
        &Term::Var(ref var) => {
            v.visit_var(var);
        }
        &Term::Abs(ref args, ref body) => {
            v.visit_abs(args, body);
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
        &Term::Bottom => {
            v.visit_bottom();
        }
        &Term::Int(n) => {
            v.visit_int(n);
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

    fn visit_abs(&mut self, args: &'a [(String, Ty)], body: &'a Term) {
        walk_abs(self, args, body);
        for &(ref var, _) in args {
            self.vs.remove(var.as_str());
        }
    }
}

#[cfg(test)]
mod test {
    use super::FVVisitor;
    use toplevel::Term;
    use lispy::scan::parse;
    use lispy::tokenize::scan;

    fn get(s: &str) -> Term {
        let term = parse(scan(s.as_bytes()).unwrap()).unwrap();

        term
    }

    #[test]
    fn test_fv() {

        {
            let check_all = |code: &str, fvs: &[&str]| {
                let term = &get(code);
                let fv = FVVisitor::new(term);

                let ot = fv.open_terms();
                for v in fvs.iter() {
                    assert!(ot.contains((*v).into()));
                }
            };

            check_all("x", &["x"]);
            check_all("(x x)", &["x"]);
            check_all("(x y)", &["x", "y"]);
            check_all("#T", &[]);
            check_all("(if #T x y)", &["x", "y"]);
            check_all("((/lam x: #B.x) y)", &["y"])
        }

        assert!(FVVisitor::new(&get("(/lam x: #B.x)")).is_closed());
        assert!(!FVVisitor::new(&get("(/lam x: #B.(y x))")).is_closed());
    }
}
