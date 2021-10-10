mod langer;
use langer::{ lexer::Lexer, parser::Parser, evaluator::Evaluator };
use std::fs;

fn main() {
    let contents = fs::read_to_string("echo.rfeel")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    let mut l = Lexer::new(String::from(contents));
    l.tokenizer();

    println!("Tokens :\n{:#?}\n", l.tokens);

    let mut parser = Parser::new(l.tokens);

    parser.build_ast();

    println!("AST:\n{:#?}\n", parser.ast);

    println!("Execution:\n");

    if let Some(ast) = parser.ast {
        let evaluator = Evaluator::new(ast);
        evaluator.run();
    }
}
