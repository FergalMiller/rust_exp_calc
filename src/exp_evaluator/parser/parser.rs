use super::exp_tree::ExpressionTree;
use crate::exp_evaluator::parser::exp_tree::{NUMBER_NODE, OPERATOR_NODE};
use crate::exp_evaluator::parser::lexer::{Lexer, ERR, NUM};
use crate::exp_evaluator::expression::Expression;

pub fn parse(line: &String) -> ExpressionTree
{
    let expression = Expression { expression: line.clone()};
    let mut lexer = Lexer { expression, number_value: 0};

    loop
    {
        let token = lexer.get_token();
        match token
        {
            ERR => break,
            NUM => println!("Number token processed: {}", lexer.number_value),
            _ => println!("Token processed: {}", token)
        }
    }

    return make_demo_tree();
}

pub fn make_demo_tree() -> ExpressionTree
{
    let left = ExpressionTree {
        node_type: NUMBER_NODE,
        value: String::from("7"),
        left: Option::None,
        right: Option::None
    };

    let right = ExpressionTree {
        node_type: NUMBER_NODE,
        value: String::from("5"),
        left: Option::None,
        right: Option::None
    };

    return ExpressionTree {
        node_type: OPERATOR_NODE,
        value: String::from("+"),
        left: Option::Some(Box::new(left)),
        right: Option::Some(Box::new(right))
    };
}