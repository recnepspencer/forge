use std::collections::BTreeSet;

pub fn markdown_headings(markdown: &str) -> BTreeSet<String> {
    markdown
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .map(str::trim)
        .map(ToOwned::to_owned)
        .collect()
}
