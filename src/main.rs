mod langer;
use langer::{ lexer::Lexer, parser::Parser };

fn main() {
    let code =
        "  start:\n\
         echo \"this is a test\"\
          stop";

    let mut l = Lexer::new(String::from(code));
    l.tokenizer();

    println!("Tokens:\n{:?}\n", l.tokens);

    let mut parser = Parser::new(l.tokens);
    parser.build_ast();
}
