pub mod lexer;
pub mod tests;

pub mod source;
pub mod utility;

use source::SourceFile;

fn main() {
    let executable_dir = utility::get_executable_dir();
    let file_path = executable_dir.join("test.ion");
    let file = SourceFile::new(file_path.to_str().unwrap());
    let tokens = file.tokenize();

    println!("{:#?}", tokens);
}
