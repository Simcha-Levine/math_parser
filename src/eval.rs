use crate::mid_token::MidToken;
use crate::token_types::TokenTypes;

fn eval_operators(list: &[MidToken]) -> Result<(), &MidToken> {
    if list[0].str != "-" && list[0].str.is_operator() {
        return Err(&list[0]);
    }
    for pair in list.windows(2) {
        if pair[1].str != "-" && pair[0].str.is_operator() && pair[1].str.is_operator() {
            return Err(&pair[0]);
        }

        if pair[0].str.is_operator() && pair[1].str == ")" {
            return Err(&pair[0]);
        }

        if pair[1].str.is_operator() && pair[1].str != "-" && pair[0].str == "(" {
            return Err(&pair[0]);
        }

        if pair[0].str == "(" && pair[1].str == ")" {
            return Err(&pair[0]);
        }
    }

    if list.last().unwrap().str.is_operator() {
        return Err(list.last().unwrap());
    }

    Ok(())
}

fn eval_operands(list: &[MidToken]) -> Result<(), &MidToken> {
    for element in list {
        if element.str.is_operator() || element.str == "(" || element.str == ")" {
            //do nothing
        } else if element.str.is_number() {
            if element.str.chars().filter(|c| *c == '.').count() > 1 {
                return Err(element);
            }
        } else if element.str.is_word() {
            if element.str.chars().next().unwrap().is_numeric() {
                return Err(element);
            }
        } else {
            return Err(element);
        }
    }

    for pair in list.windows(2) {
        if pair[0].str.is_word_or_number() && pair[1].str.is_word_or_number() {
            return Err(&pair[0]);
        }
        if pair[0].str.is_number() && pair[1].str == "(" {
            return Err(&pair[0]);
        }

        if pair[1].str.is_number() && pair[0].str == ")" {
            return Err(&pair[0]);
        }
    }
    Ok(())
}

fn eval_brackets(list: &[MidToken]) -> Result<(), &MidToken> {
    let mut brackets: usize = 0;
    for element in list.iter() {
        if element.str == "(" {
            brackets += 1;
        } else if element.str == ")" {
            if brackets == 0 {
                return Err(element);
            }
            brackets -= 1;
        }
    }
    if brackets == 0 { Ok(()) } else { Err(&list[0]) }
}

pub fn eval(list: &[MidToken]) -> Result<(), &MidToken> {
    if list.is_empty() {
        return Ok(());
    };
    eval_brackets(list)?;
    eval_operands(list)?;
    eval_operators(list)?;
    Ok(())
}
