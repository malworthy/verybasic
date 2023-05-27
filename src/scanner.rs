#[derive(Debug)]
pub struct Token {
    lexeme: String,
    line_number: u32,
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
    Keyword(Token),
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
        if let Some(char) = code.chars().nth(i) {
            if char == '\n' {
                line_number += 1;
            }
        }
        let (token, len) = make_keyword(&code[i..], line_number);
        if let TokenType::None = token {
        } else {
            tokens.push(token);
        }

        i += len;
    }
    tokens
}

fn make_keyword(code: &str, line_number: u32) -> (TokenType, usize) {
    if code.len() >= 8 && &code[..8] == "function" {
        (
            TokenType::Function(Token {
                lexeme: String::from("function"),
                line_number,
            }),
            8,
        )
    } else if code.len() >= 6 && &code[..6] == "return" {
        (
            TokenType::Return(Token {
                lexeme: String::from("return"),
                line_number,
            }),
            6,
        )
    } else if code.len() >= 5 && &code[..5] == "while" {
        (
            TokenType::Return(Token {
                lexeme: String::from("while"),
                line_number,
            }),
            5,
        )
    } else {
        (TokenType::None, 1)
    }
}
