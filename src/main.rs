mod scanner;

use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Very Basic Version 0.0");
    dbg!(&args);

    if let Some(file_path) = args.get(1) {
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let tokens = scanner::tokenize(&contents);
        dbg!(tokens);
    } else {
        println!("No filename passed as argument");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let file_path = "c:\\tmp\\test.vb";
        let contents =
            std::fs::read_to_string(file_path).expect("Should have been able to read the file");

        let tokens = crate::scanner::tokenize(&contents);
    }
}
