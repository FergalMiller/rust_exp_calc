use crate::exp_evaluator::expression::Expression;

pub const ERR:i8 = 0;
pub const NUM:i8 = 1;
pub const PLUS:i8 = 2;
pub const MINUS:i8 = 3;
pub const MULTIPLY:i8 = 4;
pub const DIVIDE:i8 = 5;
pub const POWER:i8 = 6;
pub const PAR_LEFT:i8 = 7;
pub const PAR_RIGHT:i8 = 8;

pub struct Lexer
{
    pub expression: Expression,
    pub number_value: i32
}

impl Lexer
{
    pub fn get_token(&mut self) -> i8
    {
        let next_char_opt = self.expression.pop();
        if next_char_opt.is_none() { return ERR; }

        let next_char = next_char_opt.unwrap();
        match next_char
        {
            '+' => PLUS,
            '-' => MINUS,
            '*' => MULTIPLY,
            '/' => DIVIDE,
            '^' => POWER,
            '(' => PAR_LEFT,
            ')' => PAR_RIGHT,
            _ if next_char.is_numeric() =>
                {
                    self.lex_number(next_char);
                    NUM
                }
            _ => ERR
        }
    }

    fn lex_number(&mut self, c: char)
    {
        let mut number_string = String::new();
        number_string.push(c);
        loop
        {
            let next_char_opt = self.expression.peek();
            if next_char_opt.is_some()
            {
                let next_char = next_char_opt.unwrap();
                if next_char.is_numeric()
                {
                    number_string.push(next_char);
                    self.expression.pop();
                }
                else { break; }
            }
            else { break; }
        }
        self.number_value = number_string.parse().unwrap();
    }
}
