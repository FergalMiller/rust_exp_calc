use std::io;
use std::io::Write;

mod exp_evaluator;

fn main() 
{
    println!("##-- Rust calculator says: Welcome! --##");
    run();
    println!("##-- Rust calculator says: Goodbye! --##")
}

fn run()
{
    loop
    {
        print!("\nEnter your expression: ");
        io::stdout().flush().expect("Failed to flush!");
        let input = get_user_input();

        match input
        {
            _ if input == String::from("quit") => break,
            _ => println!("User entered: {}", input)
        }

        let result_node = exp_evaluator::parser::parser::parse(&input);
        println!("result node token: {}", result_node.evaluate())
    }
}

fn get_user_input() -> String
{
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read that input :( Try again?");

    return String::from(input.trim());
}
