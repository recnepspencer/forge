//! Production-binary proofs for BC3101 Query source-path classification.

use super::query_source_fixture::{entry_case, run_source_case, SourceCase};

#[test]
fn cfg_gated_direct_engine_path_is_denied_by_ast_without_manifest_edge() {
    let mut case = entry_case(
        "source-engine-cfg",
        r#"#[cfg(any())]
fn hidden_bypass() {
    let _ = worth_query::facade::CanonicalQueryArtifact;
}
"#,
    );
    case.dependencies = "";
    let (ok, output) = run_source_case(case);
    assert!(!ok, "source-only engine bypass passed:\n{output}");
    assert!(output.contains("BC3101_QUERY_SOURCE_PATH"), "{output}");
    assert!(output.contains("worth_query::facade"), "{output}");
}

#[test]
fn wrong_band_audience_path_is_denied_without_manifest_edge() {
    let (ok, output) = run_source_case(SourceCase {
        label: "source-schema-decl",
        workspace: "worth-contracts",
        lane: "worth-contracts",
        prefix: "worth-schema-",
        package: "worth-schema-core",
        tier: "worth",
        band: "schema",
        domain: "core",
        dependencies: "",
        source: "fn hidden() { let _ = worth_query_decl::facade::CanonicalQueryArtifact; }\n",
        additional_sources: &[],
        manifest_suffix: "",
    });
    assert!(!ok, "wrong-band source path passed:\n{output}");
    assert!(output.contains("BC3101_QUERY_SOURCE_PATH"), "{output}");
}

#[test]
fn private_entry_audience_consumption_remains_legal() {
    let (ok, output) = run_source_case(entry_case(
        "source-private-legal",
        r#"use worth_query_decl::facade::CanonicalQueryArtifact;
fn retain(value: CanonicalQueryArtifact) { let _ = value; }
"#,
    ));
    assert!(ok, "legal private audience use failed:\n{output}");
}

#[test]
fn public_query_imports_in_private_modules_are_owned_by_bc3101() {
    for (label, source) in [
        (
            "source-private-module-public-use",
            "mod hidden { pub use worth_query::facade::CanonicalQueryArtifact; }\n",
        ),
        (
            "source-private-module-public-extern",
            "mod hidden { pub extern crate worth_query; }\n",
        ),
    ] {
        let mut case = entry_case(label, source);
        case.dependencies = "";
        let (ok, output) = run_source_case(case);
        assert!(!ok, "private-module public Query import passed:\n{output}");
        assert!(output.contains("BC3101_QUERY_SOURCE_PATH"), "{output}");
        assert!(!output.contains("BC3103_QUERY_PUBLIC_REEXPORT"), "{output}");
        assert!(output.contains("rule_contracts.query_audience"), "{output}");
    }
}

#[test]
fn integration_test_target_is_scanned_for_query_paths() {
    let mut case = entry_case("source-integration-target", "pub fn library_surface() {}\n");
    case.dependencies = "";
    case.additional_sources = &[(
        "tests/query_bypass.rs",
        "#[cfg(any())] pub use worth_query::facade::*;\n",
    )];
    let (ok, output) = run_source_case(case);
    assert!(!ok, "integration-test Query path passed:\n{output}");
    assert!(output.contains("BC3101_QUERY_SOURCE_PATH"), "{output}");
    assert!(output.contains("tests/query_bypass.rs"), "{output}");
}

#[test]
fn every_private_group_import_branch_is_scanned() {
    for (label, source) in [
        (
            "source-group-query-not-first",
            "use {std::fmt, worth_query::facade::CanonicalQueryArtifact};\n",
        ),
        (
            "source-group-direct",
            "use {worth_query::facade::CanonicalQueryArtifact, std::fmt};\n",
        ),
        (
            "source-group-nested",
            "use {std::{fmt, io}, worth_query::{facade::CanonicalQueryArtifact}};\n",
        ),
    ] {
        let mut case = entry_case(label, source);
        case.dependencies = "";
        let (ok, output) = run_source_case(case);
        assert!(!ok, "grouped Query import passed:\n{output}");
        assert!(output.contains("BC3101_QUERY_SOURCE_PATH"), "{output}");
    }
}

#[test]
fn private_extern_crate_imports_are_checked_by_bc3101() {
    let mut direct_engine = entry_case(
        "source-private-extern-engine",
        "#[cfg(any())] extern crate worth_query;\n",
    );
    direct_engine.dependencies = "";
    let (engine_ok, engine_output) = run_source_case(direct_engine);
    assert!(
        !engine_ok,
        "private engine extern crate passed:\n{engine_output}"
    );
    assert!(
        engine_output.contains("BC3101_QUERY_SOURCE_PATH"),
        "{engine_output}"
    );

    let (audience_ok, audience_output) = run_source_case(SourceCase {
        label: "source-private-extern-wrong-audience",
        workspace: "worth-contracts",
        lane: "worth-contracts",
        prefix: "worth-schema-",
        package: "worth-schema-core",
        tier: "worth",
        band: "schema",
        domain: "core",
        dependencies: "",
        source: "#[cfg(any())] extern crate worth_query_decl;\n",
        additional_sources: &[],
        manifest_suffix: "",
    });
    assert!(
        !audience_ok,
        "private audience extern crate passed:\n{audience_output}"
    );
    assert!(
        audience_output.contains("BC3101_QUERY_SOURCE_PATH"),
        "{audience_output}"
    );
}

#[test]
fn additional_target_child_modules_are_followed() {
    let mut binary = entry_case("source-binary-child", "pub fn library_surface() {}\n");
    binary.dependencies = "";
    binary.additional_sources = &[
        ("src/main.rs", "mod cli; fn main() {}\n"),
        (
            "src/cli.rs",
            "#[cfg(any())] fn bypass() { let _ = worth_query::facade::CanonicalQueryArtifact; }\n",
        ),
    ];
    let (binary_ok, binary_output) = run_source_case(binary);
    assert!(
        !binary_ok,
        "binary child-module Query path passed:\n{binary_output}"
    );
    assert!(
        binary_output.contains("BC3101_QUERY_SOURCE_PATH"),
        "{binary_output}"
    );
    assert!(binary_output.contains("src/cli.rs"), "{binary_output}");

    let mut custom_test = entry_case("source-custom-test-child", "pub fn library_surface() {}\n");
    custom_test.dependencies = "";
    custom_test.manifest_suffix =
        "\n[[test]]\nname = \"custom_query_test\"\npath = \"qa/main.rs\"\n";
    custom_test.additional_sources = &[
        ("qa/main.rs", "mod hidden;\n"),
        (
            "qa/hidden.rs",
            "#[cfg(any())] fn bypass() { let _ = worth_query::facade::CanonicalQueryArtifact; }\n",
        ),
    ];
    let (custom_ok, custom_output) = run_source_case(custom_test);
    assert!(
        !custom_ok,
        "custom test child-module Query path passed:\n{custom_output}"
    );
    assert!(
        custom_output.contains("BC3101_QUERY_SOURCE_PATH"),
        "{custom_output}"
    );
    assert!(custom_output.contains("qa/hidden.rs"), "{custom_output}");
}
