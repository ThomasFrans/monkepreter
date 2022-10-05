use std::fmt::Display;


#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    Eof,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    Neq,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Comma,
    Semicolon,

    Ident(String),
    Int(String),

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Token::Illegal => "Illegal",
            Token::Eof => "Eof",
            Token::Assign => "Assign",
            Token::Plus => "+",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::Comma => ",",
            Token::Semicolon => ";",
            Token::Ident(_) => "Ident",
            Token::Int(_) => "Int",
            Token::Function => "Function",
            Token::Let => "Let",
            Token::Minus => "-",
            Token::Bang => "!",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::Lt => "<",
            Token::Gt => ">",
            Token::True => "true",
            Token::False => "false",
            Token::If => "if",
            Token::Else => "else",
            Token::Return => "return",
            Token::Eq => "==",
            Token::Neq => "!=",
        })
    }
}
