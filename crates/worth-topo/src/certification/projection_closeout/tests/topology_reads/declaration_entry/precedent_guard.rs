use std::fs;
use std::path::PathBuf;

#[test]
fn declaration_entry_query_precedent_stays_topology_named() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("certification")
        .join("projection_closeout")
        .join("tests")
        .join("topology_reads")
        .join("declaration_entry");
    let forbidden = [
        "orchestrate_declaration_entry(",
        "orchestrate_declaration_entry_checked(",
        "orchestrate_declaration_entry_proof(",
    ];

    let mut violations = Vec::new();
    collect_forbidden_calls(&dir, &forbidden, &mut violations);

    assert!(
        violations.is_empty(),
        "declaration-entry certification should teach topology-named envelope lanes, not raw generic declaration-entry calls: {violations:?}"
    );
}

fn collect_forbidden_calls(path: &PathBuf, forbidden: &[&str], violations: &mut Vec<String>) {
    for entry in fs::read_dir(path).expect("declaration-entry test directory is readable") {
        let entry = entry.expect("directory entry is readable");
        let entry_path = entry.path();
        if entry.file_type().expect("file type is readable").is_dir() {
            collect_forbidden_calls(&entry_path, forbidden, violations);
            continue;
        }
        if entry_path
            .extension()
            .is_none_or(|extension| extension != "rs")
        {
            continue;
        }
        if entry_path
            .file_name()
            .is_some_and(|name| name == "precedent_guard.rs")
        {
            continue;
        }
        let text = fs::read_to_string(&entry_path).expect("test source is readable");
        for needle in forbidden {
            if text.contains(needle) {
                let relative = entry_path
                    .strip_prefix(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src"))
                    .expect("test file lives under src")
                    .display()
                    .to_string();
                violations.push(format!("{relative} contains `{needle}`"));
            }
        }
    }
}
