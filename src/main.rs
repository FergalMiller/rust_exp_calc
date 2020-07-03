use std::io;
use std::io::Write;
use crate::exp_evaluator::parser::parser::Parser;
use crate::exp_evaluator::expression::Expression;
use crate::exp_evaluator::parser::lexer::{Lexer, ERR};

mod exp_evaluator;

fn main() 
{
    println!("##-- Rust calculator says: Welcome! --##");
    run();
    println!("##-- Rust calculator says: Goodbye! --##")
}

fn run()
{
    loop
    {
        print!("\nEnter your expression: ");
        io::stdout().flush().expect("Failed to flush!");
        let input = get_user_input();

        match input
        {
            _ if input == String::from("quit") => break,
            _ => println!("User entered: {}", input)
        }

        let expression = Expression { expression: input };
        let lexer = Lexer{ expression, current_token: ERR, number_value: "".to_string() };
        let mut parser = Parser { lexer };

        let expression_tree = parser.parse();
        let result = expression_tree.evaluate();
        println!("Result: {}", result);
    }
}

fn get_user_input() -> String
{
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read that input :( Try again?");

    return String::from(input.trim());
}
