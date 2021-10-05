use super::token::Token;

pub struct Lexer {
    data: String,
    pub tokens: Vec<Token>,
    keywords: Vec<String>,
}

impl Lexer {
    pub fn new(data: String) -> Lexer {

        Lexer {
            data: data,
            tokens: Vec::new(),
            keywords: vec![String::from("echo"), String::from("stop")]
        }
    }

    pub fn tokenizer(&mut self) {
        let mut tmp: String = String::new();
        let mut tid = "";
        for l in self.data.chars() {
            if l == '\"' && tid == "" {
                tid = "char";
                tmp.clear();
            }

            else if l == '\"' && tid == "char" {
                self.tokens.push(Token { id: String::from(tid), value: tmp.clone() });
                tmp.clear();
                tid = "";
            }

            else if l == ':' {
                self.tokens.push(Token { id: String::from("label"), value: tmp.clone() });
                tmp.clear();
            }


            else if l == ' ' && tid != "char" {
                continue
            }

            else {
                if l != '\n' {
                    tmp.push(l);
                }
            }

            if self.keywords.iter().any(|i| i== &tmp) {
                self.tokens.push(Token { id: String::from("keyword"), value: tmp.clone() });
                tmp.clear();
            }
        }
    }
}
