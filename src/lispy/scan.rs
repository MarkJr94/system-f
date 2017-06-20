use core::Ty;
use toplevel::Term;

use lispy::tokens::Token as T;

pub struct Parser<Iter> {
  tokens: Iter,
}

impl<Iter> Parser<Iter> where Iter: Iterator<Item = T> {
    pub fn new(tokens: Iter) -> Parser<Iter> {
      Parser { tokens: tokens }
    }

    fn expect(&mut self, token: T) -> Result<(),()> {
      match self.tokens.next() {
        Some(t) => if t == token { Ok(()) } else { Err(()) },
        None => Err(()),
      }
    }

    fn parse_lambda(&mut self) -> Result<Term, ()> {
      ma
    }

    fn parse_var(&mut self, token: T) -> Result<Term,()> {
      if let T::Id(name) =  token {
        Ok(Term::Var(name))
      } else {
        Err(())
      }
    }

    fn parse_term(&mut self) -> Result<Term,()> {
      match self.tokens.next() {
        Ok(T::True) => Ok(Term::True),
        Ok(T::False) => Ok(Term::False),
        Ok(T::Not) => Ok(Term::Not),
        Ok(T::LParen) => match self.tokens.next()  {
          Ok(T::Lambda) => self.parse_lambda(),
          Ok(T::If) => self.parse_if(),
          Ok(f) => self.parse_app(f),
          _ => Er(()),
        }
        Ok(T::Id(x)) => self.parse_var())
      }
    }
}

fn expect<Iter>(token: T, tokens: &mut Iter) -> Option<&mut Iter>
    where Iter: Iterator<Item = T>
{
    match tokens.next() {
        Some(t) => if t == token { Some(tokens) } else { None },
        None => None,
    }
}



fn parse_(tokens: &[T]) -> (Term, &[T]) {
    unimplemented!()
}
// pub fn parse_if
