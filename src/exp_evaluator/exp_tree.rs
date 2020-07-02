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
            _ => self.left.as_ref().unwrap().evaluate() + self.right.as_ref().unwrap().evaluate()
        }
    }
}