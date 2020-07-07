use std::env;
use std::fs;
use std::io::{self, prelude::Write};

mod environment;
mod interpret;
mod lex;
mod parse;
mod stmt;
mod token;

fn main() {
    let mut lox = Lox::new();
    let args: Vec<String> = env::args().skip(1).collect();
    match args.as_slice() {
        [] => lox.run_prompt(),
        [script_name] => lox.run_file(script_name),
        _ => println!("usage: lox [filename]"),
    }
}

struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    fn new() -> Self {
        Lox {
            had_error: false,
            had_runtime_error: false,
        }
    }

    /// run the interpreter on a file
    fn run_file(&self, script_name: &str) {
        let contents =
            fs::read_to_string(script_name).expect(&format!("could not open {}", script_name));

        self.run(script_name.to_string(), &contents);

        if self.had_error {
            std::process::exit(65);
        } else if self.had_runtime_error {
            std::process::exit(70);
        }
    }

    /// run the interpreter in REPL mode
    fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut buf = String::new();
        loop {
            write!(stdout, "> ").expect("unable to write to stdout");
            stdout.flush().expect("failed to flush stdout");

            stdin
                .read_line(&mut buf)
                .expect("failed to read line from stdin");

            self.run("<repl>".to_string(), &buf);
            self.had_error = false;

            buf.clear();
        }
    }

    fn run(&self, name: String, source: &str) {
        let tokens = lex::lex(name, source);
        let statements = parse::parse(tokens);
        interpret::interpret(statements);
    }
}
