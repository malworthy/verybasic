mod common;
mod compiler;
mod scanner;
mod vm;
use colored::Colorize;
use std::{fs, io, path::PathBuf, process};
use vm::DebugSettings;

use crate::{compiler::Compiler, vm::Vm};
use clap::Parser;

/// Very Basic - A Basic interpreted programming language
#[derive(Debug, Parser)]
struct Cli {
    path: Option<std::path::PathBuf>,

    /// Output the compiled bytecode to console
    #[arg(short, long)]
    compile: bool,

    /// Set breakpoints to debug code, lines numbers separated by commas
    #[arg(short, long)]
    breakpoints: Option<String>,
    args_to_script: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    if let Some(file_path) = args.path {
        let contents =
            fs::read_to_string(file_path.clone()).expect("Should have been able to read the file");

        let mut config_file = file_path;

        config_file.set_extension("vbas.json");

        if args.compile {
            compile(&contents);
        } else if let Result::Err(_) = interpret(&contents, config_file, args.breakpoints) {
            process::exit(1);
        }
    } else {
        println!("{}", "Very Basic Version 0.1".yellow());
        loop {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let result = interpret(&line, PathBuf::from("settings.json"), None);
            match result {
                Ok(s) => println!("{}", s.bright_black()),
                Err(_) => println!(""),
            }
        }
    }
}

fn interpret(
    contents: &str,
    config_file: PathBuf,
    breakpoints: Option<String>,
) -> Result<String, String> {
    let tokens = crate::scanner::tokenize(&contents);

    match tokens {
        Ok(tokens) => {
            let mut instructions: Vec<compiler::OpCode> = Vec::new();
            let mut line_numbers: Vec<u32> = Vec::new();
            let mut compiler = Compiler::new(&tokens, &mut instructions, &mut line_numbers);
            compiler.compile();
            if compiler.in_error {
                return Result::Err(String::from("Compile Error"));
            }

            let source_lines: Vec<&str> = contents.lines().collect();

            //let mut vm = Vm::new(&mut line_numbers);
            let mut vm = match breakpoints {
                Some(break_points) => {
                    let test = DebugSettings::new(10, break_points.as_str());
                    Vm::new_debug(&mut line_numbers, &source_lines, test)
                }
                None => Vm::new(&mut line_numbers),
            };

            //let mut vm = Vm::new_debug(&mut line_numbers, &source_lines, test);

            //dbg!(&instructions);

            vm.config_file = config_file;
            let result = vm.run(&instructions);
            if !result {
                return Result::Err(String::from("Runtime Error"));
            }

            if let Some(val) = vm.return_value {
                Result::Ok(format!("{:?}", val))
            } else {
                Result::Ok(String::new())
            }
        }
        Err(msg) => {
            eprintln!("Tokenize Error: {}", msg.red());
            Result::Err(String::from("Tokenize Error"))
        }
    }
}

