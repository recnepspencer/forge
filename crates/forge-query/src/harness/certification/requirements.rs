#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequiredAssertionClass {
    Equality,
    Inequality,
    TypedFailure,
    ZeroResidue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuiteRequirements {
    pub suite_name: &'static str,
    pub required_canonical_rows: &'static [&'static str],
    pub required_rejection_rows: &'static [&'static str],
    pub required_assertion_classes: &'static [RequiredAssertionClass],
    pub missing_rows_block_full_spec: bool,
    pub missing_rows_block_offline_ready: bool,
}

pub fn milestone_one_requirements() -> SuiteRequirements {
    SuiteRequirements {
        suite_name: "Canonical Query Normalization Parity Test",
        required_canonical_rows: &[
            "detail-query-parity",
            "result-shape-helper-composition",
            "binding-descriptor-parity",
            "collection-reordered-projection-parity",
            "duplicate-clause-deduplication",
            "semantic-distinction-boundary",
        ],
        required_rejection_rows: &[
            "unsupported-authored-query-family",
            "unsupported-authored-result-shape-family",
            "forbidden-fallback-case",
        ],
        required_assertion_classes: &[
            RequiredAssertionClass::Equality,
            RequiredAssertionClass::Inequality,
            RequiredAssertionClass::TypedFailure,
            RequiredAssertionClass::ZeroResidue,
        ],
        missing_rows_block_full_spec: true,
        missing_rows_block_offline_ready: true,
    }
}

pub fn milestone_two_requirements() -> SuiteRequirements {
    SuiteRequirements {
        suite_name: "Schema-Aware Rejection And Projection Legality Test",
        required_canonical_rows: &[
            "legal-detail-query-parity",
            "equivalent-builder-composed-legal-query",
            "legal-structured-content-query-parity",
            "legal-workflow-predicate-parity",
        ],
        required_rejection_rows: &[
            "unknown-aspect-projection",
            "incompatible-predicate-family",
            "illegal-traversal-edge-or-depth",
            "invalid-result-shape-binding",
            "structured-content-illegality",
            "workflow-context-illegality",
            "forbidden-widening-case",
        ],
        required_assertion_classes: &[
            RequiredAssertionClass::Equality,
            RequiredAssertionClass::Inequality,
            RequiredAssertionClass::TypedFailure,
            RequiredAssertionClass::ZeroResidue,
        ],
        missing_rows_block_full_spec: true,
        missing_rows_block_offline_ready: true,
    }
}

pub fn milestone_three_requirements() -> SuiteRequirements {
    SuiteRequirements {
        suite_name: "Planner / Executor / Binding Parity Test",
        required_canonical_rows: &[
            "direct-runtime-plan-parity",
            "replanned-runtime-parity",
            "type-bound-runtime-parity",
            "runtime-basis-repeatability",
            "identity-bearing-binding-difference",
            "basis-difference",
            "route-semantic-difference",
        ],
        required_rejection_rows: &[
            "unsupported-backend-route",
            "unsupported-fallback-shape",
            "binding-fulfillment-conflict",
            "snapshot-basis-resolution-failure",
        ],
        required_assertion_classes: &[
            RequiredAssertionClass::Equality,
            RequiredAssertionClass::Inequality,
            RequiredAssertionClass::TypedFailure,
            RequiredAssertionClass::ZeroResidue,
        ],
        missing_rows_block_full_spec: true,
        missing_rows_block_offline_ready: true,
    }
}

pub fn milestone_four_requirements() -> SuiteRequirements {
    SuiteRequirements {
        suite_name: "Collection, Cursor, Rollup, And CDC Shape Parity Test",
        required_canonical_rows: &[
            "ordered-collection-parity",
            "cursor-advance-repeatability",
            "bounded-traversal-parity",
            "aggregate-rollup-parity",
            "derived-field-parity",
            "cdc-shaped-result-parity",
        ],
        required_rejection_rows: &[
            "unsupported-ordering-family",
            "unstable-cursor-shape",
            "unsupported-traversal-bound",
            "unsupported-aggregate-family",
            "unsupported-cdc-result-family",
        ],
        required_assertion_classes: &[
            RequiredAssertionClass::Equality,
            RequiredAssertionClass::Inequality,
            RequiredAssertionClass::TypedFailure,
            RequiredAssertionClass::ZeroResidue,
        ],
        missing_rows_block_full_spec: true,
        missing_rows_block_offline_ready: true,
    }
}

