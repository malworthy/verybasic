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
    Not(u32),
    SetGlobal(String, u32),
    GetGlobal(String, u32),
    Call(String, u32, u32),
    Pop,
}

pub struct Variable {
    depth: u32,
}

pub struct Compiler<'a> {
    instructions: &'a mut Vec<OpCode>,
    tokens: &'a Vec<TokenType>,
    token_pointer: usize,
    pub in_error: bool,
}

impl Compiler<'_> {
    pub fn new<'a>(tokens: &'a Vec<TokenType>, instructions: &'a mut Vec<OpCode>) -> Compiler<'a> {
        Compiler {
            instructions,
            tokens,
            token_pointer: 0,
            in_error: false,
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
            self.compile_error("Expected )", token);
        }
    }

    fn call(&mut self, token: &Token) -> bool {
        if self.token_pointer >= self.tokens.len() {
            self.compile_error("Syntax error", token);
            return false;
        }
        let name: String;
        if let TokenType::Identifier(t) = &self.tokens[self.token_pointer - 2] {
            name = t.lexeme.clone();
        } else {
            self.compile_error("Syntax Error", token);
            return false;
        }
        dbg!(&self.instructions);
        println!("parsing call() {name}");
        dbg!(token);
        let mut arguments = 0;
        loop {
            match &self.tokens[self.token_pointer] {
                TokenType::RightParan(_) => {
                    self.advance();
                    self.add_instr(OpCode::Call(name.clone(), arguments, token.line_number));
                    return true;
                }
                TokenType::Comma(_) => {
                    if !self.advance() {
                        self.compile_error("Expected )", token);
                        return false;
                    }
                }
                TokenType::Eof => {
                    self.compile_error("Expected )", token);
                    return false;
                }
                _ => {
                    self.expression();
                    arguments += 1;
                }
            }
        }
        true
    }

    fn variable(&mut self, token: &Token, can_assign: bool) {
        //check to see if this is a function call
        if let TokenType::LeftParan(_) = self.tokens[self.token_pointer] {
            //self.advance();
            return;
        };

        //match_
        let matched_equal = if let TokenType::Equals(_) = self.tokens[self.token_pointer] {
            self.advance();
            true
        } else {
            false
        };
        if can_assign && matched_equal {
            self.expression();
            self.add_instr(OpCode::SetGlobal(token.lexeme.clone(), token.line_number));
        } else {
            self.add_instr(OpCode::GetGlobal(token.lexeme.clone(), token.line_number));
        }
    }

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
            TokenType::LeftParan(t) => self.call(t),
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
            TokenType::LeftParan(t) => t.precedence,

            _ => precedence::NONE,
        }
    }

    fn advance(&mut self) -> bool {
        self.token_pointer += 1;
        self.token_pointer < self.tokens.len()
    }

    fn compile_error(&mut self, message: &str, token: &Token) {
        eprintln!("Compile error: {}, line {}", message, token.line_number);
        self.in_error = true;
    }

    fn parse_precedence(&mut self, precedence: u8) {
        if !self.advance() {
            return;
        }

        // run prefix rule
        let token = &self.tokens[self.token_pointer - 1];
        let can_assign = precedence <= precedence::ASSIGNMENT;
        match token {
            TokenType::Number(t) => self.number(t),
            TokenType::String(t) => self.string(t),
            TokenType::Minus(t) => {
                self.parse_precedence(precedence::UNARY);
                self.add_instr(OpCode::Negate(t.line_number))
            }
            TokenType::Not(t) => {
                self.parse_precedence(precedence::UNARY);
                self.add_instr(OpCode::Not(t.line_number))
            }
            TokenType::LeftParan(t) => self.grouping(t),
            TokenType::Identifier(t) => self.variable(t, can_assign),
            _ => {
                dbg!(token);
                eprintln!("Unexpected Statement: {:?}", token);
                self.in_error = true;
            }
        };

        // let xx = &self.tokens.get(self.token_pointer);

        //let test = self.get_precedence(&self.tokens[self.token_pointer]);
        //println!("prec: {precedence} < {test}");

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

    fn expression_statement(&mut self) {
        self.expression();
        self.add_instr(OpCode::Pop);
    }

    pub fn compile(&mut self) {
        while self.token_pointer < self.tokens.len() {
            let token = &self.tokens[self.token_pointer];
            match token {
                TokenType::Eof => break,
                _ => self.expression_statement(),
            };
        }
    }
}
