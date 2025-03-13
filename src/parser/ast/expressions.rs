use crate::lexer::syntax::Token;

use super::{
    base::{Expression, SyntaxNode},
    visitor::{ExpressionVisitor, SyntaxNodeVisitor},
};

#[derive(Debug)]
pub struct Literal<'a> {
    pub token: &'a Token,
}

impl<'a> SyntaxNode for Literal<'a> {
    fn accept<R>(&self, _visitor: &dyn SyntaxNodeVisitor<R>) -> R {
        todo!()
    }
}

impl<'a> Expression for Literal<'a> {
    fn accept<R>(&self, visitor: &dyn ExpressionVisitor<R>) -> R {
        visitor.visit_literal(self)
    }
}
