pub mod syntax;
pub mod syntax_facts;

use std::any::Any;

use syntax::{Location, Span, SyntaxKind, Token};

use crate::source::SourceFile;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    lexeme_start_location: Location,
    position: usize,
    column: usize,
    line: usize,
}

impl Lexer {
    pub fn new(file: SourceFile) -> Lexer {
        Self {
            source: file.source,
            tokens: Vec::new(),

            lexeme_start_location: Location {
                file_name: file.relative_path,
                position: 0,
                column: 0,
                line: 1,
            },
            position: 0,
            column: 0,
            line: 1,
        }
    }

    /// Tokenizes the entire source string, and returns a reference to the tokens vector
    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.is_finished() {
            self.lex();
        }

        std::mem::take(&mut self.tokens)
    }

    /// Lexes the current character, and pushes a token into `self.tokens` accordingly
    fn lex(&mut self) -> () {
        let char = self.current_char();
        self.lexeme_start_location = self.current_location();

        self.advance();
        match char {
            '+' => self.push_token(SyntaxKind::Plus, None),
            '-' => self.push_token(SyntaxKind::Minus, None),
            '*' => self.push_token(SyntaxKind::Star, None),
            '/' => {
                let mut kind = SyntaxKind::Slash;
                if self.match_char('/') {
                    kind = SyntaxKind::SlashSlash;
                }

                self.push_token(kind, None);
            }
            '%' => self.push_token(SyntaxKind::Percent, None),
            '^' => self.push_token(SyntaxKind::Carat, None),
            '~' => self.push_token(SyntaxKind::Tilde, None),
            '&' => {
                let mut kind = SyntaxKind::Ampersand;
                if self.match_char('&') {
                    kind = SyntaxKind::AmpersandAmpersand;
                }

                self.push_token(kind, None);
            }
            '|' => {
                let mut kind = SyntaxKind::Pipe;
                if self.match_char('|') {
                    kind = SyntaxKind::PipePipe;
                }

                self.push_token(kind, None);
            }
            '!' => {
                let mut kind = SyntaxKind::Bang;
                if self.match_char('=') {
                    kind = SyntaxKind::BangEquals;
                }

                self.push_token(kind, None);
            }
            '=' => {
                let mut kind = SyntaxKind::Equals;
                if self.match_char('=') {
                    kind = SyntaxKind::EqualsEquals;
                }

                self.push_token(kind, None);
            }
            '"' | '\'' => self.read_string(),
            '<' => {
                let mut kind = SyntaxKind::LT;
                if self.match_char('=') {
                    kind = SyntaxKind::LTE;
                }

                self.push_token(kind, None);
            }
            '>' => {
                let mut kind = SyntaxKind::GT;
                if self.match_char('=') {
                    kind = SyntaxKind::GTE;
                }

                self.push_token(kind, None);
            }
            ':' => self.push_token(SyntaxKind::Colon, None),

            _ => {
                if char.is_whitespace() {
                    return self.skip_whitespace();
                }

                if char.is_alphabetic() || char == '_' {
                    return self.read_identifier_or_keyword();
                }

                if char.is_numeric() || char == '.' {
                    return self.read_number();
                }

                panic!("unexpected character '{}'", char)
            }
        }
    }

    /// Reads an identifier or keyword from the current position in the source.
    /// Advances the lexer while the current character is alphanumeric or an underscore.
    /// If the lexeme matches a keyword, it pushes a token of the corresponding keyword kind.
    /// Otherwise, it pushes a token of kind `Identifier`.
    fn read_identifier_or_keyword(&mut self) -> () {
        while !self.is_finished()
            && (self.current_char().is_alphanumeric() || self.current_char() == '_')
        {
            self.advance();
        }

        let current_lexeme = self.current_lexeme();
        if current_lexeme == "true" || current_lexeme == "false" {
            return self.push_token(
                SyntaxKind::BoolLiteral,
                Some(Box::new(current_lexeme == "true")),
            );
        }

        let keyword_kind = syntax_facts::get_keyword_kind(&current_lexeme);
        if let Some(kind) = keyword_kind {
            return self.push_token(kind, None);
        }

        self.push_token(SyntaxKind::Identifier, None);
    }

    /// Advances the lexer past any whitespace characters, updating the current location accordingly.
    /// If a newline character is encountered, the line number is incremented and the column is reset to 0.
    /// After advancing past all whitespace, the `lexeme_start_location` is updated to the current
    /// location to avoid including whitespaces in lexemes.
    fn skip_whitespace(&mut self) -> () {
        while !self.is_finished() && self.current_char().is_whitespace() {
            let char = self.current_char();
            if char == '\n' {
                self.advance_new_line();
            } else {
                self.advance();
            }
        }
        self.lexeme_start_location = self.current_location();
    }

    /// Pushes a token into `self.tokens` using the current span and current lexeme
    fn push_token(&mut self, kind: SyntaxKind, value: Option<Box<dyn Any>>) -> () {
        let token = Token {
            kind,
            span: self.current_span(),
            value,
            text: self.current_lexeme(),
        };

        self.tokens.push(token);
    }

    fn current_lexeme(&self) -> String {
        let span = self.current_span();
        self.source[span.start.position..span.end.position].to_string()
    }

    /// Returns a Span representing the lexeme start location to the current location
    fn current_span(&self) -> Span {
        let end_location = self.current_location();
        Span {
            start: self.lexeme_start_location.to_owned(),
            end: end_location,
        }
    }

    /// Returns a Location representing the current position, column, and line
    fn current_location(&self) -> Location {
        Location {
            file_name: self.lexeme_start_location.file_name.clone(),
            position: self.position,
            column: self.column,
            line: self.line,
        }
    }

    /// Returns the character at the current position
    fn current_char(&self) -> char {
        self.peek_char(0)
    }

    /// Returns the character at the given offset from the current position
    fn peek_char(&self, offset: usize) -> char {
        self.source.chars().nth(self.position + offset).unwrap()
    }

    /// Returns whether the current character matched the expected character
    ///
    /// If matched, advances the current position by 1
    fn match_char(&mut self, expected: char) -> bool {
        let is_match = !self.is_finished() && self.current_char() == expected;
        if is_match {
            self.advance();
        }

        is_match
    }

    /// Shortcut for `self.advance_multiple(1)`
    fn advance(&mut self) -> () {
        self.advance_multiple(1);
    }

    /// Advances the current position (as well as the column) by `amount`
    fn advance_multiple(&mut self, amount: usize) -> () {
        self.position += amount;
        self.column += amount;
    }

    /// Advances the current position and line, and resets the column back to 0
    fn advance_new_line(&mut self) -> () {
        self.position += 1;
        self.line += 1;
        self.column = 0;
    }

    /// Determines whether the lexer has reached the end of the source with the given offset
    ///
    /// Returns true if the current position plus the specified offset is greater than or equal
    /// to the length of the source string, indicating that the end has been reached
    fn is_finished_at_offset(&self, offset: usize) -> bool {
        self.position + offset >= self.source.len()
    }

    /// Returns true if the lexer has reached the end of the source, false otherwise
    fn is_finished(&self) -> bool {
        self.position >= self.source.len()
    }

    fn read_string(&mut self) -> () {
        while !self.is_finished() && self.current_char() != '"' {
            self.advance();
        }

        let terminated = self.match_char('"');
        if !terminated {
            panic!("unterminated string literal");
        }

        let current_lexeme = self.current_lexeme();
        let value: Option<Box<dyn Any>> =
            Some(Box::new(current_lexeme.trim_matches('"').to_owned()));

        self.push_token(SyntaxKind::StringLiteral, value);
    }

    fn read_number(&mut self) -> () {
        let mut decimal_used = false;
        while !self.is_finished()
            && (self.current_char().is_numeric() || self.current_char() == '.')
        {
            let current_char_is_decimal = self.current_char() == '.';
            if decimal_used && current_char_is_decimal {
                panic!("malformed number literal");
            }

            decimal_used |= current_char_is_decimal;
            self.advance();
        }

        let current_lexeme = self.current_lexeme();
        let value = current_lexeme.parse::<f64>().unwrap_or(0.0);
        let kind = if decimal_used {
            SyntaxKind::FloatLiteral
        } else {
            SyntaxKind::IntLiteral
        };

        self.push_token(kind, Some(Box::new(value)));
    }
}
