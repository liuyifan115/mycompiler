use std::fmt::Debug;

pub mod preprocesser;
pub mod scanner;

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i32),
    Boolean(bool),
    Identifier(String),
    Keyword(Keyword),
    Operator(Operator),
    Seperator(Seperator),
}

#[derive(Debug, PartialEq)]
enum Keyword {
	Int,
	Bool,
    True,
    False,
    If,
    Then,
    Else,
    While,
    Do,
    Read,
    Write
}

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    IntegerAssign,
    BooleanAssign,
    GreaterThan,
    GreaterEqualThan,
    LessThan,
    LessEqualThan,
    Equal,
    NotEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
}


#[derive(Debug, PartialEq)]
enum Seperator {
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftParenthesis,
    RightParenthesis,
    Semicolon,
    Comma
}
