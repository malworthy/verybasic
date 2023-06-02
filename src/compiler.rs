//mod scanner;
use crate::scanner::{precedence, Token, TokenType};

#[derive(Debug)]
pub enum OpCode {
    ConstantNum(f64),
    ConstantStr(String),
    Add,
}

pub struct Compiler<'a> {
    instructions: &'a mut Vec<OpCode>,
    tokens: &'a Vec<TokenType>,
    token_pointer: usize,
}

impl Compiler<'_> {
    pub fn new<'a>(tokens: &'a Vec<TokenType>, instructions: &'a mut Vec<OpCode>) -> Compiler<'a> {
        Compiler {
            instructions,
            tokens,
            token_pointer: 0,
        }
    }

    fn number(&mut self, token: &Token) -> u8 {
        // let mut number: f64;
        // let x = token.lexeme.parse::<f64>();

        let number = match token.lexeme.parse::<f64>() {
            Ok(v) => v,
            Err(e) => 0.0,
        };
        self.instructions.push(OpCode::ConstantNum(number));
        token.precedence
    }

    fn binary(&mut self, token: &Token) {
        self.parse_precedence(token.precedence + 1);
        self.instructions.push(OpCode::Add);
    }

    fn run_infix(&mut self, token: &TokenType) -> bool {
        match token {
            TokenType::Plus(t) => {
                self.binary(&t);
                true
            }
            _ => false,
        }
    }

    fn get_precedence(&mut self, token: &TokenType) -> u8 {
        match token {
            TokenType::Plus(t) => t.precedence,
            _ => precedence::NONE,
        }
    }

    fn advance(&mut self) -> bool {
        self.token_pointer += 1;
        self.token_pointer < self.tokens.len()
    }

    fn parse_precedence(&mut self, precedence: u8) {
        self.advance();

        // run prefix rule
        let token = &self.tokens[self.token_pointer - 1];
        match token {
            TokenType::Number(t) => self.number(t),
            _ => _ = panic!("Unexpected Statement"),
        };

        while precedence < self.get_precedence(&self.tokens[self.token_pointer]) {
            if !self.advance() {
                break;
            };
            self.run_infix(&self.tokens[self.token_pointer - 1]);
        }
    }

    pub fn compile(&mut self) {
        while self.token_pointer < self.tokens.len() {
            let token = &self.tokens[self.token_pointer];
            //dbg!(&token);
            match token {
                TokenType::Number(t) => self.parse_precedence(t.precedence),
                TokenType::Plus(t) => self.parse_precedence(t.precedence),
                _ => _ = self.advance(),
            };
        }
    }
}
