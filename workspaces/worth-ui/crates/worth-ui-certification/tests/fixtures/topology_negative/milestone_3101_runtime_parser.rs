pub fn tokenize_authored_source(source_text: &str) -> Vec<char> {
    source_text.chars().filter(|character| !character.is_whitespace()).collect()
}
