use crate::calc::{Function, Scanner, Token, TokenType};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Grammar
/// Input   = [ Identifier "=" ] Exp.           // "=" requires look-ahead
/// Exp     = Added { Op0 Addend }.
/// Addend  = Factor { Op1 Factor }.
/// Factor  = Power [ Op2 Factor ].
/// Power   = Literal | Call | "(" Exp ")" | Op0 Power.
/// Literal = Float | Integer | Exponent.
/// Call    = Identifier [ "(" ExpList ")" ].
/// ExpList = [ Exp { "," Exp } ].
/// Op0     = "+" | "-".
/// Op1     = "*" | "/" | "%".
/// Op2     = "^".

#[derive(Debug, PartialEq, Eq, Hash)]
enum NonTerminal {
    Op0,
    Op1,
    Op2,
    Line,
    Exp,
    Addend,
    Factor,
    Power,
    Call,
    ExpList,
    Literal,
}

#[derive(Debug)]
pub struct PResult {
    pub var: Option<String>,
    pub val: f64,
}

pub struct Parser {
    scanner: Option<Scanner>,
    token: Token,
    first_sets: HashMap<NonTerminal, HashSet<TokenType>>,

    functions: Rc<RefCell<HashMap<&'static str, Function>>>,
    constants: Rc<HashMap<&'static str, f64>>,
    memory: Rc<RefCell<HashMap<String, f64>>>,
}

impl Parser {
    pub fn new(
        functions: Rc<RefCell<HashMap<&'static str, Function>>>,
        constants: Rc<HashMap<&'static str, f64>>,
        memory: Rc<RefCell<HashMap<String, f64>>>,
    ) -> Self {
        let mut first_sets = HashMap::new();

        let mut fs = HashSet::new();
        fs.insert(TokenType::Integer);
        fs.insert(TokenType::Float);
        fs.insert(TokenType::Exponent);
        first_sets.insert(NonTerminal::Literal, fs);

        let mut fs = HashSet::new();
        fs.insert(TokenType::Add);
        fs.insert(TokenType::Sub);
        first_sets.insert(NonTerminal::Op0, fs);

        let mut fs = HashSet::new();
        fs.insert(TokenType::Mul);
        fs.insert(TokenType::Div);
        fs.insert(TokenType::Mod);
        first_sets.insert(NonTerminal::Op1, fs);

        let mut fs = HashSet::new();
        fs.insert(TokenType::Pow);
        first_sets.insert(NonTerminal::Op2, fs);

        let mut fs = HashSet::new();
        fs.insert(TokenType::Identifier);
        first_sets.insert(NonTerminal::Call, fs);

        let mut fs = HashSet::new();
        for i in first_sets.get(&NonTerminal::Literal).unwrap() {
            fs.insert(i.clone());
        }
        for i in first_sets.get(&NonTerminal::Call).unwrap() {
            fs.insert(i.clone());
        }
        fs.insert(TokenType::OpenParen);
        for i in first_sets.get(&NonTerminal::Op0).unwrap() {
            fs.insert(i.clone());
        }
        first_sets.insert(NonTerminal::Power, fs);

        let mut fs = HashSet::new();
        for i in first_sets.get(&NonTerminal::Power).unwrap() {
            fs.insert(i.clone());
        }
        first_sets.insert(NonTerminal::Factor, fs);

        let mut fs = HashSet::new();
        for i in first_sets.get(&NonTerminal::Factor).unwrap() {
            fs.insert(i.clone());
        }
        first_sets.insert(NonTerminal::Addend, fs);

        let mut fs = HashSet::new();
        for i in first_sets.get(&NonTerminal::Addend).unwrap() {
            fs.insert(i.clone());
        }
        first_sets.insert(NonTerminal::Exp, fs);

        let mut fs = HashSet::new();
        for i in first_sets.get(&NonTerminal::Exp).unwrap() {
            fs.insert(i.clone());
        }
        first_sets.insert(NonTerminal::ExpList, fs);

        let mut fs = HashSet::new();
        for i in first_sets.get(&NonTerminal::Exp).unwrap() {
            fs.insert(i.clone());
        }
        fs.insert(TokenType::Identifier);
        first_sets.insert(NonTerminal::Line, fs);

        Parser {
            scanner: None,
            token: Token {
                token_type: TokenType::EOL,
                lexeme: "".to_owned(),
            },
            first_sets,
            functions,
            constants,
            memory,
        }
    }

    fn have_nt(&mut self, nt: &NonTerminal) -> bool {
        self.first_sets
            .get(nt)
            .unwrap()
            .contains(&self.token.token_type)
    }

    fn have(&mut self, token_type: &TokenType) -> bool {
        self.token.token_type == *token_type
    }

    fn have_next(&mut self, token_type: &TokenType) -> bool {
        self.scanner.as_mut().unwrap().look_ahead().token_type == *token_type
    }

    fn accept(&mut self, token_type: &TokenType) -> bool {
        if self.have(token_type) {
            self.token = self.scanner.as_mut().unwrap().next_token().clone();
            true
        } else {
            false
        }
    }

