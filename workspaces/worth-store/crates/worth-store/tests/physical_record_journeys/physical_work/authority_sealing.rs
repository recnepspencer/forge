mod derived_reconciliation;
mod duplicate_runtime;
mod planning_read_route;
mod reopen_boundary;
mod semantic_boundary;

pub(super) fn assert_sources_exclude(relative_root: &str, predicate: &str, forbidden: &[&str]) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_root);
    let mut pending = vec![root];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                sources.push(path);
            }
        }
    }
    sources.sort();

    for path in sources {
        let source = std::fs::read_to_string(&path).unwrap();
        let code = without_line_comments(&source);
        for fragment in forbidden {
            assert!(
                !code.contains(fragment),
                "C5_PREDICATE:{predicate}: forbidden `{fragment}` at {}",
                path.display()
            );
        }
    }
}

fn without_line_comments(source: &str) -> String {
    let mut code = String::with_capacity(source.len());
    for line in source.lines() {
        code.push_str(line.split_once("//").map_or(line, |(before, _)| before));
        code.push('\n');
    }
    code
}
