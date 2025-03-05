use prefix::{claculate_prefix, infix_to_prefix};
use token_praser::Token;

mod token_praser;
mod prefix;


fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("{:?}", token_praser::prase_token(&input));
    display_token_list(&infix_to_prefix(&token_praser::prase_token(&input)));
    println!("{}", claculate_prefix(&infix_to_prefix(&token_praser::prase_token(&input))))
}

fn display_token_list(tokens: &Vec<Token>) {
    for token in tokens {
        match token {
            Token::Operand(operand) => print!("{} ", operand),
            Token::Operator(operator, _) => print!("{} ", operator),
            Token::LeftBracket => print!("( "),
            Token::RightBracket => print!(") "),
        }
    }
    println!("");
}
