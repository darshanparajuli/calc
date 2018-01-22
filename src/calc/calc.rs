use calc::{Parser, Scanner};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Function {
    pub param_count: isize,
    pub f: (fn(&[f64]) -> f64),
    desc: &'static str,
}

pub struct Calculator {
    parser: Parser,
    memory: Rc<RefCell<HashMap<String, f64>>>,
    functions: Rc<RefCell<HashMap<&'static str, Function>>>,
}

impl Calculator {
    pub fn new() -> Self {
        let memory = Rc::new(RefCell::new(HashMap::new()));
        let mut constants = HashMap::new();
        let mut functions = HashMap::new();

        memory.borrow_mut().insert("ans".into(), 0.0);

        constants.insert("PI", ::std::f64::consts::PI);
        constants.insert("E", ::std::f64::consts::E);
        constants.insert("INF", ::std::f64::INFINITY);

        functions.insert("sin", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].sin_cos().0 },
            desc: "sin(n)",
        });
        functions.insert("cos", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].sin_cos().1 },
            desc: "cos(n)",
        });
        functions.insert("tan", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].tan() },
            desc: "tan(n)",
        });

        functions.insert("sinh", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].sinh() },
            desc: "sinh(n)",
        });
        functions.insert("cosh", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].cosh() },
            desc: "cosh(n)",
        });
        functions.insert("tanh", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].tanh() },
            desc: "tan(n)",
        });

        functions.insert("asin", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].asin() },
            desc: "asin(n)",
        });
        functions.insert("acos", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].acos() },
            desc: "acos(n)",
        });
        functions.insert("atan", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].atan() },
            desc: "atan(n)",
        });
        functions.insert("atan2", Function {
            param_count: 2,
            f: |p: &[f64]| -> f64 { p[1].atan2(p[0]) },
            desc: "atan2(x, y)",
        });

        functions.insert("abs", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].abs() },
            desc: "abs(n)",
        });
        functions.insert("log10", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].log10() },
            desc: "log10(n)",
        });
        functions.insert("ln", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].ln() },
            desc: "ln(n)",
        });
        functions.insert("log2", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].log2() },
            desc: "log2(n)",
        });
        functions.insert("log", Function {
            param_count: 2,
            f: |p: &[f64]| -> f64 { p[0].log(p[1]) },
            desc: "log(n, base)",
        });
        functions.insert("sqrt", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].sqrt() },
            desc: "sqrt(n)",
        });
        functions.insert("cbrt", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].cbrt() },
            desc: "cbrt(n)",
        });
        functions.insert("root", Function {
            param_count: 2,
            f: |p: &[f64]| -> f64 { p[0].powf(p[1].recip()) },
            desc: "root(n, root)",
        });
        functions.insert("sign", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].signum() },
            desc: "sign(n)",
        });
        functions.insert("exp", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].exp() },
            desc: "exp(n)",
        });

        functions.insert("trunc", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].trunc() },
            desc: "trunc(n)",
        });
        functions.insert("round", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].round() },
            desc: "round(n)",
        });
        functions.insert("floor", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].floor() },
            desc: "floor(n)",
        });
        functions.insert("ceil", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 { p[0].ceil() },
            desc: "ceil(n)",
        });

        functions.insert("min", Function {
            param_count: -2,
            f: |p: &[f64]| -> f64 {
                p.iter()
                 .skip(1)
                 .fold(p[0], |a, &b| a.min(b))
            },
            desc: "min(n, ...)",
        });
        functions.insert("max", Function {
            param_count: -2,
            f: |p: &[f64]| -> f64 {
                p.iter()
                 .skip(1)
                 .fold(p[0], |a, &b| a.max(b))
            },
            desc: "max(n, ...)",
        });

        functions.insert("rad2deg", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 {
                p[0] * (180_f64 / ::std::f64::consts::PI)
            },
            desc: "rad2deg(radians)",
        });
        functions.insert("deg2rad", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 {
                p[0] * (::std::f64::consts::PI / 180_f64)
            },
            desc: "deg2rad(degrees)",
        });

        functions.insert("grad2deg", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 {
                p[0] * (9_f64 / 10_f64)
            },
            desc: "grad2deg(gradians)",
        });
        functions.insert("deg2grad", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 {
                p[0] * (10_f64 / 9_f64)
            },
            desc: "deg2grad(degrees)",
        });

        functions.insert("grad2rad", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 {
                p[0] * (::std::f64::consts::PI / 200_f64)
            },
            desc: "grad2rad(gradians)",
        });
        functions.insert("rad2grad", Function {
            param_count: 1,
            f: |p: &[f64]| -> f64 {
                p[0] * (200_f64 / ::std::f64::consts::PI)
            },
            desc: "rad2grad(radians)",
        });

        let functions = Rc::new(RefCell::new(functions));

        Calculator {
            parser: Parser::new(functions.clone(), constants, memory.clone()),
            memory,
            functions,
        }
    }

    pub fn run(&mut self, input: &str) -> Result<String, String> {
        match self.parser.parse(Scanner::new(input)) {
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
            Err(e) => match self.functions.borrow().get(input) {
                Some(f) => Ok(format!("{}", f.desc)),
                None => Err(e),
            }
        }
    }

    pub fn reset(&mut self) {
        self.memory.borrow_mut().clear();
        self.memory.borrow_mut().insert("ans".into(), 0.0);
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
        run_test!("1e2", "100");
        run_test!("5 + 1e2", "105");
    }

    #[test]
    fn constants() {
        run_test!("PI", format!("{}", ::std::f64::consts::PI));
        run_test!("E", format!("{}", ::std::f64::consts::E));
        run_test!("INF", format!("{}", ::std::f64::INFINITY));
    }

    #[test]
    fn op_precedence() {
        run_test!("1+3/6*2", "2");
        run_test!("1+3*6/2", "10");
        run_test!("2^3^2", "512");
        run_test!("2^3+2", "10");
        run_test!("2^3*2", "16");
        run_test!("2^(3+2)", "32");
        run_test!("2^abs(-5)", "32");
        run_test!("1+4-2^5/2+8*2", "5");
        run_test!("(1+4-2)^5/2+8*2", "137.5");

        let a = (1_f64 + 4_f64 - 2_f64).powf(5_f64) / ((2_f64 + 8_f64).sin_cos().1) * 2_f64;
        run_test!("(1+4-2)^5/cos(2+8)*2", format!("{}", a));
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
        run_test!("sin(PI)", format!("{}", ::std::f64::consts::PI.sin_cos().0));
        run_test!("sin(PI/2)", format!("{}", (::std::f64::consts::PI / 2.0).sin_cos().0));
        run_test!("cos(PI)", format!("{}", ::std::f64::consts::PI.sin_cos().1));
        run_test!("cos(PI/2)", format!("{}", (::std::f64::consts::PI / 2.0).sin_cos().1));
        run_test!("tan(PI)", format!("{}", ::std::f64::consts::PI.tan()));
        run_test!("tan(PI/2)", format!("{}", (::std::f64::consts::PI / 2.0).tan()));
        run_test!("abs(5)", "5");
        run_test!("abs(-5)", "5");
        run_test!("abs(-1.23)", "1.23");
        run_test!("abs(4.5)", "4.5");
        run_test!("sinh(20)", format!("{}", 20_f64.sinh()));
        run_test!("cosh(20)", format!("{}", 20_f64.cosh()));
        run_test!("tanh(20)", format!("{}", 20_f64.tanh()));
        run_test!("asin(20)", format!("{}", 20_f64.asin()));
        run_test!("acos(20)", format!("{}", 20_f64.acos()));
        run_test!("atan(20)", format!("{}", 20_f64.atan()));
        run_test!("atan2(10, 20)", format!("{}", 20_f64.atan2(10_f64)));
        run_test!("sin", "sin(n)");
        run_test!("log(64, 8)", "2");
        run_test!("ln(E)", "1");
        run_test!("log2(32)", "5");
        run_test!("sqrt(16)", format!("{}", 16_f64.sqrt()));
        run_test!("cbrt(125)", format!("{}", 125_f64.cbrt()));
        run_test!("root(140, 5)", format!("{}", 140_f64.powf(5_f64.recip())));
        run_test!("sign(1)", format!("{}", 1_f64.signum()));
        run_test!("sign(-1)", format!("{}", -1_f64.signum()));
        run_test!("exp(20)", format!("{}", 20_f64.exp()));
        run_test!("trunc(43.123)", format!("{}", 43.123_f64.trunc()));
        run_test!("round(12.1234)", format!("{}", 12.1234_f64.round()));
        run_test!("floor(18.9)", format!("{}", 18.9_f64.floor()));
        run_test!("ceil(4.1)", format!("{}", 4.1_f64.ceil()));
        run_test!("min(-5, -1, 2, 30, -49)", "-49");
        run_test!("max(-5, -1, 8, 15, 4)", "15");

        run_test!("rad2deg(16)", format!("{}", 16_f64.to_degrees()));
        run_test!("deg2rad(16)", format!("{}", 16_f64.to_radians()));

        run_test!("rad2grad(16)", format!("{}", 16_f64 * 200_f64 / ::std::f64::consts::PI));
        run_test!("grad2rad(16)", format!("{}", 16_f64 * ::std::f64::consts::PI / 200_f64));

        run_test!("deg2grad(16)", format!("{}", 16_f64 * 10_f64 / 9_f64));
        run_test!("grad2deg(16)", format!("{}", 16_f64 * 9_f64 / 10_f64));
    }

    #[test]
    fn error_handling() {
        run_test_err!("(2");
        run_test_err!("2-");
        run_test_err!("sin 2");
        run_test_err!("()");
        run_test_err!("a");
    }

    #[test]
    fn misc() {
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
