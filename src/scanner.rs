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
    pub const PRIMARY: u8 = 10;
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
    Comma(Token),
    Identifier(Token),
    String(Token),
    Number(Token),
    Error,
    Eof,
}

pub fn tokenize(code: &str) -> Vec<TokenType> {
    let mut i = 0;
    let mut line_number = 1;
    let mut tokens: Vec<TokenType> = Vec::new();

    while i < code.len() {
        let mut current_char = code.chars().nth(i).unwrap();
        if current_char == '\n' {
            line_number += 1;
        }

        let (token, len) = make_keyword(&code[i..], line_number);
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
                    if current_char != '"' {
                        lexeme.push(current_char);
                    } else {
                        break;
                    }
                }
                tokens.push(TokenType::String(Token {
                    lexeme,
                    line_number,
                    precedence: precedence::NONE,
                }));
                i += 1;
            } else if current_char.is_ascii_alphabetic() {
                // Identifier
                let mut lexeme = String::new();
                while i < code.len() && (current_char.is_ascii_alphabetic()) {
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
    tokens.push(TokenType::Eof);
    tokens
}

fn make_keyword(code: &str, line_number: u32) -> (TokenType, usize) {
    if code.len() >= 8 && &code[..8] == "function" {
        (
            TokenType::Function(Token {
                lexeme: String::from("function"),
                line_number,
                precedence: precedence::NONE,
            }),
            8,
        )
    } else if code.len() >= 6 && &code[..6] == "return" {
        (
            TokenType::Return(Token {
                lexeme: String::from("return"),
                line_number,
                precedence: precedence::NONE,
            }),
            6,
        )
    } else if code.len() >= 5 && &code[..5] == "while" {
        (
            TokenType::While(Token {
                lexeme: String::from("while"),
                line_number,
                precedence: precedence::NONE,
            }),
            5,
        )
    } else if code.len() >= 4 && &code[..4] == "then" {
        (
            TokenType::Then(Token {
                lexeme: String::from("then"),
                line_number,
                precedence: precedence::NONE,
            }),
            4,
        )
    } else if code.len() >= 4 && &code[..4] == "then" {
        (
            TokenType::Else(Token {
                lexeme: String::from("else"),
                line_number,
                precedence: precedence::NONE,
            }),
            4,
        )
    } else if code.len() >= 3 && &code[..3] == "and" {
        (
            TokenType::And(Token {
                lexeme: String::from("and"),
                line_number,
                precedence: precedence::AND,
            }),
            3,
        )
    } else if code.len() >= 3 && &code[..3] == "not" {
        (
            TokenType::Not(Token {
                lexeme: String::from("not"),
                line_number,
                precedence: precedence::NONE,
            }),
            3,
        )
    } else if code.len() >= 3 && &code[..3] == "end" {
        (
            TokenType::End(Token {
                lexeme: String::from("end"),
                line_number,
                precedence: precedence::NONE,
            }),
            3,
        )
    } else if code.len() >= 2 && &code[..2] == "if" {
        (
            TokenType::If(Token {
                lexeme: String::from("if"),
                line_number,
                precedence: precedence::NONE,
            }),
            2,
        )
    } else if code.len() >= 2 && &code[..2] == "or" {
        (
            TokenType::Or(Token {
                lexeme: String::from("or"),
                line_number,
                precedence: precedence::OR,
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
                "," => TokenType::Comma(Token {
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
