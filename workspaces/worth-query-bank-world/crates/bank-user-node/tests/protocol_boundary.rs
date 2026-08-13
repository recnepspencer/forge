#[test]
fn user_node_wire_protocol_carries_no_runtime_or_lifecycle_authority() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/protocol");
    let forbidden = [
        "worth_query",
        "WorthQueryAuthority",
        "BankRequestedEstateElevation",
        "BankApprovedEstateElevation",
        "BankEstateMandatoryReview",
        "BankCommitReceipt",
        "BankCommitRecoveryHandle",
    ];
    let mut hits = Vec::new();
    for path in rust_files(&root) {
        let text = std::fs::read_to_string(&path).expect("read user-node protocol source");
        for needle in forbidden {
            if text.contains(needle) {
                hits.push(format!("{}:{needle}", path.display()));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "user-node wire protocol must remain descriptive and authority-free: {hits:?}"
    );
}

fn rust_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files = vec![root.with_extension("rs")];
    if root.is_dir() {
        files.extend(
            std::fs::read_dir(root)
                .expect("protocol directory")
                .map(|entry| entry.expect("protocol entry").path())
                .filter(|path| path.extension().is_some_and(|extension| extension == "rs")),
        );
    }
    files
}
