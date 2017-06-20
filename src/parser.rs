use std::str::from_utf8;

use nom;
use nom::{IResult, ErrorKind, digit, alpha, alphanumeric, multispace};

use core::{Ty, arrow};
use toplevel as tl;
use toplevel::Term;

type bytes<'a> = &'a [u8];

named!(false_<&[u8], Term>, map!(tag!("#F"), |_| Term::False ));

named!(true_<&[u8], Term>, map!(tag!("#T"), |_| Term::True ));

named!(bool_<&[u8], Term>,
    alt!(false_ | true_) );

named!(identifier<&[u8], &str>,
    do_parse!(
        peek!(alpha) >>
        var: dbg!(map_res!(alphanumeric, from_utf8)) >>
        (var)
    )
);

named!(variable(bytes) -> Term, map!(identifier, |s: &str| Term::Var(s.into())));

named!(not( bytes ) -> Term, map!(tag!("!"), |_| Term::Not));

named!(ty( bytes ) -> Ty, alt!(map!(tag!("#B"), |_| Ty::Bool) 
    | map!(
        ws!(delimited!(
            tag!("("), 
            separated_pair!(ty, tag!("->"), ty),
            tag!(")")
        )),
        |(t1, t2)| arrow(t1, t2))
    | map!(tag!("_|_"), |_| Ty::Bottom)
    ));

named!(app( bytes ) -> Term,
    map!(
        ws!(delimited!(
            tag!("("),
            pair!(term, term),
            tag!(")")
        )),
        |(f, x)| tl::app(f, x)
    )
);

named!(if__( bytes ) -> Term, map!(
    ws!(delimited!(
        tag!("("),
        do_parse!(
            tag!("if") >>
            cond: term >>
            pass: term >>
            fail: term >>
            (cond, pass, fail)
        ),
        tag!(")")
    )),
    |(cond, pass, fail)| tl::if_(cond, pass, fail)
));

named!(abs(bytes) -> Term, map!(
    ws!(delimited!(
        dbg!(tag!("(")),
        do_parse!(
            tag!("lam") >>
            var: identifier >>
            tag!(":") >>
            typ: ty >>
            tag!(".") >>
            body: term >>
            (var, typ, body)
        ),
        tag!(")")
    )),
    |(var, typ, body)| tl::abs(var, typ, body)
));

#[cfg(test)]
mod test {
    use super::*;
    use core::{Ty, arrow};
    use toplevel as tl;
    use toplevel::{Term};

    use nom::{ErrorKind, GetOutput, IResult};
    use nom::Needed::Size;


    #[test]
    fn test_parse_bool() {
        assert_eq!(bool_(b"#F"), IResult::Done(&[][..], Term::False));
        assert_eq!(bool_(b"#"), IResult::Incomplete(Size(2)));
        assert_eq!(bool_(b"#T"), IResult::Done(&[][..], Term::True));
    }

    #[test]
    fn test_parse_variable() {
        assert_eq!(variable(b"a1v1a1r"),
                   IResult::Done(&[][..], Term::Var("a1v1a1r".into())));
        assert_eq!(variable(b"2badvar"), IResult::Error(ErrorKind::Alpha));
        assert_eq!(variable(b"GoodVar"),
                   IResult::Done(&[][..], Term::Var("GoodVar".into())));
        assert_eq!(variable(b"almost-goodvar"),
                   IResult::Done(&b"-goodvar"[..], Term::Var("almost".into())));
    }

    #[test]
    fn test_parse_ty() {
        assert_eq!(ty(b"#B"), IResult::Done(&[][..], Ty::Bool));
        assert_eq!(ty(b"_|_"), IResult::Done(&[][..], Ty::Bottom));
        assert_eq!(ty(b"B"), IResult::Error(ErrorKind::Alt));
        assert_eq!(ty(b"(#B -> #B)"),
                   IResult::Done(&[][..], arrow(Ty::Bool, Ty::Bool)));
        assert_eq!(ty(b"(#B->#B)"),
                   IResult::Done(&[][..], arrow(Ty::Bool, Ty::Bool)));
        assert_eq!(ty(b"(#B -> (#B -> #B))"),
                   IResult::Done(&b""[..], arrow(Ty::Bool, arrow(Ty::Bool, Ty::Bool))));
        assert_eq!(ty(b"((#B->#B)-> (#B -> #B))"),
                   IResult::Done(&b""[..],
                                 arrow(arrow(Ty::Bool, Ty::Bool), arrow(Ty::Bool, Ty::Bool))));
    }

    #[test]
    fn test_parse_abs() {
        let r = abs("( lam x: #B. x)".as_bytes());
        println!("{:?}", r);
        assert_eq!(r.to_result().unwrap(), tl::abs("x", Ty::Bool, tl::var("x")));
    }

    #[test]
    fn test_parse_term() {

        let mut p = term(b"#T");
        assert_eq!(p.to_result().unwrap(), Term::True);

        p = term(b"#F");
        assert_eq!(p.to_result().unwrap(), Term::False);

        p = term(b"x");
        assert_eq!(p.to_result().unwrap(), tl::var("x"));

        p = abs(b"( lam x: #B. x)");
        println!("{:?}", p);
        assert_eq!(p.to_result().unwrap(), tl::abs("x", Ty::Bool, tl::var("x")));

        p = term(b"(if #T #T #F)");
        assert!(p.is_done());

        p = term(b"(if (! #T) #T #F)");
        assert!(p.is_done());

        p = term(b"((lam x: (#B -> #B). (x #F)) !)");
        assert!(p.is_done());

        p = term(b"! )");
        println!("{:?}", p);
        assert!(p.is_err());
    }
}