    fn accept_nt(&mut self, nt: &NonTerminal) -> bool {
        if self.have_nt(nt) {
            self.token = self.scanner.as_mut().unwrap().next_token().clone();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, token_type: &TokenType) -> Result<(), String> {
        if self.accept(token_type) {
            Ok(())
        } else {
            Err("invalid syntax".to_owned())
        }
    }

    fn expect_retrieve(&mut self, token_type: &TokenType) -> Result<Token, String> {
        let token = self.token.clone();
        if self.accept(token_type) {
            Ok(token)
        } else {
            Err("invalid syntax".to_owned())
        }
    }

    fn expect_nt_retrieve(&mut self, nt: &NonTerminal) -> Result<Token, String> {
        let token = self.token.clone();
        if self.accept_nt(nt) {
            Ok(token)
        } else {
            Err("invalid syntax".to_owned())
        }
    }

    fn op0(&mut self) -> Result<Token, String> {
        self.expect_nt_retrieve(&NonTerminal::Op0)
    }

    fn op1(&mut self) -> Result<Token, String> {
        self.expect_nt_retrieve(&NonTerminal::Op1)
    }

    fn op2(&mut self) -> Result<Token, String> {
        self.expect_nt_retrieve(&NonTerminal::Op2)
    }

    fn literal(&mut self) -> Result<f64, String> {
        let token = self.expect_nt_retrieve(&NonTerminal::Literal)?;
        match token.lexeme.parse::<f64>() {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn power(&mut self) -> Result<f64, String> {
        if self.have_nt(&NonTerminal::Literal) {
            Ok(self.literal()?)
        } else if self.have_nt(&NonTerminal::Call) {
            Ok(self.call()?)
        } else if self.accept(&TokenType::OpenParen) {
            let v = self.exp()?;
            self.expect(&TokenType::CloseParen)?;
            Ok(v)
        } else if self.have_nt(&NonTerminal::Op0) {
            let op0 = self.op0()?;
            let val = self.power()?;
            match op0.lexeme.as_ref() {
                "-" => Ok(-val),
                "+" => Ok(val),
                _ => unreachable!("BUG ALERT"),
            }
        } else {
            Err("invalid syntax".to_owned())
        }
    }

    fn factor(&mut self) -> Result<f64, String> {
        let mut a = self.power()?;
        if self.have_nt(&NonTerminal::Op2) {
            let op2 = self.op2()?;
            let b = self.factor()?;

            match op2.lexeme.as_ref() {
                "^" => a = a.powf(b),
                _ => unreachable!("BUG ALERT"),
            }
        }

        Ok(a)
    }

    fn addend(&mut self) -> Result<f64, String> {
        let mut a = self.factor()?;

        while self.have_nt(&NonTerminal::Op1) {
            let op1 = self.op1()?;
            let b = self.factor()?;

            match op1.lexeme.as_ref() {
                "*" => a = a * b,
                "/" => a = a / b,
                "%" => a = a % b,
                _ => unreachable!("BUG ALERT!"),
            }
        }

        Ok(a)
    }

    fn exp(&mut self) -> Result<f64, String> {
        let mut a = self.addend()?;

        while self.have_nt(&NonTerminal::Op0) {
            let op0 = self.op0()?;
            let b = self.addend()?;

            match op0.lexeme.as_ref() {
                "-" => a = a - b,
                "+" => a = a + b,
                _ => unreachable!("BUG ALERT"),
            }
        }

        Ok(a)
    }

    fn exp_list(&mut self) -> Result<Vec<f64>, String> {
        let mut exp_list = Vec::new();
        if self.have_nt(&NonTerminal::Exp) {
            exp_list.push(self.exp()?);

            while self.accept(&TokenType::Comma) {
                exp_list.push(self.exp()?);
            }
        }

        Ok(exp_list)
    }

    fn call(&mut self) -> Result<f64, String> {
        let lexeme = self.expect_retrieve(&TokenType::Identifier)?.lexeme;

        if self.accept(&TokenType::OpenParen) {
            let params = self.exp_list()?;
            self.expect(&TokenType::CloseParen)?;
            match self.functions.borrow().get(&lexeme.as_ref()) {
                Some(function) => {
                    if (function.param_count < 0
                        && params.len() >= function.param_count.abs() as usize)
                        || (params.len() == function.param_count as usize)
                    {
                        let f = function.f;
                        Ok(f(&params))
                    } else {
                        if function.param_count < 0 {
                            Err(format!(
                                "{} requires {} or more arguments",
                                lexeme,
                                function.param_count.abs()
                            ))
                        } else {
                            Err(format!(
                                "{} requires {} arguments",
                                lexeme, function.param_count
                            ))
                        }
                    }
                }
                None => Err(format!("unknown function: {}", lexeme)),
            }
        } else {
            match self.constants.get(&lexeme.as_ref()) {
                Some(v) => Ok(*v),
                None => match self.memory.borrow().get(&lexeme) {
                    Some(v) => Ok(*v),
                    None => Err(format!("unknown variable: {}", lexeme)),
                },
            }
        }
    }

    fn is_valid_assignment(&self, var: &str) -> bool {
        match var {
            "ans" => false,
            v => {
                if self.constants.get(v).is_some() {
                    return false;
                }

                if self.functions.borrow().get(v).is_some() {
                    return false;
                }

                true
            }
        }
    }

    fn input(&mut self) -> Result<PResult, String> {
        let mut result = PResult {
            var: None,
            val: 0.0,
        };
        if self.have(&TokenType::Identifier) && self.have_next(&TokenType::Equal) {
            if !self.is_valid_assignment(&self.token.lexeme) {
                return Err("cannot assign to built-in vars, constants or functions".into());
            }

            result.var = Some(self.token.lexeme.clone());
            self.accept(&TokenType::Identifier);
            self.accept(&TokenType::Equal);
        }

        result.val = self.exp()?;

        Ok(result)
    }

    pub fn parse(&mut self, scanner: Scanner) -> Result<Option<PResult>, String> {
        self.scanner = Some(scanner);
        self.token = self.scanner.as_mut().unwrap().next_token();

        let mut result: Option<PResult> = None;

        if self.have_nt(&NonTerminal::Line) {
            result = Some(self.input()?);
        }

        self.expect(&TokenType::EOL)?;
        Ok(result)
    }
}
