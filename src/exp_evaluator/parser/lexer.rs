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
        let mut floating_point_found = false;
        self.number_value = String::new();
        self.number_value.push(c);
        loop
        {
            if !self.process_next_char(&mut floating_point_found) { break; }
        }
        self.current_token = NUM;
    }

    /// Processes the next peeked character in the expression.
    ///
    /// returns
    ///     -`true` if the character is processed as a number value
    ///     -`false` if the peeked character is not processed as a number
    fn process_next_char(&mut self, floating_point_found: &mut bool) -> bool
    {
        let next_char_opt = self.expression.peek();
        if next_char_opt.is_some()
        {
            let next_char = &next_char_opt.unwrap();
            if char_is_numeric(next_char) { self.push_next_char_to_number_value(floating_point_found); }
            else if next_char.is_whitespace() { self.expression.pop(); }
            else { return false; }
            true
        }
        else { false }
    }

    fn push_next_char_to_number_value(&mut self, floating_point_found: &mut bool)
    {
        let next_char = self.expression.pop().expect("Expected value when popping from expression.");
        if next_char.is_numeric()
        {
            self.number_value.push(next_char);
        }
        else if next_char == '.'
        {
            println!("Found . b = {}", *floating_point_found);
            if *floating_point_found { panic!("Second floating point found in number.")}
            *floating_point_found = true;
            self.number_value.push(next_char);
        }
        else { panic!("Cannot push character to number value!"); }
    }
}

fn char_is_numeric(c: &char) -> bool
{
    c.is_numeric() || *c == '.'
}
