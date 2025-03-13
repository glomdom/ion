use crate::lexer::syntax::Token;

use super::{
    base::{Expression, SyntaxNode},
    visitor::{ExpressionVisitor, SyntaxNodeVisitor},
};

#[derive(Debug, Clone)]
pub struct Literal {
    pub token: Token,
}

impl SyntaxNode for Literal {
    fn accept<R>(&self, _visitor: &dyn SyntaxNodeVisitor<R>) -> R {
        todo!()
    }
}

impl Expression for Literal {
    fn accept<R>(&self, visitor: &dyn ExpressionVisitor<R>) -> R {
        visitor.visit_literal(self)
    }
}
