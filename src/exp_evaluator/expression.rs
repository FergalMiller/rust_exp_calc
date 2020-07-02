pub struct Expression
{
    pub expression: String
}

impl Expression
{
    pub fn pop(&mut self) -> Option<char>
    {
        let mut chars = self.expression.chars();

        let mut c: Option<char>;
        loop
        {
            c = chars.next();

            if c.is_none() { return Option::None::<char>; }
            if !c.unwrap().is_whitespace() { break; }
        }

        self.expression = String::from(chars.as_str());
        c
    }

    pub fn peek(&self) -> Option<char>
    {
        let mut chars = self.expression.chars();

        let mut c: Option<char>;
        loop
        {
            c = chars.next();

            if c.is_none() { return Option::None::<char>; }
            if !c.unwrap().is_whitespace() { break; }
        }
        c
    }
}
