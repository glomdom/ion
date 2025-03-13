use crate::lexer::syntax::{SyntaxKind, TokenStream};
use ast::{base::Expression, expressions::Literal};

pub mod ast;

pub struct Parser {
    tokens: TokenStream,
}

impl Parser {
    pub fn new(tokens: TokenStream) -> Self {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> impl Expression {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> impl Expression {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> impl Expression {
        if self.tokens.check_set(vec![
            SyntaxKind::IntLiteral,
            SyntaxKind::FloatLiteral,
            SyntaxKind::StringLiteral,
            SyntaxKind::BoolLiteral,
            SyntaxKind::NullLiteral,
        ]) {
            let token_ref = self.tokens.advance();
            let token = token_ref.clone();
            return Literal { token };
        }

        panic!("unexpected token {:?}", self.tokens.current().kind)
    }
}
