use self::lexer::Lexer;
use self::token::Token;

mod lexer;
mod token;

fn main() {
    let mut lexer = Lexer::from(
        "
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        "
        );

    let expected_tokens = [
        Token::Let,
        Token::Ident("five".to_string()),
        Token::Assign,
        Token::Int("5".to_string()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("10".to_string()),
        Token::Assign,
        Token::Int("10".to_string()),
        Token::Semicolon,
        Token::Let,
        Token::Ident("add".to_string()),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident("x".to_string()),
        Token::Comma,
        Token::Ident("y".to_string()),
        Token::RParen,
        Token::LBrace,
        Token::Ident("x".to_string()),
        Token::Plus,
        Token::Ident("y".to_string()),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident("result".to_string()),
        Token::Assign,
        Token::Ident("add".to_string()),
        Token::LParen,
        Token::Ident("five".to_string()),
        Token::Comma,
        Token::Ident("ten".to_string()),
        Token::RParen,
        Token::Semicolon,
        Token::Eof
            ];

    for token in expected_tokens {
        assert_eq!(lexer.next_token(), token);
    }
}
