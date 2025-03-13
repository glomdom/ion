pub mod lexer;
pub mod parser;
pub mod tests;

pub mod source;
pub mod utility;

use parser::ast::{
    base::Expression,
    expressions::Literal,
    visitor::{ExpressionVisitor, SyntaxNodeVisitor},
};
use source::SourceFile;

struct TestVisitor;

impl SyntaxNodeVisitor<()> for TestVisitor {}

impl ExpressionVisitor<()> for TestVisitor {
    fn visit_literal(&self, literal: &Literal) -> () {
        println!("{:#?}", literal);
    }
}

fn main() {
    let executable_dir = utility::get_executable_dir();
    let file_path = executable_dir.join("test.ion");
    let file = SourceFile::new(file_path.to_str().unwrap());
    let tokens = file.tokenize();

    // println!("{:#?}", tokens);

    let token = tokens.first().unwrap();
    let literal = Literal { token };
    let visitor = TestVisitor {};
    literal.accept(&visitor);
}
