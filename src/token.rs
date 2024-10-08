#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(i64),
    Assign,
    Eq,
    NotEq,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Comma,
    Semicolon,
    Bang,
    Lt,
    Gt,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Func,
    Let,
    If,
    Else,
    True,
    False,
    Return,
}
