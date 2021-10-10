use super::token::Token;
use super::ast::{AST, AstNode, NodeValue};

pub struct Parser {
    tokens: Vec<Token>,
    pub ast: Option<AST>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens: tokens, ast: None }
    }


    pub fn build_ast(&mut self) {
        let mut parent: String = String::from("");
        let mut collect = false;

        let iter = self.tokens.iter();
        let mut saved: Option<&Token> = None;


        let mut ast = AST::new();

        for token in iter {
            if token.id == "label" {
                let an = AstNode::new(token.value.clone(), NodeValue::List(vec![]));
                if parent != token.value {
                    parent = token.value.clone();
                    ast.addn(an);
                }
            }

            else if token.id == "keyword" {
                if token.value == "stop" {
                    let an = AstNode::new(token.value.clone(), NodeValue::Str(String::from("")));
                    ast.add_to(parent.clone(), an);
                }
                else {
                    if collect == false {
                        saved = Some(token);
                        collect = true;
                     }
                    else {
                        collect = false;
                    }
                }
            }

            else if token.id == "char" || token.id == "atom" {
                if (token.id == "atom") {
                    println!("atom parse {:?}", token.value);
                }
                if collect == false {
                    saved = Some(token);
                    collect = true;
                }
                else {
                    match saved {
                        Some(v) => {
                            let an = AstNode::new(v.value.clone(), NodeValue::Str(token.value.clone()));
                            ast.add_to(parent.clone(), an);
                        },
                        None => {
                            println!("had no saved")
                        }
                    }
                    collect = false;
                }
            }

        }

        self.ast = Some(ast);
    }
}
