pub mod tokens;
pub mod tokenize;
pub mod scan;

use errors::*;
use toplevel::Term;

pub fn get_code(data: &[u8]) -> Result<Term> {
    let lex = tokenize::scan(data)?;
    scan::parse(lex)
}
