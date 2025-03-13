#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use crate::{
        lexer::syntax::{SyntaxKind, Token},
        source::SourceFile,
    };

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    #[test]
    #[should_panic(expected = "unexpected character")]
    fn panics_when_unexpected_character() -> () {
        tokenize("`");
    }

    #[test]
    #[should_panic(expected = "unterminated string literal")]
    fn panics_when_unterminated_string_literal() -> () {
        tokenize("'abc");
    }

    #[test]
    #[should_panic(expected = "malformed number literal")]
    fn panics_when_malformed_number_literal() -> () {
        tokenize("1.2.3");
    }

    #[test]
    fn skips_whitespaces_and_newlines() -> () {
        let tokens = tokenize("+    - \n  *  ");
        let plus = &tokens[0];
        let minus = &tokens[1];
        let star = &tokens[2];

        for token in &tokens {
            assert_eq!(1, token.span.len());
        }

        assert_eq!(SyntaxKind::Plus, plus.kind);
        assert_eq!("+", plus.text);
        assert_eq!(SyntaxKind::Minus, minus.kind);
        assert_eq!("-", minus.text);
        assert_eq!(SyntaxKind::Star, star.kind);
        assert_eq!("*", star.text);
        assert_eq!(2, star.span.start.line);
        assert_eq!(2, star.span.start.column);
        assert_eq!(2, star.span.end.line);
        assert_eq!(3, star.span.end.column);
        assert_eq!(3, tokens.len());
    }

    #[test]
    fn lexes_identifiers() -> () {
        let cases = vec!["abc", "a", "_", "abc123", "_abc", "_123_abc_"];

        for input in cases {
            let tokens = tokenize(input);
            let token = tokens.first().unwrap();

            assert_eq!(SyntaxKind::Identifier, token.kind);
            assert_eq!(input, token.text);
        }
    }

    #[test]
    fn lexes_bool_literals() -> () {
        let values = vec![("true", true), ("false", false)];

        for (input, value) in values {
            let tokens = tokenize(input);
            let token = tokens.first().unwrap();

            assert_eq!(SyntaxKind::BoolLiteral, token.kind);
            assert_eq!(
                value,
                *token // fuck rust
                    .value
                    .as_ref()
                    .unwrap()
                    .downcast_ref::<bool>()
                    .unwrap()
            );
        }
    }

    #[test]
    fn lexes_number_literals() -> () {
        let values = vec![
            ("123", SyntaxKind::IntLiteral, 123.0),
            ("123.456", SyntaxKind::FloatLiteral, 123.456),
        ];

        for (input, kind, value) in values {
            let tokens = tokenize(input);
            let token = tokens.first().unwrap();

            assert_eq!(kind, token.kind);
            assert_eq!(value, *token.downcast_value::<f64>().unwrap());
        }
    }

    #[test]
    fn lexes_string_literals() -> () {
        let values = vec![("\"abc123\"", "abc123")];

        for (input, value) in values {
            let tokens = tokenize(input);
            let token = tokens.first().unwrap();

            assert_eq!(SyntaxKind::StringLiteral, token.kind);
            assert_eq!(value, *token.downcast_value::<String>().unwrap());
        }
    }

    #[test]
    fn lexes_keywords() -> () {
        assert_kinds(vec![
            ("let", SyntaxKind::LetKeyword),
            ("fn", SyntaxKind::FnKeyword),
        ]);
    }

    #[test]
    fn lexes_operators() -> () {
        assert_kinds(vec![
            ("+", SyntaxKind::Plus),
            ("-", SyntaxKind::Minus),
            ("*", SyntaxKind::Star),
            ("/", SyntaxKind::Slash),
            ("//", SyntaxKind::SlashSlash),
            ("%", SyntaxKind::Percent),
            ("^", SyntaxKind::Carat),
            ("~", SyntaxKind::Tilde),
            ("&", SyntaxKind::Ampersand),
            ("|", SyntaxKind::Pipe),
            ("&&", SyntaxKind::AmpersandAmpersand),
            ("||", SyntaxKind::PipePipe),
            ("!", SyntaxKind::Bang),
            ("<", SyntaxKind::LT),
            ("<=", SyntaxKind::LTE),
            (">", SyntaxKind::GT),
            (">=", SyntaxKind::GTE),
            ("=", SyntaxKind::Equals),
            ("==", SyntaxKind::EqualsEquals),
            ("!=", SyntaxKind::BangEquals),
            (":", SyntaxKind::Colon),
        ]);
    }

    fn assert_kinds(cases: Vec<(&str, SyntaxKind)>) -> () {
        for (input, expected_kind) in cases {
            let tokens = tokenize(input);
            let token = tokens.first().unwrap();

            assert_eq!(expected_kind, token.kind);
        }
    }

    fn tokenize(input: &str) -> Vec<Token> {
        let path = env::current_dir()
            .expect("failed to find current working directory")
            .join(
                "unit-tests.tuna".to_owned() + &COUNTER.fetch_add(1, Ordering::Relaxed).to_string(),
            );

        match fs::write(&path, input) {
            Ok(_) => (),
            Err(e) => panic!("failed to write test file: {}", e),
        }

        let source_file = SourceFile::new(path.to_str().unwrap());
        match fs::remove_file(path) {
            Ok(_) => (),
            Err(e) => panic!("failed to remove test file: {}", e),
        }

        source_file.tokenize()
    }
}
