#[derive(Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    Equal,
    Comma,
    OpenParen,
    CloseParen,

    Identifier,
    Integer,
    Float,
    Exponent,

    Error,
    EOL,
}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        *self
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub lexeme: String,
    pub token_type: TokenType,
}

impl Clone for Token {
    fn clone(&self) -> Token {
        Token {
            lexeme: self.lexeme.clone(),
            token_type: self.token_type.clone(),
        }
    }
}

#[derive(Copy)]
enum State {
    Start,

    Integer,
    Float,
    Identifier,
    Exponent,

    Finish,
}

impl Clone for State {
    fn clone(&self) -> State {
        *self
    }
}

pub struct Scanner {
    chars: Vec<char>,
    char_pos: usize,
    next_char: char,
    next_state: State,
}

impl Scanner {
    pub fn new(s: &str) -> Self {
        let mut scanner = Scanner {
            chars: s.chars().collect(),
            char_pos: 0,
            next_char: '\0',
            next_state: State::Start,
        };
        scanner.read_next_char();
        scanner
    }

    fn read_next_char(&mut self) {
        if self.char_pos < self.chars.len() {
            self.next_char = self.chars[self.char_pos];
            self.char_pos += 1;
        } else {
            self.next_char = '\0';
        }
    }

    fn eat(&mut self, c: char) {
        loop {
            self.read_next_char();
            if self.next_char != c {
                break;
            }
        }
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
    }

    pub fn next_token(&mut self) -> Token {
        let mut lexeme = String::new();

        loop {
            match self.next_state {
                State::Start => {
                    lexeme.clear();
                    match self.next_char {
                        '\0' | '\n' => {
                            self.next_state = State::Finish;
                        }
                        c @ '\t' | c @  ' ' => {
                            self.eat(c);
                        }
                        '=' => {
                            self.read_next_char();
                            return Token {
                                lexeme: "=".to_owned(),
                                token_type: TokenType::Equal,
                            }
                        }
                        '+' => {
                            self.read_next_char();
                            return Token {
                                lexeme: "+".to_owned(),
                                token_type: TokenType::Add,
                            }
                        }
                        '-' => {
                            self.read_next_char();
                            return Token {
                                lexeme: "-".to_owned(),
                                token_type: TokenType::Sub,
                            }
                        }
                        '*' => {
                            self.read_next_char();
                            return Token {
                                lexeme: "*".to_owned(),
                                token_type: TokenType::Mul,
                            }
                        }
                        '/' => {
                            self.read_next_char();
                            return Token {
                                lexeme: "/".to_owned(),
                                token_type: TokenType::Div,
                            }
                        }
                        '%' => {
                            self.read_next_char();
                            return Token {
                                lexeme: "%".to_owned(),
                                token_type: TokenType::Mod,
                            }
                        }
                        '^' => {
                            self.read_next_char();
                            return Token {
                                lexeme: "^".to_owned(),
                                token_type: TokenType::Pow,
                            }
                        }
                        ',' => {
                            self.read_next_char();
                            return Token {
                                lexeme: ",".to_owned(),
                                token_type: TokenType::Comma,
                            }
                        }
                        '(' => {
                            self.read_next_char();
                            return Token {
                                lexeme: "(".to_owned(),
                                token_type: TokenType::OpenParen,
                            }
                        }
                        ')' => {
                            self.read_next_char();
                            return Token {
                                lexeme: ")".to_owned(),
                                token_type: TokenType::CloseParen,
                            }
                        }
                        c @ '.' => {
                            self.read_next_char();
                            lexeme.push(c);
                            self.next_state = State::Float;
                        }
                        c => {
                            lexeme.push(c);
                            self.read_next_char();
                            if Self::is_alpha(c) || c == '_' {
                                self.next_state = State::Identifier;
                            } else if Self::is_digit(c) {
                                self.next_state = State::Integer;
                            } else {
                                self.next_state = State::Start;
                                return Token {
                                    lexeme,
                                    token_type: TokenType::Error,
                                }
                            }
                        }
                    }
                }
                State::Identifier => {
                    while Self::is_alpha(self.next_char)
                            || Self::is_digit(self.next_char)
                            || self.next_char == '_' {
                        lexeme.push(self.next_char);
                        self.read_next_char();
                    }

                    self.next_state = State::Start;
                    return Token {
                        lexeme,
                        token_type: TokenType::Identifier,
                    }
                }
                State::Integer => {
                    while Self::is_digit(self.next_char) {
                        lexeme.push(self.next_char);
                        self.read_next_char();
                    }

                    if self.next_char == '.' {
                        lexeme.push(self.next_char);
                        self.read_next_char();
                        self.next_state = State::Float;
                    } else if self.next_char == 'e' {
                        lexeme.push(self.next_char);
                        self.read_next_char();
                        self.next_state = State::Exponent;
                    } else {
                        self.next_state = State::Start;
                        return Token {
                            lexeme,
                            token_type: TokenType::Integer,
                        }
                    }
                }
                State::Float => {
                    while Self::is_digit(self.next_char) {
                        lexeme.push(self.next_char);
                        self.read_next_char();
                    }

                    if self.next_char == 'e' {
                        lexeme.push(self.next_char);
                        self.read_next_char();
                        self.next_state = State::Exponent;
                    } else {
                        self.next_state = State::Start;
                        return Token {
                            lexeme,
                            token_type: TokenType::Float,
                        }
                    }
                }
                State::Exponent => {
                    match self.next_char {
                        '+' | '-' => {
                            lexeme.push(self.next_char);
                            self.read_next_char();
                        }
                        _ => {}
                    }

                    while Self::is_digit(self.next_char) {
                        lexeme.push(self.next_char);
                        self.read_next_char();
                    }

                    self.next_state = State::Start;
                    return Token {
                        lexeme,
                        token_type: TokenType::Exponent,
                    }
                }
                State::Finish => {
                    return Token {
                        lexeme: "".to_owned(),
                        token_type: TokenType::EOL,
                    }
                }
            }
        }
    }

    pub fn look_ahead(&mut self) -> Token {
        let char_pos = self.char_pos;
        let next_char = self.next_char;
        let next_state = self.next_state.clone();

        let token = self.next_token();

        self.char_pos  = char_pos;
        self.next_char = next_char;
        self.next_state = next_state;

        return token;
    }
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_state {
            State::Finish => None,
            _ => Some(self.next_token())
        }
    }
}

#[cfg(test)]
mod test {

}
