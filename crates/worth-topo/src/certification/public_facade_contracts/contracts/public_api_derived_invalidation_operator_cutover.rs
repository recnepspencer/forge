use topology::derived_invalidation_operator_cutover::{
    current_operator_cutover_source_firewall, DerivedInvalidationOperatorCutoverCloseout,
    DerivedInvalidationOperatorCutoverCounters, DerivedInvalidationOperatorCutoverError,
    DerivedInvalidationOperatorCutoverErrorKind, DerivedInvalidationOperatorCutoverReceipt,
    DerivedInvalidationOperatorCutoverSourceFirewall,
    DerivedInvalidationOperatorCutoverSourceFirewallViolation, DerivedInvalidationPhaseEightSeed,
    DerivedInvalidationProjectionReadStageReceipt, ProjectionReadStageConsumptionScope,
};

fn _derived_invalidation_operator_cutover_contract() {
    let _: fn(&DerivedInvalidationOperatorCutoverError) -> DerivedInvalidationOperatorCutoverErrorKind =
        DerivedInvalidationOperatorCutoverError::kind;
    let _: fn(&DerivedInvalidationOperatorCutoverError) -> &str =
        DerivedInvalidationOperatorCutoverError::reason;

    let _: fn(&DerivedInvalidationOperatorCutoverReceipt) -> &str =
        DerivedInvalidationOperatorCutoverReceipt::phase_seven_seed_digest;
    let _: fn(&DerivedInvalidationOperatorCutoverReceipt) -> &str =
        DerivedInvalidationOperatorCutoverReceipt::operator_touched_basis_digest;
    let _: fn(&DerivedInvalidationOperatorCutoverReceipt) -> &str =
        DerivedInvalidationOperatorCutoverReceipt::selected_plan_digest;
    let _: fn(&DerivedInvalidationOperatorCutoverReceipt) -> &str =
        DerivedInvalidationOperatorCutoverReceipt::execution_receipt_digest;
    let _: fn(&DerivedInvalidationOperatorCutoverReceipt) -> &str =
        DerivedInvalidationOperatorCutoverReceipt::touched_closure_digest;
    let _: fn(&DerivedInvalidationOperatorCutoverReceipt) -> &str =
        DerivedInvalidationOperatorCutoverReceipt::graph_obligation_envelope_digest;
    let _: fn(&DerivedInvalidationOperatorCutoverReceipt) -> Option<&str> =
        DerivedInvalidationOperatorCutoverReceipt::graph_obligation_dispatch_digest;
    let _: fn(
        &DerivedInvalidationOperatorCutoverReceipt,
    ) -> &DerivedInvalidationOperatorCutoverCounters =
        DerivedInvalidationOperatorCutoverReceipt::counters;
    let _: fn(&DerivedInvalidationOperatorCutoverReceipt) -> &str =
        DerivedInvalidationOperatorCutoverReceipt::receipt_digest;

    let _: fn(&DerivedInvalidationOperatorCutoverCloseout) -> &DerivedInvalidationOperatorCutoverReceipt =
        DerivedInvalidationOperatorCutoverCloseout::operator_cutover;
    let _: fn(&DerivedInvalidationOperatorCutoverCloseout) -> &DerivedInvalidationProjectionReadStageReceipt =
        DerivedInvalidationOperatorCutoverCloseout::projection_read_stage;
    let _: fn(&DerivedInvalidationOperatorCutoverCloseout) -> &DerivedInvalidationOperatorCutoverCounters =
        DerivedInvalidationOperatorCutoverCloseout::counters;
    let _: fn(&DerivedInvalidationOperatorCutoverCloseout) -> &DerivedInvalidationPhaseEightSeed =
        DerivedInvalidationOperatorCutoverCloseout::phase_eight_seed;
    let _: fn(&DerivedInvalidationOperatorCutoverCloseout) -> &str =
        DerivedInvalidationOperatorCutoverCloseout::closeout_digest;

    let _: fn(&DerivedInvalidationProjectionReadStageReceipt) -> &str =
        DerivedInvalidationProjectionReadStageReceipt::operator_cutover_receipt_digest;
    let _: fn(&DerivedInvalidationProjectionReadStageReceipt) -> &str =
        DerivedInvalidationProjectionReadStageReceipt::execution_receipt_digest;
    let _: fn(&DerivedInvalidationProjectionReadStageReceipt) -> &str =
        DerivedInvalidationProjectionReadStageReceipt::selected_plan_digest;
    let _: fn(&DerivedInvalidationProjectionReadStageReceipt) -> &str =
        DerivedInvalidationProjectionReadStageReceipt::touched_closure_digest;
    let _: fn(&DerivedInvalidationProjectionReadStageReceipt) -> ProjectionReadStageConsumptionScope =
        DerivedInvalidationProjectionReadStageReceipt::consumption_scope;
    let _: fn(&DerivedInvalidationProjectionReadStageReceipt) -> usize =
        DerivedInvalidationProjectionReadStageReceipt::projection_dirty_expansion_count;
    let _: fn(&DerivedInvalidationProjectionReadStageReceipt) -> &str =
        DerivedInvalidationProjectionReadStageReceipt::receipt_digest;

    let _: fn(ProjectionReadStageConsumptionScope) -> &'static str =
        ProjectionReadStageConsumptionScope::as_str;

    let _: fn(&DerivedInvalidationOperatorCutoverCounters) -> usize =
        DerivedInvalidationOperatorCutoverCounters::selected_product_count;
    let _: fn(&DerivedInvalidationOperatorCutoverCounters) -> usize =
        DerivedInvalidationOperatorCutoverCounters::executed_product_count;
    let _: fn(&DerivedInvalidationOperatorCutoverCounters) -> usize =
        DerivedInvalidationOperatorCutoverCounters::projection_dirty_expansion_count;
    let _: fn(&DerivedInvalidationOperatorCutoverCounters) -> usize =
        DerivedInvalidationOperatorCutoverCounters::whole_view_fallback_count;
    let _: fn(&DerivedInvalidationOperatorCutoverCounters) -> usize =
        DerivedInvalidationOperatorCutoverCounters::caller_owned_graph_work_count;
    let _: fn(&DerivedInvalidationOperatorCutoverCounters) -> &str =
        DerivedInvalidationOperatorCutoverCounters::counters_digest;

    let _: fn(&DerivedInvalidationPhaseEightSeed) -> &str =
        DerivedInvalidationPhaseEightSeed::operator_cutover_receipt_digest;
    let _: fn(&DerivedInvalidationPhaseEightSeed) -> &str =
        DerivedInvalidationPhaseEightSeed::projection_read_stage_receipt_digest;
    let _: fn(&DerivedInvalidationPhaseEightSeed) -> &str =
        DerivedInvalidationPhaseEightSeed::selected_plan_digest;
    let _: fn(&DerivedInvalidationPhaseEightSeed) -> &str =
        DerivedInvalidationPhaseEightSeed::execution_receipt_digest;
    let _: fn(&DerivedInvalidationPhaseEightSeed) -> &str =
        DerivedInvalidationPhaseEightSeed::touched_closure_digest;
    let _: fn(&DerivedInvalidationPhaseEightSeed) -> &str =
        DerivedInvalidationPhaseEightSeed::query_support_digest;
    let _: fn(&DerivedInvalidationPhaseEightSeed) -> &str =
        DerivedInvalidationPhaseEightSeed::legality_support_digest;
    let _: fn(&DerivedInvalidationPhaseEightSeed) -> &str =
        DerivedInvalidationPhaseEightSeed::seed_digest;

    let _: fn() -> DerivedInvalidationOperatorCutoverSourceFirewall =
        current_operator_cutover_source_firewall;
    let _: fn(&DerivedInvalidationOperatorCutoverSourceFirewall) -> &[DerivedInvalidationOperatorCutoverSourceFirewallViolation] =
        DerivedInvalidationOperatorCutoverSourceFirewall::violations;
    let _: fn(&DerivedInvalidationOperatorCutoverSourceFirewall) -> &str =
        DerivedInvalidationOperatorCutoverSourceFirewall::report_digest;
    let _: fn(&DerivedInvalidationOperatorCutoverSourceFirewallViolation) -> &'static str =
        DerivedInvalidationOperatorCutoverSourceFirewallViolation::source_path;
    let _: fn(&DerivedInvalidationOperatorCutoverSourceFirewallViolation) -> &'static str =
        DerivedInvalidationOperatorCutoverSourceFirewallViolation::forbidden_surface;
}
