use crate::eval::eval;
use crate::mid_token::SliceExpression;
use crate::node::Node;
use crate::token::{Token, Tokens};
use crate::token_types::TokenTypes;
use core::panic;
use std::{format, println};

fn handle_endpoint(list: &[Token]) -> Option<Result<Node, &Token>> {
    if list.len() == 1 {
        if let Ok(n) = list[0].str.parse::<f32>() {
            return Some(Ok(Node::Number(n)));
        } else {
            if list[0].str.is_word() {
                return Some(Ok(Node::Variable(list[0].str.clone())));
            }
            return Some(Err(&list[0]));
        }
    }
    None
}
fn handle_brackets(list: &[Token]) -> Option<Result<Node, &Token>> {
    if list[0].str == "("
        && list.last().unwrap().str == ")"
        && list[0].layer == list.last().unwrap().layer
    {
        let cut = &list[1..list.len() - 1];
        return Some(build_exp_tree(cut, list[0].layer).map(|node| Node::Brackets(Box::new(node))));
    }
    None
}
fn handle_functions(list: &[Token]) -> Option<Result<Node, &Token>> {
    if list[0].str.is_word()
        && list[1].str == "("
        && list.last().unwrap().str == ")"
        && list[1].layer == list.last().unwrap().layer
    {
        let b_layer = list[1].layer;
        let splitted: Result<Vec<Node>, &Token> = list[2..list.len() - 1]
            .split(|element| element.str == "," && element.layer == b_layer)
            .map(|element| build_exp_tree(element, b_layer))
            .collect();
        let params = match splitted {
            Ok(params) => params,
            Err(err) => return Some(Err(err)),
        };

        return Some(Ok(Node::Function {
            name: list[0].str.to_string(),
            params,
        }));
    }
    None
}
fn handle_uni_minus(list: &[Token], layer: i32) -> Option<Result<Node, &Token>> {
    let mut only_uni_minus = true;
    for (index, token) in list.iter().enumerate() {
        if token.layer == layer
            && token.str.is_operator()
            && (token.str != "-" || (index != 0 && list[index - 1].str != "-"))
        {
            only_uni_minus = false;
            break;
        }
    }
    if only_uni_minus {
        return Some(build_exp_tree(&list[1..], layer).map(|node| Node::UniMinus(Box::new(node))));
    }
    None
}

pub fn build_exp_tree(list: &[Token], layer: i32) -> Result<Node, &Token> {
    if list.is_empty() {
        panic!("somehow got empty list");
    }

    if let Some(result) = handle_endpoint(list) {
        return result;
    };
    if let Some(result) = handle_brackets(list) {
        return result;
    };
    if let Some(result) = handle_functions(list) {
        return result;
    };
    if let Some(result) = handle_uni_minus(list, layer) {
        return result;
    };

    let mut operators = ["+-", "*", "/", "%", "^"].map(|s| (s.to_string(), None));
    for (index, element) in list.iter().enumerate().rev() {
        if element.layer == layer {
            if let Some((_, seen)) = operators.iter_mut().find(|(s, _)| s.contains(&element.str)) {
                if let None = seen
                    && !(element.str == "-" && (index == 0 || list[index - 1].str.is_operator()))
                {
                    *seen = Some(index);
                }
            } else if element.str == "," {
                return Err(element);
            }
        }
    }
    let index = operators.iter().find_map(|(_, i)| *i).unwrap();

    let first = &list[..index];
    let second = &list[(index + 1)..];

    Ok(Node::Operator {
        operator: list[index].str.to_string(),
        exp1: Box::new(build_exp_tree(first, layer)?),
        exp2: Box::new(build_exp_tree(second, layer)?),
    })
}

pub fn get_expression(line: &String) -> Result<Node, String> {
    let list = line.slice_expr();
    if list.is_empty() {
        return Err("empty".to_string());
    }
    match eval(&list) {
        Ok(_) => (),
        Err(token) => {
            return Err(format!(
                "\"{line}\" syntax eval error for '{}' at index {}",
                token.str, token.index
            ));
        }
    }
    let list = list.tokens();

    match build_exp_tree(&list, 0) {
        Ok(tree) => Ok(tree),
        Err(token) => Err(format!(
            "\"{line}\" syntax tree error for '{}' at index {}",
            token.str, token.index
        )),
    }
}

#[allow(unused)]
pub fn exe_expression(line: &String) -> Result<f32, String> {
    let tree = get_expression(line)?;
    tree.print_tree();
    println!("{}", tree.latex(false));
    let result = match tree.calculate() {
        Ok(result) => result,
        Err(err) => {
            return Err(err);
        }
    };
    Ok(result)
}
