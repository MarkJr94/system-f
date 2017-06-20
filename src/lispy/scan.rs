use core::{Ty, arrow};
use errors::*;
use toplevel::Term;

use lispy::tokens::Token as T;

pub struct Parser<'a> {
    tokens: &'a [T],
    idx: isize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [T]) -> Parser<'a> {
        Parser {
            tokens: tokens,
            idx: -1,
        }
    }

    fn expect(&mut self, token: T) -> Result<()> {
        if self.next()? == token {
            Ok(())
        } else {
            Err(())
        }
    }

    fn parse_lambda(&mut self) -> Result<Term> {

        let var = self.parse_var()?;
        self.expect(T::Colon)?;

        let ty = self.parse_ty()?;

        self.expect(T::Dot)?;

        let body = self.parse_term()?;

        if let Term::Var(x) = var {
            Ok(Term::abs(x, ty, body))
        } else {
            Err(())
        }
    }

    fn parse_var(&mut self) -> Result<Term> {
        if let T::Id(name) = self.next()? {
            Ok(Term::Var(name))
        } else {
            Err(())
        }
    }

    fn parse_app(&mut self) -> Result<Term> {
        self.idx -= 1;
        let f = self.parse_term()?;
        let x = self.parse_term()?;
        self.expect(T::RParen)?;

        Ok(Term::app(f, x))
    }

    fn parse_if(&mut self) -> Result<Term> {
        let cond = self.parse_term()?;
        let then = self.parse_term()?;
        let else_ = self.parse_term()?;

        Ok(Term::if_(cond, then, else_))
    }

    fn parse_ty(&mut self) -> Result<Ty> {

        match self.next()? {
            T::Bool => Ok(Ty::Bool),
            T::LParen => {
                let ty1 = self.parse_ty()?;
                self.expect(T::Arrow)?;
                let ty2 = self.parse_ty()?;
                self.expect(T::RParen)?;

                Ok(arrow(ty1, ty2))
            }
            _ => Err(())
        }
    }

    fn parse_term(&mut self) -> Result<Term> {
    
        match self.next()? {
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
            T::Not => Ok(Term::Not),
            _ => Err(()),
        }
    }

    fn next(&mut self) -> Result<T> {
        self.idx += 1;
        let t = self.tokens.get(self.idx as usize).ok_or(())?;

        Ok(t.clone())
    }

    fn prev(&mut self) -> Result<T> {
        self.idx -= 1;
        let t = self.tokens.get(self.idx as usize).ok_or(())?;

        Ok(t.clone())
    }
}

fn parse_(tokens: &[T]) -> Result<Term, ()> {
    let mut p = Parser::new(tokens);

    p.parse_term()
}

#[cfg(test)]
mod test {
  use super::*;
  use lispy::tokenize::scan;

  fn get_parser<'a>(bytes: &'a [u8]) -> Result<Term> {
    let tokens = scan(bytes).unwrap();
    println!("{:?}", tokens);

    Parser::new(&tokens).parse_term()
  }

  #[test]
    fn test_parse() {

        let mut p = get_parser(b"#T");
        println!("{:?}", p);
        assert_eq!(p.unwrap(), Term::True);

        p = get_parser(b"#F");
        println!("{:?}", p);
        assert_eq!(p.unwrap(), Term::False);

        p = get_parser(b"x");
        println!("{:?}", p);
        assert_eq!(p.unwrap(), Term::Var("x".into()));

        p = get_parser(b"( /lam x: #B. x)");
        println!("{:?}", p);
        assert_eq!(p.unwrap(),
                   Term::abs("x", Ty::Bool, Term::var("x")));

        p = get_parser(b"(if #T #T #F)");
        println!("{:?}", p);
        let test = vec![T::LParen, T::If, T::True, T::True, T::False, T::RParen];
        assert_eq!(p.unwrap(), Term::if_(Term::True, Term::True, Term::False));

        p = get_parser(b"(if (! #T) #T #F)");
        println!("{:?}", p);
        let test = Term::if_(Term::app(Term::Not, Term::True), Term::True, Term::False);
        assert_eq!(p.unwrap(), test);

        p = get_parser(b"((/lam x: (#B -> #B). (x #F)) !)");
        println!("{:?}", p);
        let test = Term::app(
          Term::abs("x", arrow(Ty::Bool, Ty::Bool), Term::app(Term::var("x"), Term::False)),
          Term::Not);
        assert_eq!(p.unwrap(), test);
    }
}
