use crate::token_praser::Token;

pub fn infix_to_prefix(infix: &Vec<Token>) -> Vec<Token> {
    let mut stack: Vec<Token> = Vec::new();
    let mut prefix: Vec<Token> = Vec::new();
    
    for token in infix.iter() {
        match token {
            Token::Operand(operand) => {
                prefix.push(Token::Operand(*operand));
            }
            Token::LeftBracket => {
                stack.push(Token::LeftBracket);
            
            }
            Token::RightBracket => {
                while let Some(top) = stack.pop() {
                    if top == Token::LeftBracket {
                        break;
                    }
                    prefix.push(top);
                }
            }
            Token::Operator(operator, prev) => {
                if stack.is_empty() {
                    stack.push(Token::Operator(*operator, *prev));
                } else {
                    loop {
                        if *stack.last().unwrap() == Token::LeftBracket || match stack.last().unwrap() {
                            Token::Operator(_, prev2 ) => prev > prev2,
                            _ => false,
                        } {
                            stack.push(Token::Operator(*operator, *prev));
                            break;
                        } else {
                            prefix.push(stack.pop().unwrap());
                        }
                    }
                }
            }
        }            
    }

    for token in stack.into_iter() {
        prefix.push(token);
    }

    prefix
}

pub fn claculate_prefix(tokens: &Vec<Token>) -> i64 {
	let mut stack: Vec<i64> = vec![];
	for token in tokens.iter() {
		match token {
			Token::Operand(operand) => stack.push(*operand),
			Token::Operator(operator, _) => {
				let op2 = stack.pop().unwrap();
				let op1 = stack.pop().unwrap();
				match operator {
					'+' => stack.push(op1 + op2),
					'-' => stack.push(op1 - op2),
					'*' => stack.push(op1 * op2),
					'/' => stack.push(op1 / op2),
					_ => panic!("error"),
				}
			}
			_ => panic!("error"),
		}	
	}

	stack.pop().unwrap()
}