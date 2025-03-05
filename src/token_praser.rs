#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(char, i32),
    Operand(i64),
    LeftBracket,
    RightBracket,
}

pub fn prase_token(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for ch in input.chars() {
        match ch {
            ' ' => continue,
            '(' => tokens.push(Token::LeftBracket),
            ')' => tokens.push(Token::RightBracket),
            '+' | '-' => tokens.push(Token::Operator(ch, 1)),
            '*' | '/' => tokens.push(Token::Operator(ch, 2)),
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                if !tokens.is_empty() {
                    match tokens.last_mut().unwrap() {
                        Token::Operand(num) => {
                            let tmp = *num;
                            tokens.pop();
                            tokens.push(Token::Operand(tmp * 10 + ch.to_digit(10).unwrap() as i64));
                        }
                        _ => tokens.push(Token::Operand(ch.to_digit(10).unwrap() as i64))
                    };
                } else {
                    tokens.push(Token::Operand(ch.to_digit(10).unwrap() as i64));
                }
            }
			'\n' => continue,
            _ => panic!("error"),
        }

    }
    tokens
}