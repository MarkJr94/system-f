use toplevel::{Term, Ty};
use vars as v;
use vars::{Var, VarSet};

use parens;
use parens::{parse_Term};

pub struct Parser;

#[cfg(test)]
mod test {
    use parens;

    #[test]
    fn test_parse_term() {
        let np = parens::parse_Term;

        let mut p = np("T");
        assert!(p.is_ok());

        p = np("F");
        assert!(p.is_ok());

        p = np("x");
        println!("{:?}", p);
        assert!(p.is_ok());

        p = np("(lam x: #B. x)");
        assert!(p.is_ok());

        p = np("(if #T #T #F)");
        assert!(p.is_ok());

        p = np("(if (! #T) #T #F)");
        assert!(p.is_ok());

        p = np("((lam x: (#B -> #B). (x #F)) !)");
        assert!(p.is_ok());

        p = np("! )");
        assert!(p.is_err());
    }

    #[test]
    fn test_parse_type() {
        let pt = parens::parse_TyP;

        let mut t = pt("(#B -> #B)");
        assert!(t.is_ok());

        t = pt("#B");
        assert!(t.is_ok());

        t = pt("((#B -> #B) -> (#B -> #B))");
        assert!(t.is_ok());
    }
}
