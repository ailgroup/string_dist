struct TextSplitter;

impl TextSplitter {
    fn word_match(c: char) -> bool {
        match c {
            ' ' | ',' | '.' | '!' | '?' | ';' | '\'' | '"' | ':' | '\t' | '\n' | '(' | ')'
            | '-' => true,
            _ => false,
        }
    }
    fn word_to_tokens_match(c: char) -> bool {
        match c {
            ' ' | ',' | '.' | '(' | ')' | '-' => true,
            _ => false,
        }
    }
}
/// words takes borrowed str and splits on word_match
/// filtering out empty slots
fn words(t: &str) -> Vec<&str> {
    t.split(TextSplitter::word_match)
        .filter(|s| !s.is_empty())
        .collect()
}

/// tokens takes borrowed str and splits all
/// filtering out empty slots
fn tokens(t: &str) -> Vec<&str> {
    t.split("").filter(|s| !s.is_empty()).collect()
}

/// split_tokens_lower takes borrowed str
/// and uses custom splitter, collects, concats,
/// then lowercase each str, returning new String
fn tokens_lower(t: &str) -> String {
    // TODO make this &str
    t.split(TextSplitter::word_to_tokens_match)
        .collect::<Vec<&str>>()
        .concat()
        .split("")
        .map(|s| s.to_lowercase())
        .filter(|s| !s.is_empty()) //needed a split pads
        .collect::<Vec<String>>()
        .concat()
}
