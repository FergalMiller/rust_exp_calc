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

    pub fn to_string(&self) -> String
    {
        if self.node_type == NUMBER_NODE { return self.to_string_number_node() }
        if self.node_type == OPERATOR_NODE { return self.to_string_operator_node() }
        panic!("No valid node type identified for to_string process.")
    }

    fn to_string_number_node(&self) -> String
    {
        self.value.clone()
    }

    fn to_string_operator_node(&self) -> String
    {

        let mut s = String::new();
        let mut parenthesis_left = false;
        let mut parenthesis_right = false;

        let precedence = get_precedence(&self.value);

        let left = self.left.as_ref().unwrap();
        if left.node_type == OPERATOR_NODE
        {
            let left_precedence = get_precedence(&left.value);
            //If precedence is lower in the left tree, or both are equally highest precedent (power)
            if left_precedence < precedence  ||
                left_precedence + precedence == 6
            {
                parenthesis_left = true;
            }
        }

        let right = self.right.as_ref().unwrap();
        if right.node_type == OPERATOR_NODE
        {
            let right_precedence = get_precedence(&right.value);
            //If precedence is lower in the right tree, or both are equal and no of highest precedent (power)
            if right_precedence < precedence ||
                (right_precedence == precedence && precedence != 3)
            {
                parenthesis_right = true;
            }

        }

        if left.node_type == OPERATOR_NODE
        {
            if parenthesis_left
            {
                s.push('(');
                s.push_str(&left.to_string_operator_node());
                s.push(')');
            }
            else { s.push_str(&left.to_string_operator_node()) }
        }
        else { s.push_str(&left.to_string_number_node()) }

        s.push_str(&self.value);

        if right.node_type == OPERATOR_NODE
        {
            if parenthesis_right
            {
                s.push('(');
                s.push_str(&right.to_string_operator_node());
                s.push(')');
            }
            else { s.push_str(&right.to_string_operator_node()) }
        }
        else { s.push_str(&right.to_string_number_node()) }

        s
    }
}

fn get_precedence(operator_value: &String) -> u8
{
    match operator_value
    {
        _ if operator_value == "+" || operator_value == "-" => 1,
        _ if operator_value == "*" || operator_value == "/" => 2,
        _ if operator_value == "^" => 3,
        _ => panic!("No precedence value could be found for operator '{}'", operator_value)
    }
}
