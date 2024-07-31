mod lex {
    pub mod lexer;
}

use crate::lex::lexer::lexical_analysis;

fn main() {
    let text = String::from("
    int main()
    {
        int x = (100 - 200) * 2 / 5;
        return 0;
    }
    ");
    
    lexical_analysis(&text);
}
