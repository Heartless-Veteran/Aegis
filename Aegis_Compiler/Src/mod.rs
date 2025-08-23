// Token and Span are already imported via lib.rs

/// The Scribe (Lexer) turns a string of source code into a stream of tokens.
pub struct Scribe<'a> {
    input: &'a str,
    /// Current position in input (points to current char).
    position: usize,
    /// Current reading position in input (points to next char).
    read_position: usize,
    /// Current char under examination.
    ch: u8,
}

impl<'a> Scribe<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut scribe = Self { input, position: 0, read_position: 0, ch: 0 };
        scribe.read_char();
        scribe
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0; // EOF
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() { 0 } else { self.input.as_bytes()[self.read_position] }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' | b'\n' | b'\r' => self.read_char(),
                b'#' => { while self.ch != b'\n' && self.ch != 0 { self.read_char(); } }
                _ => break,
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let start = self.position;
        let span = |end_offset: usize| Span { start, end: start + end_offset };

        let tok = match self.ch {
            b'=' => if self.peek_char() == b'>' { self.read_char(); Token::FatArrow(Span { start, end: self.position + 1 }) }
                     else if self.peek_char() == b'=' { self.read_char(); Token::Equals(Span { start, end: self.position + 1 }) }
                     else { Token::Assign(span(1)) },
            b'!' => if self.peek_char() == b'=' { self.read_char(); Token::NotEquals(Span { start, end: self.position + 1 }) } else { Token::Bang(span(1)) },
            b'>' => Token::GreaterThan(span(1)),
            b'<' => Token::LessThan(span(1)),
            b'+' => Token::Plus(span(1)),
            b'-' => Token::Minus(span(1)),
            b'*' => Token::Asterisk(span(1)),
            b'/' => Token::Slash(span(1)),
            b'.' => Token::Dot(span(1)),
            b'(' => Token::LParen(span(1)),
            b')' => Token::RParen(span(1)),
            b'{' => Token::LBrace(span(1)),
            b'}' => Token::RBrace(span(1)),
            b'[' => Token::LBracket(span(1)),
            b']' => Token::RBracket(span(1)),
            b':' => Token::Colon(span(1)),
            b',' => Token::Comma(span(1)),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => return self.read_identifier(),
            b'0'..=b'9' => return self.read_number(),
            b'"' => return self.read_string(),
            0 => Token::Eof(span(0)),
            _ => Token::Illegal(self.ch as char, span(1)),
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.ch == b'\'' { self.read_char(); }
        let end = self.position;
        let literal = &self.input[start..end];
        let span = Span { start, end };
        match literal {
            "let's" => Token::Let(span), "app" => Token::App(span), "track" => Token::Track(span),
            "when" => Token::When(span), "if" => Token::If(span), "else" => Token::Else(span),
            "true" => Token::True(span), "false" => Token::False(span), "contract" => Token::Contract(span),
            "for" => Token::For(span), "in" => Token::In(span), "is" => Token::Is(span),
            "async" => Token::Async(span), "await" => Token::Await(span), "show" => Token::Show(span),
            "nothing" => Token::Nothing(span), "return" => Token::Return(span),
            "enum" => Token::Enum(span),
            _ => Token::Identifier(literal.to_string(), span),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.ch.is_ascii_digit() { self.read_char(); }
        let end = self.position;
        Token::Number(self.input[start..end].to_string(), Span { start, end })
    }
    
    fn read_string(&mut self) -> Token {
        let start_pos = self.position;
        let start = self.position + 1;
        loop { self.read_char(); if self.ch == b'"' || self.ch == 0 { break; } }
        let end = self.position;
        let full_span = Span { start: start_pos, end: end + 1 };
        if self.ch == 0 { return Token::Illegal('"', full_span); } // Unterminated string
        self.read_char();
        Token::String(self.input[start..end].to_string(), full_span)
    }
}
