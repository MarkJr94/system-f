use core::TyLit;
use slog::{Value, Serializer, Record, Key, Result};

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Debug, Clone)]
pub enum Token {
    True,
    False,
    Not,
    If,
    Id(String),
    Int(i64),
    Comma,
    Lambda,
    Colon,
    Arrow,
    Dot,
    LParen,
    RParen,
    Bottom,
    TypeLit(TyLit),
}

impl Token {
    pub fn unparse(&self) -> String {
        use self::Token::*;

        match *self {
            Token::True => "#T".into(),
            Token::False => "#F".into(),
            Token::TypeLit(tl) => tl.to_lit().into(),
            Not => "!".into(),
            If => "if".into(),
            Token::Id(ref s) => s.clone(),
            Int(n) => n.to_string(),
            Comma => ",".into(),
            Lambda => "/lam".into(),
            Colon => ":".into(),
            Arrow => "->".into(),
            Dot => ".".into(),
            LParen => "(".into(),
            RParen => ")".into(),
            Bottom => "_|_".into(),
        }
    }
}

impl Value for Token {
    fn serialize(&self, _: &Record, key: Key, serializer: &mut Serializer) -> Result {
        serializer.emit_str(key, &self.unparse())
    }
}
