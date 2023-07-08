use crate::Vm;
use colored::Colorize;

//mod scanner;
use crate::scanner::{precedence, Token, TokenType};

#[derive(Debug)]
pub enum OpCode {
    ConstantNum(f64),
    ConstantStr(String),
    ConstantBool(bool),
    Add,
    Subtract,
    Negate,
    Multiply,
    Divide,
    GreaterThan,
    GreaterThanEq,
    LessThan,
    LessThanEq,
    Equal,
    NotEqual,
    Not,
    And,
    Or,
    Mod,
    Pow,
    SetGlobal(String),
    GetGlobal(String),
    Call(usize, u32),
    CallSystem(String, u32, u32),
    CallNative(usize, u32),
    CallNativeGr(usize, u32),
    Pop,
    SetLocal(usize),
    DefineLocal(usize),
    GetLocal(usize),
    JumpIfFalse(usize),
    Jump(i32),
    Subscript,
    Return,
}

pub fn print_instr(instructions: Vec<OpCode>) {
    let mut addr = 0;
    for i in instructions {
        let x = match i {
            OpCode::Add => format!("{} ADD", addr),
            OpCode::Call(ptr, argc) => format!("{} CALL {} {}", addr, ptr, argc),
            OpCode::CallNative(index, argc) => format!("{} CALN {} {}", addr, index, argc),
            OpCode::CallNativeGr(index, argc) => format!("{} CALG {} {}", addr, index, argc),
            OpCode::And => format!("{} AND", addr),
            OpCode::CallSystem(name, argc, _) => format!("{} SYS  {} {}", addr, name, argc),
            OpCode::ConstantNum(num) => format!("{} NUM  {}", addr, num),
            OpCode::ConstantStr(str) => format!("{} STR  {}", addr, str),
            OpCode::DefineLocal(num) => format!("{} DEF {}", addr, num),
            OpCode::Divide => format!("{} DIV", addr),
            OpCode::Equal => format!("{} EQ", addr),
            OpCode::GetGlobal(name) => format!("{} GLOB {}", addr, name),
            OpCode::GetLocal(index) => format!("{} LOC {}", addr, index),
            OpCode::GreaterThan => format!("{} GT", addr),
            OpCode::GreaterThanEq => format!("{} GTEQ", addr),
            OpCode::Jump(ptr) => format!("{} JUMP {}", addr, ptr),
            OpCode::JumpIfFalse(ptr) => format!("{} JUMF {}", addr, ptr),
            OpCode::LessThan => format!("{} LT", addr),
            OpCode::LessThanEq => format!("{} LTEQ", addr),
            OpCode::Multiply => format!("{} MUL", addr),
            OpCode::Mod => format!("{} MOD", addr),
            OpCode::Pow => format!("{} POW", addr),
            OpCode::Negate => format!("{} NEG", addr),
            OpCode::Not => format!("{} NOT", addr),
            OpCode::NotEqual => format!("{} NEQ", addr),
            OpCode::Or => format!("{} OR", addr),
            OpCode::Pop => format!("{} POP", addr),
            OpCode::Return => format!("{} RET", addr),
            OpCode::SetGlobal(name) => format!("{} SETG {}", addr, name),
            OpCode::SetLocal(index) => format!("{} SET {}", addr, index),
            OpCode::Subscript => format!("{} SBPT", addr),
            OpCode::Subtract => format!("{} SUB", addr),
            OpCode::ConstantBool(val) => format!("{} BOOL {}", addr, val),
        };
        addr += 1;
        println!("{}", x);
    }
}

pub struct Variable {
    depth: u8,
    name: String,
}

impl Variable {
    pub fn new(name: String, depth: u8) -> Self {
        Variable { depth, name }
    }
}

pub struct Compiler<'a> {
    instructions: &'a mut Vec<OpCode>,
    line_numbers: &'a mut Vec<u32>,
    tokens: &'a Vec<TokenType>,
    variables: Vec<Variable>,
    pub functions: Vec<(String, u8, usize)>,
    token_pointer: usize,
    pub in_error: bool,
    depth: u8,
}

