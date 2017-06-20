#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Token {
    True,
    False,
    Not,
    If,
    Id(String),
    Comma,
    Lambda,
    Colon,
    Bool,
    Arrow,
    Dot,
    LParen,
    RParen,
    Bottom,
}
