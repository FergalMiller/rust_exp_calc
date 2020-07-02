pub struct Expression
{
    pub expression: String
}

impl Expression
{
    pub fn pop(&mut self) -> Option<char>
    {
        if self.expression.len() == 0 { return Option::None::<char>; }
        let c = self.expression.chars().next();
        self.expression = String::from(&self.expression[1..]);
        c
    }

    pub fn peek(&self) -> Option<char>
    {
        self.expression.chars().next()
    }
}
