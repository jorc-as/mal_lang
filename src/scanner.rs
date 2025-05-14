#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
    Comment,
}
pub struct Lexer<'a> {
    lexeme: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(lexeme: &'a str) -> Self {
        Self {
            lexeme,
            position: 0,
        }
    }
    fn is_at_end(&self) -> bool {
        self.lexeme.len() == self.position + 1
    }
    fn peek(&self) -> char {
        self.lexeme[self.position..].chars().next().unwrap_or('\0')
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.peek();
        self.position += 1;
        let next_c = self.peek();

        match c {
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            '-' => Some(Token::Minus),
            '+' => Some(Token::Plus),
            ';' => Some(Token::Semicolon),
            '*' => Some(Token::Star),
            '/' => {
                if next_c == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.position += 1;
                    }
                    Some(Token::Comment)
                } else {
                    Some(Token::Slash)
                }
            }
            '!' => {
                if next_c == '=' {
                    self.position += 1;
                    Some(Token::BangEqual)
                } else {
                    Some(Token::Bang)
                }
            }
            '=' => {
                if next_c == '=' {
                    self.position += 1;
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equal)
                }
            }
            '<' => {
                if next_c == '=' {
                    self.position += 1;
                    Some(Token::LessEqual)
                } else {
                    Some(Token::Less)
                }
            }
            '>' => {
                if next_c == '=' {
                    self.position += 1;
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::Greater)
                }
            }
            _ => None,
        }
    }
}
