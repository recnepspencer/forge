use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

use super::{facade_runtime, ledger, runtime_language_ownership, source_semantics};

fn workspace_inventory() -> WorkspaceSourceInventory {
    WorkspaceSourceInventory::capture(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate parent")
            .parent()
            .expect("workspace root"),
    )
}

fn source_ledger() -> toml::Value {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("repository root");
    ledger::load(
        &repository_root.join("_docs/worth-ui/milestone-3.10.1-source-semantics-inventory.toml"),
    )
    .expect("source ledger")
}

#[test]
fn milestone_3101_rejects_a_disguised_second_parser_file() {
    let fixture = include_str!(
        "../../../tests/fixtures/topology_negative/milestone_3101_disguised_front_end.rs"
    );
    assert!(fixture.contains("decode_authored_units"));
    assert!(!fixture.contains("parser"));
    assert!(!fixture.contains("ast"));
    let structural_error = runtime_language_ownership::reject_runtime_language_owner(
        Path::new("crates/worth-ui-runtime/src/dependency/disguised_front_end.rs"),
        fixture,
    )
    .expect_err("renamed raw-text compiler should fail structural ownership audit");
    assert!(structural_error.contains("authored-text lexical work"));
    let document = source_ledger();
    let error = source_semantics::audit_with_extra_path(
        &workspace_inventory(),
        &document,
        "dependency/disguised_front_end.rs",
    )
    .expect_err("new parser file should invalidate frozen coverage");
    assert!(error.contains("files; expected"));
}

#[test]
fn milestone_3101_rejects_runtime_authored_source_tokenization() {
    let fixture =
        include_str!("../../../tests/fixtures/topology_negative/milestone_3101_runtime_parser.rs");
    let error = runtime_language_ownership::reject_runtime_language_owner(
        Path::new("crates/worth-ui-runtime/src/private/source_machine.rs"),
        fixture,
    )
    .expect_err("runtime source tokenizer should fail");
    assert!(error.contains("authored-text lexical work"));
}

#[test]
fn milestone_3101_rejects_runtime_ast_to_semantic_lowering() {
    let fixture = include_str!(
        "../../../tests/fixtures/topology_negative/milestone_3101_runtime_ast_lowerer.rs"
    );
    let error = runtime_language_ownership::reject_runtime_language_owner(
        Path::new("crates/worth-ui-runtime/src/private/meaning_bridge.rs"),
        fixture,
    )
    .expect_err("runtime AST lowering should fail");
    assert!(error.contains("syntax shape into semantic meaning"));
}

#[test]
fn milestone_3101_rejects_dsl_runtime_dependency() {
    let fixture = include_str!(
        "../../../tests/fixtures/topology_negative/milestone_3101_dsl_forbidden_dependency.toml"
    );
    let error = runtime_language_ownership::reject_dsl_manifest_dependencies(fixture)
        .expect_err("DSL runtime dependency should fail");
    assert!(error.contains("worth-ui-runtime"));
}

#[test]
fn milestone_3101_rejects_dsl_owned_runtime_authority() {
    let error = runtime_language_ownership::reject_dsl_runtime_authority_owner(
        Path::new("crates/worth-ui-dsl/src/forged_runtime.rs"),
        "pub struct WorthUiMountedApplicationPublicationAuthority;",
    )
    .expect_err("DSL-owned mounted publication authority should fail");
    assert!(error.contains("runtime authority ownership"));
    assert!(error.contains("WorthUiMountedApplicationPublicationAuthority"));
}

#[test]
fn milestone_3101_rejects_runtime_forwarding_export() {
    for source in [
        "pub use worth_ui_dsl::WorthUiDslCompiler;",
        "pub use {worth_ui_dsl::WorthUiDslCompiler};",
    ] {
        let error = runtime_language_ownership::reject_runtime_language_owner(
            Path::new("crates/worth-ui-runtime/src/facade/source_ingress.rs"),
            source,
        )
        .expect_err("runtime source forwarding export should fail");
        assert!(error.contains("forwarding export"));
    }
}

#[test]
fn milestone_3101_rejects_watcher_owned_legality() {
    let mut document = source_ledger();
    let rows = document
        .get_mut("classification")
        .and_then(toml::Value::as_array_mut)
        .expect("classification rows");
    let watcher = rows
        .iter_mut()
        .find(|row| {
            row.get("id").and_then(toml::Value::as_str) == Some("source-ingress-filesystem")
        })
        .expect("watcher row");
    watcher
        .get_mut("capabilities")
        .and_then(toml::Value::as_array_mut)
        .expect("watcher capabilities")
        .push(toml::Value::String("authored-legality".to_owned()));
    let error = source_semantics::audit(&workspace_inventory(), &document)
        .expect_err("watcher should not own authored legality");
    assert!(error.contains("filesystem transport"));
}

#[test]
fn milestone_3101_rejects_certification_authority_in_product_facade() {
    let fixture =
        include_str!("../../../tests/fixtures/topology_negative/milestone_3101_cert_export.rs");
    let error = facade_runtime::reject_certification_export("facade/app.rs", fixture)
        .expect_err("certification-only type should not be a product export");
    assert!(error.contains("certification-only authority"));
}

#[test]
fn milestone_3101_rejects_bidirectional_dsl_runtime_split() {
    let fixture = include_str!(
        "../../../tests/fixtures/topology_negative/milestone_3101_bidirectional_split.toml"
    )
    .parse::<toml::Value>()
    .expect("cycle fixture");
    let mut edges = BTreeMap::<String, BTreeSet<String>>::new();
    for edge in fixture
        .get("edge")
        .and_then(toml::Value::as_array)
        .expect("edge rows")
    {
        let source = edge
            .get("from")
            .and_then(toml::Value::as_str)
            .expect("from");
        let target = edge.get("to").and_then(toml::Value::as_str).expect("to");
        edges
            .entry(source.to_owned())
            .or_default()
            .insert(target.to_owned());
    }
    let error = facade_runtime::reject_bidirectional_edges(&edges)
        .expect_err("bidirectional split should fail");
    assert!(error.contains("bidirectional"));
}

#[test]
fn milestone_3101_rejects_a_second_runtime_semantic_spec_constructor() {
    let error = runtime_language_ownership::reject_direct_runtime_semantic_spec_construction(
        std::path::Path::new("crates/worth-ui-runtime/src/runtime/disguised_bootstrap.rs"),
        "fn make() { UiDslSemanticArtifactSpec::new(); }",
    )
    .expect_err("a second direct runtime semantic-spec constructor should fail");
    assert!(error.contains("direct DSL semantic-spec constructions"));
}
