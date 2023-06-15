mod compiler;
mod scanner;
mod vm;

use colored::Colorize;
use std::{env, fs, io, process};

use crate::{compiler::Compiler, vm::Vm};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", "Very Basic Version 0.0".yellow());
    //dbg!(&args);

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

    //dbg!(&instructions);
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
    use crate::interpret;
    use crate::scanner::TokenType;

    #[test]
    fn calling() {
        assert_eq!(interpret("print("), "Compile Error");
        assert_eq!(interpret("print(\"hello\")"), "String(\"hello\")");
    }

    #[test]
    fn and() {
        //assert_eq!(interpret("1==1 and 2==2"), "Boolean(true)");
        //assert_eq!(interpret("1==1 and 1==2"), "Boolean(false)");
        assert_eq!(interpret("x2=1:y2=1:iteration=1:max_iteration=1 x2 + y2 <= 4 and iteration < max_iteration"), "Boolean(false)");
        //
    }

    #[test]
    fn multiple_fns() {
        let code = "function setgraphics(x)
                        0
                    end
                    
                    function plot(x,y,c)
                        0
                    end";
        assert_eq!(interpret(code), "");
    }

    #[test]
    fn shadowing() {
        assert_eq!(
            interpret("x = 5.5; function test(x) x * 2 end; test(20); "),
            "Number(40.0)"
        );
        assert_eq!(
            interpret("x = 5.5; function test(y) y * 2 end; test(20); "),
            "Number(40.0)"
        );
    }

    #[test]
    fn locals() {
        assert_eq!(
            interpret(
                "x = 5.5; y = 6.6; function test(x,y,z) a=1 b=2 a+b+x+y+z end; test(3,4,5); "
            ),
            "Number(15.0)"
        );
    }

    #[test]
    fn define_function() {
        assert_eq!(
            interpret("function test() print(45) end; test(); print(66);"),
            "String(\"66\")"
        );
        assert_eq!(interpret("function test(a,b,c) print(45) end"), "");

        assert_eq!(
            interpret("function test() 45 end; test(); "),
            "Number(45.0)"
        );

        assert_eq!(
            interpret("function test(x) x * 2 end; test(20); "),
            "Number(40.0)"
        );
    }

    #[test]
    fn ifthenelse() {
        assert_eq!(interpret("if 1==1 then 666 end"), "Number(666.0)");
        assert_eq!(interpret("if 1==1 then 666 else 555 end"), "Number(666.0)");
        assert_eq!(interpret("if 1==2 then 666 else 555 end"), "Number(555.0)");
        assert_eq!(
            interpret("if 1==1 then x=1; x+5 else x=6; x+5 end"),
            "Number(6.0)"
        );
        assert_eq!(
            interpret("if 1==2 then x=1; x+5 else x=6; x+5 end"),
            "Number(11.0)"
        );
    }

    #[test]
    fn while_loop() {
        assert_eq!(interpret("x = 0; while x < 10 x=x+1 end x"), "Number(10.0)");
    }

    #[test]
    fn variables() {
        assert_eq!(interpret("x = 1000"), "Number(1000.0)");
        assert_eq!(interpret("print(x)"), "Compile Error");
        assert_eq!(interpret("x = 1000; x / 100"), "Number(10.0)");
    }

    #[test]
    fn arithmatic() {
        let contents = "-((1+1)*(1+1)) * (10.0-6) -20+1-8+9*10/5/9-2*7+1";

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
        //dbg!(tokens.len());
        assert_eq!(tokens.len(), 32);
        let t = &tokens[10]; // if
                             //dbg!(t);
        if let TokenType::If(token) = t {
            assert_eq!(token.line_number, 4);
        } else {
            assert!(false);
        }
    }
}
