use std::{fmt::Debug, format, vec};

use crate::mid_token::MidToken;

#[derive(Debug, Clone)]
pub struct Token {
    pub str: String,
    pub index: usize,
    pub layer: i32,
}
impl Token {
    fn from(mid: MidToken, layer: i32) -> Token {
        Token {
            str: mid.str,
            index: mid.index,
            layer,
        }
    }

    fn string(&self) -> String {
        format!("['{}',{},{}] ", self.str, self.index, self.layer)
    }
}

#[allow(unused)]
pub fn list_to_string(list: &[Token]) -> String {
    list.iter().map(|token| token.string()).collect()
}

pub trait Tokens {
    fn tokens(self) -> Vec<Token>;
}
impl Tokens for Vec<MidToken> {
    fn tokens(self) -> Vec<Token> {
        let mut free = 1;
        let mut layers: Vec<i32> = vec![0];

        self.into_iter()
            .map(|element| {
                if element.str == "(" {
                    layers.push(free);
                    free += 1;
                    Token::from(element, *layers.last().unwrap())
                } else if element.str == ")" {
                    Token::from(element, layers.pop().unwrap())
                } else {
                    Token::from(element, *layers.last().unwrap())
                }
            })
            .collect()
    }
}
