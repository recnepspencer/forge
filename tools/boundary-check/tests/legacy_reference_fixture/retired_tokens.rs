//! Retired-name token construction without contiguous spellings in source.

/// Build the retired underscore fragment without storing it contiguously.
pub fn retired_underscore_fragment() -> String {
    let mut fragment = String::from("forge");
    fragment.push('_');
    fragment
}

/// Build the retired hyphen fragment without storing it contiguously.
pub fn retired_hyphen_fragment() -> String {
    let mut fragment = String::from("forge");
    fragment.push('-');
    fragment
}

/// Build a retired package-style underscore spelling.
pub fn retired_query_token() -> String {
    let mut token = retired_underscore_fragment();
    token.push_str("query");
    token
}

/// Build a retired package-style hyphen spelling.
pub fn retired_hyphen_query_token() -> String {
    let mut token = retired_hyphen_fragment();
    token.push_str("query");
    token
}
