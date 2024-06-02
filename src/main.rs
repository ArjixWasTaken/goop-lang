#![allow(unused)]

use crate::lexer::Lexer;
use colored::*;

mod lexer;
mod tests;

pub fn indent(s: &str, n: usize) -> String {
    let mut out = String::new();
    for line in s.lines() {
        out.push_str(&format!("{}{}\n", " ".repeat(n), line));
    }
    out
}

fn repl() {
    loop {
        let input: String = casual::input().default("".into()).prompt("> ").get();

        if input.trim().len() == 0 || vec!["exit", "quit"].contains(&input.trim()) {
            println!("Exiting...");
            break;
        }

        let lexer = lexer::Lexer::new(input);
        println!("{:#?}", lexer.lex());
    }
}

fn main() {
    let mut args = std::env::args();
    args.next(); // skip the first argument

    println!(
        "{} -- version v{} -- {}",
        "arlang".green(),
        option_env!("CARGO_PKG_VERSION").unwrap_or("N/A").yellow(),
        "https://github.com/ArjixWasTaken/goop-lang".bright_blue()
    );

    repl();
}
