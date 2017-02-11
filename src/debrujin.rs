use ast::term::{Term, Ty};
use vars::{VarAbs, BasicVar};
use visitor as v;
use visitor::{Visitor};
use toplevel::{Term as TTerm};

use std::collections::{HashMap, HashSet};

type BVar = <BasicVar as VarAbs>::Var;

pub struct RenameVisitor {
    vs: HashMap<BVar, Vec<u32>>,
    state: Vec<Term>,
    n: u32,
}

impl RenameVisitor {
    pub fn new() -> RenameVisitor {
        RenameVisitor {
            vs: HashMap::new(),
            state: Vec::new(),
            n: 0,
        }
    }

    pub fn rename_term(&mut self, term: &TTerm) -> Term {
        v::walk_term(self, term);
        let ret = self.state.pop().expect("lol");
        self.reset();
        ret
    }

    fn reset(&mut self) {
        self.n = 0;
        self.vs.clear();
        self.state.clear();
    }
}

impl<'a> Visitor<'a> for RenameVisitor {
    fn visit_abs(&mut self, v: &'a str, ty: &'a Ty, body: &'a TTerm) {
        // Increase distance from binder of all variables
        for stack in self.vs.values_mut() {
            for vi in stack.iter_mut() {
                *vi += 1;
            }
        }

        self.n += 1;

        let old_n = self.n;

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

        v::walk_term(self, body);
        let new_body = self.state.pop().unwrap();
        self.state.push(Term::Abs(ty.clone(), Box::new(new_body)));

        // Decreasing depth and removing scope
        self.n -= 1;
        for stack in self.vs.values_mut() {
            for vi in stack.iter_mut() {
                *vi -= 1;
            }
            stack.pop();
        }
    }

    fn visit_var(&mut self, v: &'a str) {
        let idx = self.vs.get_mut(v)
            .expect("This should never happen, means free variables")
            .last()
            .expect("This also shouldn't happen");

        self.state.push(Term::Var(*idx));
    }

    fn visit_app(&mut self, f: &'a TTerm, x: &'a TTerm) {
        v::walk_app(self, f, x);
        let (x, f) = (self.state.pop().unwrap(), self.state.pop().unwrap());
        self.state.push(Term::App(Box::new(f), Box::new(x)));
    }
}

#[cfg(test)]
mod test {
    use super::RenameVisitor;
    use toplevel::{Term, Ty, arrow};
    use parser::{parse_Term};
    use vars::{BasicVar};

    fn get(s: &str) -> Term<BasicVar> {
        parse_Term(s).unwrap()
    }

    #[test]
    fn test_rename() {
        let mut rv = RenameVisitor::new();

        let t = rv.rename_term(&get("(lam x: #B. x)"));
        assert_eq!("(lam : B. 1)", t.unparse());

        let t = rv.rename_term(&get("(lam x: (#B -> (#B -> #B)). (lam z: #B. z))"));
        assert_eq!("(lam : (B -> (B -> B)). (lam : B. 1))",t.unparse());

        let t = rv.rename_term(&get("(lam y: ((#B -> #B) -> #B). (y (lam y: #B. y)))"));
        assert_eq!("(lam : ((B -> B) -> B). (1 (lam : B. 1)))", t.unparse());

        let t = rv.rename_term(&get("(lam x: #B. (lam y: #B. (lam z: #B. ((x z) (y z)) ) ))"));
        assert_eq!("(lam : B. (lam : B. (lam : B. ((3 1) (2 1)))))", t.unparse());

        // K Combinator
        let t = rv.rename_term(&get("(lam x: #B. (lam y: #B. x))"));
        assert_eq!("(lam : B. (lam : B. 2))", t.unparse());
    }
}
