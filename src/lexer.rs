use crate::token::Token;



// Pretends to support unicode, but the filereading doesn't!
pub struct Lexer {
    input: String,
    /// Always goes forward, aint looking back, no never.
    read_position: usize,
    /// Stays at the beginning of a lexeme.
    hold_position: usize,
    char: char,
}

impl Lexer {
    pub fn next_token(&mut self) -> Token {

        self.skip_whitespace();

        let token = match self.char {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            },
            '+' => Token::Plus,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Neq
                } else {
                    Token::Bang
                }
            },
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => Token::Lt,
            '>' => Token::Gt,
            '\x00'  => Token::Eof,
            _ => {
                // This handles any case where the lexeme isn't a single char.
                if is_letter(self.char) {
                    let value = self.read_identifier();
                    return lookup_ident(value)
                } else if is_digit(self.char) {
                    let value = self.read_number();
                    return Token::Int(value)
                } else {
                    Token::Illegal
                }
            },
        };
        self.read_char();
        token
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = 0 as char;
        } else {
            self.char = self.input.as_bytes()[self.read_position] as char;
        }
        self.hold_position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return 0 as char;
        } else {
            // TODO: Might be a cleaner way...
            return self.input.get(self.read_position..).unwrap().chars().next().unwrap();
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.hold_position;
    
        while is_letter(self.char) {
            self.read_char();
        }

        self.input.get(position..self.hold_position).unwrap().to_string()
    }

    pub fn read_number(&mut self) -> String {
        let position = self.hold_position;

        while is_digit(self.char) {
            self.read_char();
        }

        self.input.get(position..self.hold_position).unwrap().to_string()
    }

    pub fn skip_whitespace(&mut self) {
        while self.char == ' ' || self.char == '\t' || self.char == '\n' || self.char == '\r' {
            self.read_char();
        }
    }
}

/// Check if a string represents a keyword and if so, return a corresponding Token.
pub fn lookup_ident(ident: String) -> Token {
    match ident.as_ref() {
        "fn" => Token::Function,
        "let" => Token::Let,
        "return" => Token::Return,
        "if" => Token::If,
        "else" => Token::Else,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Ident(ident),
    }
}

/// Test whether the Unicode character is an ASCII letter or the '_' character.
pub fn is_letter(symbol: char) -> bool {
    'a' <= symbol && symbol <= 'z' || 'A' <= symbol && symbol <= 'Z' || symbol == '_'
}

pub fn is_digit(symbol: char) -> bool {
    '0' <= symbol && symbol <= '9'
}

impl<T: ToString> From<T> for Lexer {
    fn from(input: T) -> Self {
        let mut lexer = Self {
            input: input.to_string(),
            read_position: 0,
            hold_position: 0,
            char: 0 as char,
        };
        lexer.read_char();
        lexer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lexing() {
        let mut lexer = Lexer::from("+=(){},;");
        let expected_tokens = [
            Token::Plus,
            Token::Assign,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        for token in expected_tokens {
            assert_eq!(lexer.next_token(), token);
        }
    }

    #[test]
    fn test_lexing() {
        let mut lexer = Lexer::from(
            "
            let five = 5;
            let ten = 10;
            
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);

            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            "
        );

        let expected_tokens = [
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
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
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::Int("5".to_string()),
            Token::Lt,
            Token::Int("10".to_string()),
            Token::Gt,
            Token::Int("5".to_string()),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int("5".to_string()),
            Token::Lt,
            Token::Int("10".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int("10".to_string()),
            Token::Eq,
            Token::Int("10".to_string()),
            Token::Semicolon,
            Token::Int("10".to_string()),
            Token::Neq,
            Token::Int("9".to_string()),
            Token::Semicolon,
            Token::Eof,
        ];

        for token in expected_tokens {
            let next_token = lexer.next_token();
            println!("___________________________________");
            println!("expected: {:?}\nfound: {:?}", token, next_token);
            assert_eq!(next_token, token);
        }
    }
}
