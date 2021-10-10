#[derive(Debug)]
pub enum NodeValue {
    Str(String),
    List(Vec<AstNode>)
}

#[derive(Debug)]
pub struct AstNode {
    pub key: String,
    pub value: NodeValue
}

impl AstNode {
    pub fn new(key: String, node: NodeValue) -> AstNode {
        AstNode { key: key, value: node }
    }
}

#[derive(Debug)]
pub struct AST {
    pub list: Vec<AstNode>
}

impl AST {
    pub fn new() -> AST {
        AST {
            list: vec![]
        }
    }

    pub fn addn(&mut self, node: AstNode) {
        self.list.push(node);
    }

    pub fn add_to(&mut self, parent: String, node: AstNode) {
        if let Some(a) = self.list.iter_mut().find(|i| i.key == parent ) {
            if let NodeValue::List(l) = &mut a.value {
                l.push(node);
            }
        }
    }
}
