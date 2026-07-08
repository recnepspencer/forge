use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tools directory")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn naming_doc_declares_canonical_machine_constitution() {
    let root = workspace_root();
    let naming = fs::read_to_string(root.join("_docs/worthy/NAMING.md")).expect("read naming doc");
    assert!(
        naming.contains("Canonical machine constitution: `tools/boundary-check/config/road1.toml`")
    );
    assert!(naming.contains("`worth-entry-adoption`"));
    assert!(naming.contains("`worth-derived-publication`"));
}

#[test]
fn boundaries_doc_routes_match_machine_contract_nouns() {
    let root = workspace_root();
    let boundaries =
        fs::read_to_string(root.join("_docs/worthy/BOUNDARIES.md")).expect("read boundaries doc");
    assert!(boundaries.contains("worth-entry-adoption"));
    assert!(boundaries.contains("worth-derived-publication"));
    assert!(boundaries.contains("worthy-derived-brep"));
}

#[test]
fn deferred_follow_on_surface_is_named_not_smuggled() {
    let root = workspace_root();
    assert!(!root
        .join("cad/workspaces/worth-entry/crates/worth-entry-adoption")
        .exists());
    assert!(!root
        .join("cad/workspaces/worth-derived/crates/worth-derived-publication")
        .exists());
    assert!(!root
        .join("cad/workspaces/worthy-derived/crates/worthy-derived-brep")
        .exists());
}
