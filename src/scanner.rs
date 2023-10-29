pub mod precedence {
    pub const NONE: u8 = 0;
    pub const ASSIGNMENT: u8 = 1;
    pub const OR: u8 = 2;
    pub const AND: u8 = 3;
    pub const EQUALITY: u8 = 4;
    pub const COMPARISON: u8 = 5;
    pub const TERM: u8 = 6;
    pub const FACTOR: u8 = 7;
    pub const UNARY: u8 = 8;
    pub const CALL: u8 = 9;
}

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    pub line_number: u32,
    pub precedence: u8,
}

#[derive(Debug)]
pub enum TokenType {
    None,
    Function(Token),
    Return(Token),
    While(Token),
    Then(Token),
    Else(Token),
    And(Token),
    Not(Token),
    End(Token),
    If(Token),
    Or(Token),
    NotEquals(Token),
    LessThanOrEqual(Token),
    GreaterThanOrEqual(Token),
    Equality(Token),
    LessThan(Token),
    GreaterThan(Token),
    Equals(Token),
    Plus(Token),
    Minus(Token),
    Times(Token),
    Divide(Token),
    LeftParan(Token),
    RightParan(Token),
    LeftBracket(Token),
    RightBracket(Token),
    Comma(Token),
    Dot(Token),
    Identifier(Token),
    String(Token),
    Number(Token),
    Bool(Token),
    Hat(Token),
    Mod(Token),

    For(Token),
    Next(Token),
    To(Token),
    Step(Token),

    ElseIf(Token),
    Eol(Token),
    In(Token),
    Match(Token),
    When(Token),
    Eof,
}

impl TokenType {
    pub fn get_token(&self) -> Option<&Token> {
        match self {
            TokenType::Function(t)
            | TokenType::Return(t)
            | TokenType::While(t)
            | TokenType::Then(t)
            | TokenType::Else(t)
            | TokenType::And(t)
            | TokenType::Not(t)
            | TokenType::End(t)
            | TokenType::If(t)
            | TokenType::Or(t)
            | TokenType::NotEquals(t)
            | TokenType::LessThanOrEqual(t)
            | TokenType::GreaterThanOrEqual(t)
            | TokenType::Equality(t)
            | TokenType::LessThan(t)
            | TokenType::GreaterThan(t)
            | TokenType::Equals(t)
            | TokenType::Plus(t)
            | TokenType::Minus(t)
            | TokenType::Times(t)
            | TokenType::Divide(t)
            | TokenType::LeftParan(t)
            | TokenType::RightParan(t)
            | TokenType::Comma(t)
            | TokenType::Dot(t)
            | TokenType::Identifier(t)
            | TokenType::String(t)
            | TokenType::LeftBracket(t)
            | TokenType::RightBracket(t)
            | TokenType::Number(t)
            | TokenType::Hat(t)
            | TokenType::Mod(t)
            | TokenType::For(t)
            | TokenType::To(t)
            | TokenType::Step(t)
            | TokenType::Next(t)
            | TokenType::ElseIf(t)
            | TokenType::In(t)
            | TokenType::Match(t)
            | TokenType::When(t)
            | TokenType::Bool(t) => Some(t),
            _ => None,
        }
    }
}

fn continue_line(token_type: Option<&TokenType>) -> bool {
    match token_type {
        Some(token_type) => match token_type {
            TokenType::LeftParan(_)
            | TokenType::RightParan(_)
            | TokenType::Comma(_)
            | TokenType::Plus(_) => true,
            _ => false,
        },
        _ => false,
    }
}

