#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Import, Class, Trf, Print, Semicolon, LeftParen, RightParen, Colon, Equal, DoubleColon, EOF,
    Identifier(String), StringLiteral(String),
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self { input: input.chars().collect(), pos: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        if self.pos >= self.input.len() { return Token::EOF; }
        let current = self.input[self.pos];

        if current == '"' { return self.read_string(); }

        match current {
            ';' => { self.pos += 1; Token::Semicolon }
            '(' => { self.pos += 1; Token::LeftParen }
            ')' => { self.pos += 1; Token::RightParen }
            '=' => { self.pos += 1; Token::Equal }
            ':' => {
                if self.pos + 1 < self.input.len() && self.input[self.pos + 1] == ':' {
                    self.pos += 2; Token::DoubleColon
                } else {
                    self.pos += 1; Token::Colon
                }
            }
            _ => {
                if current.is_alphabetic() || current == '.' || current == '_' {
                    let ident = self.read_identifier();
                    match ident.as_str() {
                        "import" => Token::Import,
                        "class" => Token::Class,
                        "trf" => Token::Trf,
                        "print" => Token::Print,
                        _ => Token::Identifier(ident),
                    }
                } else {
                    self.pos += 1;
                    self.next_token()
                }
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '.' || self.input[self.pos] == '_') {
            self.pos += 1;
        }
        self.input[start..self.pos].iter().collect()
    }

    fn read_string(&mut self) -> Token {
        self.pos += 1;
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] != '"' { self.pos += 1; }
        let content = self.input[start..self.pos].iter().collect();
        if self.pos < self.input.len() { self.pos += 1; }
        Token::StringLiteral(content)
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            if c.is_whitespace() { self.pos += 1; }
            else if c == '/' && self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '/' {
                while self.pos < self.input.len() && self.input[self.pos] != '\n' { self.pos += 1; }
            } else { break; }
        }
    }
}