use super::ast::{AST, AstNode, NodeValue};

pub struct Evaluator {
    ast: AST
}

impl Evaluator {

    pub fn new(ast: AST) -> Evaluator {
        Evaluator { ast: ast }
    }

    fn runNode (&self, node: &AstNode) {
        match &node.value {
            NodeValue::List(list) => {
                for i in list.iter() {
                    self.exec(i);
                }
            },
            _ => { println!("    probably string") }
        }
    }

    fn exec(&self, node: &AstNode) {
        if let NodeValue::List(l) = &node.value {
            println!("        exec on list -> to runNode : {:?}", node.key);
            self.runNode(node);
        }
        else {
            if node.key == "echo" {
                match &node.value {
                    NodeValue::Str(echoMessage) => {
                        println!("{:?}", echoMessage);
                    },
                    _ => {
                        println!("echo Unknown value for {:?}", node.key);
                    }
                }
            }
            else if node.key == "stop" {
                std::process::exit(0);
            }
            else if node.key == "goto" {
                if let NodeValue::Str(labelName) = &node.value {
                    self.goto(labelName);
                } else {
                    println!("Label name error - incorrect type for {:?}", node);
                }
            }
        }
    }

    fn goto(&self, label: &String) {
        for n in self.ast.list.iter() {
            if n.key.eq(label) {
                self.runNode(n);
            }
        }
    }

    pub fn run(&self) {
        for n in self.ast.list.iter() {
            self.runNode(n);
        }
    }
}
