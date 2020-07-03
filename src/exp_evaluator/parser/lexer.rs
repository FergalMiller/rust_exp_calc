use crate::exp_evaluator::expression::Expression;

pub const ERR:u8 = 0;
pub const NUM:u8 = 1;
pub const PLUS:u8 = 2;
pub const MINUS:u8 = 3;
pub const MULTIPLY:u8 = 4;
pub const DIVIDE:u8 = 5;
pub const POWER:u8 = 6;
pub const PAR_LEFT:u8 = 7;
pub const PAR_RIGHT:u8 = 8;

pub struct Lexer
{
    pub expression: Expression,
    pub current_token: u8,
    pub number_value: String
}

impl Lexer
{
    pub fn next(&mut self)
    {
        let next_char_opt = self.expression.pop();
        if next_char_opt.is_none() { self.current_token = ERR; return; }

        let next_char = next_char_opt.unwrap();
        match next_char
        {
            '+' => self.current_token = PLUS,
            '-' => self.current_token = MINUS,
            '*' => self.current_token = MULTIPLY,
            '/' => self.current_token = DIVIDE,
            '^' => self.current_token = POWER,
            '(' => self.current_token = PAR_LEFT,
            ')' => self.current_token = PAR_RIGHT,
            _ if next_char.is_numeric() => self.lex_number(next_char),
            _ => self.current_token = ERR
        }
    }

    fn lex_number(&mut self, c: char)
    {
        self.number_value = String::new();
        self.number_value.push(c);
        loop
        {
            let next_char_opt = self.expression.peek();
            if next_char_opt.is_some()
            {
                let next_char = next_char_opt.unwrap();
                if next_char.is_numeric()
                {
                    self.number_value.push(next_char);
                    self.expression.pop();
                }
                else { break; }
            }
            else { break; }
        }
        self.current_token = NUM;
    }
}
