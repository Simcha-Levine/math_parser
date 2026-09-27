use std::format;

pub enum Node {
    Brackets(Box<Node>),
    UniMinus(Box<Node>),
    Function {
        name: String,
        params: Vec<Node>,
    },
    Number(f32),
    Operator {
        operator: String,
        exp1: Box<Node>,
        exp2: Box<Node>,
    },
    Variable(String),
}

impl Node {
    #[allow(unused)]
    pub fn print_tree(&self) {
        self.print_tree_rec(0, &[], "");
    }

    pub fn calculate(&self) -> Result<f32, String> {
        match self {
            Node::Brackets(node) => node.calculate(),
            Node::Number(n) => Ok(*n),
            Node::Operator {
                operator,
                exp1,
                exp2,
            } => {
                let result = if operator == "+" {
                    exp1.calculate()? + exp2.calculate()?
                } else if operator == "-" {
                    exp1.calculate()? - exp2.calculate()?
                } else if operator == "*" {
                    exp1.calculate()? * exp2.calculate()?
                } else if operator == "/" {
                    let left = exp1.calculate()?;
                    let right = exp2.calculate()?;
                    if right == 0. {
                        return Err("division by 0".to_string());
                    }
                    left / right
                } else if operator == "%" {
                    let left = exp1.calculate()?;
                    let right = exp2.calculate()?;
                    if right == 0. {
                        return Err("division by 0".to_string());
                    }
                    left % right
                } else if operator == "^" {
                    exp1.calculate()?.powf(exp2.calculate()?)
                } else {
                    return Err("Invalid Operator".to_string());
                };
                Ok(result)
            }
            Node::Variable(name) => Err(format!("no variable named {name}")),
            Node::UniMinus(node) => Ok(-node.calculate()?),
            Node::Function { name, params } => {
                let result = if name == "max" && params.len() == 2 {
                    params[0].calculate()?.max(params[1].calculate()?)
                } else if name == "min" && params.len() == 2 {
                    params[0].calculate()?.min(params[1].calculate()?)
                } else if name == "sqrt" && params.len() == 1 {
                    params[0].calculate()?.sqrt()
                } else if name == "sin" && params.len() == 1 {
                    params[0].calculate()?.sin()
                } else if name == "cos" && params.len() == 1 {
                    params[0].calculate()?.cos()
                } else if name == "tan" && params.len() == 1 {
                    params[0].calculate()?.tan()
                } else if name == "mod" && params.len() == 2 {
                    params[0].calculate()? % params[1].calculate()?
                } else if name == "pow" && params.len() == 2 {
                    params[0].calculate()?.powf(params[1].calculate()?)
                } else if name == "abs" && params.len() == 1 {
                    params[0].calculate()?.abs()
                } else {
                    return Err(format!("no function named {name}"));
                };
                Ok(result)
            }
        }
    }

    fn print_tree_rec(&self, level: i32, con_list: &[i32], arm: &str) {
        let arm_t = "├── ";
        let arm_l = "╰── ";
        let arm_i = "│   ";
        let arm_e = "    ";

        let mut tabs: String = (0..level - 1)
            .map(|i| if con_list.contains(&i) { arm_i } else { arm_e })
            .collect();

        tabs.push_str(arm);

        match self {
            Node::Number(n) => println!("{tabs}{n}"),
            Node::Operator {
                operator,
                exp1,
                exp2,
            } => {
                println!("{tabs}{}", operator);
                let mut new_list = con_list.to_vec();
                new_list.push(level);
                exp1.print_tree_rec(level + 1, &new_list, arm_t);
                new_list.pop();
                // tabs.pop();
                exp2.print_tree_rec(level + 1, &new_list, arm_l);
            }
            Node::Variable(v) => println!("{tabs}{v}"),
            Node::Brackets(exp) => {
                println!("{tabs}()");
                exp.print_tree_rec(level + 1, con_list, arm_l);
            }
            Node::UniMinus(exp) => {
                println!("{tabs}-x");
                exp.print_tree_rec(level + 1, con_list, arm_l);
            }
            Node::Function { name, params } => {
                println!("{tabs}{}", name);
                let mut new_list = con_list.to_vec();
                new_list.push(level);
                for param in params[0..params.len() - 1].iter() {
                    param.print_tree_rec(level + 1, &new_list, arm_t);
                }
                new_list.pop();
                params
                    .last()
                    .unwrap()
                    .print_tree_rec(level + 1, &new_list, arm_l);
            }
        }
    }

