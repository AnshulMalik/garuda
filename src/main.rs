mod lexer;
mod token;

use lexer::{Lexer};

fn main() {
    let mut lexer = Lexer::new("const hello = 1.12;".to_string());
    lexer.print_tokens();
}
