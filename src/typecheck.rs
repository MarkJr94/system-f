use toplevel::{Term, Ty};
use vars::{VarAbs};
use visitor as v;
use visitor::{Visitor};

use std::collections::{HashMap};

pub type TypeEnv<VA> = HashMap<VA, Ty>;

pub struct TypeCheckVisitor<VA>
    where VA: VarAbs {

    gamma: TypeEnv<VA::Var>,
    ty: Ty,
    failed: bool,
}

impl<VA> TypeCheckVisitor<VA>
    where VA: VarAbs {

    pub fn new() -> TypeCheckVisitor<VA> {
        TypeCheckVisitor {
            gamma: TypeEnv::new(),
            ty: Ty::Bottom,
            failed: false,
        }
    }

    pub fn type_of(&mut self, t: &Term<VA>) -> Ty {
        v::walk_term(self, t);
        let ret = self.ty.clone();
        self.reset();
        ret
    }

    pub fn is_typed(&mut self, t: &Term<VA>) -> bool {
        v::walk_term(self, t);
        let ret = !self.failed;
        self.reset();
        ret
    }

    fn reset(&mut self) {
        self.ty = Ty::Bottom;
        self.failed = false;
        self.gamma.clear();
    }
}

impl<'a, VA> Visitor<'a, VA> for TypeCheckVisitor<VA>
    where VA: VarAbs {

    fn visit_true(&mut self) {
        self.ty = Ty::Bool;
    }

    fn visit_false(&mut self) {
        self.ty = Ty::Bool;
    }

    fn visit_not(&mut self) {
        self.ty = Ty::Arrow(Box::new(Ty::Bool), Box::new(Ty::Bool));
    }

    fn visit_var(&mut self, v: &'a VA::Var) {
        match self.gamma.get(v) {
            Some(ty) => { self.ty = ty.clone(); }
            None => {
                self.ty = Ty::Bottom;
                self.failed = true;
            }
        }
    }

    fn visit_abs(&mut self, v: &'a VA::Var, ty_var: &'a Ty, body: &'a Term<VA>) {
        self.gamma.insert(v.clone(), ty_var.clone());
        v::walk_term(self, body);
        let ty_body = self.ty.clone();
        self.ty = Ty::Arrow(Box::new(ty_var.clone()), Box::new(ty_body));
    }

    fn visit_if(&mut self, cond: &'a Term<VA>, b1: &'a Term<VA>, b2: &'a Term<VA>) {
        v::walk_term(self, cond);
        let ty_cond = self.ty.clone();
        v::walk_term(self, b1);
        let ty_b1 = self.ty.clone();
        v::walk_term(self, b2);
        let ty_b2 = self.ty.clone();

        if ty_cond == Ty::Bool {
            if ty_b1 == ty_b2 {
                self.ty = ty_b1.clone();
            }
        } else {
            self.ty = Ty::Bottom;
            self.failed = true;
        }
    }

    fn visit_app(&mut self, f: &'a Term<VA>, x: &'a Term<VA>) {
        v::walk_term(self, f);
        let ty_f = self.ty.clone();
        v::walk_term(self, x);
        let ty_x = self.ty.clone();

        match ty_f {
            Ty::Arrow(arg, res) => {
                if &*arg == &ty_x {
                    self.ty = *res;
                } else {
                    // Type mismatch in function App
                    self.ty = Ty::Bottom;
                    self.failed = true;
                }
            }
            _ => {
                // Attempt to apply non-function as function
                self.ty = Ty::Bottom;
                self.failed = true;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::TypeCheckVisitor;
    use toplevel::{Term, Ty, arrow};
    use parser::{parse_Term};
    use vars::{BasicVar};

    fn get(s: &str) -> Term<BasicVar> {
        parse_Term(s).unwrap()
    }

    #[test]
    fn test_tyck() {
        let mut tc = TypeCheckVisitor::new();

        assert_eq!(tc.type_of(&get("#T")), Ty::Bool);
        assert_eq!(tc.type_of(&get("!")), arrow(Ty::Bool, Ty::Bool));
        assert_eq!(tc.type_of(&get("(lam x: #B.x)")), arrow(Ty::Bool, Ty::Bool));
        assert_eq!(tc.type_of(&get("(! #T)")),Ty::Bool);
        assert_eq!(tc.type_of(&get("!")), arrow(Ty::Bool, Ty::Bool));
        assert_eq!(tc.type_of(&get("((lam x: #B.x) #T)")), Ty::Bool);
        assert_eq!(tc.type_of(&get("(if (! #F) ! (lam x: #B. x))")), arrow(Ty::Bool, Ty::Bool));
    }
}
