extern crate ansi_term;

mod calc;

use calc::Calculator;
use std::io::{self, Write};
use ansi_term::Color::{Green, Red, RGB};

fn main() {
    let mut input = String::new();
    let mut calc = Calculator::new();

    loop {
        prompt();

        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(n) => if n == 0 {
                println!();
                break;
            }
            Err(e) => {
                println!("stdin error: {}\n", e);
                continue;
            }
        }

        match input.trim() {
            "exit" => break,
            "reset" => calc.reset(),
            input => {
                match calc.run(&input) {
                    Ok(result) => {
                        if !result.is_empty() {
                            print_result(&result);
                        }
                    }
                    Err(e) => print_err(&e),
                }
            }
        }
    }
}

fn prompt() {
    print!("{} ", RGB(161, 176, 184).bold().paint("λ"));
    io::stdout().flush().unwrap();
}

fn print_result(result: &str) {
    println!("{} {}\n", Green.paint("=>"), result);
}

fn print_err(err: &str) {
    println!("{} Error, {}\n", Red.paint("=>"), err);
}
