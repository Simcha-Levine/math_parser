#[cfg(test)]
mod tests {
    use crate::eval::eval;
    use crate::mid_token::SliceExpression;
    use crate::token::Tokens;
    use crate::tree::build_exp_tree;

    use std::format;

    fn exe_expression(line: &String) -> Result<f32, String> {
        let list = line.slice_expr();
        if list.is_empty() {
            return Err(format!("empty"));
        }
        match eval(&list) {
            Ok(_) => (),
            Err(token) => {
                return Err(format!(
                    "{line}syntax eval error for {} at index {}",
                    token.str, token.index
                ));
            }
        }
        let list = list.tokens();
        let tree = match build_exp_tree(&list, 0) {
            Ok(tree) => tree,
            Err(token) => {
                return Err(format!(
                    "{line}syntax tree error for {} at index {}",
                    token.str, token.index
                ));
            }
        };
        let result = match tree.calculate() {
            Ok(result) => result,
            Err(err) => {
                println!("\x1b[31m{err}\x1b[0m");
                return Err(err);
            }
        };
        Ok(result)
    }

    macro_rules! expression_test {
        ($name:ident, $expr:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let expr = $expr.to_string();

                let result = exe_expression(&expr);

                match result {
                    Ok(value) => {
                        let expected: f32 = $expected;

                        assert!(
                            (value - expected).abs() < 0.0001,
                            "Expression: {}\nExpected: {}\nGot: {}",
                            expr,
                            expected,
                            value
                        );
                    }

                    Err(error) => {
                        panic!(
                            "Expression: {}\nExpected: {}\nGot error: {}",
                            expr, $expected, error
                        );
                    }
                }
            }
        };
    }

    macro_rules! expression_error_test {
        ($name:ident, $expr:expr) => {
            #[test]
            fn $name() {
                let expr = $expr.to_string();

                let result = exe_expression(&expr);

                assert!(
                    result.is_err(),
                    "Expression: {}\nExpected: Err\nGot: {:?}",
                    expr,
                    result
                );
            }
        };
    }
    // =========================
    // Basic arithmetic
    // =========================

    expression_test!(test_001, "1", 1.0);
    expression_test!(test_002, "2", 2.0);
    expression_test!(test_003, "1+2", 3.0);
    expression_test!(test_004, "5-3", 2.0);
    expression_test!(test_005, "4*3", 12.0);
    expression_test!(test_006, "12/4", 3.0);
    expression_test!(test_007, "2^3", 8.0);
    expression_test!(test_008, "10+5", 15.0);
    expression_test!(test_009, "10-7", 3.0);
    expression_test!(test_010, "6*7", 42.0);

    // =========================
    // Operator precedence
    // =========================

    expression_test!(test_011, "1+2*3", 7.0);
    expression_test!(test_012, "2*3+1", 7.0);
    expression_test!(test_013, "10-2*3", 4.0);
    expression_test!(test_014, "10+6/2", 13.0);
    expression_test!(test_015, "20/5+3", 7.0);
    expression_test!(test_016, "2+3*4-5", 9.0);
    expression_test!(test_017, "2*3+4*5", 26.0);
    expression_test!(test_018, "10-6/2", 7.0);
    expression_test!(test_019, "2^3*4", 32.0);
    expression_test!(test_020, "2*3^2", 18.0);

    // =========================
    // Parentheses
    // =========================

    expression_test!(test_021, "(1+2)", 3.0);
    expression_test!(test_022, "(1+2)*3", 9.0);
    expression_test!(test_023, "1+(2*3)", 7.0);
    expression_test!(test_024, "(10-3)*2", 14.0);
    expression_test!(test_025, "10/(2+3)", 2.0);
    expression_test!(test_026, "(2+3)*(4+5)", 45.0);
    expression_test!(test_027, "(10+5)/3", 5.0);
    expression_test!(test_028, "((1+2))", 3.0);
    expression_test!(test_029, "(((5)))", 5.0);
    expression_test!(test_030, "(2*(3+4))", 14.0);

    // =========================
    // Nested expressions
    // =========================

    expression_test!(test_031, "1+(2*(3+4))", 15.0);
    expression_test!(test_032, "(1+2)*(3+4)", 21.0);
    expression_test!(test_033, "((1+2)*3)-4", 5.0);
    expression_test!(test_034, "2*(3+(4*5))", 46.0);
    expression_test!(test_035, "(2+3)*(4-1)", 15.0);
    expression_test!(test_036, "(10-(2+3))*2", 10.0);
    expression_test!(test_037, "((10/2)+3)*2", 16.0);
    expression_test!(test_038, "(2^3)+(4*5)", 28.0);
    expression_test!(test_039, "(2+3)^2", 25.0);
    expression_test!(test_040, "2^(3+2)", 32.0);

    // =========================
    // Unary minus
    // =========================

    expression_test!(test_041, "-1", -1.0);
    expression_test!(test_042, "-5+3", -2.0);
    expression_test!(test_043, "5+-3", 2.0);
    expression_test!(test_044, "5--3", 8.0);
    expression_test!(test_045, "-2*3", -6.0);
    expression_test!(test_046, "-2*3+4", -2.0);
    expression_test!(test_047, "-(1+2)", -3.0);
    expression_test!(test_048, "-(2*3)", -6.0);
    expression_test!(test_049, "-((1+2)*3)", -9.0);
    expression_test!(test_050, "(-5)+10", 5.0);

    // =========================
    // Powers
    // =========================

    expression_test!(test_051, "2^2", 4.0);
    expression_test!(test_052, "2^3", 8.0);
    expression_test!(test_053, "2^4", 16.0);
    expression_test!(test_054, "3^2", 9.0);
    expression_test!(test_055, "3^3", 27.0);
    expression_test!(test_056, "5^2", 25.0);
    expression_test!(test_057, "10^2", 100.0);
    expression_test!(test_058, "(2^3)^2", 64.0);
    expression_test!(test_059, "2^(3^2)", 512.0);
    expression_test!(test_060, "(2+1)^3", 27.0);

    // =========================
    // Decimal arithmetic
    // =========================

    expression_test!(test_061, "1.5+2.5", 4.0);
    expression_test!(test_062, "5.5-2.5", 3.0);
    expression_test!(test_063, "2.5*4", 10.0);
    expression_test!(test_064, "10/4", 2.5);
    expression_test!(test_065, "0.5+0.25", 0.75);
    expression_test!(test_066, "1.5*2", 3.0);
    expression_test!(test_067, "7.5/2.5", 3.0);
    expression_test!(test_068, "(1.5+2.5)*2", 8.0);
    expression_test!(test_069, "10.5-5.25", 5.25);
    expression_test!(test_070, "2.5^2", 6.25);

    // =========================
    // max()
    // =========================

    expression_test!(test_071, "max(1,2)", 2.0);
    expression_test!(test_072, "max(10,5)", 10.0);
    expression_test!(test_073, "max(-1,3)", 3.0);
    expression_test!(test_074, "max(5,5)", 5.0);
    expression_test!(test_075, "max(1+2,5)", 5.0);

    // =========================
    // min()
    // =========================

    expression_test!(test_076, "min(1,2)", 1.0);
    expression_test!(test_077, "min(10,5)", 5.0);
    expression_test!(test_078, "min(-1,3)", -1.0);
    expression_test!(test_079, "min(5,5)", 5.0);
    expression_test!(test_080, "min(1+2,5)", 3.0);

    // =========================
    // sqrt()
    // =========================

    expression_test!(test_081, "sqrt(4)", 2.0);
    expression_test!(test_082, "sqrt(9)", 3.0);
    expression_test!(test_083, "sqrt(16)", 4.0);
    expression_test!(test_084, "sqrt(25)", 5.0);
    expression_test!(test_085, "sqrt(2)^2", 2.0);

    // =========================
    // sin()
    // radians
    // =========================

    expression_test!(test_086, "sin(0)", 0.0);
    expression_test!(test_087, "sin(1.57079632679)", 1.0);
    expression_test!(test_088, "sin(3.14159265359)", 0.0);

    // =========================
    // cos()
    // =========================

    expression_test!(test_089, "cos(0)", 1.0);
    expression_test!(test_090, "cos(1.57079632679)", 0.0);
    expression_test!(test_091, "cos(3.14159265359)", -1.0);

    // =========================
    // tan()
    // =========================

    expression_test!(test_092, "tan(0)", 0.0);
    expression_test!(test_093, "tan(0.78539816339)", 1.0);

    // =========================
    // Functions + arithmetic
    // =========================

    expression_test!(test_094, "sqrt(16)+2", 6.0);
    expression_test!(test_095, "sqrt(9)*2", 6.0);
    expression_test!(test_096, "max(2,5)+3", 8.0);
    expression_test!(test_097, "min(10,4)*2", 8.0);
    expression_test!(test_098, "max(1+2,2*3)", 6.0);
    expression_test!(test_099, "sqrt(16)+(max(2,5)*2)", 14.0);
    expression_test!(test_100, "min(sqrt(16),max(2,5))", 4.0);

    // =========================
    // Expressions that should fail
    // =========================

    // Empty / incomplete expressions
    expression_error_test!(error_001, "");
    expression_error_test!(error_002, "+");
    expression_error_test!(error_003, "-");
    expression_error_test!(error_004, "*");
    expression_error_test!(error_005, "/");
    expression_error_test!(error_006, "^");
    expression_error_test!(error_007, "1+");
    expression_error_test!(error_008, "1-");
    expression_error_test!(error_009, "1*");
    expression_error_test!(error_010, "1/");
    expression_error_test!(error_011, "1^");

    // Invalid operator combinations
    expression_error_test!(error_012, "1++2");
    expression_error_test!(error_013, "1**2");
    expression_error_test!(error_014, "1//2");
    expression_error_test!(error_015, "1^^2");
    expression_error_test!(error_016, "1+*2");
    expression_error_test!(error_017, "1*/2");
    expression_error_test!(error_018, "*1");
    expression_error_test!(error_019, "/1");
    expression_error_test!(error_020, "^1");

    // Parentheses
    expression_error_test!(error_021, "(");
    expression_error_test!(error_022, ")");
    expression_error_test!(error_023, "(1");
    expression_error_test!(error_024, "1)");
    expression_error_test!(error_025, "((1+2)");
    expression_error_test!(error_026, "(1+2))");
    expression_error_test!(error_027, "()");
    expression_error_test!(error_028, "(+)");
    expression_error_test!(error_029, "(*)");
    expression_error_test!(error_030, "1+(2");

    // Invalid characters
    expression_error_test!(error_031, "1&2");
    expression_error_test!(error_032, "1$2");
    expression_error_test!(error_033, "1@2");
    expression_error_test!(error_034, "1#2");
    expression_error_test!(error_035, "1%2");
    expression_error_test!(error_036, "1=2");
    expression_error_test!(error_037, "abc");
    expression_error_test!(error_038, "1a2");
    expression_error_test!(error_039, "hello+2");
    expression_error_test!(error_040, "1+hello");

    // Invalid numbers
    expression_error_test!(error_041, "1.2.3");
    expression_error_test!(error_042, "1..2");
    expression_error_test!(error_043, "..1");
    expression_error_test!(error_044, "1..");
    expression_error_test!(error_045, ".1.2");

    // Invalid function calls
    expression_error_test!(error_046, "max()");
    expression_error_test!(error_047, "min()");
    expression_error_test!(error_048, "sqrt()");
    expression_error_test!(error_049, "sin()");
    expression_error_test!(error_050, "cos()");
    expression_error_test!(error_051, "tan()");

    // Wrong number of arguments
    expression_error_test!(error_052, "max(1)");
    expression_error_test!(error_053, "min(1)");
    expression_error_test!(error_054, "max(1,2,3)");
    expression_error_test!(error_055, "min(1,2,3)");

    // Missing arguments
    expression_error_test!(error_056, "max(,2)");
    expression_error_test!(error_057, "max(1,)");
    expression_error_test!(error_058, "min(,2)");
    expression_error_test!(error_059, "min(1,)");
    expression_error_test!(error_060, "sqrt(,1)");

    // Unknown functions
    expression_error_test!(error_061, "foo(1)");
    expression_error_test!(error_062, "bar(1)");
    expression_error_test!(error_063, "average(1,2)");
    expression_error_test!(error_064, "power(2,3)");

    // Division by zero
    expression_error_test!(error_065, "1/0");
    expression_error_test!(error_066, "10/0");
    expression_error_test!(error_067, "1/(2-2)");
    expression_error_test!(error_068, "10/(5-5)");

    // Functions used incorrectly
    expression_error_test!(error_072, "max(1)");
    expression_error_test!(error_073, "min(1)");
    expression_error_test!(error_074, "sqrt(1,2)");
    expression_error_test!(error_075, "sin(1,2)");
    expression_error_test!(error_076, "cos(1,2)");
    expression_error_test!(error_077, "tan(1,2)");

    // Operators with parentheses in bad positions
    expression_error_test!(error_078, "1+()");
    expression_error_test!(error_079, "1*()");
    expression_error_test!(error_080, "()+1");
    expression_error_test!(error_081, "(1+)*2");
    expression_error_test!(error_082, "(*1)");
    expression_error_test!(error_083, "(1*)");
    expression_error_test!(error_084, "(( ))");

    // Multiple expressions / junk after expression
    expression_error_test!(error_085, "1 2");
    expression_error_test!(error_086, "1+2 3");
    expression_error_test!(error_087, "1+2abc");
    expression_error_test!(error_088, "1+2)");
    expression_error_test!(error_089, "(1+2))");
    expression_error_test!(error_090, "1(2)");
    expression_error_test!(error_091, ")1(");

    // More malformed expressions
    expression_error_test!(error_092, "1+(2*)");
    expression_error_test!(error_093, "1+((2+3)");
    expression_error_test!(error_094, "((1+2))(");
    expression_error_test!(error_095, "1+2+");
    expression_error_test!(error_096, "1*2*");
    expression_error_test!(error_097, "1/2/");
    expression_error_test!(error_098, "1^2^");
    expression_error_test!(error_099, "max(1,(2+))");
    expression_error_test!(error_100, "sqrt((1+2)");
}
