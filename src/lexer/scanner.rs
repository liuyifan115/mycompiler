//! # 扫描器
//! 使用有限状态机，将字符串转换为Token
//! 
//! ## 状态转换表：
//! 状态1: 初始状态
//!
use super::{Keyword, Operator, Seperator, Token};

enum ScannerState {
    /// 初始状态
    Init,
    /// 读入int类型数字
    Integer,

    /// 读入单词（标识符/关键字）
    Word,

    /// 读入长符号
    /// 长符号有：= > < ! | & :
    LongSymbol,

}


pub fn scanner(input: Vec<(String, usize)>) -> Result<Vec<Token>, ()> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut state = ScannerState::Init;
    let mut scanner_buf = String::new();

    for line in input.iter() {
        let mut line_iter = line.0.chars().peekable();
        while let Some(ch) = line_iter.next() {
            match state {
                ScannerState::Init => {
                    match ch {
                        'a'..='z' | 'A'..='Z' | '_' => {
                            scanner_buf.push(ch);
                            state = ScannerState::Word;
                        }
                        '0'..='9' => {
                            scanner_buf.push(ch);
                            state = ScannerState::Integer;
                        }
                        '{' | '}' | '(' | ')' | ';' | ',' | '+' | '*' | '/' | '-' => {
                            tokens.push(symbol_to_token(ch)?);
                            state = ScannerState::Init;
                        }
                        '=' | '>' | '<' | '!' | '|' | '&' | ':' => {
                            scanner_buf.push(ch);
                            state = ScannerState::LongSymbol;
                        }
                        _ => {}
                    }
                }
                ScannerState::Word => {
                    match ch {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            scanner_buf.push(ch);
                        }
                        ' ' => {
                            tokens.push(word_to_token(&scanner_buf)?);
                            scanner_buf.clear();
                            state = ScannerState::Init;
                        }
                        '{' | '}' | '(' | ')' | ';' | ',' | '+' | '*' | '/' | '-' => {
                            tokens.push(word_to_token(&scanner_buf)?);
                            scanner_buf.clear();
                            tokens.push(symbol_to_token(ch)?);
                            state = ScannerState::Init;
                        }
                        '=' | '>' | '<' | '!' | '|' | '&' | ':' => {
                            tokens.push(word_to_token(&scanner_buf)?);
                            scanner_buf.clear();
                            scanner_buf.push(ch);
                            state = ScannerState::LongSymbol;
                        }
                        _ => {
                            panic!("1你不好😡");
                        }
                    }
                }
                ScannerState::LongSymbol => {
                    match ch {
                        '=' | '|' | '&' => {
                            scanner_buf.push(ch);
                            tokens.push(long_symbol_to_token(&scanner_buf)?);
                            scanner_buf.clear();
                            state = ScannerState::Init;
                        }
                        '{' | '}' | '(' | ')' | ';' | ',' | '+' | '*' | '/' | '-' | '>' | '<' | '!' | ':' => {
                            tokens.push(symbol_to_token(scanner_buf.chars().last().unwrap())?);
                            tokens.push(symbol_to_token(ch)?);
                            scanner_buf.clear();
                            state = ScannerState::Init;
                        }
                        ' ' => {
                            tokens.push(symbol_to_token(scanner_buf.chars().last().unwrap())?);
                            scanner_buf.clear();
                            state = ScannerState::Init;
                        }
                        'a'..='z' | 'A'..='Z' | '_' => {
                            tokens.push(symbol_to_token(scanner_buf.chars().last().unwrap())?);
                            scanner_buf.clear();
                            scanner_buf.push(ch);
                            state = ScannerState::Word;
                        }
                        '0'..='9' => {
                            tokens.push(symbol_to_token(scanner_buf.chars().last().unwrap())?);
                            scanner_buf.clear();
                            scanner_buf.push(ch);
                            state = ScannerState::Integer;
                        }
                        _ => {
                            panic!("太坏了");
                        }
                    }
                }
                ScannerState::Integer => {
                    match ch {
                        '0'..='9' => {
                            scanner_buf.push(ch);
                        }
                        ' ' => {
                            tokens.push(Token::Integer(scanner_buf.parse::<i32>().unwrap()));
                            scanner_buf.clear();
                            state = ScannerState::Init;
                        }
                        '{' | '}' | '(' | ')' | ';' | ',' | '+' | '*' | '/' | '-' => {
                            tokens.push(Token::Integer(scanner_buf.parse::<i32>().unwrap()));
                            scanner_buf.clear();
                            tokens.push(symbol_to_token(ch)?);
                            state = ScannerState::Init;
                        }
                        '=' | '>' | '<' | '!' | '|' | '&' | ':' => {
                            tokens.push(Token::Integer(scanner_buf.parse::<i32>().unwrap()));
                            scanner_buf.clear();
                            scanner_buf.push(ch);
                            state = ScannerState::LongSymbol;
                        }
                        _ => {
                            panic!("1你不好😡");
                        }
                    }
                }
            }
        }
    }
    Ok(tokens)
}

