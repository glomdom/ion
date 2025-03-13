use std::{any::Any, collections::HashSet, fmt};

#[derive(Clone)]
pub struct Location {
    pub file_name: String,
    pub line: usize,
    pub column: usize,
    pub position: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file_name, self.line, self.column)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file_name, self.line, self.column)
    }
}

#[derive(Clone)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    pub fn len(&self) -> usize {
        self.end.position - self.start.position
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

pub struct Token {
    pub kind: SyntaxKind,
    pub span: Span,
    pub text: String,
    pub value: Option<Box<dyn Any>>,
}

impl Token {
    pub fn downcast_value<T: 'static>(&self) -> Option<&T> {
        self.value.as_ref().unwrap().downcast_ref::<T>()
    }
}

pub struct TokenStream {
    tokens: Vec<Token>,
    position: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream {
            tokens,
            position: 0,
        }
    }

    pub fn consume(&mut self, kind: SyntaxKind) -> () {
        if self.is_finished() {
            panic!("Expected {:?}, got EOF", kind);
        }

        let token = self.advance();
        if token.kind == kind {
            return;
        }

        panic!("Expected {:?}, got {:?}", kind, token.kind);
    }

    pub fn match_kind(&mut self, kind: SyntaxKind) -> bool {
        if self.check_kind(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn check_set(&mut self, kinds: Vec<SyntaxKind>) -> bool {
        self.check_set_at_offset(kinds, 0)
    }

    pub fn check_set_at_offset(&mut self, kinds: Vec<SyntaxKind>, offset: usize) -> bool {
        kinds
            .iter()
            .any(|kind| self.check_kind_at_offset(*kind, offset))
    }

    pub fn check_kind(&mut self, kind: SyntaxKind) -> bool {
        self.check_kind_at_offset(kind, 0)
    }

    pub fn check_kind_at_offset(&mut self, kind: SyntaxKind, offset: usize) -> bool {
        !self.is_finished() && self.peek(offset).kind == kind
    }

    pub fn current(&self) -> &Token {
        self.peek(0)
    }

    pub fn peek(&self, offset: usize) -> &Token {
        &self.tokens[self.position + offset]
    }

    pub fn peek_previous(&self, offset: usize) -> &Token {
        &self.tokens[self.position - offset]
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished_at_offset(0)
    }

    pub fn is_finished_at_offset(&self, offset: usize) -> bool {
        self.position + offset >= self.tokens.len()
    }

    pub fn advance(&mut self) -> &Token {
        self.position += 1;
        self.peek_previous(1)
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("Token");

        dbg.field("kind", &self.kind)
            .field("span", &self.span)
            .field("text", &self.text);

        if let Some(v) = &self.value {
            if let Some(s) = v.downcast_ref::<String>() {
                dbg.field("value", &Some(s)); // Pass Some(s) directly
            } else if let Some(s) = v.downcast_ref::<bool>() {
                dbg.field("value", &Some(s));
            } else if let Some(s) = v.downcast_ref::<f64>() {
                dbg.field("value", &Some(s));
            } else {
                dbg.field("value", &"Unknown Type");
            }
        } else {
            dbg.field("value", &None::<String>); // Explicitly show None
        }

        dbg.finish()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SyntaxKind {
    Plus, // operators
    Minus,
    Star,
    Slash,
    SlashSlash,
    Percent,
    Carat,
    Tilde,
    Ampersand,
    Pipe,
    AmpersandAmpersand,
    PipePipe,
    Bang,
    LT,
    LTE,
    GT,
    GTE,
    Equals,
    EqualsEquals,
    BangEquals,
    Colon,

    Identifier,
    LetKeyword,
    FnKeyword,

    StringLiteral, // literals
    IntLiteral,
    FloatLiteral,
    BoolLiteral,
    NullLiteral,
}
