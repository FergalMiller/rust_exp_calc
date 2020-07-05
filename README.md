**rust_exp_calc**
-

*rust_exp_calc* is a simple expression lexer, parser, and evaluator 
written in Rust. 

Expressions in *rust_exp_calc* currently support:
 - Floating point operands
 - `+`, `-`, `*`, `/`, and `^` operations
 - Built in order-of-operation recognition, with the option of 
 parenthesised expressions
 
 Users may choose what to do with the parsed expression tree. 
 *rust_exp_calc* currently supports :
 - Expression evaluation (Command: `1`)
 - In-order expression printing: (Command: `2`)
