//mod scanner;
use crate::scanner::{precedence, Token, TokenType};

#[derive(Debug)]
pub enum OpCode {
    ConstantNum(f64, u32),
    ConstantStr(String, u32),
    Add(u32),
    Subtract(u32),
    Negate(u32),
    Multiply(u32),
    Divide(u32),
    GreaterThan(u32),
    GreaterThanEq(u32),
    LessThan(u32),
    LessThanEq(u32),
    Equal(u32),
    NotEqual(u32),
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

    fn add_instr(&mut self, op: OpCode) {
        self.instructions.push(op);
    }

    fn number(&mut self, token: &Token) {
        let number = match token.lexeme.parse::<f64>() {
            Ok(v) => v,
            Err(e) => 0.0,
        };
        self.add_instr(OpCode::ConstantNum(number, token.line_number));
    }

    fn string(&mut self, token: &Token) {
        self.add_instr(OpCode::ConstantStr(token.lexeme.clone(), token.line_number));
    }

    fn grouping(&mut self, token: &Token) {
        self.expression();

        let end_token = &self.tokens[self.token_pointer];
        if let TokenType::RightParan(_) = end_token {
            self.advance();
        } else {
            dbg!(end_token);
            Compiler::compile_error("Expected )", token);
        }
    }

    // fn binary(&mut self, token: &Token) {
    //     self.parse_precedence(token.precedence + 1);
    //     self.add_instr(OpCode::Add(token.line_number));
    // }

    fn run_infix(&mut self, token: &TokenType) -> bool {
        match token {
            TokenType::Plus(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Add(t.line_number));
                true
            }
            TokenType::Minus(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Subtract(t.line_number));
                true
            }
            TokenType::Times(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Multiply(t.line_number));
                true
            }
            TokenType::Divide(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Divide(t.line_number));
                true
            }
            TokenType::GreaterThan(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::GreaterThan(t.line_number));
                true
            }
            TokenType::GreaterThanOrEqual(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::GreaterThanEq(t.line_number));
                true
            }
            TokenType::LessThan(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::LessThan(t.line_number));
                true
            }
            TokenType::LessThanOrEqual(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::LessThanEq(t.line_number));
                true
            }
            TokenType::Equality(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Equal(t.line_number));
                true
            }
            TokenType::NotEquals(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::NotEqual(t.line_number));
                true
            }
            _ => false,
        }
    }

    fn get_precedence(&mut self, token: &TokenType) -> u8 {
        match token {
            TokenType::Plus(t) => t.precedence,
            TokenType::Minus(t) => t.precedence,
            TokenType::Times(t) => t.precedence,
            TokenType::Divide(t) => t.precedence,
            TokenType::GreaterThan(t) => t.precedence,
            TokenType::LessThan(t) => t.precedence,
            TokenType::GreaterThanOrEqual(t) => t.precedence,
            TokenType::LessThanOrEqual(t) => t.precedence,
            TokenType::Equality(t) => t.precedence,
            TokenType::NotEquals(t) => t.precedence,

            _ => precedence::NONE,
        }
    }

    fn advance(&mut self) -> bool {
        self.token_pointer += 1;
        self.token_pointer < self.tokens.len()
    }

    fn compile_error(message: &str, token: &Token) {
        panic!("Compile error: {}, line {}", message, token.line_number);
    }

    fn parse_precedence(&mut self, precedence: u8) {
        self.advance();

        // run prefix rule
        let token = &self.tokens[self.token_pointer - 1];
        match token {
            TokenType::Number(t) => self.number(t),
            TokenType::String(t) => self.string(t),
            TokenType::Minus(t) => {
                self.parse_precedence(precedence::UNARY);
                self.add_instr(OpCode::Negate(t.line_number))
            }
            TokenType::LeftParan(t) => self.grouping(t),
            _ => {
                dbg!(token);
                panic!("Unexpected Statement:")
            }
        };

        // let xx = &self.tokens.get(self.token_pointer);

        let test = self.get_precedence(&self.tokens[self.token_pointer]);
        println!("prec: {precedence} < {test}");

        while self.token_pointer < self.tokens.len()
            && precedence <= self.get_precedence(&self.tokens[self.token_pointer])
        {
            if !self.advance() {
                break;
            };
            if !self.run_infix(&self.tokens[self.token_pointer - 1]) {
                break;
            }
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(precedence::ASSIGNMENT);
    }

    pub fn compile(&mut self) {
        while self.token_pointer < self.tokens.len() {
            let token = &self.tokens[self.token_pointer];
            match token {
                TokenType::Eof => break,
                _ => self.expression(),
            };
        }
    }
}
