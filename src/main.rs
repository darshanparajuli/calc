mod calc;

use calc::Calculator;
use std::io::{self, Write};

macro_rules! print_result {
    ($msg:expr) => (
        println!("=> {}\n", $msg);
    )
}

macro_rules! print_err {
    ($err:expr) => (
        println!("=> Error, {}\n", $err);
    )
}

fn prompt() {
    print!("λ ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut input = String::new();
    let mut calc = Calculator::new();

    loop {
        input.clear();
        prompt();

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                panic!("Error: {}", e);
            }
        }

        input = input.trim().into();
        match input.as_str() {
            "exit" => break,
            "reset" => calc.reset(),
            _ => {
                match calc.run(&input) {
                    Ok(result) => {
                        if !result.is_empty() {
                            print_result!(result);
                        }
                    }
                    Err(e) => print_err!(e),
                }
            }
        }
    }
}