    fn get_type(&self) -> String {
        match self {
            Node::Brackets(_) => "Brackets",
            Node::UniMinus(_) => "UniMinus",
            Node::Function { .. } => "Function",
            Node::Number(_) => "Number",
            Node::Operator { .. } => "Operator",
            Node::Variable(_) => "Variable",
        }
        .to_string()
    }
    fn get_value(&self) -> String {
        match self {
            Node::Brackets(_) => "()".to_string(),
            Node::UniMinus(_) => "-".to_string(),
            Node::Function { name, .. } => name.to_string(),
            Node::Number(num) => num.to_string(),
            Node::Operator { operator, .. } => operator.to_string(),
            Node::Variable(name) => name.to_string(),
        }
    }
    fn get_len(&self) -> usize {
        match self {
            Node::Brackets(_) => 1,
            Node::UniMinus(_) => 1,
            Node::Function { name, .. } => name.len(),
            Node::Number(num) => num.to_string().len(),
            Node::Operator { operator, .. } => operator.len(),
            Node::Variable(var) => var.len(),
        }
    }

    pub fn latex(&self, nested: bool) -> String {
        match self {
            Node::Brackets(node) => {
                if nested {
                    node.latex(true)
                } else {
                    format!("({})", node.latex(true))
                }
            }
            Node::UniMinus(node) => format!("{{-{}}}", node.latex(true)),
            Node::Function { name, params } => {
                let l_params = params
                    .iter()
                    .map(|param| param.latex(true))
                    .collect::<Vec<String>>()
                    .join(", ");

                if name == "sqrt" && params.len() == 1 {
                    format!(r"\sqrt{{{}}}", l_params)
                } else if name == "pow" && params.len() == 2 {
                    format!(r"{}^{{{}}}", params[0].latex(false), params[1].latex(true))
                } else if name == "abs" && params.len() == 1 {
                    format!(r"\left|{}\right|", l_params,)
                } else if name.len() > 1 {
                    format!(r"\text{{{}}}({})", name, l_params)
                } else {
                    format!(r"{}({})", name, l_params)
                }
            }
            Node::Number(num) => format!("{num}"),
            Node::Operator {
                operator,
                exp1,
                exp2,
            } => {
                let type1 = exp1.get_type();
                let type2 = exp2.get_type();
                if operator == "*"
                    && (type1 == "Brackets"
                        || type2 == "Brackets"
                        || (type1 == "Variable" && type2 == "Number" && exp1.get_len() == 1)
                        || (type2 == "Variable" && type1 == "Number" && exp2.get_len() == 1)
                        || exp1.get_value() == "/"
                        || exp2.get_value() == "/"
                        || exp1.get_value() == "sqrt"
                        || exp2.get_value() == "sqrt")
                {
                    format!(r"{}{}", exp1.latex(false), exp2.latex(false))
                } else if operator == "/" {
                    format!(r"\frac{{{}}}{{{}}}", exp1.latex(true), exp2.latex(true))
                } else if operator == "%" {
                    format!(r"{}\bmod{}", exp1.latex(false), exp2.latex(false))
                } else if operator == "^" {
                    format!(r"{}^{{{}}}", exp1.latex(false), exp2.latex(true))
                } else {
                    format!(r"{}{}{}", exp1.latex(false), operator, exp2.latex(false))
                }
            }
            Node::Variable(name) => {
                if name.len() > 1 {
                    format!(r"\text{{{}}}", name)
                } else {
                    name.to_string()
                }
            }
        }
    }
}
