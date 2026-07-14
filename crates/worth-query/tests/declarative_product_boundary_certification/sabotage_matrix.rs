use worth_query::facade::certification::{
    audit_consumer_orchestration_sources, audit_declarative_surface_sources,
    audit_ordinary_api_snapshot_source_for_certification,
    WorthQueryDeclarativeSurfaceFindingKind, WorthQueryDeclarativeSurfaceSource,
};
use worth_query::{
    hard_prohibition_boundary_audit, WorthQueryBoundaryAuditSourceSet, WorthQueryProhibitedSeam,
};

#[test]
fn public_phase_constructor_hits_declarative_surface_audit() {
    let audit = audit_declarative_surface_sources(&[WorthQueryDeclarativeSurfaceSource::new(
        "src/ordinary/read/sabotaged_constructor.rs",
        "pub fn canonicalize_request() {}",
    )]);
    assert!(audit.findings().iter().any(|finding| {
        finding.kind() == WorthQueryDeclarativeSurfaceFindingKind::UnclassifiedPublicPhaseSurface
            && finding.site().function_name() == "canonicalize_request"
    }));
}

#[test]
fn deep_transition_hits_hard_prohibition_audit() {
    let report = hard_prohibition_boundary_audit()
        .covering_sources(WorthQueryBoundaryAuditSourceSet::new("phase-12-sabotage").source(
            "deep-transition",
            "fn sabotage() { worth_query::planning::plan_validated_bundle(); }",
        ))
        .evaluate()
        .expect("sabotage source should parse");
    assert!(report
        .findings()
        .iter()
        .any(|finding| finding.seam() == WorthQueryProhibitedSeam::DeepPhaseModuleImport));
}

#[test]
fn backend_selector_changes_facade_snapshot() {
    assert_read_snapshot_sabotage("pub fn select_query_backend() {}");
}

#[test]
fn success_envelope_builder_changes_facade_snapshot() {
    assert_read_snapshot_sabotage("pub fn build_success_envelope() {}");
}

#[test]
fn compatibility_alias_changes_facade_snapshot() {
    assert_read_snapshot_sabotage(
        "pub type WorthQueryReadCompatibilityAlias = WorthQueryReadOutcome;",
    );
}

#[test]
fn consumer_local_coordinator_hits_call_graph_audit() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "consumer/sabotaged_coordinator.rs",
        r#"
fn compile(input: Input) -> Canonical { worth_query::facade::canonicalize_query(input) }
fn admit(value: Canonical) -> Admitted { worth_query::facade::admit_query_context(value) }
fn execute(value: Admitted) -> Output { worth_query::facade::execute_read_family(value) }
pub fn coordinate(input: Input) -> Output { execute(admit(compile(input))) }
"#,
    );
    let audit = audit_consumer_orchestration_sources(&[source]).expect("sabotage should parse");
    assert!(audit
        .findings()
        .iter()
        .any(|finding| finding.site().function_name() == "coordinate"));
}

fn assert_read_snapshot_sabotage(injected: &str) {
    let source = format!(
        "{}\n{}",
        include_str!("../../src/facade/exports_read.rs"),
        injected
    );
    let audit = audit_ordinary_api_snapshot_source_for_certification(
        "read",
        "src/facade/exports_read.rs",
        &source,
    )
    .expect("read namespace must be frozen");
    assert!(!audit.is_complete());
    assert_eq!(audit.findings().len(), 1);
    assert_eq!(audit.findings()[0].namespace(), "read");
}
