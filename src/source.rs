use pathdiff::diff_paths;
use std::{fs, path::PathBuf, rc::Rc, str::FromStr};

use crate::{
    lexer::{syntax::TokenStream, Lexer},
    parser::{ast::base::Expression, Parser},
    utility,
};

pub struct SourceFile {
    pub absolute_path: String,
    pub relative_path: String,
    pub source: String,
}

impl SourceFile {
    pub fn new(path: &str) -> Self {
        let source = fs::read_to_string(path).unwrap();
        let executable_dir = utility::get_executable_dir();
        let absolute_path = path.to_string();

        Self {
            absolute_path,
            relative_path: diff_paths(PathBuf::from_str(path).unwrap(), executable_dir)
                .expect("could not compute relative path")
                .to_str()
                .unwrap()
                .to_owned(), // hell.
            source,
        }
    }

    pub fn tokenize(self) -> TokenStream {
        let mut lexer = Lexer::new(self);
        lexer.tokenize()
    }

    pub fn parse(self) -> impl Expression {
        let tokens = self.tokenize();
        let mut parser = Parser::new(tokens);
        parser.parse()
    }
}
