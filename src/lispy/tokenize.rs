use std::str::from_utf8;

use nom::{Slice, alpha, alphanumeric, digit};

use core::TyLit;
use errors::*;
use lispy::tokens::Token;

#[allow(non_camel_case_types)]
type bytes<'a> = &'a [u8];

pub struct Lex<'a> {
    pub orig: &'a str,
    pub tokens: Vec<Token>,
}

named!(false_<&[u8], Token>, map!(tag!("#F"), |_| Token::False ));

named!(true_<&[u8], Token>, map!(tag!("#T"), |_| Token::True ));

named!(not( bytes ) -> Token, map!(tag!("!"), |_| Token::Not));

named!(if_token(bytes) -> Token, map!(tag!("if"), |_| Token::If));

named!(identifier<&[u8], &str>,
    do_parse!(
        peek!(alpha) >>
        var: dbg!(map_res!(alphanumeric, from_utf8)) >>
        (var)
    )
);

named!(int_lit<&[u8], i64>,
    flat_map!(
    recognize!(
      tuple!(
        opt!(alt!(tag!("+") | tag!("-"))),
        digit
      )
    ),
    parse_to!(i64)
  )
);

named!(int(bytes) -> Token, map!(int_lit, |i: i64| Token::Int(i)));

named!(variable(bytes) -> Token, map!(identifier, |s: &str| Token::Id(s.into())));

named!(comma(bytes) -> Token, map!(tag!(","), |_| Token::Comma ));

named!(lambda(bytes) -> Token, map!(tag!("/lam"), |_| Token::Lambda));

named!(colon(bytes) -> Token, map!(tag!(":"), |_| Token::Colon));

named!(bool_( bytes ) -> Token, map!(tag!("#B"), |_| Token::TypeLit(TyLit::Bool)));
named!(int_ty( bytes ) -> Token, map!(tag!("#Int"), |_| Token::TypeLit(TyLit::Int)));

named!(arrow( bytes ) -> Token, map!(tag!("->"), |_| Token::Arrow));

named!(dot( bytes ) -> Token, map!(tag!("."), |_| Token::Dot));

named!(lparen( bytes ) -> Token, map!(tag!("("), |_| Token::LParen));

named!(rparen( bytes ) -> Token, map!(tag!(")"), |_| Token::RParen));

named!(bottom( bytes ) -> Token, map!(tag!("_|_"), |_| Token::Bottom));

named!(token (bytes) -> Token, alt!(false_ | true_ | not | if_token | variable | comma
| lambda | colon | bool_ | arrow | dot | lparen | rparen | bottom | int | int_ty));

named!(tokenize(bytes) -> Vec<Token>, ws!(many0!(token)));

pub fn scan<'a>(data: &'a [u8]) -> Result<Lex<'a>> {
    use std::str::from_utf8;

    let toks = tokenize(data).to_full_result()?;
    let s = from_utf8(data)?;

    Ok(Lex {
           tokens: toks,
           orig: s,
       })
}

#[cfg(test)]
mod test {
    use super::*;

    use lispy::tokens::Token as T;

    use nom::IResult;
    use nom::Needed::Size;


    #[test]
    fn test_parse_bool() {
        assert_eq!(false_(b"#F").unwrap().1, T::False);
        assert_eq!(false_(b"#"), IResult::Incomplete(Size(2)));
        assert_eq!(true_(b"#T").unwrap().1, T::True);
    }

    #[test]
    fn test_parse_int() {
        let res = int(b"0");
        assert_eq!(res.unwrap().1, T::Int(0));

        assert!(!int(b"#").is_done());

        let res = int(b"-534");
        assert_eq!(res.unwrap().1, T::Int(-534));

        let res = int(b"+534");
        assert_eq!(res.unwrap().1, T::Int(534));
    }

