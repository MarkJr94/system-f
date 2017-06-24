use core::{Term, Ty};
use errors::*;
use visitor as v;
use visitor::Visitor;
use toplevel::Term as TTerm;

use std::collections::HashMap;

type BVar = String;

pub struct RenameVisitor {
    vs: HashMap<BVar, Vec<u32>>,
    state: Vec<Term>,
}

impl RenameVisitor {
    pub fn new() -> RenameVisitor {
        RenameVisitor {
            vs: HashMap::new(),
            state: Vec::new(),
        }
    }

    pub fn rename_term(&mut self, term: &TTerm) -> Result<Term> {
        v::walk_term(self, term);
        let ret = self.state.pop().ok_or(rename_err("something weird"));
        self.reset();
        ret
    }

    fn reset(&mut self) {
        self.vs.clear();
        self.state.clear();
    }
}

impl<'a> Visitor<'a> for RenameVisitor {
    fn visit_true(&mut self) {
        self.state.push(Term::True);
    }

    fn visit_false(&mut self) {
        self.state.push(Term::False);
    }

    fn visit_not(&mut self) {
        self.state.push(Term::Not);
    }

    fn visit_bottom(&mut self) {
        self.state.push(Term::Stuck);
    }

    fn visit_int(&mut self, n: i64) {
        self.state.push(Term::Int(n));
    }

    fn visit_abs(&mut self, args: &[(String, Ty)], body: &'a TTerm) {


        for &(ref v, _) in args {
            // Increase distance from binder of all variables
            for stack in self.vs.values_mut() {
                for vi in stack.iter_mut() {
                    *vi += 1;
                }
            }

            // Adding new scope or creating 1st
            let test = if let Some(scope_stack) = self.vs.get_mut(v) {
                scope_stack.push(1);
                false
            } else {
                true
            };

            if test {
                self.vs.insert(v.to_owned(), vec![1]);
            }
        }

        for &(ref s, _) in args {
            let c = args.iter()
                .fold(0,
                      |acc, &(ref elem, _)| if elem == s { acc + 1 } else { acc });

            if c > 1 {
                panic!("Do not duplicate argument names");
            }
        }

        let new_tys = args.iter().map(|&(_, ref ty)| ty.clone()).collect();

        v::walk_term(self, body);
        let new_body = self.state.pop().unwrap();
        self.state.push(Term::Abs(new_tys, Box::new(new_body)));

        // Decreasing depth and removing scope
        for stack in self.vs.values_mut() {
            for vi in stack.iter_mut() {
                *vi -= 1;
            }
            stack.pop();
        }
    }

    fn visit_var(&mut self, v: &'a str) {
        let idx = self.vs
            .get_mut(v)
            .expect("This should never happen, means free variables")
            .last()
            .expect("This also shouldn't happen");

        self.state.push(Term::Var(*idx));
    }

    fn visit_app(&mut self, f: &'a TTerm, args: &'a [TTerm]) {
        v::walk_app(self, f, args);

        let mut new_args = Vec::new();
        for _ in args {
            new_args.push(self.state.pop().unwrap());
        }
        let f = self.state.pop().unwrap();
        self.state
            .push(Term::App(Box::new(f), new_args.drain(..).rev().collect()));
    }

    fn visit_if(&mut self, cond: &'a TTerm, b1: &'a TTerm, b2: &'a TTerm) {
        v::walk_if(self, cond, b1, b2);
        let (e, t, c) =
            (self.state.pop().unwrap(), self.state.pop().unwrap(), self.state.pop().unwrap());
        self.state.push(Term::if_(c, t, e));
    }
}

#[cfg(test)]
mod test {
    use super::RenameVisitor;
    use toplevel::Term;
    use lispy::scan::parse;
    use lispy::tokenize::scan;

    fn get(s: &str) -> Term {
        let term = parse(scan(s.as_bytes()).unwrap()).unwrap();

        term
    }

    #[test]
    fn test_rename() {
        let mut rv = RenameVisitor::new();

        let t = rv.rename_term(&get("(/lam x: #B. x)")).unwrap();
        assert_eq!("(/lam : #B. 1)", t.unparse());

        let t = rv.rename_term(&get("(/lam x: (#B -> (#B -> #B)). (/lam z: #B. z))"))
            .unwrap();
        assert_eq!("(/lam : (#B -> (#B -> #B)). (/lam : #B. 1))", t.unparse());

        let t = rv.rename_term(&get("(/lam y: ((#B -> #B) -> #B). (y (/lam y: #B. y)))"))
            .unwrap();
        assert_eq!("(/lam : ((#B -> #B) -> #B). (1 (/lam : #B. 1)))",
                   t.unparse());

        let t = rv.rename_term(&get("(/lam x: #B. (/lam y: #B. (/lam z: #B. ((x z) (y z)) ) ))"))
            .unwrap();
        assert_eq!("(/lam : #B. (/lam : #B. (/lam : #B. ((3 1) (2 1)))))",
                   t.unparse());

        // K Combinator
        let t = rv.rename_term(&get("(/lam x: #B. (/lam y: #B. x))"))
            .unwrap();
        assert_eq!("(/lam : #B. (/lam : #B. 2))", t.unparse());
    }
}
