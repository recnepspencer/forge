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
fn split_subscription_lifecycle_helpers_cannot_hide_local_ownership() {
    let source = WorthQueryDeclarativeSurfaceSource::new(
        "seeded/local_subscription.rs",
        r#"
fn begin_live(input: Input) -> Active {
    worth_query::facade::activate_query_subscription(input)
}
fn refresh_live(active: Active) -> Active {
    worth_query::facade::maintain_query_subscription(active)
}
fn finish_live(active: Active) -> Closed {
    worth_query::facade::close_query_subscription(active)
}
pub fn own_live_lifecycle(input: Input) -> Closed {
    let active = begin_live(input);
    let active = refresh_live(active);
    finish_live(active)
}
"#,
    );

    let audit = audit_consumer_orchestration_sources(&[source]).expect("fixture parses");

    assert_eq!(audit.findings().len(), 1);
    let finding = &audit.findings()[0];
    assert_eq!(finding.site().path(), "seeded/local_subscription.rs");
    assert_eq!(finding.site().line(), 11);
    assert_eq!(finding.site().function_name(), "own_live_lifecycle");
    assert_eq!(
        finding.phases(),
        &[Phase::Activate, Phase::Maintain, Phase::Close]
    );
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
