mod compiler;
mod scanner;

use std::{env, fs, process};

use crate::compiler::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Very Basic Version 0.0");
    dbg!(&args);

    if let Some(file_path) = args.get(1) {
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let tokens = scanner::tokenize(&contents);
        let mut instructions: Vec<compiler::OpCode> = Vec::new();
        let mut compiler = Compiler::new(&tokens, &mut instructions);
        compiler.compile();
        //dbg!(tokens);
    } else {
        println!("No filename passed as argument");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler;
    use crate::scanner::TokenType;

    #[test]
    fn it_works() {
        let contents = "+2+3+4+5";

        let tokens = crate::scanner::tokenize(&contents);

        let mut instructions: Vec<compiler::OpCode> = Vec::new();
        let mut compiler = compiler::Compiler::new(&tokens, &mut instructions);
        compiler.compile();

        dbg!(&instructions);
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
        assert_eq!(tokens.len(), 31);
        let t = &tokens[10]; // if
        dbg!(t);
        if let TokenType::If(token) = t {
            assert_eq!(token.line_number, 4);
        } else {
            assert!(false);
        }
    }
}