pub fn milestone_five_requirements() -> SuiteRequirements {
    SuiteRequirements {
        suite_name: "Live Promotion Convergence And Suppression Test",
        required_canonical_rows: &[
            "detail-live-convergence",
            "ordered-collection-live-convergence",
            "bounded-materialization-live-convergence",
            "irrelevant-update-suppression",
            "refresh-fallback-equivalence",
            "coalesced-sequence-replay-parity",
            "patch-width-budget-overflow-policy",
            "work-avoided-counter-parity",
        ],
        required_rejection_rows: &[
            "unsupported-live-family",
            "unsupported-patch-family",
            "raw-cdc-leakage-forbidden",
            "invalid-live-basis-promotion",
            "forbidden-refresh-escape-hatch",
            "non-monotonic-change-sequence",
            "forbidden-coalescing-class",
            "forbidden-width-budget-overflow-behavior",
        ],
        required_assertion_classes: &[
            RequiredAssertionClass::Equality,
            RequiredAssertionClass::TypedFailure,
            RequiredAssertionClass::ZeroResidue,
        ],
        missing_rows_block_full_spec: true,
        missing_rows_block_offline_ready: true,
    }
}

pub fn milestone_five_point_one_requirements() -> SuiteRequirements {
    SuiteRequirements {
        suite_name: "Region-Scoped Live Narrowing And Stream Contract Test",
        required_canonical_rows:
            crate::harness::region_live_certification::REGION_LIVE_REQUIRED_CANONICAL_ROW_NAMES,
        required_rejection_rows:
            crate::harness::region_live_certification::REGION_LIVE_REQUIRED_REJECTION_ROW_NAMES,
        required_assertion_classes: &[
            RequiredAssertionClass::Equality,
            RequiredAssertionClass::TypedFailure,
            RequiredAssertionClass::ZeroResidue,
        ],
        missing_rows_block_full_spec: true,
        missing_rows_block_offline_ready: true,
    }
}

pub fn milestone_five_point_two_requirements() -> SuiteRequirements {
    SuiteRequirements {
        suite_name: "Preview Session Basis And Promotion Parity Test",
        required_canonical_rows:
            crate::harness::preview_certification::PREVIEW_REQUIRED_CANONICAL_ROW_NAMES,
        required_rejection_rows:
            crate::harness::preview_certification::PREVIEW_REQUIRED_REJECTION_ROW_NAMES,
        required_assertion_classes: &[
            RequiredAssertionClass::Inequality,
            RequiredAssertionClass::Equality,
            RequiredAssertionClass::TypedFailure,
            RequiredAssertionClass::ZeroResidue,
        ],
        missing_rows_block_full_spec: true,
        missing_rows_block_offline_ready: true,
    }
}

pub fn milestone_five_point_three_requirements() -> SuiteRequirements {
    SuiteRequirements {
        suite_name: "Frontier Planning And Parallel Admission Parity Test",
        required_canonical_rows:
            crate::harness::frontier_certification::FRONTIER_REQUIRED_CANONICAL_ROW_NAMES,
        required_rejection_rows:
            crate::harness::frontier_certification::FRONTIER_REQUIRED_REJECTION_ROW_NAMES,
        required_assertion_classes: &[
            RequiredAssertionClass::Equality,
            RequiredAssertionClass::TypedFailure,
            RequiredAssertionClass::ZeroResidue,
        ],
        missing_rows_block_full_spec: true,
        missing_rows_block_offline_ready: true,
    }
}

pub fn milestone_five_point_four_requirements() -> SuiteRequirements {
    SuiteRequirements {
        suite_name: "Structural Correspondence And Historical Materialization Path Test",
        required_canonical_rows: crate::harness::correspondence_history_certification::
            CORRESPONDENCE_HISTORY_REQUIRED_CANONICAL_ROW_NAMES,
        required_rejection_rows: crate::harness::correspondence_history_certification::
            CORRESPONDENCE_HISTORY_REQUIRED_REJECTION_ROW_NAMES,
        required_assertion_classes: &[
            RequiredAssertionClass::Equality,
            RequiredAssertionClass::Inequality,
            RequiredAssertionClass::TypedFailure,
            RequiredAssertionClass::ZeroResidue,
        ],
        missing_rows_block_full_spec: true,
        missing_rows_block_offline_ready: true,
    }
}
