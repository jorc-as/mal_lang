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
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.lexeme[self.position..].chars().next()?;
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
            '/' => Some(Token::Slash),
            _ => None,
        }
    }
}
