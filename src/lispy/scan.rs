use slog::{Drain, Logger};
use slog_async;
use slog_term;

use core::{Ty, TyLit};
use errors::*;
use lispy::tokens::Token as T;
use lispy::tokenize::Lex;
use toplevel::Term;


pub struct Parser<'a> {
    lex: Lex<'a>,
    idx: isize,
    logger: Logger,
}

impl<'a> Parser<'a> {
    pub fn new(lex: Lex<'a>, logger: &Logger) -> Parser<'a> {
        let p = Parser {
            logger: logger.new(o!("input" => lex.orig.to_owned(), "phase" => "Parsing")),
            lex: lex,
            idx: -1,
        };

        trace!(p.logger, "New parser"; "size" => p.lex.tokens.len());

        p
    }

    fn expect(&mut self, token: T) -> Result<()> {
        let t = self.next()?;

        if t == token {
            Ok(())
        } else {
            Err(parse_error("Expected literal failed to match", Some(t)))
        }
    }

    fn peek_expect(&mut self, token: T) -> Result<()> {
        let ret = self.expect(token);
        self.idx -= 1;
        ret
    }

    fn parse_lambda(&mut self) -> Result<Term> {
        trace!(self.logger, "parsing lambda");

        let mut args = vec![];

        loop {
            let var = self.parse_var()?;
            self.expect(T::Colon)?;

            let ty = self.parse_ty()?;

            if let Term::Var(x) = var {
                args.push((x, ty));
            } else {
                return Err(parse_error("Var was not a var", None));
            }

            let test = self.expect(T::Comma).is_ok();
            if !test {
                self.idx -= 1;
                break;
            }
        }

        self.expect(T::Dot)?;

        let body = self.parse_term()?;
        self.expect(T::RParen)?;


        Ok(Term::Abs(args, Box::new(body)))
    }

    fn parse_var(&mut self) -> Result<Term> {
        let t = self.next()?;
        trace!(self.logger, "parsing var"; "next token" => &t);

        if let T::Id(name) = t {
            Ok(Term::Var(name))
        } else {
            Err(parse_error("Error parsing var: unexpected token encountered", Some(t)))
        }
    }

    fn parse_app(&mut self) -> Result<Term> {
        trace!(self.logger, "parsing application");
        self.idx -= 1;
        let f = self.parse_term()?;

        let mut args = vec![];


        loop {
            trace!(self.logger, "parsing app arg");
            let x = self.parse_term()?;
            let test = self.expect(T::RParen).is_ok();

            args.push(x);
            if test {
                break;
            } else {
                self.idx -= 1;
                trace!(self.logger, "another one");
            }
        }

        trace!(self.logger, "done parsing application");
        Ok(Term::app(f, &args))
    }

    fn parse_if(&mut self) -> Result<Term> {
        trace!(self.logger, "parsing if");
        let cond = self.parse_term()?;
        let then = self.parse_term()?;
        let else_ = self.parse_term()?;
        self.expect(T::RParen)?;

        Ok(Term::if_(cond, then, else_))
    }

    fn parse_ty(&mut self) -> Result<Ty> {
        trace!(self.logger, "parsing type");

        match self.next()? {
            T::TypeLit(ref lit) => Ok((*lit).into()),
            T::LParen => {
                let ty1 = self.parse_ty()?;
                self.expect(T::Arrow)?;
                let ty2 = self.parse_ty()?;
                self.expect(T::RParen)?;

                Ok(Ty::arrow(&[ty1], ty2))
            }
            t => Err(parse_error("Error parsing type: unexpected token encountered", Some(t))),
        }
    }

    fn parse_term(&mut self) -> Result<Term> {
        let tok = self.next()?;

        trace!(self.logger, "parsing term"; "next token" => &tok);

        match tok {
            T::True => Ok(Term::True),
            T::False => Ok(Term::False),
            T::Not => Ok(Term::Not),
            T::LParen => {
                match self.next()? {
                    T::Lambda => self.parse_lambda(),
                    T::If => self.parse_if(),
                    _ => self.parse_app(),
                }
            }
            T::Id(ref x) => Ok(Term::Var(x.clone())),
            T::Int(n) => Ok(Term::Int(n)),
            t => Err(parse_error("Error parsing term: unexpected token encountered", Some(t))),
        }
    }

    fn next(&mut self) -> Result<T> {
        self.idx += 1;
        let t = self.lex
            .tokens
            .get(self.idx as usize)
            .ok_or(parse_error("Reached end of input", None))?;

        Ok(t.clone())
    }

    fn prev(&mut self) -> Result<T> {
        self.idx -= 1;
        let t = self.lex
            .tokens
            .get(self.idx as usize)
            .ok_or(parse_error("Backtrack at start of input", None))?;

        Ok(t.clone())
    }
}

pub fn parse(tokens: Lex) -> Result<Term> {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let _log = Logger::root(drain, o!("context" => "main"));

    let mut p = Parser::new(tokens, &_log);

    p.parse_term()
}

#[cfg(test)]
mod test {
    use super::*;
    use lispy::tokenize::scan;

