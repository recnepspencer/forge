use super::{
    audit_consumer_orchestration_sources, WorthQueryConsumerOrchestrationErrorKind,
    WorthQueryConsumerOrchestrationPhase as Phase, WorthQueryDeclarativeSurfaceSource,
};

#[test]
fn renamed_split_helpers_cannot_hide_local_query_orchestration() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "seeded/renamed_split.rs",
        r#"
fn compile_subject(input: Input) -> Canonical {
    worth_query::facade::canonicalize_query(input)
}
fn resolve_world(canonical: Canonical) -> Admitted {
    worth_query::facade::admit_query_context(canonical)
}
fn dispatch_subject(admitted: Admitted) -> Output {
    worth_query::facade::execute_read_family(admitted)
}
pub fn serve_subject(input: Input) -> Output {
    let canonical = compile_subject(input);
    let admitted = resolve_world(canonical);
    dispatch_subject(admitted)
}
"#,
    );

    let audit = audit_consumer_orchestration_sources(&[source]).expect("fixture parses");
    let finding = audit
        .findings()
        .iter()
        .find(|finding| finding.site().function_name() == "serve_subject")
        .expect("renamed coordinator must be detected");

    assert_eq!(finding.site().path(), "seeded/renamed_split.rs");
    assert_eq!(finding.site().line(), 11);
    assert_eq!(
        finding.phases(),
        &[Phase::Canonicalize, Phase::Admit, Phase::Execute]
    );
}

#[test]
fn unrelated_domain_functions_with_phase_like_names_are_not_query_residue() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "seeded/domain_pipeline.rs",
        r#"
fn plan_experiment() {}
fn execute_experiment() {}
pub fn research() {
    plan_experiment();
    execute_experiment();
}
"#,
    );

    let audit = audit_consumer_orchestration_sources(&[source]).expect("fixture parses");

    assert!(!audit.has_local_orchestration());
}

#[test]
fn invalid_consumer_source_returns_typed_location_evidence() {
    let source =
        WorthQueryDeclarativeSurfaceSource::new("seeded/invalid_consumer.rs", "fn broken( {");

    let error = audit_consumer_orchestration_sources(&[source])
        .expect_err("invalid consumer source must fail closed");

    assert_eq!(
        error.kind(),
        WorthQueryConsumerOrchestrationErrorKind::InvalidRustSource
    );
    assert_eq!(error.source_path(), "seeded/invalid_consumer.rs");
    assert_eq!(error.line(), 1);
    assert!(error.column() > 0);
    assert!(!error.message().is_empty());
}

#[test]
fn query_orchestration_is_reported_at_the_outermost_local_entry() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "seeded/nested.rs",
        r#"
fn inner(input: Input) -> Output {
    let admitted = worth_query::facade::admit_query_context(input);
    worth_query::facade::execute_read_family(admitted)
}
pub fn outer(input: Input) -> Output { inner(input) }
"#,
    );

    let audit = audit_consumer_orchestration_sources(&[source]).expect("fixture parses");

    assert_eq!(audit.findings().len(), 1);
    assert_eq!(audit.findings()[0].site().function_name(), "outer");
}

#[test]
fn required_reference_consumer_trees_are_parseable_as_one_call_graph() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_root
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-query must remain below the workspace root");
    let mut sources = Vec::new();
    for relative_root in [
        "crates/hadwiger-research/src",
        "workspaces/worth-ui/crates/worth-ui-query-binding/src",
        "workspaces/worth-ui/crates/worth-ui-runtime/src",
    ] {
        collect_rust_sources(
            workspace_root,
            &workspace_root.join(relative_root),
            &mut sources,
        );
    }

    let audit = audit_consumer_orchestration_sources(&sources)
        .expect("required reference consumer sources must remain valid Rust syntax");

    assert!(
        sources.len() > 100,
        "consumer inventory must remain substantial"
    );
    assert!(
        audit.scanned_function_count() > 1_000,
        "consumer call graph unexpectedly contracted"
    );
    for finding in audit.findings() {
        assert!(finding.phases().len() >= 2);
        assert!(finding.site().line() > 0);
        assert!(finding.site().column() > 0);
    }
}

fn collect_rust_sources(
    workspace_root: &std::path::Path,
    directory: &std::path::Path,
    sources: &mut Vec<WorthQueryDeclarativeSurfaceSource>,
) {
    let mut entries = std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()))
        .map(|entry| {
            entry
                .expect("consumer source entry must be readable")
                .path()
        })
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_rust_sources(workspace_root, &path, sources);
        } else if path.extension().and_then(std::ffi::OsStr::to_str) == Some("rs") {
            let relative_path = path
                .strip_prefix(workspace_root)
                .expect("consumer source must remain in workspace")
                .to_string_lossy()
                .replace('\\', "/");
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
            sources.push(WorthQueryDeclarativeSurfaceSource::new(relative_path, text));
        }
    }
}
