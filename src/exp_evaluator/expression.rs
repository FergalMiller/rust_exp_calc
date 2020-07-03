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

#[cfg(test)]
mod tests
{
    use crate::exp_evaluator::expression::{Expression, find_next_non_whitespace};

    #[test]
    fn test_pop()
    {
        let mut exp = Expression { expression: String::from("blink") };

        let actual_popped = exp.pop().unwrap();
        let actual_expression = exp.expression;

        assert_eq!(actual_popped, 'b');
        assert_eq!(actual_expression, "link")
    }

    #[test]
    fn test_pop_with_leading_whitespace()
    {
        let mut exp = Expression { expression: String::from(" blink") };

        let actual_popped = exp.pop().unwrap();
        let actual_expression = exp.expression;

        assert_eq!(actual_popped, 'b');
        assert_eq!(actual_expression, "link")
    }

    #[test]
    fn test_peek()
    {
        let exp = Expression { expression: String::from("blink") };

        let actual_popped = exp.peek().unwrap();
        let actual_expression = exp.expression;

        assert_eq!(actual_popped, 'b');
        assert_eq!(actual_expression, "blink")
    }

    #[test]
    fn test_peek_with_leading_whitespace()
    {
        let exp = Expression { expression: String::from(" blink") };

        let actual_popped = exp.peek().unwrap();
        let actual_expression = exp.expression;

        assert_eq!(actual_popped, 'b');
        assert_eq!(actual_expression, " blink")
    }

    #[test]
    fn test_find_next_non_whitespace()
    {
        let s = String::from("  donkey");
        let mut chars = s.chars();

        let actual = find_next_non_whitespace(&mut chars).unwrap();

        assert_eq!(actual, 'd');
    }

    #[test]
    fn test_find_next_non_whitespace_with_tab()
    {
        let s = String::from("\tdonkey");
        let mut chars = s.chars();

        let actual = find_next_non_whitespace(&mut chars).unwrap();

        assert_eq!(actual, 'd');
    }
}
