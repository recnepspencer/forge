use super::support::*;

#[test]
fn capability_gap_dispatch_keeps_bounded_evidence_without_rich_policy() {
    let registration = task_collection_registration(
        ForgeQueryGraphObligationKind::CapabilityGapScreen,
        "bounded-capability-gap",
        ForgeQueryGraphObligationSupportPosture::diagnostic_only(
            ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    );
    let mut runtime = runtime_with_registration(registration);

    let receipt = runtime
        .write_batch(vec![task_insert_command("bounded-capability-gap")])
        .expect("diagnostic-only capability gap should not block write execution");
    let row = receipt
        .obligation_dispatch()
        .expect("bounded capability gap should attach dispatch")
        .evidence_projection()
        .rows()
        .first()
        .expect("bounded capability gap should project execution evidence")
        .clone();

    assert_eq!(
        row.diagnostic_materialization(),
        Some(ForgeQueryGraphObligationDiagnosticMaterialization::BoundedEvidenceOnly)
    );
}

#[test]
fn rich_capability_gap_diagnostics_require_explicit_execution_policy() {
    let registration = task_collection_registration(
        ForgeQueryGraphObligationKind::CapabilityGapScreen,
        "rich-capability-gap",
        ForgeQueryGraphObligationSupportPosture::diagnostic_only(
            ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    );
    let mut runtime = runtime_with_registration(registration);

    let receipt = runtime
        .write_batch_with_graph_obligation_artifact_policy(
            vec![task_insert_command("rich-capability-gap")],
            ForgeQueryGraphObligationArtifactPolicy::rich_capability_gap_diagnostics(),
        )
        .expect("diagnostic-only capability gap should not block rich diagnostic execution");
    let row = receipt
        .obligation_dispatch()
        .expect("rich capability gap should attach dispatch")
        .execution_results()
        .expect("rich capability gap should attach execution evidence")
        .rows()
        .first()
        .expect("rich capability gap should project execution evidence")
        .clone();

    assert_eq!(
        row.diagnostic_materialization(),
        ForgeQueryGraphObligationDiagnosticMaterialization::RichCapabilityGapDiagnostics
    );
    assert_eq!(row.state_load_counters().materialized_row_count(), 0);
}
