use std::{format, vec};

#[derive(Debug)]
pub struct MidToken {
    pub str: String,
    pub index: usize,
}
impl MidToken {
    pub fn new(str: String, index: usize) -> MidToken {
        MidToken { str, index }
    }
    fn string(&self) -> String {
        format!("['{}',{}] ", self.str, self.index)
    }
}

#[allow(unused)]
pub fn mid_list_to_string(list: &[MidToken]) -> String {
    list.iter().map(|token| token.string()).collect()
}

fn is_same(a: &char, b: &char) -> bool {
    if (a.is_alphanumeric() || *a == '.') && (b.is_alphanumeric() || *b == '.') {
        return true;
    }
    false
}

pub trait SliceExpression {
    fn slice_expr(&self) -> Vec<MidToken>;
}
impl SliceExpression for str {
    fn slice_expr(&self) -> Vec<MidToken> {
        let mut list = vec![];
        let mut word = String::new();
        let mut start = 0;

        for (index, char) in self.chars().enumerate() {
            if word.is_empty() {
                if char != ' ' {
                    start = index;
                    word.push(char);
                }
            } else if is_same(&word.chars().last().unwrap(), &char) {
                word.push(char);
            } else {
                list.push(MidToken::new(std::mem::take(&mut word), start));
                if char != ' ' {
                    start = index;
                    word.push(char);
                }
            }
        }
        if !word.is_empty() {
            list.push(MidToken::new(std::mem::take(&mut word), start))
        }
        list
    }
}
