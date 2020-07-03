use crate::exp_evaluator::exp_tree::ExpressionTree;
use crate::exp_evaluator::exp_tree::{NUMBER_NODE, OPERATOR_NODE};
use crate::exp_evaluator::parser::lexer::{Lexer, ERR, NUM, PLUS, MINUS, MULTIPLY, DIVIDE, POWER, PAR_LEFT, PAR_RIGHT};

pub struct Parser
{
    pub(crate) lexer: Lexer
}

impl Parser
{
    pub fn parse(&mut self) -> ExpressionTree
    {
        self.lexer.next();
        return self.parse_precedence_1();
    }

    fn parse_precedence_1(&mut self) -> ExpressionTree
    {
        let mut left = self.parse_precedence_2();
        while self.lexer.current_token == PLUS || self.lexer.current_token == MINUS
        {
            let operator = self.lexer.current_token;
            self.lexer.next();
            if operator == PLUS { left = make_operator_tree(left, self.parse_precedence_2(), String::from("+")); }
            if operator == MINUS { left = make_operator_tree(left, self.parse_precedence_2(), String::from("-")); }
        }
        left
    }

    fn parse_precedence_2(&mut self) -> ExpressionTree
    {
        let mut left = self.parse_precedence_3();
        while self.lexer.current_token == MULTIPLY || self.lexer.current_token == DIVIDE
        {
            let operator = self.lexer.current_token;
            self.lexer.next();
            if operator == MULTIPLY { left = make_operator_tree(left, self.parse_precedence_3(), String::from("*")); }
            if operator == DIVIDE { left = make_operator_tree(left, self.parse_precedence_3(), String::from("/")); }
        }
        left
    }

    fn parse_precedence_3(&mut self) -> ExpressionTree
    {
        let mut left = self.parse_precedence_4();
        while self.lexer.current_token == POWER
        {
            self.lexer.next();
            left = make_operator_tree(left, self.parse_precedence_4(), String::from("^"));
        }
        left
    }

    fn parse_precedence_4(&mut self) -> ExpressionTree
    {
        match self.lexer.current_token
        {
            NUM => self.parse_number(),
            PAR_LEFT => self.parse_left_parenthesis(),
            _ => panic!("Could not parse. Invalid token from lexer found. Token: {}", self.lexer.current_token)
        }
    }

    fn parse_number(&mut self) -> ExpressionTree
    {
        let leaf = make_leaf(self.lexer.number_value.clone());
        self.lexer.next();
        leaf
    }

    fn parse_left_parenthesis(&mut self) -> ExpressionTree
    {
        self.lexer.next();
        let result = self.parse_precedence_1();
        if self.lexer.current_token != PAR_RIGHT { panic!("Right parenthesis expected!") }
        result
    }
}

fn make_operator_tree(left: ExpressionTree, right: ExpressionTree, operator_value: String) -> ExpressionTree
{
    ExpressionTree
    {
        node_type: OPERATOR_NODE,
        value: operator_value,
        left: Option::Some(Box::new(left)),
        right: Option::Some(Box::new(right))
    }
}

fn make_leaf(leaf_value: String) -> ExpressionTree
{
    ExpressionTree
    {
        node_type: NUMBER_NODE,
        value: leaf_value,
        left: Option::None::<Box<ExpressionTree>>,
        right: Option::None::<Box<ExpressionTree>>
    }
}