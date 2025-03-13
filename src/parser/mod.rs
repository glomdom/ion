use crate::lexer::syntax::TokenStream;

pub mod ast;

pub struct Parser {
    tokens: TokenStream,
}
