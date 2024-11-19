mod ast;
mod parser;

use std::io::{self, BufRead};
use pest::Parser;
use crate::parser::{parse_expr, CalculatorParser, Rule};

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        match CalculatorParser::parse(Rule::equation, &line?) {
            Ok(mut pairs) => {
                let expr = parse_expr(pairs.next().unwrap().into_inner());
                println!("Result: {}", expr.evaluate());
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
    }
    Ok(())
}

