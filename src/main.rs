mod compiler;
mod scanner;
mod vm;

use colored::Colorize;
use std::{env, fs, io, process};

use crate::{compiler::Compiler, vm::Vm};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", "Very Basic Version 0.0".yellow());
    dbg!(&args);

    if let Some(file_path) = args.get(1) {
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        interpret(&contents);
    } else {
        loop {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            interpret(&line);
        }
    }
}

fn interpret(contents: &str) -> String {
    let tokens = crate::scanner::tokenize(&contents);

    let mut instructions: Vec<compiler::OpCode> = Vec::new();
    let mut compiler = Compiler::new(&tokens, &mut instructions);
    compiler.compile();
    if compiler.in_error {
        return String::from("Compile Error");
    }

    dbg!(&instructions);
    let mut vm = Vm::new();
    vm.init();

    let result = vm.run(&instructions);
    if !result {
        std::process::exit(1);
    }

    if let Some(val) = vm.return_value {
        format!("{:?}", val)
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::TokenType;
    use crate::{compiler, interpret, vm};

    #[test]
    fn calling() {
        assert_eq!(interpret("print("), "Compile Error");
        assert_eq!(interpret("print(\"hello\")"), "String(\"hello\")");
    }

    #[test]
    fn arithmatic() {
        let contents = "-((1+1)*(1+1)) * (10-6) -20+1-8+9*10/5/9-2*7+1";

        let result = interpret(contents);

        assert_eq!(result, "Number(-54.0)");
    }

    #[test]
    fn comparisons() {
        assert_eq!(interpret("3 > (2-2)"), "Boolean(true)");
        assert_eq!(interpret("3 >= (2-2)"), "Boolean(true)");
        assert_eq!(interpret("3 < (2-2)"), "Boolean(false)");
        assert_eq!(interpret("3 <= (2-2)"), "Boolean(false)");
        assert_eq!(interpret("3 == (2-2)"), "Boolean(false)");
        assert_eq!(interpret("3 <> (2-2)"), "Boolean(true)");
    }

    #[test]
    fn not() {
        let result = interpret("not (1==1)");
        assert_eq!(result, "Boolean(false)");

        let result = interpret("not 0");
        assert_eq!(result, "Boolean(true)");

        let result = interpret("not \"\"");
        assert_eq!(result, "Boolean(true)");

        let result = interpret("not (\"hello\" + \"world\") ");
        assert_eq!(result, "Boolean(false)");

        let result = interpret("not (1==0)");
        assert_eq!(result, "Boolean(true)");

        let result = interpret("not 1");
        assert_eq!(result, "Boolean(false)");

        let result = interpret("not \"hi\"");
        assert_eq!(result, "Boolean(false)");
    }

    #[test]
    fn test_tokenize() {
        let code = "
            function test(a,b)
              x = 1
              if x == 1 then
                x = 1+2-3*4/5
              end
            end
            z=\"string\"
        ";
        let tokens = crate::scanner::tokenize(code);
        dbg!(tokens.len());
        assert_eq!(tokens.len(), 32);
        let t = &tokens[10]; // if
        dbg!(t);
        if let TokenType::If(token) = t {
            assert_eq!(token.line_number, 4);
        } else {
            assert!(false);
        }
    }
}