pub fn tokenize(code: &str) -> Result<Vec<TokenType>, &str> {
    let mut i = 0;
    let mut line_number = 1;
    let mut tokens: Vec<TokenType> = Vec::new();
    let mut interpolation = 0;

    while i < code.len() {
        let mut current_char = code.chars().nth(i).unwrap();
        if current_char == '\n' {
            if !continue_line(tokens.last()) {
                tokens.push(TokenType::Eol(Token {
                    lexeme: String::new(),
                    line_number,
                    precedence: precedence::NONE,
                }));
            }

            line_number += 1;
        }

        if current_char == '\'' {
            while i < code.len() && code.chars().nth(i).unwrap() != '\n' {
                i += 1
            }
            line_number += 1;
        }
        if i >= code.len() {
            if interpolation > 0 {
                return Err("missing '}' in string interpolation");
            }
            return Ok(tokens);
        }

        // ending interpolation
        let (token, len) = if interpolation > 0 && current_char == '}' {
            interpolation -= 1;
            // )
            let (t, _) = make_keyword(")", line_number);
            tokens.push(t);

            // +
            let (t, _) = make_keyword("+", line_number);
            tokens.push(t);

            current_char = '"';

            (TokenType::None, 0)
        } else {
            make_keyword(&code[i..], line_number)
        };

        //let (token, len) = make_keyword(&code[i..], line_number);
        if let TokenType::None = token {
            //Numbers
            if current_char.is_numeric() {
                let mut lexeme = String::new();
                while i < code.len() && (current_char.is_ascii_digit() || current_char == '.') {
                    lexeme.push(current_char);
                    i += 1;
                    if let Some(char) = code.chars().nth(i) {
                        current_char = char;
                    } else {
                        break;
                    }
                }
                tokens.push(TokenType::Number(Token {
                    lexeme,
                    line_number,
                    precedence: precedence::NONE,
                }));
            // raw string (""")
            } else if start_raw_string(&code[i..]) {
                let mut lexeme = String::new();
                i += 2;
                loop {
                    i += 1;
                    if let Some(char) = code.chars().nth(i) {
                        current_char = char;
                    } else {
                        break;
                    }

                    if !end_raw_string(&code[i..]) {
                        lexeme.push(current_char);
                    } else {
                        break;
                    }
                    if current_char == '\n' {
                        line_number += 1;
                    }
                }
                tokens.push(TokenType::String(Token {
                    lexeme,
                    line_number,
                    precedence: precedence::NONE,
                }));

                i += 3;

            // Strings
            } else if current_char == '"' {
                let mut lexeme = String::new();
                loop {
                    i += 1;
                    if let Some(char) = code.chars().nth(i) {
                        current_char = char;
                    } else {
                        break;
                    }
                    if current_char == '{' {
                        interpolation += 1;

                        // first bit of string
                        tokens.push(TokenType::String(Token {
                            lexeme: lexeme.clone(),
                            line_number,
                            precedence: precedence::NONE,
                        }));
                        // plus
                        let (t, _) = make_keyword("+", line_number);
                        tokens.push(t);
                        // str
                        tokens.push(TokenType::Identifier(Token {
                            lexeme: String::from("str"),
                            line_number,
                            precedence: precedence::NONE,
                        }));

                        // (
                        let (t, _) = make_keyword("(", line_number);
                        tokens.push(t);

                        break;
                    }

                    if current_char != '"' {
                        lexeme.push(current_char);
                    } else {
                        break;
                    }
                    if current_char == '\n' {
                        line_number += 1;
                    }
                }
                if current_char != '{' {
                    tokens.push(TokenType::String(Token {
                        lexeme,
                        line_number,
                        precedence: precedence::NONE,
                    }));
                }

                i += 1;
            } else if current_char.is_ascii_alphabetic()
                || current_char == '@'
                || current_char == '_'
            {
                // Identifier
                let mut lexeme = String::new();
                while i < code.len()
                    && (current_char.is_ascii_alphabetic()
                        || current_char == '_'
                        || current_char == '@'
                        || current_char.is_numeric())
                {
                    lexeme.push(current_char);
                    i += 1;
                    if let Some(char) = code.chars().nth(i) {
                        current_char = char;
                    } else {
                        break;
                    }
                }
                tokens.push(TokenType::Identifier(Token {
                    lexeme,
                    line_number,
                    precedence: precedence::NONE,
                }));
            } else {
                i += 1;
            }
        } else {
            tokens.push(token);
            i += len;
        }
    }
    if interpolation > 0 {
        return Err("missing '}' in string interpolation");
    }
    tokens.push(TokenType::Eof);
    //dbg!(&tokens);
    Ok(tokens)
}

