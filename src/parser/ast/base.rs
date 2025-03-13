use std::fmt::Debug;

use super::visitor::{ExpressionVisitor, StatementVisitor, SyntaxNodeVisitor};

pub trait SyntaxNode: Debug + Clone {
    fn accept<R>(&self, visitor: &dyn SyntaxNodeVisitor<R>) -> R;
}

pub trait Expression: SyntaxNode {
    fn accept<R>(&self, visitor: &dyn ExpressionVisitor<R>) -> R;
}

pub trait Statement: SyntaxNode {
    fn accept<R>(&self, visitor: &dyn StatementVisitor<R>) -> R;
}