#[inline]
fn word_to_token(word: &str) -> Result<Token, ()> {
    match word {
        "int" => Ok(Token::Keyword(Keyword::Int)),
        "bool" => Ok(Token::Keyword(Keyword::Bool)),
        "true" => Ok(Token::Keyword(Keyword::True)),
        "false" => Ok(Token::Keyword(Keyword::False)),
        "if" => Ok(Token::Keyword(Keyword::If)),
        "then" => Ok(Token::Keyword(Keyword::Then)),
        "else" => Ok(Token::Keyword(Keyword::Else)),
        "while" => Ok(Token::Keyword(Keyword::While)),
        "do" => Ok(Token::Keyword(Keyword::Do)),
        "read" => Ok(Token::Keyword(Keyword::Read)),
        "write" => Ok(Token::Keyword(Keyword::Write)),
        _ => Ok(Token::Identifier(word.to_string()))
    }
}

#[inline]
fn symbol_to_token(separator: char) -> Result<Token, ()> {
    match separator {
        '{' => Ok(Token::Seperator(Seperator::LeftCurlyBracket)),
        '}' => Ok(Token::Seperator(Seperator::RightCurlyBracket)),
        ';' => Ok(Token::Seperator(Seperator::Semicolon)),
        '(' => Ok(Token::Seperator(Seperator::LeftParenthesis)),
        ')' => Ok(Token::Seperator(Seperator::RightParenthesis)),
        ',' => Ok(Token::Seperator(Seperator::Comma)),

        '+' => Ok(Token::Operator(Operator::Plus)),
        '-' => Ok(Token::Operator(Operator::Minus)),
        '*' => Ok(Token::Operator(Operator::Multiply)),
        '/' => Ok(Token::Operator(Operator::Divide)),
        
        '=' => Ok(Token::Operator(Operator::IntegerAssign)),
        '>' => Ok(Token::Operator(Operator::GreaterThan)),
        '<' => Ok(Token::Operator(Operator::LessThan)),
        '!' => Ok(Token::Operator(Operator::LogicalNot)),
        
        _ => panic!("你好👋")
    }
}

#[inline]
fn long_symbol_to_token(special_symbol: &str) -> Result<Token, ()> {
    match special_symbol {
        "==" => Ok(Token::Operator(Operator::Equal)),
        ":=" => Ok(Token::Operator(Operator::BooleanAssign)),
        ">=" => Ok(Token::Operator(Operator::GreaterEqualThan)),
        "<=" => Ok(Token::Operator(Operator::LessEqualThan)),
        "!=" => Ok(Token::Operator(Operator::NotEqual)),
        "&&" => Ok(Token::Operator(Operator::LogicalAnd)),
        "||" => Ok(Token::Operator(Operator::LogicalOr)),
        _ => panic!("你好👋")
    }
}