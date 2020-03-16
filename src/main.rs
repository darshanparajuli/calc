mod calc;

use calc::Calculator;
use std::io::{self, BufRead, Write};
use termion;
use termion::{color, style};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    let mut calc = Calculator::new();

    let stdin = io::stdin();
    let mut handle_in = stdin.lock();

    let stdout = io::stdout();
    let mut handle_out = stdout.lock();

    loop {
        prompt(&mut handle_out)?;

        input.clear();
        match handle_in.read_line(&mut input) {
            Ok(0) => {
                writeln!(handle_out)?;
                handle_out.flush()?;
                break;
            }
            Ok(_) => match input.trim() {
                "exit" => break,
                "reset" => calc.reset(),
                "help" => help(&mut handle_out, &calc)?,
                input => match calc.run(&input) {
                    Ok(result) => {
                        if !result.is_empty() {
                            print_result(&mut handle_out, &result)?;
                        }
                    }
                    Err(e) => print_err(&mut handle_out, &e)?,
                },
            },
            Err(e) => {
                writeln!(handle_out, "stdin error: {}", e)?;
                handle_out.flush()?;
                continue;
            }
        }
    }

    Ok(())
}

fn prompt<W: Write>(w: &mut W) -> Result<(), io::Error> {
    write!(
        w,
        "{}{}λ {}",
        style::Bold,
        color::Fg(color::Reset),
        style::Reset
    )?;
    w.flush()?;

    Ok(())
}

fn print_result<W: Write>(w: &mut W, result: &str) -> Result<(), io::Error> {
    writeln!(w, "{}=> {}\n", color::Fg(color::Green), result)?;
    w.flush()?;
    Ok(())
}

fn print_err<W: Write>(w: &mut W, err: &str) -> Result<(), io::Error> {
    writeln!(w, "{}=> {}\n", color::Fg(color::Red), err)?;
    w.flush()?;
    Ok(())
}

fn help<W: Write>(w: &mut W, calc: &Calculator) -> Result<(), io::Error> {
    write!(
        w,
        "{}=> Constants: {}",
        color::Fg(color::Green),
        calc.get_constants().join(", ")
    )?;
    write!(w, "   Functions: {}", calc.get_functions().join(", "))?;
    write!(w, "   Variables: ans")?;
    writeln!(w, "   Commands:  help, reset, exit")?;
    writeln!(w)?;
    w.flush()?;

    Ok(())
}
