pub const NUMBER_NODE:i8 = 1;
pub const OPERATOR_NODE:i8 = 2;

pub struct ExpressionTree
{
    pub node_type: i8,
    pub value: String,
    pub left: Option<Box<ExpressionTree>>,
    pub right: Option<Box<ExpressionTree>>
}

impl ExpressionTree
{
    pub fn evaluate(&self) -> i32
    {
        match self.node_type
        {
            NUMBER_NODE => self.value.parse().unwrap(),
            _ => self.evaluate_branch()
        }
    }

    fn evaluate_branch(&self) -> i32
    {
        let left = self.left.as_ref().unwrap();
        let right = self.right.as_ref().unwrap();
        match self.value.as_str()
        {
            "+" => left.evaluate() + right.evaluate(),
            "-" => left.evaluate() - right.evaluate(),
            "*" => left.evaluate() * right.evaluate(),
            "/" => left.evaluate() / right.evaluate(),
            "^" => left.evaluate() ^ right.evaluate(),
            _ => panic!("Operator could not be matched in branch evaluation.")
        }
    }
}