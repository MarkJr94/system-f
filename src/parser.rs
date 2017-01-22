use toplevel::{Term, Ty, Repr};
use vars as v;
use vars::{Var, VarSet};

use pest::prelude::*;

impl_rdp! {
    grammar! {
        lp = { ["("]}
        rp = { [")"]}
        true_ = {["T"]}
        false_ = {["F"] }
        bool_ = { true_ | false_ }
        not = { ["!"] }
        lam = { ["lam"] }
        sep = { [","]}
        if_lit = { ["if"] }
        ty = { ["B"] | (lp ~ ty ~ ["->"] ~ ty ~ rp) }
        alpha = { ['a'..'z'] | ['A'..'Z']}
        digit = {['0'..'9']}
        var = @{ alpha ~ (alpha | digit | ["-"])* }
        if_ = { lp ~ if_lit ~ term ~ sep ~ term ~ sep ~ term ~ rp}
        app = { lp ~ term ~ [" "] ~ term ~ rp }
        abs = {lp ~ lam ~ [" "] ~ var ~ [":"] ~ [" "] ~ ty ~ ["."] ~ [" "] ~ term ~ rp}
        term = _{ var | abs | app | bool_ | not | if_ }
        whitespace = {[" "]}
    }
}

#[cfg(test)]
mod test {
    use pest::prelude::*;
    use super::Rdp;

    fn np(s: &str) -> Rdp<StringInput> {
        Rdp::new(StringInput::new(s))
    }

    #[test]
    fn test_basic_parse() {
        let mut p = np("T");
        assert!(p.true_());

        p = np("F");
        assert!(p.false_());

        p = np("x");
        assert!(p.var());

        p = np("(lam x: B. x)");
        println!("{:?}", p.queue());
        println!("{:?}", p.stack());
        println!("{:?}", p.queue_with_captures());
        assert!(p.abs());

        p = np("(if T T F)");
        assert!(p.if_());

        p = np("(if (! T) T F)");
        assert!(p.if_());

        p = np("((lam x: (B -> B). (x F)) !)");
        assert!(p.app());

        p = np("! )");
        assert!(p.app());
    }
}
