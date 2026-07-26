pub fn decode_authored_units(source: &str) -> Vec<&str> {
    source
        .split(|character: char| character.is_whitespace() || "{}();,".contains(character))
        .filter(|unit| !unit.is_empty())
        .collect()
}