fn compile(contents: &str) {
    let tokens = crate::scanner::tokenize(&contents);

    match tokens {
        Ok(tokens) => {
            let mut instructions: Vec<compiler::OpCode> = Vec::new();
            let mut line_numbers: Vec<u32> = Vec::new();
            let mut compiler = Compiler::new(&tokens, &mut instructions, &mut line_numbers);
            compiler.compile();
            if compiler.in_error {
                return;
            }

            compiler::print_instr(instructions);
        }
        Err(msg) => {
            eprintln!("{}", msg.red());
        }
    }
}
// *****************************************************
// NOTE: DO NOT COMMIT WITHOUT FIRST RUNNING UNIT TESTS!
// *****************************************************

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::interpret;

    fn interpret_test(contents: &str) -> String {
        let result = interpret(contents, PathBuf::from("settings_test.json"), None);
        match result {
            Ok(s) => s,
            Err(s) => s,
        }
    }

    #[test]
    fn global_var_same_as_fn_name() {
        let code = "len = get_bet()
        
        function get_bet()
            h = len(123) 
            h = asc(ucase(h)) - 65
            bet = len(333)
            array(h,bet)
        end
    
    ";
        let result = interpret_test(code);
        assert_eq!(result, "Compile Error");
    }

    #[test]
    fn method_call_native() {
        let code = "\"hello\".len()";
        let result = interpret_test(code);
        assert_eq!(result, "Number(5.0)");
    }

    #[test]
    fn method_call_native2() {
        let code = "function get_odds(results)
        odds = array()
        total = 2000
        for i = 0 to len(results)-1
            print(i)
            if results[i] == 0 then
                odds.push(100)
            else
                odds.push((total/results[i]) * 0.8)
            end
        next
        odds
    end
    get_odds(array(100,200,400))";
        let result = interpret_test(code);
        assert_eq!(result, "Array([Number(16.0), Number(8.0), Number(4.0)])");
    }

    #[test]
    fn method_call_user() {
        let code = "function add(n,x) n+x end : x = 100: x.add(10)";
        let result = interpret_test(code);
        assert_eq!(result, "Number(110.0)");
    }

    #[test]
    fn method_call_user2() {
        let code = "x = 100
                    x.add(10)
                    function add(n,x) n+x end ";
        let result = interpret_test(code);
        assert_eq!(result, "Number(110.0)");
    }

    #[test]
    fn array_push() {
        let code = "a = array()
                    a.push(123)
                    a.push(456)
                    a";
        let result = interpret_test(code);
        assert_eq!(result, "Array([Number(123.0), Number(456.0)])");
    }

    #[test]
    fn array_push2() {
        let code = "a = array()
                    a.push(123)
                    a.push(456)
                    ";
        let result = interpret_test(code);
        assert_eq!(result, "Boolean(true)");
    }

    #[test]
    fn array_slice() {
        let code = "array(1,2,3,4,5,6).slice(1,3)
                    ";
        let result = interpret_test(code);
        assert_eq!(result, "Array([Number(2.0), Number(3.0)])");
    }

    #[test]
    fn grouping2() {
        let code = "rate=24  
                    (rate/12)";
        let result = interpret_test(code);
        assert_eq!(result, "Number(2.0)");
    }

    #[test]
    fn system_call() {
        let code = "@dummy_system_call_will_fail()";
        let result = interpret_test(code);
        assert_eq!(result, "Runtime Error");
    }

    #[test]
    fn system_call_invalid() {
        let code = "@crash";
        let result = interpret_test(code);
        assert_eq!(result, "Compile Error");
    }

    #[test]
    fn non_existing_function() {
        let code = "doesnt_exist()";
        let result = interpret_test(code);
        assert_eq!(result, "Compile Error");
    }

    #[test]
    fn else_if() {
        let code = "
            x = 5
            if x == 3 then
                print(1)
                print(123)
            elseif x == 4 then
                print(1)
                print(2)
                print(456)
            elseif x == 5 then
                (666)
            else
                print(777)
            end
                ";
        let result = interpret_test(code);
        assert_eq!(result, "Number(666.0)");
    }

    #[test]
    fn else_if2() {
        let code = "
            x = 3
            if x == 3 then
                1
            elseif x == 4 then
                2
            elseif x == 5 then
                3
            else
                4
            end
                ";
        let result = interpret_test(code);
        assert_eq!(result, "Number(1.0)");
    }

    #[test]
    fn else_if3() {
        let code = "
            x = 4
            if x == 3 then
                1
            elseif x == 4 then
                2
            elseif x == 5 then
                3
            else
                4
            end
                ";
        let result = interpret_test(code);
        assert_eq!(result, "Number(2.0)");
    }

    #[test]
    fn else_if4() {
        let code = "
            x = 666
            if x == 3 then
                1
            elseif x == 4 then
                2
            elseif x == 5 then
                3
            else
                4
            end
                ";
        let result = interpret_test(code);
        assert_eq!(result, "Number(4.0)");
    }

    #[test]
    fn while_loop_crash() {
        let code = "
        function get_human_move()
            while true
                inp = 1 'input(\"Enter your move: \")
                direction = left(inp,1)
                force = val(mid(inp,2))
                if force > 5 and direction ==  \"<\" or direction == \">\"  then
                    inp exit
                else
                    print(\"invalid bowl\")
                end
            end
        end
        ";
        interpret_test(code);
    }

    #[test]
    fn for_loop() {
        let code = "
            glob = 0
            for i = 1 to 5
                print(i)
                glob = i
            next
            glob
        ";

        assert_eq!(interpret_test(code), "Number(5.0)");
    }

    #[test]
    fn for_loop2() {
        let code = "
            result = 0
            for i = 0 to 0
                result = print(i)
            next

            for i = 10 to 0
                1234
            next
            result
        ";

        assert_eq!(interpret_test(code), "String(\"0\")");
    }

    #[test]
    fn for_step_up() {
        let code = "
            result = 0
            for i = 0 to 10 step 2
                result = result + 1
            next
            result
        ";

        assert_eq!(interpret_test(code), "Number(6.0)");
    }

    #[test]
    fn for_step_down() {
        let code = "
            result = 0
            for i = 5 to 1 step -1
                result = result + 1
            next
            result
        ";

        assert_eq!(interpret_test(code), "Number(5.0)");
    }

    #[test]
    fn for_loop_scope() {
        let code = "
            i = 10
            for i = 1 to 5
                print(i)
            next
            i
        ";

        assert_eq!(interpret_test(code), "Number(10.0)");
    }

    #[test]
    fn array_in_array() {
        let code = "
            a = array(array(5,10), array(6,11), array(7,12))
            a[1][1]
        ";
        // x is out of scope so compile error
        assert_eq!(interpret_test(code), "Number(11.0)");
    }

    #[test]
    fn subscript_set() {
        let code = "
            a = array(1,2,3)
            a[0] = 5
            a[0]
        ";

        assert_eq!(interpret_test(code), "Number(5.0)");
    }

    #[test]
    fn subscript_set_local() {
        let code = "
            if true then
                a = array(1,2,3)
                a[0] = 5
                a[0]
            end
        ";

        assert_eq!(interpret_test(code), "Number(5.0)");
    }

    #[test]
    fn array_in_array_set() {
        let code = "
            a = array(array(5,10), array(6,11), array(7,12))
            a[1][1] = 13.5
            a[1]
        ";

        assert_eq!(interpret_test(code), "Compile Error");
    }

    #[test]
    fn array_in_array_set_legal() {
        let code = "
            function f()

                a = array(array(5,10), array(6,11), array(7,12))

                if true then

                    b = a[0]

                    if b[1] == 10 then
                        if true then
                            b[0] = 10
                        end
                    end


                    b[0]
                end
            end

            function main()
                a=1
                b=2
                c=3
                f()
            end

            main()
        ";

        assert_eq!(interpret_test(code), "Number(10.0)");
    }

    #[test]
    fn array_in_array_get() {
        let code = "
            a = array(array(5,10), array(6,11), array(7,12))

            function test(x)   
                aa = a[x]
                aa[1]
            end

            function main()
                x = 1
                b = 2
                c = 3
                test(1)
            end

            main()
        ";

        assert_eq!(interpret_test(code), "Number(11.0)");
    }

    #[test]
    fn block_scope_while() {
        let code = "
            i=0
            
            while i < 10
                x = 20
                i = i + 1
            end
            print(x)
        ";
        // x is out of scope so compile error
        assert_eq!(interpret_test(code), "Compile Error");
    }

    #[test]
    fn block_scope_if() {
        let code = "
            global = 0
            if true then
                final = 0
                a = 1
                if true then
                    b = a + 1
                    if true then 
                        c = a + b + 1
                        final = c
                        if true then
                            ' do nothing - no crashing cause no variables in scope!
                        end
                    end
                end
                global = final
            end

            global
        ";
        assert_eq!(interpret_test(code), "Number(4.0)");
    }

    #[test]
    fn block_scope_if_else() {
        let code = "
            function test(x)
                result = 0
                if x == 1 then
                    b = 20
                    result = b
                else
                    c = 10
                    result = c
                end

                result
            end

            test(1)
        ";
        assert_eq!(interpret_test(code), "Number(20.0)");
    }

    #[test]
    fn block_scope_if_else2() {
        let code = "
            function test(x)
                result = 0
                if x == 1 then
                    b = 20
                    result = b
                else
                    c = 10 
                    result = c + x
                end

                result
            end

            test(2)
        ";
        assert_eq!(interpret_test(code), "Number(12.0)");
    }

    #[test]
    fn format_string() {
        let code = "str(1234.5678,\"N2\")";
        assert_eq!(interpret_test(code), "String(\"1,234.57\")");
    }

    #[test]
    fn interpolation() {
        let code = "\"a {1+1} b\"";
        assert_eq!(interpret_test(code), "String(\"a 2 b\")");
    }

    #[test]
    fn interpolation2() {
        let code = "\"a {\"2\"} b\"";
        assert_eq!(interpret_test(code), "String(\"a 2 b\")");
    }

    #[test]
    fn interpolation3() {
        let code = "\"{\"hello {66}\"}\"";
        assert_eq!(interpret_test(code), "String(\"hello 66\")");
    }

    #[test]
    fn raw_string() {
        let code = r#""""hello " {66}""""#;
        let exp = r#"Str("hello \" {66}")"#;
        let actual = interpret_test(code);
        println!("{}", actual);
        assert_eq!(actual, exp);
    }

    #[test]
    fn test_variables() {
        let code = "
        global = 0
        main()
        
        function main()
            i = 0
            while i < 3
                print(i)
                word = get_word()
                print(word)
                i=i+1
                word
            end
        end
        
        function get_word()
            global = global + 1
        end";
        assert_eq!(interpret_test(code), "Number(3.0)");
    }

    #[test]
    fn arity_wrong() {
        let code = "
        function test(a,b,c)
            a+b+c
        end

        test(1)
        ";
        assert_eq!(interpret_test(code), "Runtime Error");
    }

    #[test]
    fn recursion() {
        let code = "
        function fib(n) 
            if n < 2 then n exit end
            fib(n - 2) + fib(n - 1)
        end

        fib(20)
        ";
        assert_eq!(interpret_test(code), "Number(6765.0)");
    }

    #[test]
    fn duplicate_functions() {
        let code = "
        function test(a,b,c)
            a+b+c
        end

        function test(a)
            a * 10
        end
        ";
        assert_eq!(interpret_test(code), "Compile Error");
    }

    #[test]
    fn function_in_function() {
        let code = "
        function test(a,b,c)

            function inner_test(a)
                a * 10
            end

            a+b+c
        end
        ";
        assert_eq!(interpret_test(code), "Compile Error");
    }

    #[test]
    fn funtion_no_body() {
        let code = "function test() end print(test())";
        assert_eq!(interpret_test(code), "Compile Error");
    }

    #[test]
    fn token_scans_whole_words() {
        assert_eq!(
            interpret_test("note = 1234 print(note)"),
            "String(\"1234\")"
        );
    }

    #[test]
    fn func_first_class_citz() {
        assert_eq!(
            interpret_test(
                "function foo(x) x = x * 2 end function bar(f, n) f(n) end bar(foo, 10)"
            ),
            "Number(20.0)"
        );
    }

    #[test]
    fn func_first_class_citz2() {
        assert_eq!(
            interpret_test(
                "function foo(x)
                    a = 1
                    b = 2 
                    x = x * 2  
                end 
                
                x = foo
                y = x
                y(10)"
            ),
            "Number(20.0)"
        );
    }

    #[test]
    fn len() {
        assert_eq!(interpret_test("len(array(1,1,1,1,2))"), "Number(5.0)");

        assert_eq!(interpret_test("len(\"hello\")"), "Number(5.0)");
        assert_eq!(
            interpret_test("len(\"hello\" + \" world\")"),
            "Number(11.0)"
        );

        assert_eq!(interpret_test("len(555.45)"), "Number(8.0)");
        assert_eq!(interpret_test("len(true)"), "Number(1.0)");
    }

    #[test]
    fn calling() {
        assert_eq!(interpret_test("print("), "Compile Error");
        assert_eq!(interpret_test("print(\"hello\")"), "String(\"hello\")");
    }

    #[test]
    fn booleans() {
        assert_eq!(interpret_test("true"), "Boolean(true)");
        assert_eq!(interpret_test("false"), "Boolean(false)");
    }

    #[test]
    fn arrays() {
        assert_eq!(interpret_test("x=array(1,2,3) : x[0]"), "Number(1.0)");
        assert_eq!(interpret_test("x=array(1,2,3) : x[1]"), "Number(2.0)");
        assert_eq!(interpret_test("x=array(1,2,3) : x[2]"), "Number(3.0)");
        assert_eq!(interpret_test("x=array(1,2,3) : x[3]"), "Runtime Error");
        assert_eq!(interpret_test("x=46 : x[3]"), "Runtime Error");

        assert_eq!(
            interpret_test(
                " 
        function test() 
            x=array(1,2,3) 
            x[2] 
        end
        test() 
        "
            ),
            "Number(3.0)"
        );
    }

    #[test]
    fn and() {
        assert_eq!(interpret_test("1==1 and 2==2"), "Boolean(true)");
        assert_eq!(interpret_test("1==1 and 1==2"), "Boolean(false)");
        assert_eq!(
            interpret_test(
                "x2=1:y2=1:iteration=1:max_iteration=1 x2 + y2 <= 4 and iteration < max_iteration"
            ),
            "Boolean(false)"
        );
        //
    }

    #[test]
    fn or() {
        assert_eq!(interpret_test("1==1 or 2==2"), "Boolean(true)");
        assert_eq!(interpret_test("1==1 or 1==2"), "Boolean(true)");

        //
    }

    #[test]
    fn multiple_fns() {
        let code = "function setgraphics(x)
                        0
                    end
                    
                    function test(x,y,c)
                        x = 1
                        b = 2
                        setgraphics(2)
                        print(x)
                    end
                    test(1,2,3)";
        assert_eq!(interpret_test(code), "String(\"1\")");
    }

    #[test]
    fn shadowing() {
        assert_eq!(
            interpret_test("x = 5.5 function test(x) x * 2 end test(20) "),
            "Number(40.0)"
        );
        assert_eq!(
            interpret_test("x = 5.5 function test(y) y * 2 end test(20) "),
            "Number(40.0)"
        );
    }

    #[test]
    fn locals() {
        assert_eq!(
            interpret_test(
                "x = 5.5: y = 6.6: function test(x,y,z) a=1 b=2 a+b+x+y+z end: test(3,4,5): "
            ),
            "Number(15.0)"
        );
    }

    #[test]
    fn define_function() {
        assert_eq!(
            interpret_test("function test() print(45) end test() print(66)"),
            "String(\"66\")"
        );
        assert_eq!(interpret_test("function test(a,b,c) print(45) end"), "");

        assert_eq!(
            interpret_test("function test() 45 end test() "),
            "Number(45.0)"
        );

        assert_eq!(
            interpret_test("function test(x) x * 2 end test(20) "),
            "Number(40.0)"
        );
    }

    #[test]
    fn ifthenelseif() {
        let code = "
        i = 0
        answer = 44
        while i < 10
            print(i)
            word = input(\"Enter a word\")
            if left(word,5) > answer then
                print(\"guess is > answer\")
            else if i = 1 then
                if word < answer then
                    print(\"guess < answer\")
                else
                    print(\"you got it\")
                end
            end
        end
        ";
        assert_eq!(interpret_test(code), "Compile Error");
    }

    #[test]
    fn stack_overflow() {
        let code = "function stack_overflow(x) stack_overflow(x+1) end stack_overflow(1)";
        assert_eq!(interpret_test(code), "Runtime Error");
    }

    #[test]
    fn ifthenelse() {
        assert_eq!(interpret_test("if 1==1 then 666 end"), "Number(666.0)");
        assert_eq!(
            interpret_test("if 1==1 then 666 else 555 end"),
            "Number(666.0)"
        );
        assert_eq!(
            interpret_test("if 1==2 then 666 else 555 end"),
            "Number(555.0)"
        );
        assert_eq!(
            interpret_test("if 1==1 then x=1 x+5 else x=6 x+5 end"),
            "Number(6.0)"
        );
        assert_eq!(
            interpret_test("if 1==2 then x=1 x+5 else x=6 x+5 end"),
            "Number(11.0)"
        );

        assert_eq!(
            interpret_test("if 1 then x=1 x+5 else x=6 x+5 end"),
            "Runtime Error"
        );
    }

    #[test]
    fn while_loop() {
        assert_eq!(
            interpret_test("x = 0: while x < 10 x=x+1 end x"),
            "Number(10.0)"
        );
    }

    #[test]
    fn variables() {
        assert_eq!(interpret_test("x = 1000"), "Number(1000.0)");
        assert_eq!(interpret_test("print(x)"), "Compile Error");
        assert_eq!(interpret_test("x = 1000: x / 100"), "Number(10.0)");
    }

    #[test]
    fn arithmatic() {
        let contents = "-((1+1)*(1+1)) * (10.0-6) -20+1-8+9*10/5/9-2*7+1";

        let result = interpret_test(contents);

        assert_eq!(result, "Number(-54.0)");
    }

    #[test]
    fn comparisons() {
        assert_eq!(interpret_test("3 > (2-2)"), "Boolean(true)");
        assert_eq!(interpret_test("3 >= (2-2)"), "Boolean(true)");
        assert_eq!(interpret_test("3 < (2-2)"), "Boolean(false)");
        assert_eq!(interpret_test("3 <= (2-2)"), "Boolean(false)");
        assert_eq!(interpret_test("3 == (2-2)"), "Boolean(false)");
        assert_eq!(interpret_test("3 <> (2-2)"), "Boolean(true)");
    }

    #[test]
    fn not() {
        let result = interpret_test("not (1==1)");
        assert_eq!(result, "Boolean(false)");

        let result = interpret_test("not 0");
        assert_eq!(result, "Boolean(true)");

        let result = interpret_test("not \"\"");
        assert_eq!(result, "Boolean(true)");

        let result = interpret_test("not (\"hello\" + \"world\") ");
        assert_eq!(result, "Boolean(false)");

        let result = interpret_test("not (1==0)");
        assert_eq!(result, "Boolean(true)");

        let result = interpret_test("not 1");
        assert_eq!(result, "Boolean(false)");

        let result = interpret_test("not \"hi\"");
        assert_eq!(result, "Boolean(false)");
    }

    #[test]
    fn string_functions_left() {
        assert_eq!(interpret_test("left(\"hello\", 1)"), "String(\"h\")");
        assert_eq!(interpret_test("left(\"hello\", 100)"), "String(\"hello\")");
        assert_eq!(interpret_test("left(\"hello\", -1)"), "Runtime Error");
        assert_eq!(interpret_test("left(\"hello\", 0)"), "String(\"\")");
    }

    #[test]
    fn string_functions_right() {
        assert_eq!(interpret_test("right(\"hello\", 1)"), "String(\"o\")");
        assert_eq!(interpret_test("right(\"hello\", 100)"), "String(\"hello\")");
        assert_eq!(interpret_test("right(\"hello\", -1)"), "Runtime Error");
        assert_eq!(interpret_test("right(\"hello\", 0)"), "String(\"\")");
    }

    #[test]
    fn golf_features() {
        let code = "fn x() 66; if true then x() else 2;";
        assert_eq!(interpret_test(code), "Number(66.0)");
    }
}
