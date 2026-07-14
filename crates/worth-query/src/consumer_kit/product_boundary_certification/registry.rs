use super::{
    WorthQueryProductBoundaryEvidenceKind as Kind, WorthQueryProductBoundaryEvidenceRow as Row,
    WorthQueryProductBoundaryHostileCase as Hostile,
    WorthQueryProductBoundarySabotageCase as Sabotage,
};

const HOSTILE_PATH: &str = "tests/declarative_product_boundary_certification/hostile_matrix.rs";
const SABOTAGE_PATH: &str = "tests/declarative_product_boundary_certification/sabotage_matrix.rs";
const PARITY_PATH: &str = "tests/declarative_product_boundary_certification/parity_bounded.rs";
const COMPILE_PATH: &str = "tests/declarative_product_boundary_compile_fail.rs";

const fn hostile(id: &'static str, case: Hostile, path: &'static str, probe: &'static str) -> Row {
    Row::new(
        id,
        if matches!(
            case,
            Hostile::CrossCapabilityOptionRejection | Hostile::ReceiptNonPromotion
        ) {
            Kind::CompileBoundary
        } else {
            Kind::HostileRuntime
        },
        Some(case),
        None,
        path,
        probe,
        "runtime-backed hostile matrix",
    )
}

const fn sabotage(
    id: &'static str,
    case: Sabotage,
    probe: &'static str,
    layer: &'static str,
) -> Row {
    Row::new(
        id,
        Kind::Sabotage,
        None,
        Some(case),
        SABOTAGE_PATH,
        probe,
        layer,
    )
}

#[rustfmt::skip]
static EVIDENCE: &[Row] = &[
    hostile("hostile-equivalent-declarations", Hostile::EquivalentDeclarationConvergence, HOSTILE_PATH, "fn equivalent_declarations_converge()"),
    hostile("hostile-cross-capability", Hostile::CrossCapabilityOptionRejection, COMPILE_PATH, "fn cross_capability_options_are_rejected()"),
    hostile("hostile-cross-basis", Hostile::CrossBasisDenial, HOSTILE_PATH, "fn cross_basis_denies_before_execution()"),
    hostile("hostile-stale-context", Hostile::StaleContextDenial, HOSTILE_PATH, "fn stale_context_denies_before_execution()"),
    hostile("hostile-one-shot-live", Hostile::OneShotLiveParity, HOSTILE_PATH, "fn one_shot_and_live_results_match()"),
    hostile("hostile-history-ambiguity", Hostile::HistoricalAmbiguity, HOSTILE_PATH, "fn historical_ambiguity_remains_advisory()"),
    hostile("hostile-preview-workflow", Hostile::PreviewWorkflowDenial, HOSTILE_PATH, "fn preview_workflow_cross_session_denies()"),
    hostile("hostile-receipt-non-promotion", Hostile::ReceiptNonPromotion, COMPILE_PATH, "fn receipts_cannot_promote_to_authority()"),
    hostile("hostile-diagnostic-policy", Hostile::DiagnosticPolicyEquivalence, HOSTILE_PATH, "fn diagnostic_policy_preserves_operational_truth()"),
    sabotage("sabotage-public-phase", Sabotage::PublicPhaseConstructor, "fn public_phase_constructor_hits_declarative_surface_audit()", "declarative-surface-audit"),
    sabotage("sabotage-deep-transition", Sabotage::DeepTransition, "fn deep_transition_hits_hard_prohibition_audit()", "hard-prohibition-boundary-audit"),
    sabotage("sabotage-backend-selector", Sabotage::BackendSelector, "fn backend_selector_changes_facade_snapshot()", "ordinary-api-snapshot"),
    sabotage("sabotage-success-envelope", Sabotage::SuccessEnvelopeBuilder, "fn success_envelope_builder_changes_facade_snapshot()", "ordinary-api-snapshot"),
    sabotage("sabotage-compatibility-alias", Sabotage::CompatibilityAlias, "fn compatibility_alias_changes_facade_snapshot()", "ordinary-api-snapshot"),
    sabotage("sabotage-local-coordinator", Sabotage::ConsumerLocalCoordinator, "fn consumer_local_coordinator_hits_call_graph_audit()", "consumer-orchestration-call-graph"),
    Row::new("reference-semantic-parity", Kind::SemanticParity, None, None, PARITY_PATH, "fn reference_read_matches_internal_phase_chain_oracle()", "instrumented-phase-chain-parity"),
    Row::new("managed-lifecycle-closeout", Kind::Lifecycle, None, None, PARITY_PATH, "fn managed_lifecycle_has_exact_open_and_close_work()", "managed-resource-lifecycle"),
    Row::new("invalid-denial-counters", Kind::BoundedWork, None, None, PARITY_PATH, "fn invalid_context_has_zero_planning_and_runtime_work()", "exact-journey-counters"),
    Row::new("unrelated-size-counters", Kind::BoundedWork, None, None, PARITY_PATH, "fn unrelated_workspace_size_does_not_change_ergonomic_lowering_work()", "bounded-work-slope"),
    Row::new("reference-consumer-cutover", Kind::ReferenceConsumer, None, None, PARITY_PATH, "fn reference_consumers_are_source_backed_and_orchestration_free()", "reference-consumer-adoption-audit"),
];

pub fn worth_query_product_boundary_evidence_rows() -> &'static [Row] {
    EVIDENCE
}
