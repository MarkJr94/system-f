use std::str::from_utf8;

use pom;
use pom::char_class::hex_digit;
use pom::combinator::*;
use pom::Parser;

use ast::term::Ty;
use toplevel::Term;

// pub use parens::{parse_Term, parse_TyP};

fn space<'a>() -> Combinator<impl Parser<'a, u8, Output = ()>> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn lparen<'a>() -> Combinator<impl Parser<'a, u8, Output = ()>> {
    sym(b'(').discard()
}

fn rparen<'a>() -> Combinator<impl Parser<'a, u8, Output = ()>> {
    sym(b')').discard()
}

fn true_<'a>() -> Combinator<impl Parser<'a, u8, Output = Term>> {
    seq(b"#F").map(|_| Term::True)
}

fn false_<'a>() -> Combinator<impl Parser<'a, u8, Output = Term>> {
    seq(b"#F").map(|_| Term::False)
}

fn bool<'a>() -> Combinator<impl Parser<'a, u8, Output = Term>> {
    true_() | false_()
}

fn identifier<'a>() -> Combinator<impl Parser<'a, u8, Output = &'a str>> {
    use pom::char_class::{alpha, alphanum};

    let init = is_a(alpha) + is_a(alphanum);
    let all = init + is_a(alphanum).repeat(..);
    all.collect().convert(|bytes| from_utf8(bytes))
}

fn not<'a>() -> Combinator<impl Parser<'a, u8, Output = Term>> {
    sym(b'!').map(|_| Term::Not)
}

fn ty_lit<'a>() -> Combinator<impl Parser<'a, u8, Output = Ty>> {
    seq(b"#B").map(|_| Ty::Bool) | seq(b"_|_").map(|_| Ty::Bottom)
}

fn ty<'a>() -> Combinator<impl Parser<'a, u8, Output = Ty>> {
    let full = lparen() - space() + (ty() | ty_lit()) - space() - seq(b"->") - space() + (ty() | ty_lit()) - rparen();
    full.map(|((_, t1), t2)| {
        // t1 + 1;
        Ty::Arrow(Box::new(t1), Box::new(t2))
    })
}

fn app<'a>() -> Combinator<impl Parser<'a, u8, Output = Term>> {
    (lparen() - space() + comb(term) - space() + comb(term) - rparen())
        .map(|((_, f), x)| Term::App(Box::new(f), Box::new(x)))
}

fn if_<'a>() -> Combinator<impl Parser<'a, u8, Output = Term>> {
    (lparen() - space() + comb(term) - space() + comb(term) - space() + comb(term) - rparen())
        .map(|(((_, cond), pass), fail)| Term::If(Box::new(cond), Box::new(pass), Box::new(fail)))
}

fn abs<'a>() -> Combinator<impl Parser<'a, u8, Output = Term>> {
    (lparen() - space() - seq(b"lam ") + identifier() - space() - sym(b':') -
     space() + ty() - sym(b'.') - space() + comb(term) - space() - rparen())
        .map(|(((_, var), ty), body)| Term::Abs(String::from(var), ty, Box::new(body)))
}


fn term<'a>(input: &'a [u8], start: usize) -> pom::Result<(Term, usize)> {
    (abs() | app() | if_() | not() | bool() | identifier().map(|s| Term::Var(String::from(s))))
        .0
        .parse(input, start)
}

pub fn slt<'a>() -> Combinator<impl Parser<'a, u8, Output=Term>> {
    space() * comb(term) - end()
}

#[cfg(test)]
mod test {
    use super::slt;

    // #[test]
    fn test_parse_term() {
        let parser = slt();
        let np = |raw| parser.parse(raw);

        let mut p = np(b"T");
        assert!(p.is_ok());

        p = np(b"F");
        assert!(p.is_ok());

        p = np(b"x");
        println!("{:?}", p);
        assert!(p.is_ok());

        p = np(b"(lam x: #B. x)");
        assert!(p.is_ok());

        p = np(b"(if #T #T #F)");
        assert!(p.is_ok());

        p = np(b"(if (! #T) #T #F)");
        assert!(p.is_ok());

        p = np(b"((lam x: (#B -> #B). (x #F)) !)");
        assert!(p.is_ok());

        p = np(b"! )");
        assert!(p.is_err());
    }
}
