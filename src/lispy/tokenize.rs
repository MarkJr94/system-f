use std::str::from_utf8;

use nom;
use nom::{IResult, ErrorKind, digit, alpha, alphanumeric, multispace};

use super::tokens::Token;

type bytes<'a> = &'a [u8];

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

named!(variable(bytes) -> Token, map!(identifier, |s: &str| Token::Id(s.into())));

named!(comma(bytes) -> Token, map!(tag!(","), |_| Token::Comma ));

named!(lambda(bytes) -> Token, map!(tag!("/lam"), |_| Token::Lambda));

named!(colon(bytes) -> Token, map!(tag!(":"), |_| Token::Colon));

named!(bool_( bytes ) -> Token, map!(tag!("#B"), |_| Token::Bool));

named!(arrow( bytes ) -> Token, map!(tag!("->"), |_| Token::Arrow));

named!(dot( bytes ) -> Token, map!(tag!("."), |_| Token::Dot));

named!(lparen( bytes ) -> Token, map!(tag!("("), |_| Token::LParen));

named!(rparen( bytes ) -> Token, map!(tag!(")"), |_| Token::RParen));

named!(bottom( bytes ) -> Token, map!(tag!("_|_"), |_| Token::Bottom));

named!(token (bytes) -> Token, alt!(false_ | true_ | not | if_token | variable | comma
| lambda | colon | bool_ | arrow | dot | lparen | rparen | bottom));

named!(tokenize(bytes) -> Vec<Token>, ws!(many0!(token)));

pub fn scan(data: &[u8]) -> Result<Vec<Token>, ()> {
    tokenize(data).to_full_result().map_err(|_|())
}

#[cfg(test)]
mod test {
    use super::*;

    use lispy::tokens::Token;
    use lispy::tokens::Token as T;

    use nom::{ErrorKind, GetOutput, IResult};
    use nom::Needed::Size;


    #[test]
    fn test_parse_bool() {
        assert_eq!(false_(b"#F").unwrap().1, T::False);
        assert_eq!(false_(b"#"), IResult::Incomplete(Size(2)));
        assert_eq!(true_(b"#T").unwrap().1, T::True);
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
        println!("{:?}", p);
        assert_eq!(p.unwrap().1,
                   vec![T::LParen,
                        T::Lambda,
                        T::Id("x".into()),
                        T::Colon,
                        T::Bool,
                        T::Dot,
                        T::Id("x".into()),
                        T::RParen]);

        p = tokenize(b"(if #T #T #F)");
        let test = vec![T::LParen, T::If, T::True, T::True, T::False, T::RParen];
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
                        T::Bool,
                        T::Arrow,
                        T::Bool,
                        T::RParen,
                        T::Dot,
                        T::LParen,
                        T::Id("x".into()),
                        T::False,
                        T::RParen,
                        T::RParen,
                        T::Not,
                        T::RParen];
        assert_eq!(p.unwrap().1, test);
    }
}
