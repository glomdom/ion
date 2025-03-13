use std::collections::HashMap;

use super::syntax::SyntaxKind;

/// Given a keyword string, returns the corresponding keyword syntax kind, or None if it was not found
pub fn get_keyword_kind(keyword: &str) -> Option<SyntaxKind> {
    let (map, _) = get_keyword_maps();
    map.get(keyword).copied()
}

/// Given a SyntaxKind, returns the corresponding keyword lexeme, or None if it was not found
pub fn get_keyword_lexeme(kind: &SyntaxKind) -> Option<&str> {
    let (_, reverse_map) = get_keyword_maps();
    reverse_map.get(kind).copied()
}

fn get_keyword_maps<'a>() -> (HashMap<&'a str, SyntaxKind>, HashMap<SyntaxKind, &'a str>) {
    let mut map = HashMap::new();
    let mut reverse_map = HashMap::new();
    map.insert("let", SyntaxKind::LetKeyword);
    map.insert("fn", SyntaxKind::FnKeyword);

    for (key, value) in map.iter() {
        reverse_map.insert(value.to_owned(), key.to_owned());
    }

    (map, reverse_map)
}
