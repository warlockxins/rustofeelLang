use super::token::Token;

#[derive(Debug)]
enum NodeValue {
    Str(String),
    List(Vec<AstNode>)
}

#[derive(Debug)]
struct AstNode {
    key: String,
    value: NodeValue
}

impl AstNode {
    pub fn new(key: String, node: NodeValue) -> AstNode {
        AstNode { key: key, value: node }
    }
}
/*
type HashNode = HashMap<String, Vec<Node>>;
#[derive(Debug)]
enum Node {
    StringMap(HashMap<String, String>),
    HashMap(HashNode)
}
*/

#[derive(Debug)]
struct AST {
    astn: Vec<AstNode>
}

impl AST {
    pub fn new() -> AST {
        AST {
            astn: vec![]
        }
    }

    fn addn(&mut self, node: AstNode) {
        self.astn.push(node);
    }

    fn add_to(&mut self, parent: String, node: AstNode) {
        if let Some(a) = self.astn.iter_mut().find(|i| i.key == parent ) {
            if let NodeValue::List(l) = &mut a.value {
                l.push(node);
            }
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens: tokens }
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
                    println!("stop Feature");
                    //ast.addn()
                }
                else {
                    if collect == false {
                        saved = Some(token);
                        collect = true;
                     }
                    else {
                        println!("kill me now {:?}", token);
                        collect = false;
                    }
                }
            }

            else if token.id == "char" {
                if collect == false {
                    saved = Some(token);
                    collect = true;
                }
                else {
                    println!("   stop collecting {:?}", saved);

                    match saved {
                        Some(v) => {
                            let an = AstNode::new(v.value.clone(), NodeValue::Str(token.value.clone()));
                            ast.add_to(parent.clone(), an);
                        },
                        None => {
                        }
                    }
                    collect = false;
                }
            }

        }

         println!("res {:?}", ast);
    }
}
