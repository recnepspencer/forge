use std::fs;
use std::path::PathBuf;

#[test]
fn raw_json_cannot_enter_canonical_basis_construction() {
    assert_sources_do_not_expose_forbidden_constructor_inputs();
    assert_compile_fail_proofs_cover_every_native_constructor_lane();
}

fn assert_sources_do_not_expose_forbidden_constructor_inputs() {
    let construction_source = aspect_native_source("canonical_basis_construction.rs");
    let construction_contents = fs::read_to_string(&construction_source).unwrap();
    for forbidden_input in [
        "serde_json",
        "Serialize",
        "from_json",
        "from_terminal_projection",
        "StoreTerminalProjectionText",
        "String",
    ] {
        assert!(
            !construction_contents.contains(forbidden_input),
            "{} exposes forbidden canonical basis input {forbidden_input}",
            construction_source.display()
        );
    }

    let entry_source = aspect_native_source("canonical_basis_entries.rs");
    let entry_contents = fs::read_to_string(&entry_source).unwrap();
    for forbidden_input in [
        "serde_json",
        "Serialize",
        "from_json",
        "from_terminal_projection",
        "StoreTerminalProjectionText",
    ] {
        assert!(
            !entry_contents.contains(forbidden_input),
            "{} exposes forbidden canonical basis input {forbidden_input}",
            entry_source.display()
        );
    }
}

fn assert_compile_fail_proofs_cover_every_native_constructor_lane() {
    let compile_fail_proofs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("certification_compile_fail_proofs.md"),
    )
    .unwrap();

    for required_proof in [
        "Raw JSON cannot satisfy page-header native Store canonical basis construction",
        "Raw JSON cannot satisfy aspect-boundary native Store canonical basis construction",
        "Raw JSON cannot satisfy aspect-patch native Store canonical basis construction",
        "String text cannot satisfy page-header native Store canonical basis construction",
        "String text cannot satisfy aspect-boundary native Store canonical basis construction",
        "String text cannot satisfy aspect-patch native Store canonical basis construction",
        "Terminal projection text cannot satisfy page-header native Store canonical basis construction",
        "Terminal projection text cannot satisfy aspect-boundary native Store canonical basis construction",
        "Terminal projection text cannot satisfy aspect-patch native Store canonical basis construction",
        "Generic Serialize inputs cannot satisfy page-header native Store canonical basis construction",
        "Generic Serialize inputs cannot satisfy aspect-boundary native Store canonical basis construction",
        "Generic Serialize inputs cannot satisfy aspect-patch native Store canonical basis construction",
    ] {
        assert!(
            compile_fail_proofs.contains(required_proof),
            "missing compile-fail proof: {required_proof}"
        );
    }
}

fn aspect_native_source(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("worth-store-aspect-native")
        .join("src")
        .join(file)
}
