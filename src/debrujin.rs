use toplevel::{Term, Ty};
use vars::{VarAbs, BasicVar};
use visitor as v;
use visitor::{Visitor};

use std::collections::{HashMap, HashSet};

type BVar = <BasicVar as VarAbs>::Var;

pub struct RenameVisitor {
    vs: HashSet<BVar>,
    state: Term<BasicVar>,
    n: i32,
}

impl RenameVisitor {
    pub fn new() -> RenameVisitor
}

impl<'a> Visitor<'a, BasicVar> for DeBrujinVisitor {
    fn visit_abs(&mut self, v: &'a BVar, ty: &'a Ty, body: &'a Term<BasicVar>) {
        if let Some(var) = self.vs.get(v) {
            self.state = Term::Abs()
        }
    }

    fn visit_var(&mut self, v: &'a VA::Var) {
        if let Some(idx) = self.vs.get(v) {

        }
    }
}

fn
