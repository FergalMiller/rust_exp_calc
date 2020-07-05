use std::io;
use std::io::Write;
use crate::exp_evaluator::parser::parser::Parser;
use crate::exp_evaluator::expression::Expression;
use crate::exp_evaluator::parser::lexer::{Lexer, ERR};
use crate::exp_evaluator::exp_tree::ExpressionTree;

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

        let expression_input = get_user_input(&"\nEnter your expression: ".to_string());

        match expression_input
        {
            _ if expression_input == String::from("quit") => break,
            _ => println!("User entered: {}", expression_input)
        }

        let expression_tree = parse_expression(Expression{ expression: expression_input } );

        let command_input = get_user_input(&"\nEnter your command: ".to_string());

        match command_input
        {
            _ if command_input == String::from("1") => println!("Evaluated: {}", expression_tree.evaluate()),
            _ if command_input == String::from("2") => println!("In order: {}", expression_tree.to_string()),
            _ => println!("Command not recognised.")
        }
    }
}

fn get_user_input(message: &String) -> String
{
    print!("{}", message);
    io::stdout().flush().expect("Failed to flush!");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read that input :( Try again?");

    return String::from(input.trim());
}

fn parse_expression(expression: Expression) -> ExpressionTree
{
    let lexer = Lexer{ expression, current_token: ERR, number_value: "".to_string() };
    let mut parser = Parser { lexer };
    parser.parse()
}
