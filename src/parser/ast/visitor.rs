use super::expressions::Literal;

pub trait SyntaxNodeVisitor<R> {}

pub trait ExpressionVisitor<R>: SyntaxNodeVisitor<R> {
    fn visit_literal(&self, literal: &Literal) -> R;
}

pub trait StatementVisitor<R>: SyntaxNodeVisitor<R> {}