fn start_raw_string(code: &str) -> bool {
    code.len() >= 3 && code[..3] == *"\"\"\""
}

fn end_raw_string(code: &str) -> bool {
    let only_three_quotes = if let Some(ch) = code.chars().nth(3) {
        ch != '"'
    } else {
        true
    };
    only_three_quotes && code.len() >= 3 && code[..3] == *"\"\"\""
}

fn is_word(code: &str, i: usize) -> bool {
    if let Some(ch) = code.chars().nth(i) {
        !(ch.is_ascii_alphanumeric() || ch == '_' || ch == '@')
    } else {
        true
    }
}

fn match_word(code: &str, word: &str) -> bool {
    let len = word.len();
    code.len() >= len && &code[..len] == word && is_word(code, len)
}

fn make_keyword(code: &str, line_number: u32) -> (TokenType, usize) {
    if match_word(code, "function") {
        (
            TokenType::Function(Token {
                lexeme: String::from("function"),
                line_number,
                precedence: precedence::NONE,
            }),
            8,
        )
    } else if match_word(code, "elseif") {
        (
            TokenType::ElseIf(Token {
                lexeme: String::from("elseif"),
                line_number,
                precedence: precedence::NONE,
            }),
            6,
        )
    } else if match_word(code, "while") {
        (
            TokenType::While(Token {
                lexeme: String::from("while"),
                line_number,
                precedence: precedence::NONE,
            }),
            5,
        )
    } else if match_word(code, "match") {
        (
            TokenType::Match(Token {
                lexeme: String::from("match"),
                line_number,
                precedence: precedence::NONE, // TODO: this is a guess, change to what makes sense
            }),
            5,
        )
    } else if match_word(code, "when") {
        (
            TokenType::When(Token {
                lexeme: String::from("when"),
                line_number,
                precedence: precedence::NONE,
            }),
            4,
        )
    } else if match_word(code, "exit") {
        (
            TokenType::Return(Token {
                lexeme: String::from("exit"),
                line_number,
                precedence: precedence::NONE,
            }),
            4,
        )
    } else if match_word(code, "fn") {
        (
            TokenType::Function(Token {
                lexeme: String::from("fn"),
                line_number,
                precedence: precedence::NONE,
            }),
            2,
        )
    } else if match_word(code, "then") {
        (
            TokenType::Then(Token {
                lexeme: String::from("then"),
                line_number,
                precedence: precedence::NONE,
            }),
            4,
        )
    } else if match_word(code, "false") {
        (
            TokenType::Bool(Token {
                lexeme: String::from("false"),
                line_number,
                precedence: precedence::NONE,
            }),
            5,
        )
    } else if match_word(code, "true") {
        (
            TokenType::Bool(Token {
                lexeme: String::from("true"),
                line_number,
                precedence: precedence::NONE,
            }),
            4,
        )
    } else if match_word(code, "else") {
        (
            TokenType::Else(Token {
                lexeme: String::from("else"),
                line_number,
                precedence: precedence::NONE,
            }),
            4,
        )
    } else if match_word(code, "next") {
        (
            TokenType::Next(Token {
                lexeme: String::from("next"),
                line_number,
                precedence: precedence::NONE,
            }),
            4,
        )
    } else if match_word(code, "step") {
        (
            TokenType::Step(Token {
                lexeme: String::from("step"),
                line_number,
                precedence: precedence::NONE,
            }),
            4,
        )
    } else if match_word(code, "for") {
        (
            TokenType::For(Token {
                lexeme: String::from("for"),
                line_number,
                precedence: precedence::NONE,
            }),
            3,
        )
    } else if match_word(code, "to") {
        (
            TokenType::To(Token {
                lexeme: String::from("to"),
                line_number,
                precedence: precedence::NONE,
            }),
            2,
        )
    } else if match_word(code, "and") {
        (
            TokenType::And(Token {
                lexeme: String::from("and"),
                line_number,
                precedence: precedence::AND,
            }),
            3,
        )
    } else if match_word(code, "mod") {
        (
            TokenType::Mod(Token {
                lexeme: String::from("mod"),
                line_number,
                precedence: precedence::FACTOR, // mayby up this
            }),
            3,
        )
    } else if match_word(code, "not") {
        (
            TokenType::Not(Token {
                lexeme: String::from("not"),
                line_number,
                precedence: precedence::NONE,
            }),
            3,
        )
    } else if match_word(code, "end") {
        (
            TokenType::End(Token {
                lexeme: String::from("end"),
                line_number,
                precedence: precedence::NONE,
            }),
            3,
        )
    } else if match_word(code, "if") {
        (
            TokenType::If(Token {
                lexeme: String::from("if"),
                line_number,
                precedence: precedence::NONE,
            }),
            2,
        )
    } else if match_word(code, "or") {
        (
            TokenType::Or(Token {
                lexeme: String::from("or"),
                line_number,
                precedence: precedence::OR,
            }),
            2,
        )
    } else if match_word(code, "in") {
        (
            TokenType::In(Token {
                lexeme: String::from("in"),
                line_number,
                precedence: precedence::COMPARISON,
            }),
            2,
        )
    } else if code.len() >= 2 && &code[..2] == "<>" {
        (
            TokenType::NotEquals(Token {
                lexeme: String::from("<>"),
                line_number,
                precedence: precedence::COMPARISON,
            }),
            2,
        )
    } else if code.len() >= 2 && &code[..2] == "<=" {
        (
            TokenType::LessThanOrEqual(Token {
                lexeme: String::from("<="),
                line_number,
                precedence: precedence::COMPARISON,
            }),
            2,
        )
    } else if code.len() >= 2 && &code[..2] == ">=" {
        (
            TokenType::GreaterThanOrEqual(Token {
                lexeme: String::from(">="),
                line_number,
                precedence: precedence::COMPARISON,
            }),
            2,
        )
    } else if code.len() >= 2 && &code[..2] == "==" {
        (
            TokenType::Equality(Token {
                lexeme: String::from("=="),
                line_number,
                precedence: precedence::EQUALITY,
            }),
            2,
        )
    } else {
        // Single character tokens

        let single_char = &code[..1];

        (
            match single_char {
                "<" => TokenType::LessThan(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::COMPARISON,
                }),
                ">" => TokenType::GreaterThan(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::COMPARISON,
                }),
                "=" => TokenType::Equals(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::NONE,
                }),
                "+" => TokenType::Plus(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::TERM,
                }),
                "-" => TokenType::Minus(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::TERM,
                }),
                "*" => TokenType::Times(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::FACTOR,
                }),
                "/" => TokenType::Divide(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::FACTOR,
                }),
                "^" => TokenType::Hat(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::FACTOR,
                }),
                "(" => TokenType::LeftParan(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::CALL,
                }),
                ")" => TokenType::RightParan(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::NONE,
                }),
                "[" => TokenType::LeftBracket(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::CALL,
                }),
                "]" => TokenType::RightBracket(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::NONE,
                }),
                "," => TokenType::Comma(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::NONE,
                }),
                "." => TokenType::Dot(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::CALL,
                }),
                ";" => TokenType::End(Token {
                    lexeme: single_char.to_string(),
                    line_number,
                    precedence: precedence::NONE,
                }),
                _ => TokenType::None,
            },
            1,
        )
    }
}
