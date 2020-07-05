use std::io;
use std::io::Write;
use crate::exp_evaluator::parser::parser::Parser;
use crate::exp_evaluator::expression::Expression;
use crate::exp_evaluator::parser::lexer::{Lexer, ERR};
use crate::exp_evaluator::exp_tree::ExpressionTree;

mod exp_evaluator;

const COMMAND_NEW_EXPRESSION: &str = "0";
const COMMAND_EVALUATE: &str = "1";
const COMMAND_IN_ORDER: &str = "2";

fn main() 
{
    println!("##-- rust_exp_calc says: Welcome! --##");
    run();
    println!("##-- rust_exp_calc says: Goodbye! --##")
}

fn run()
{
    loop
    {

        let expression_input = get_user_input(&"\nEnter new expression: ".to_string());

        match expression_input
        {
            _ if expression_input == String::from("quit") => break,
            _ => println!("User entered: {}", expression_input)
        }

        let expression_tree = parse_expression(Expression{ expression: expression_input } );

        use_tree(&expression_tree);
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

fn use_tree(exp_tree: &ExpressionTree)
{
    loop
    {
        let command_input = get_user_input(&"\nEnter your command: ".to_string());

        match command_input
        {
            _ if command_input == COMMAND_NEW_EXPRESSION => break,
            _ if command_input == COMMAND_EVALUATE => println!("Evaluated: {}", exp_tree.evaluate()),
            _ if command_input == COMMAND_IN_ORDER => println!("In order: {}", exp_tree.to_string()),
            _ => { print!("Command not recognised. "); print_commands(); }
        }
    }
}

fn print_commands()
{
    println!("Available commands:");
    println!("\t\"{}\": To restart with a new expression", COMMAND_NEW_EXPRESSION);
    println!("\t\"{}\": To evaluate the current expression", COMMAND_EVALUATE);
    println!("\t\"{}\": To print the current tree with in-order traversal", COMMAND_IN_ORDER);
}
