use super::{
    WorthQueryReferenceConsumerAdoptionRow, WorthQueryReferenceConsumerDeletedResidue,
    WorthQueryReferenceConsumerDxCounters, WorthQueryReferenceConsumerResidueKind,
};

const HADWIGER: &str = "hadwiger-research";
const WORTH_UI: &str = "worth-ui";

const ADOPTION_ROWS: [WorthQueryReferenceConsumerAdoptionRow; 2] = [
    WorthQueryReferenceConsumerAdoptionRow::new(
        HADWIGER,
        "crates/hadwiger-research/src/query_entry/ordinary_query.rs",
        "impl HadwigerResearchQueryExt",
        WorthQueryReferenceConsumerDxCounters::new(10, 4, 4, 1, 4),
        WorthQueryReferenceConsumerDxCounters::new(2, 2, 0, 0, 0),
    ),
    WorthQueryReferenceConsumerAdoptionRow::new(
        WORTH_UI,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/installed_measurements.rs",
        "impl WorthUiQueryExt",
        WorthQueryReferenceConsumerDxCounters::new(12, 5, 4, 1, 3),
        WorthQueryReferenceConsumerDxCounters::new(2, 2, 0, 0, 0),
    ),
];

const DELETED_RESIDUE: [WorthQueryReferenceConsumerDeletedResidue; 11] = [
    WorthQueryReferenceConsumerDeletedResidue::new(
        HADWIGER,
        "crates/hadwiger-research/src/domain_artifacts/query_references.rs",
        "HadwigerQueryEnvelopeReference",
        WorthQueryReferenceConsumerResidueKind::LocalType,
    ),
    WorthQueryReferenceConsumerDeletedResidue::new(
        HADWIGER,
        "crates/hadwiger-research/src/query_entry/admitted_handle.rs",
        "orchestrate_declaration_entry_outcome",
        WorthQueryReferenceConsumerResidueKind::LocalHelper,
    ),
    WorthQueryReferenceConsumerDeletedResidue::new(
        HADWIGER,
        "crates/hadwiger-research/src/research_graph_invariants/operations.rs",
        "evaluate_requested_domain_capability_contribution",
        WorthQueryReferenceConsumerResidueKind::LocalTransition,
    ),
    WorthQueryReferenceConsumerDeletedResidue::new(
        HADWIGER,
        "crates/hadwiger-research/src/research_graph_invariants/operations.rs",
        "WorthQueryInvariantCapabilityContributionAuthoring",
        WorthQueryReferenceConsumerResidueKind::DeepImport,
    ),
    WorthQueryReferenceConsumerDeletedResidue::new(
        HADWIGER,
        "crates/hadwiger-research/src/research_graph_invariants/operations.rs",
        "materialize_graph_composition_domain_invariant_denial",
        WorthQueryReferenceConsumerResidueKind::BackendDecision,
    ),
    WorthQueryReferenceConsumerDeletedResidue::new(
        WORTH_UI,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/prerequisites/query_measurement_fact_receipt_tests.rs",
        "ProjectionAuthorityOutcome",
        WorthQueryReferenceConsumerResidueKind::LocalType,
    ),
    WorthQueryReferenceConsumerDeletedResidue::new(
        WORTH_UI,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/prerequisites/query_measurement_fact_receipt_tests.rs",
        "execute_read_family(",
        WorthQueryReferenceConsumerResidueKind::LocalHelper,
    ),
    WorthQueryReferenceConsumerDeletedResidue::new(
        WORTH_UI,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/prerequisites/query_measurement_fact_receipt_tests.rs",
        "define_read_family(",
        WorthQueryReferenceConsumerResidueKind::LocalTransition,
    ),
    WorthQueryReferenceConsumerDeletedResidue::new(
        WORTH_UI,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/prerequisites/query_measurement_fact_receipt_tests.rs",
        "public_bridge_projection_artifacts_for_read_graph",
        WorthQueryReferenceConsumerResidueKind::DeepImport,
    ),
    WorthQueryReferenceConsumerDeletedResidue::new(
        WORTH_UI,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/prerequisites/query_measurement_fact_receipt_tests.rs",
        "resolve_runtime_current_snapshot_basis(",
        WorthQueryReferenceConsumerResidueKind::BackendDecision,
    ),
    WorthQueryReferenceConsumerDeletedResidue::new(
        WORTH_UI,
        "workspaces/worth-ui/crates/worth-ui-runtime/src/runtime/allocation_frame_dispatch/gateway/query_test_support.rs",
        "bind_query_basis_context(",
        WorthQueryReferenceConsumerResidueKind::LocalTransition,
    ),
];

pub fn worth_query_reference_consumer_adoption_rows(
) -> &'static [WorthQueryReferenceConsumerAdoptionRow] {
    &ADOPTION_ROWS
}

pub fn worth_query_reference_consumer_deleted_residue(
) -> &'static [WorthQueryReferenceConsumerDeletedResidue] {
    &DELETED_RESIDUE
}
