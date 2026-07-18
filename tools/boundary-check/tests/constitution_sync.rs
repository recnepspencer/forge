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
    let naming = fs::read_to_string(root.join("cad/docs/worthy-foundations/NAMING.md"))
        .expect("read naming doc");
    assert!(
        naming.contains("Canonical machine constitution: `tools/boundary-check/config/road1.toml`")
    );
    assert!(naming.contains("`worth-entry-adoption`"));
    assert!(naming.contains("`worth-derived-publication`"));
}

#[test]
fn naming_doc_and_config_agree_on_query_audience_framework_family() {
    let root = workspace_root();
    let naming = fs::read_to_string(root.join("cad/docs/worthy-foundations/NAMING.md"))
        .expect("read naming doc");
    let config_text = fs::read_to_string(root.join("tools/boundary-check/config/road1.toml"))
        .expect("read road1.toml");
    let config: toml::Value = toml::from_str(&config_text).expect("parse road1.toml");
    let query_audience = config
        .get("rule_contracts")
        .and_then(|rules| rules.get("query_audience"))
        .expect("rule_contracts.query_audience");

    assert_eq!(
        query_audience
            .get("engine_package")
            .and_then(|value| value.as_str()),
        Some("worth-query")
    );
    let audiences = query_audience
        .get("audiences")
        .and_then(|value| value.as_array())
        .expect("audiences array");
    assert_eq!(audiences.len(), 3);

    let packages: Vec<&str> = audiences
        .iter()
        .filter_map(|row| row.get("package").and_then(|value| value.as_str()))
        .collect();
    assert_eq!(
        packages,
        ["worth-query-decl", "worth-query-host", "worth-query-replay"]
    );

    for package in [
        "worth-query",
        "worth-query-decl",
        "worth-query-host",
        "worth-query-replay",
    ] {
        assert!(
            naming.contains(&format!("`{package}`")),
            "NAMING.md must name framework package {package}"
        );
    }

    assert!(
        naming.contains("Framework-family exception: Query audience facades")
            || naming.contains("framework-family exception"),
        "NAMING.md must declare the Query framework-family exception"
    );
    assert!(
        !config_text.contains("query_host_bands"),
        "retired query_host_bands must not remain in road1.toml"
    );
    assert!(
        config_text
            .replace("\r\n", "\n")
            .contains("[rule_contracts.query_audience]\nengine_package = \"worth-query\""),
        "the Query audience contract must name the canonical engine package"
    );
}

#[test]
fn boundaries_doc_routes_match_machine_contract_nouns() {
    let root = workspace_root();
    let boundaries = fs::read_to_string(root.join("cad/docs/worthy-foundations/BOUNDARIES.md"))
        .expect("read boundaries doc");
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

#[test]
fn worth_proof_law_substrate_row_matches_canonical_naming_sets() {
    let root = workspace_root();
    let config_text = fs::read_to_string(root.join("tools/boundary-check/config/road1.toml"))
        .expect("read road1.toml");
    let config: toml::Value = toml::from_str(&config_text).expect("parse road1.toml");

    let bands = config
        .get("naming")
        .and_then(|naming| naming.get("bands"))
        .and_then(|bands| bands.as_array())
        .expect("naming.bands")
        .iter()
        .filter_map(|value| value.as_str())
        .collect::<Vec<_>>();

    let substrates = config
        .get("law_substrates")
        .and_then(|value| value.as_array())
        .expect("law_substrates array");
    assert!(
        !substrates.is_empty(),
        "law_substrates must record at least worth-proof"
    );

    let worth_proof = substrates
        .iter()
        .find(|row| row.get("package").and_then(|value| value.as_str()) == Some("worth-proof"))
        .expect("worth-proof law substrate row");

    let tiers = worth_proof
        .get("tiers")
        .and_then(|value| value.as_array())
        .expect("tiers")
        .iter()
        .filter_map(|value| value.as_str())
        .collect::<Vec<_>>();
    assert_eq!(tiers, ["worth", "worthy"]);

    let substrate_bands = worth_proof
        .get("bands")
        .and_then(|value| value.as_array())
        .expect("bands")
        .iter()
        .filter_map(|value| value.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        substrate_bands, bands,
        "worth-proof bands must equal naming.bands exactly"
    );
}
