use calc::{Parser, Scanner};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Function {
    pub param_count: usize,
    pub f: (fn(&[f64]) -> f64),
}

pub struct Calculator {
    parser: Parser,
    memory: Rc<RefCell<HashMap<String, f64>>>,
}

impl Calculator {
    pub fn new() -> Self {
        let memory = Rc::new(RefCell::new(HashMap::new()));
        let mut constants = HashMap::new();
        let mut functions = HashMap::new();

        memory.borrow_mut().insert("ans".into(), 0.0);

        constants.insert("pi", ::std::f64::consts::PI);
        constants.insert("e", ::std::f64::consts::E);

        functions.insert("sin", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].sin_cos().0 },
        });
        functions.insert("cos", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].sin_cos().1 },
        });
        functions.insert("tan", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].tan() },
        });

        functions.insert("sinh", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].sinh() },
        });
        functions.insert("cosh", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].cosh() },
        });
        functions.insert("tanh", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].tanh() },
        });

        functions.insert("asin", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].asin() },
        });
        functions.insert("acos", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].acos() },
        });
        functions.insert("atan", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].atan() },
        });

        functions.insert("abs", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].abs() },
        });
        functions.insert("log10", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].log10() },
        });
        functions.insert("ln", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].ln() },
        });
        functions.insert("log2", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].log2() },
        });
        functions.insert("log", Function {
            param_count: 2,
            f: |p: &[f64]| -> f64 { p[0].log(p[1]) },
        });

        Calculator {
            parser: Parser::new(functions, constants, memory.clone()),
            memory,
        }
    }

    pub fn run(&mut self, input: &str) -> Result<String, String> {
        match self.parser.parse(Scanner::new(&input)) {
            Ok(result) => {
                match result {
                    Some(result) => {
                        self.memory.borrow_mut().insert("ans".into(), result.val);
                        if result.var.is_some() {
                            self.memory.borrow_mut().insert(result.var.as_ref().unwrap().clone(), result.val);
                            Ok(format!("{} = {}", result.var.unwrap(), result.val))
                        } else {
                            Ok(format!("{}", result.val))
                        }
                    }
                    None => Ok("".into()),
                }
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use calc::Calculator;

    macro_rules! run_test {
        ($input:expr, $expected: expr) => {
            let result = Calculator::new().run($input).unwrap();
            println!("{} => {}", $input, result);
            assert_eq!(result, $expected);
        };
        ($calc:expr, $input:expr, $expected: expr) => {
            let result = $calc.run($input).unwrap();
            println!("{} => {}", $input, result);
            assert_eq!(result, $expected);
        };
    }

    macro_rules! run_test_err {
        ($input:expr) => {
            let result = Calculator::new().run($input);
            println!("{} => {:?}", $input, result);
            assert!(result.is_err());
        };
        ($calc:expr, $input:expr) => {
            let result = $calc.run($input);
            println!("{} => {:?}", $input, result);
            assert!(result.is_err());
        };
    }

    #[test]
    fn basic() {
        run_test!("2+3", "5");
        run_test!("2-3", "-1");
        run_test!("5*10", "50");
        run_test!("5*-10", "-50");
        run_test!("-5*-10", "50");
        run_test!("-5*10", "-50");
        run_test!("10/2", "5");
        run_test!("26%12", "2");
        run_test!("3^3", "27");
    }

    #[test]
    fn constants() {
        run_test!("pi", format!("{}", ::std::f64::consts::PI));
        run_test!("e", format!("{}", ::std::f64::consts::E));
    }

    #[test]
    fn ans() {
        let mut calc = Calculator::new();
        assert_eq!(calc.run("1+2").unwrap(), "3");
        assert_eq!(calc.run("ans").unwrap(), "3");
        assert_eq!(calc.run("4*(2*(3+2))").unwrap(), "40");
        assert_eq!(calc.run("5*4").unwrap(), "20");
        assert_eq!(calc.run("12/10").unwrap(), (12.0 as f64 / 10.0 as f64).to_string());
        assert_eq!(calc.run("2+5").unwrap(), "7");
        assert_eq!(calc.run("ans").unwrap(), "7");
        assert_eq!(calc.run("ans*2").unwrap(), "14");
    }

    #[test]
    fn vars() {
        let mut calc = Calculator::new();
        run_test!(calc, "a=2", "a = 2");
        run_test!(calc, "a*5", "10");
        run_test!(calc, "-a", "-2");
        run_test!(calc, "-(a)", "-2");
        run_test!(calc, "a = -(a)*10", "a = -20");
        run_test!(calc, "a = -(a) - 21", "a = -1");
        run_test!(calc, "-(a)^2", "1");
        run_test!(calc, "-(a^2)", "-1");
    }

    #[test]
    fn builtins() {
        run_test!("sin(pi)", format!("{}", ::std::f64::consts::PI.sin_cos().0));
        run_test!("sin(pi/2)", format!("{}", (::std::f64::consts::PI / 2.0).sin_cos().0));
        run_test!("cos(pi)", format!("{}", ::std::f64::consts::PI.sin_cos().1));
        run_test!("cos(pi/2)", format!("{}", (::std::f64::consts::PI / 2.0).sin_cos().1));
        run_test!("tan(pi)", format!("{}", ::std::f64::consts::PI.tan()));
        run_test!("tan(pi/2)", format!("{}", (::std::f64::consts::PI / 2.0).tan()));
        run_test!("abs(5)", "5");
        run_test!("abs(-5)", "5");
        run_test!("abs(-1.23)", "1.23");
        run_test!("abs(4.5)", "4.5");
        run_test!("sinh(20)", format!("{}", (20.0 as f64).sinh()));
        run_test!("cosh(20)", format!("{}", (20.0 as f64).cosh()));
        run_test!("tanh(20)", format!("{}", (20.0 as f64).tanh()));
        run_test!("asin(20)", format!("{}", (20.0 as f64).asin()));
        run_test!("acos(20)", format!("{}", (20.0 as f64).acos()));
        run_test!("atan(20)", format!("{}", (20.0 as f64).atan()));
    }

    #[test]
    fn error_handling() {
        run_test_err!("(2");
        run_test_err!("2-");
        run_test_err!("sin");
        run_test_err!("sin 2");
        run_test_err!("()");
        run_test_err!("a");
    }

    #[test]
    fn regression() {
        run_test!("2", "2");
        run_test!("(2)", "2");
        run_test!("-2", "-2");
        run_test!("--2", "2");
        run_test!("---2", "-2");
        run_test!("2", "2");
        run_test!("2+-3", "-1");
        run_test!("-2+3", "1");
        run_test!("-2+-3", "-5");
        run_test!("2--3", "5");
        run_test!("-(2)", "-2");
        run_test!("-(-2)", "2");
        run_test!("-(-2) + 3", "5");
        run_test!("-(-2) + (3-4)", "1");
        run_test!("1 - -(-2) + (3-4)", "-2");
        run_test!("(2+2)", "4");
        run_test!("(-2+2)", "0");
    }
}