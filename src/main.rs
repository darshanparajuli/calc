extern crate ansi_term;

mod calc;

use ansi_term::Color::{Green, Red};
use ansi_term::Style;
use calc::Calculator;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let mut calc = Calculator::new();

    loop {
        prompt();

        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => match input.trim() {
                "exit" => break,
                "reset" => calc.reset(),
                "help" => help(&calc),
                input => match calc.run(&input) {
                    Ok(result) => {
                        if !result.is_empty() {
                            print_result(&result);
                        }
                    }
                    Err(e) => print_err(&e),
                },
            },
            Err(e) => {
                println!("stdin error: {}\n", e);
                continue;
            }
        }
    }
}

fn prompt() {
    print!("{} ", Style::new().bold().paint("λ"));
    io::stdout().flush().unwrap();
}

fn print_result(result: &str) {
    println!("{} {}\n", Green.paint("=>"), result);
}

fn print_err(err: &str) {
    println!("{} {}\n", Red.paint("=>"), err);
}

fn help(calc: &Calculator) {
    println!(
        "{} Constants: {}",
        Green.paint("=>"),
        calc.get_constants().join(", ")
    );
    println!("   Functions: {}", calc.get_functions().join(", "));
    println!("   Variables: ans");
    println!("   Commands:  help, reset, exit\n");
}