fn is_native(name: &str) -> Result<usize, usize> {
    let mut i = 0;
    for s in Vm::NATIVES {
        if s.1 == name {
            return Ok(i);
        }
        i += 1;
    }
    Err(1)
}

fn is_native_graphics(name: &str) -> Result<usize, usize> {
    if let Some(i) = Vm::NATIVES_GR.into_iter().position(|x| x.1 == name) {
        return Ok(i);
    }
    Err(1)
}

impl Compiler<'_> {
    pub fn new<'a>(
        tokens: &'a Vec<TokenType>,
        instructions: &'a mut Vec<OpCode>,
        line_numbers: &'a mut Vec<u32>,
    ) -> Compiler<'a> {
        Compiler {
            instructions,
            line_numbers,
            tokens,
            variables: Vec::new(),
            token_pointer: 0,
            in_error: false,
            depth: 0,
            functions: Vec::new(),
        }
    }

    fn add_instr(&mut self, op: OpCode, line_number: u32) -> usize {
        self.line_numbers.push(line_number);
        self.instructions.push(op);
        self.instructions.len() - 1
    }

    fn add_fn(&mut self, name: String, arity: u8, fn_start: usize) -> bool {
        if self.functions.iter().any(|x| *x.0 == name) {
            return false;
        };
        self.functions.push((name, arity, fn_start));
        true
    }

    fn number(&mut self, token: &Token) {
        let number = match token.lexeme.parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                self.compile_error("Could not parse number", token);
                return;
            }
        };
        self.add_instr(OpCode::ConstantNum(number), token.line_number);
    }

    fn string(&mut self, token: &Token) {
        self.add_instr(OpCode::ConstantStr(token.lexeme.clone()), token.line_number);
    }

    fn bool(&mut self, token: &Token) {
        self.add_instr(
            OpCode::ConstantBool(token.lexeme == "true"),
            token.line_number,
        );
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

    fn subscript(&mut self, token: &Token) -> bool {
        // get the index of the array
        self.expression();
        if let TokenType::RightBracket(_) = &self.tokens[self.token_pointer] {
            self.add_instr(OpCode::Subscript, token.line_number);
            self.advance();
        } else {
            self.compile_error("Missing ]", token);
            return false;
        }
        true
    }

    fn call(&mut self, token: &Token) -> bool {
        if self.token_pointer >= self.tokens.len() {
            self.compile_error("Syntax error", token);
            return false;
        }
        // get the name of the function
        let name: String;
        if let TokenType::Identifier(t) = &self.tokens[self.token_pointer - 2] {
            name = t.lexeme.clone();
        } else {
            self.compile_error("Syntax Error", token);
            return false;
        }

        let mut arguments = 0;
        loop {
            match &self.tokens[self.token_pointer] {
                TokenType::RightParan(_) => {
                    self.advance();
                    // check if it's native
                    if let Ok(index) = is_native(name.as_str()) {
                        self.add_instr(OpCode::CallNative(index, arguments), token.line_number);
                    } else if let Ok(index) = is_native_graphics(name.as_str()) {
                        self.add_instr(OpCode::CallNativeGr(index, arguments), token.line_number);
                    } else {
                        // get index of fn
                        if let Some(index) = self.functions.iter().position(|x| x.0 == name) {
                            let f = &self.functions[index];
                            if f.1 != arguments as u8 {
                                self.compile_error(
                                    "Wrong number of arguments pass to function",
                                    token,
                                );
                                return false;
                            }
                            self.add_instr(OpCode::Call(f.2, arguments), token.line_number);
                        } else {
                            // can't find anything so try a system call
                            self.add_instr(
                                OpCode::CallSystem(name, arguments, token.line_number),
                                token.line_number,
                            );
                        }
                    }

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
    }

    fn add_variable(&mut self, name: String) -> (Option<usize>, bool) {
        let index: Option<usize>;
        let mut added = false;
        if let Some(i) = self.variables.iter().position(|x| x.name == name) {
            if self.variables[i].depth > 0 {
                let start = self
                    .variables
                    .iter()
                    .position(|x| x.depth == self.depth)
                    .unwrap();
                index = Some(i - start);
            } else {
                index = None; // global, so we don't use index.
            }
        } else {
            self.variables.push(Variable::new(name, self.depth));
            let start = self
                .variables
                .iter()
                .position(|x| x.depth == self.depth)
                .unwrap();
            if self.depth == 0 {
                index = None
            } else {
                index = Some(self.variables.len() - start - 1);
            }

            added = true;
        }
        (index, added)
    }

    fn add_fn_param(&mut self, name: String) -> bool {
        // let index: usize;
        let mut added = false;
        if let Some(_) = self
            .variables
            .iter()
            .position(|x| x.name == name && x.depth == self.depth)
        {
            let message = format!("Duplicate parameters in function {name}");
            self.compile_error_message(&message);
        } else {
            // let start = if let Some(i) = self.variables.iter().position(|x| x.depth == self.depth) {
            //     i
            // } else {
            //     self.variables.len()
            // };
            self.variables.push(Variable::new(name, self.depth));
            // index = self.variables.len() - start - 1;
            added = true;
        }
        added
    }

    fn variable(&mut self, token: &Token, can_assign: bool) {
        //check to see if this is a function call
        if let TokenType::LeftParan(_) = self.tokens[self.token_pointer] {
            //self.advance();
            return;
        };

        //check to see if we are setting a variable
        let matched_equal = if let TokenType::Equals(_) = self.tokens[self.token_pointer] {
            self.advance();
            true
        } else {
            false
        };

        if can_assign && matched_equal {
            // Setting a variable
            self.expression();
            let (index, added) = self.add_variable(token.lexeme.clone());
            match index {
                Some(index) => {
                    if added {
                        self.add_instr(OpCode::DefineLocal(index), token.line_number);
                    } else {
                        self.add_instr(OpCode::SetLocal(index), token.line_number);
                    }
                }
                None => {
                    self.add_instr(OpCode::SetGlobal(token.lexeme.clone()), token.line_number);
                }
            }
        } else {
            // Getting value from a variable
            if let Some(index) = self
                .variables
                .iter()
                .rev()
                .position(|x| x.name == token.lexeme)
            {
                let index = self.variables.len() - 1 - index;
                if self.variables[index].depth == 0 {
                    self.add_instr(OpCode::GetGlobal(token.lexeme.clone()), token.line_number);
                } else {
                    let start = self
                        .variables
                        .iter()
                        .position(|x| x.depth == self.depth)
                        .unwrap();
                    let index = index - start;
                    self.add_instr(OpCode::GetLocal(index), token.line_number);
                }
            } else {
                // compile error - can't find variable
                let message = format!("Variable {} not found", token.lexeme);
                self.compile_error(&message, token);
            }
        }
    }
    // 0 1 2 3 4 5
    // 0 0 0 1 1 1
    // x y z x y z
    // x = 1
    fn run_infix(&mut self, token: &TokenType) -> bool {
        match token {
            TokenType::Plus(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Add, t.line_number);
                true
            }
            TokenType::Minus(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Subtract, t.line_number);
                true
            }
            TokenType::Times(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Multiply, t.line_number);
                true
            }
            TokenType::Divide(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Divide, t.line_number);
                true
            }
            TokenType::Hat(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Pow, t.line_number);
                true
            }
            TokenType::Mod(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Mod, t.line_number);
                true
            }
            TokenType::GreaterThan(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::GreaterThan, t.line_number);
                true
            }
            TokenType::GreaterThanOrEqual(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::GreaterThanEq, t.line_number);
                true
            }
            TokenType::LessThan(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::LessThan, t.line_number);
                true
            }
            TokenType::LessThanOrEqual(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::LessThanEq, t.line_number);
                true
            }
            TokenType::Equality(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Equal, t.line_number);
                true
            }
            TokenType::NotEquals(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::NotEqual, t.line_number);
                true
            }
            TokenType::And(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::And, t.line_number);
                true
            }
            TokenType::Or(t) => {
                self.parse_precedence(t.precedence + 1);
                self.add_instr(OpCode::Or, t.line_number);
                true
            }
            TokenType::LeftParan(t) => self.call(t),
            TokenType::LeftBracket(t) => self.subscript(t),
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
            TokenType::LeftParan(t)
            | TokenType::And(t)
            | TokenType::Or(t)
            | TokenType::Hat(t)
            | TokenType::Mod(t)
            | TokenType::LeftBracket(t) => t.precedence,

            _ => precedence::NONE,
        }
    }

    fn advance(&mut self) -> bool {
        self.token_pointer += 1;
        self.token_pointer < self.tokens.len()
    }

    fn compile_error(&mut self, message: &str, token: &Token) {
        eprintln!(
            "Compile error: {}, line {}",
            message.red(),
            token.line_number
        );
        self.in_error = true;
    }

    fn compile_error_line(&mut self, message: &str, line_number: u32) {
        eprintln!("Compile error: {}, line {}", message.red(), line_number);
        self.in_error = true;
    }

    fn compile_error_message(&mut self, message: &str) {
        eprintln!("Compile error: {}", message.red());
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
            TokenType::Bool(t) => self.bool(t),
            TokenType::Minus(t) => {
                self.parse_precedence(precedence::UNARY);
                self.add_instr(OpCode::Negate, t.line_number);
            }
            TokenType::Not(t) => {
                self.parse_precedence(precedence::UNARY);
                self.add_instr(OpCode::Not, t.line_number);
            }
            TokenType::LeftParan(t) => self.grouping(t),
            TokenType::Identifier(t) => self.variable(t, can_assign),
            _ => {
                let result = token.get_token();
                if let Some(t) = result {
                    let message = format!("Unexpected statement '{}'", t.lexeme);
                    self.compile_error(message.as_str(), t);
                } else {
                    panic!("Unexpected Token Type");
                }
            }
        };

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
        self.add_instr(OpCode::Pop, 0);
    }

    fn if_statement(&mut self, if_token: &Token) {
        self.advance();
        self.expression();
        let token = &self.tokens[self.token_pointer];
        if let TokenType::Then(t) = token {
            self.advance();
            let if_index = self.add_instr(OpCode::JumpIfFalse(0), if_token.line_number);
            self.block();
            let else_index = self.add_instr(OpCode::Jump(0), if_token.line_number);
            self.instructions[if_index] =
                OpCode::JumpIfFalse(self.instructions.len() - if_index - 1);
            if let TokenType::Else(_) = self.tokens[self.token_pointer] {
                self.advance();
                self.block();
                self.instructions[else_index] = OpCode::Jump(
                    (self.instructions.len() - else_index - 1)
                        .try_into()
                        .unwrap(),
                );
            }
            if let TokenType::End(_) = self.tokens[self.token_pointer] {
                self.advance();
            } else {
                self.compile_error("If without end", t);
            }
        } else {
            self.compile_error("If without then", if_token);
        }
    }

    fn while_statement(&mut self, while_token: &Token) {
        let loop_start: i32 = (self.instructions.len() - 1).try_into().unwrap();
        if !self.advance() {
            return;
        }
        self.expression();
        let jump_index = self.add_instr(OpCode::JumpIfFalse(0), while_token.line_number);
        if !self.block() {
            return;
        }
        let len: i32 = self.instructions.len().try_into().unwrap();
        self.add_instr(
            OpCode::Jump((loop_start - len).try_into().unwrap()),
            while_token.line_number,
        );
        self.instructions[jump_index] =
            OpCode::JumpIfFalse(self.instructions.len() - jump_index - 1);
        let token = &self.tokens[self.token_pointer];
        if let TokenType::End(_) = token {
            self.advance();
        } else {
            self.compile_error("while without end", while_token)
        }
    }

    fn def_fn(&mut self, fn_token: &Token) {
        if self.depth > 0 {
            self.advance();
            self.compile_error("Can't define a function within a function", fn_token);
            return;
        }
        let jump_index = self.add_instr(OpCode::Jump(0), 0);
        let fn_start = self.instructions.len();
        self.depth += 1;
        self.advance();

        // get function name
        let name: String;
        if let TokenType::Identifier(t) = &self.tokens[self.token_pointer] {
            name = t.lexeme.clone();
            self.advance();
        } else {
            self.compile_error("missing function name", fn_token);
            return;
        }
        // consume (
        if let TokenType::LeftParan(_) = &self.tokens[self.token_pointer] {
            self.advance();
        } else {
            self.compile_error("missing '(' after function name", fn_token);
            return;
        }

        // define params and local variables
        let mut arity: u8 = 0;
        loop {
            match &self.tokens[self.token_pointer] {
                TokenType::RightParan(_) => {
                    self.advance();
                    break;
                }
                TokenType::Comma(_) => {
                    if !self.advance() {
                        self.compile_error("Expected )", fn_token);
                        return;
                    }
                }
                TokenType::Eof => {
                    self.compile_error("Expected )", fn_token);
                    return;
                }
                TokenType::Identifier(param) => {
                    if !self.add_fn_param(param.lexeme.clone()) {
                        return;
                    }
                    arity += 1;
                    self.advance();
                }
                _ => {
                    self.compile_error("Function parameter expected", fn_token);
                    return;
                }
            }
        }
        // add the function here before compiling the body - that way we support recursion
        if !self.add_fn(name, arity, fn_start) {
            self.compile_error("Attempt to define the same function twice", fn_token);
            return;
        }

        // The function body
        let body_check = self.instructions.len();
        self.block();
        if body_check == self.instructions.len() {
            self.compile_error("all functions must have a body", fn_token);
            return;
        }

        // Check for end
        if let TokenType::End(_) = &self.tokens[self.token_pointer] {
            self.advance();
        } else {
            self.compile_error("Function without end", fn_token);
            return;
        }

        // add return in case there isn't one
        self.add_instr(OpCode::Return, 0);

        // remove the local variables as we are done with them
        if let Some(index) = self.variables.iter().position(|x| x.depth == self.depth) {
            self.variables.truncate(index);
        }

        self.depth -= 1;

        let to_jump: i32 = (self.instructions.len() - fn_start).try_into().unwrap();
        //self.add_instr(OpCode::DefFn(name.clone(), fn_start));
        // if !self.add_fn(name, arity, fn_start) {
        //     self.compile_error("Attempt to define the same function twice", fn_token);
        //     return;
        // }

        // patch jump so we jump over the function if not calling it

        self.instructions[jump_index] = OpCode::Jump(to_jump);
    }

    fn block(&mut self) -> bool {
        loop {
            if let Some(token) = self.tokens.get(self.token_pointer) {
                match token {
                    TokenType::Else(_) | TokenType::End(_) | TokenType::Eof => break,
                    _ => {
                        self.statement();
                    }
                }
            } else {
                self.compile_error_message("Unexpected Error - Possibly missing 'end'");
                return false;
            }
        }
        true
    }

    fn statement(&mut self) {
        let token = &self.tokens[self.token_pointer];
        match token {
            TokenType::If(t) => self.if_statement(t),
            TokenType::While(t) => self.while_statement(t),
            TokenType::Function(t) => self.def_fn(t),
            TokenType::Return(t) => {
                self.add_instr(OpCode::Return, t.line_number);
                self.advance();
            }
            _ => self.expression_statement(),
        }
    }

    pub fn compile(&mut self) {
        while self.token_pointer < self.tokens.len() {
            let token = &self.tokens[self.token_pointer];
            match token {
                TokenType::Eof => break,
                _ => self.statement(),
            };
            if self.in_error {
                break;
            }
        }

        self.second_pass();
    }

    pub fn second_pass(&mut self) {
        let mut index: usize = 0;
        while index < self.instructions.len() {
            let inst = self.instructions.get(index).unwrap();
            if let OpCode::CallSystem(name, arguments, line_number) = inst {
                if let Some(fi) = self.functions.iter().position(|x| x.0 == *name) {
                    let f = &self.functions[fi];
                    if f.1 != *arguments as u8 {
                        self.compile_error_line(
                            "Wrong number of arguments pass to function",
                            *line_number,
                        );
                        return;
                    }

                    self.instructions[index] = OpCode::Call(f.2, *arguments);
                }
            }
            index += 1;
        }
    }
}