    fn get_parser<'a>(bytes: &'a [u8]) -> Result<Term> {
        let lex = scan(bytes).unwrap();

        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        let _log = Logger::root(drain, o!("context" => "test"));

        Parser::new(lex, &_log).parse_term()
    }

    #[test]
    fn test_parse() {

        let mut p = get_parser(b"#T");
        assert_eq!(p.unwrap(), Term::True);

        p = get_parser(b"#F");
        assert_eq!(p.unwrap(), Term::False);

        p = get_parser(b"x");
        assert_eq!(p.unwrap(), Term::Var("x".into()));

        p = get_parser(b"( /lam x: #B. x)");
        assert_eq!(p.unwrap(),
                   Term::abs(&[("x", Ty::Base(TyLit::Bool))], Term::var("x")));

        p = get_parser(b"( /lam x: #Int. x)");
        assert_eq!(p.unwrap(),
                   Term::abs(&[("x", Ty::Base(TyLit::Int))], Term::var("x")));

        p = get_parser(b"( /lam x: #Int, t: #B. (if t x 0))");
        assert_eq!(p.unwrap(),
                   Term::abs(&[("x", Ty::Base(TyLit::Int)), ("t", Ty::Base(TyLit::Bool))],
                             Term::if_(Term::var("t"), Term::var("x"), Term::Int(0))));

        p = get_parser(b"(if (! #T) #T #F)");
        let test = Term::if_(Term::app(Term::Not, &[Term::True]), Term::True, Term::False);
        assert_eq!(p.unwrap(), test);

        p = get_parser(b"(/lam x: (#B -> #B). (x #F))");
        let test = Term::abs(&[("x", Ty::arrow(&[TyLit::Bool.into()], TyLit::Bool.into()))],
                             Term::app(Term::var("x"), &[Term::False]));
        assert_eq!(p.unwrap(), test);

        p = get_parser(b"((/lam x: (#B -> #B). (x #F)) !)");
        let test = Term::app(Term::abs(&[("x",
                                          Ty::arrow(&[TyLit::Bool.into()], TyLit::Bool.into()))],
                                       Term::app(Term::var("x"), &[Term::False])),
                             &[Term::Not]);
        assert_eq!(p.unwrap(), test);

        p = get_parser(b"((/lam test: #B, val: #Int. (if test val -2000)) #T 2000)");
        let test =
            Term::app(Term::abs(&[("test", TyLit::Bool.into()), ("val", TyLit::Int.into())],
                                Term::if_(Term::var("test"), Term::var("val"), Term::Int(-2000))),
                      &[Term::True, Term::Int(2000)]);
        assert_eq!(p.unwrap(), test);
    }
}
