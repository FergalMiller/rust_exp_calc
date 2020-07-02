use std::str::Chars;

pub struct Expression
{
    pub expression: String
}

impl Expression
{
    pub fn pop(&mut self) -> Option<char>
    {
        let mut chars = self.expression.chars();
        let c = find_next_non_whitespace(&mut chars);
        self.expression = String::from(chars.as_str());
        c
    }

    pub fn peek(&self) -> Option<char>
    {
        find_next_non_whitespace(&mut self.expression.chars())
    }
}

fn find_next_non_whitespace(chars: &mut Chars) -> Option<char>
{
    let mut c: Option<char>;
    loop
    {
        c = chars.next();
        if c.is_none() { return Option::None::<char>; }
        if !c.unwrap().is_whitespace() { break; }
    }
    c
}