    #[test]
    fn test_parse_variable() {
        assert_eq!(variable(b"a1v1a1r").unwrap().1, T::Id("a1v1a1r".into()));
        assert!(variable(b"2badvar").is_err());
        assert_eq!(variable(b"GoodVar").unwrap().1, T::Id("GoodVar".into()));
        assert_eq!(variable(b"almost-goodvar").unwrap().1,
                   T::Id("almost".into()));
    }

    #[test]
    fn test_tokenize() {

        let mut p = tokenize(b"#T");
        assert_eq!(p.unwrap().1, vec![T::True]);

        p = tokenize(b"#F");
        assert_eq!(p.unwrap().1, vec![T::False]);

        p = tokenize(b"x");
        assert_eq!(p.unwrap().1, vec![T::Id("x".into())]);

        p = tokenize(b"( /lam x: #B. x)");
        assert_eq!(p.unwrap().1,
                   vec![T::LParen,
                        T::Lambda,
                        T::Id("x".into()),
                        T::Colon,
                        T::TypeLit(TyLit::Bool),
                        T::Dot,
                        T::Id("x".into()),
                        T::RParen]);

        p = tokenize(b"(if #T #T #F)");
        let test = vec![T::LParen, T::If, T::True, T::True, T::False, T::RParen];
        assert_eq!(p.unwrap().1, test);

        p = tokenize(b"(if #T +100 -2)");
        let test = vec![T::LParen,
                        T::If,
                        T::True,
                        T::Int(100),
                        T::Int(-2),
                        T::RParen];
        assert_eq!(p.unwrap().1, test);

        p = tokenize(b"( /lam x: #Int. x)");
        let test = vec![T::LParen,
                        T::Lambda,
                        T::Id("x".into()),
                        T::Colon,
                        T::TypeLit(TyLit::Int),
                        T::Dot,
                        T::Id("x".into()),
                        T::RParen];
        assert_eq!(p.unwrap().1, test);

        p = tokenize(b"(/lam x: #B. (if x +100 -2))");
        let test = vec![T::LParen,
                        T::Lambda,
                        T::Id("x".into()),
                        T::Colon,
                        T::TypeLit(TyLit::Bool),
                        T::Dot,
                        T::LParen,
                        T::If,
                        T::Id("x".into()),
                        T::Int(100),
                        T::Int(-2),
                        T::RParen,
                        T::RParen];
        assert_eq!(p.unwrap().1, test);

        p = tokenize(b"(if (! #T) #T #F)");
        let test = vec![T::LParen, T::If, T::LParen, T::Not, T::True, T::RParen, T::True,
                        T::False, T::RParen];
        assert_eq!(p.unwrap().1, test);

        p = tokenize(b"((/lam x: (#B -> #B). (x #F)) !)");
        let test = vec![T::LParen,
                        T::LParen,
                        T::Lambda,
                        T::Id("x".into()),
                        T::Colon,
                        T::LParen,
                        T::TypeLit(TyLit::Bool),
                        T::Arrow,
                        T::TypeLit(TyLit::Bool),
                        T::RParen,
                        T::Dot,
                        T::LParen,
                        T::Id("x".into()),
                        T::False,
                        T::RParen,
                        T::RParen,
                        T::Not,
                        T::RParen];

        let _ = b"((/lam test: #B, val: #Int. (if test val -2000)) #T 2000)";

        p = tokenize(b"((/lam test: #B, val: #Int. (if test val -2000)) #T 2000)");
        let test = vec![T::LParen,
                        T::LParen,
                        T::Lambda,
                        T::Id("test".into()),
                        T::Colon,
                        T::TypeLit(TyLit::Bool),
                        T::Comma,
                        T::Id("val".into()),
                        T::Colon,
                        T::TypeLit(TyLit::Int),
                        T::Dot,
                        T::LParen,
                        T::If,
                        T::Id("test".into()),
                        T::Id("val".into()),
                        T::Int(-2000),
                        T::RParen,
                        T::RParen,
                        T::True,
                        T::Int(2000),
                        T::RParen];
        assert_eq!(p.unwrap().1, test);
    }
}
