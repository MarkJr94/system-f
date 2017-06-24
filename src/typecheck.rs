use core::{Term, Ty, TyLit};
use core::supervisitor::{self as sv, SuperVisitor, TermFold};
use errors::*;

pub type TypeEnv = Vec<Ty>;

pub struct TypeCheckVisitor {
    gamma: TypeEnv,
}

pub type Judgement = Result<Ty>;

impl TermFold for Judgement {
    fn empty<'a, SV>(_: &mut SV) -> Judgement
        where SV: SuperVisitor<'a, Output = Self>
    {
        Ok(Ty::Bottom)
    }

    fn fold_if<'a, SV>(_: &mut SV, cond: Judgement, then: Judgement, else_: Judgement) -> Judgement
        where SV: SuperVisitor<'a, Output = Self>
    {
        let (c, t, e) = (cond?, then?, else_?);

        match (c == Ty::Base(TyLit::Bool), t == e) {
            (true, true) => Ok(t),
            (false, true) => Err(type_err("If condition is not boolean")),
            (true, false) => Err(type_err("If condition branches differ in type")),
            (false, false) => Err(type_err("simply terrible")),
        }
    }

    fn fold_app<'a, SV>(_: &mut SV, f: Judgement, js: &[Judgement]) -> Judgement
        where SV: SuperVisitor<'a, Output = Self>
    {
        let f = f?;

        match f {
            Ty::Arrow(arg, res) => {
                let mut test = true;
                for (x, y) in arg.iter().zip(js.iter()) {
                    test &= if let &Ok(ref j) = y { x == j } else { false };

                }

                test &= js.len() == arg.len();

                println!("js={:?}", js);
                println!("arg={:?}", arg);

                if test {
                    Ok(res.as_ref().clone())
                } else {
                    Err(type_err("Type mismatch in function application"))
                }

            }
            _ => Err(type_err("Non-function in function application position")),
        }
    }

    fn fold_abs<'a, SV>(_: &mut SV, ty: &[Judgement], body: Judgement) -> Judgement
        where SV: SuperVisitor<'a, Output = Self>
    {


        let huh = ty.iter()
            .fold(Ok(Vec::new()), |acc, elem| if let Ok(mut vec) = acc {
                if elem.is_ok() {
                    let j = elem.as_ref().unwrap().clone();
                    vec.push(j);
                    Ok(vec)
                } else {
                    Err(type_err("error among abs whatever"))
                }
            } else {
                acc
            });

        let ty = huh?;
        let by = body?;
        Ok(Ty::arrow(&ty, by))
    }
}

impl TypeCheckVisitor {
    pub fn new() -> TypeCheckVisitor {
        TypeCheckVisitor { gamma: TypeEnv::new() }
    }

    pub fn type_of(&mut self, t: &Term) -> Judgement {
        let ret = sv::walk_term(self, t);
        self.reset();
        ret
    }

    fn reset(&mut self) {
        self.gamma.clear();
    }
}

impl<'a> SuperVisitor<'a> for TypeCheckVisitor {
    type Output = Judgement;

    fn visit_true(&mut self) -> Judgement {
        Ok(TyLit::Bool.into())
    }

    fn visit_false(&mut self) -> Judgement {
        Ok(TyLit::Bool.into())
    }

    fn visit_not(&mut self) -> Judgement {
        Ok(Ty::arrow(&[TyLit::Bool.into()], TyLit::Bool.into()))
    }

    fn visit_stuck(&mut self) -> Judgement {
        Ok(Ty::Bottom)
    }

    fn visit_int(&mut self, _: i64) -> Judgement {
        Ok(TyLit::Int.into())
    }

    fn visit_ty(&mut self, ty: &'a Ty) -> Judgement {
        Ok(ty.clone())
    }

    fn visit_var(&mut self, v: u32) -> Judgement {
        let idx = self.gamma.len() - v as usize;
        match self.gamma.get(idx) {
            Some(ty) => Ok(ty.clone()),
            None => Err(type_err("Unknown variable")),
        }
    }

    fn visit_abs(&mut self, ty_vars: &'a [Ty], body: &'a Term) -> Judgement {
        self.gamma.extend(ty_vars.iter().cloned());
        let ret = sv::walk_abs(self, ty_vars, body);
        self.gamma.pop();
        ret
    }
}

#[cfg(test)]
mod test {
    use super::TypeCheckVisitor;
    use debrujin::RenameVisitor;
    use core::{Term, Ty, TyLit};
    use lispy;

    fn get(s: &str) -> Term {
        let tl = lispy::get_code(s.as_bytes()).unwrap();

        let mut rv = RenameVisitor::new();
        rv.rename_term(&tl).unwrap()
    }

    #[test]
    fn test_tyck() {
        let mut tc = TypeCheckVisitor::new();

        assert_eq!(tc.type_of(&get("#T")).unwrap(), TyLit::Bool.into());
        assert_eq!(tc.type_of(&get("!")).unwrap(),
                   Ty::arrow(&[TyLit::Bool.into()], TyLit::Bool.into()));
        assert_eq!(tc.type_of(&get("(/lam x: #B.x)")).unwrap(),
                   Ty::arrow(&[TyLit::Bool.into()], TyLit::Bool.into()));
        assert_eq!(tc.type_of(&get("(! #T)")).unwrap(), TyLit::Bool.into());
        assert_eq!(tc.type_of(&get("!")).unwrap(),
                   Ty::arrow(&[TyLit::Bool.into()], TyLit::Bool.into()));
        assert_eq!(tc.type_of(&get("((/lam x: #B.x) #T)")).unwrap(),
                   TyLit::Bool.into());
        assert_eq!(tc.type_of(&get("((/lam x: #Int.x) 200)")).unwrap(),
                   TyLit::Int.into());
        assert_eq!(tc.type_of(&get("(if (! #F) ! (/lam x: #B. x))")).unwrap(),
                   Ty::arrow(&[TyLit::Bool.into()], TyLit::Bool.into()));

        let ast = get("( /lam x: #Int, t: #B. (if t x 0))");
        println!("{:?}", ast);
        assert_eq!(tc.type_of(&ast).unwrap(),
                   Ty::arrow(&[TyLit::Int.into(), TyLit::Bool.into()], TyLit::Int.into()));

        let ast = get("((/lam test: #B, val: #Int, dummy: #Int. (if test val -2000)) #T 2000 0)");
        println!("{:?}", ast);
        assert_eq!(tc.type_of(&ast).unwrap(), TyLit::Int.into());
    }
}
