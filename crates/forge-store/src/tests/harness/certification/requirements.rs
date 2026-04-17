use std::collections::BTreeSet;

use super::core::{AssertionClass, CertificationSuite, CompletenessReport};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuiteRequirement {
    pub suite_name: &'static str,
    pub required_rows: &'static [&'static str],
    pub required_assertion_classes: &'static [AssertionClass],
}

pub const DURABLE_ARTIFACT_AUTHORITY_EQUIVALENCE_TEST: SuiteRequirement = SuiteRequirement {
    suite_name: "Durable Artifact Authority Equivalence Test",
    required_rows: &[
        "semantic_parity",
        "export_json_parity",
        "lane_local_counter_divergence",
    ],
    required_assertion_classes: &[AssertionClass::Equality, AssertionClass::Inequality],
};

pub const OPERATING_MODE_CONTRACT_PARITY_TEST: SuiteRequirement = SuiteRequirement {
    suite_name: "Operating Mode Contract Parity Test",
    required_rows: &["mode_contract_parity", "typed_mode_failure"],
    required_assertion_classes: &[
        AssertionClass::Equality,
        AssertionClass::TypedFailure,
        AssertionClass::ExactCounter,
    ],
};

pub const WAL_CRASH_BOUNDARY_EXACTNESS_TEST: SuiteRequirement = SuiteRequirement {
    suite_name: "WAL Crash Boundary Exactness Test",
    required_rows: &["recovery_rebuild_equivalence", "typed_recovery_failure"],
    required_assertion_classes: &[
        AssertionClass::Equality,
        AssertionClass::TypedFailure,
        AssertionClass::ExactCounter,
    ],
};

pub const SNAPSHOT_PLUS_TAIL_RESTORE_EQUIVALENCE_TEST: SuiteRequirement = SuiteRequirement {
    suite_name: "Snapshot-Plus-Tail Restore Equivalence Test",
    required_rows: &[
        "restore_rebuild_equivalence",
        "backend_variation_delete_rebuild",
        "typed_snapshot_failure",
    ],
    required_assertion_classes: &[
        AssertionClass::Equality,
        AssertionClass::TypedFailure,
        AssertionClass::ExactCounter,
    ],
};

pub const BRANCH_DELTA_PROPORTIONALITY_AND_REPLAY_PARITY_TEST: SuiteRequirement =
    SuiteRequirement {
        suite_name: "Branch Delta Proportionality And Replay Parity Test",
        required_rows: &[
            "backend_variation_parity",
            "delta_growth_tracks_semantic_delta",
            "rewritten_stack_control_lane_parity",
        ],
        required_assertion_classes: &[
            AssertionClass::Equality,
            AssertionClass::Inequality,
            AssertionClass::ExactCounter,
        ],
    };

pub const DURABLE_MEDIA_WRITE_PATH_CERTIFICATION_TEST: SuiteRequirement = SuiteRequirement {
    suite_name: "Durable Media And Write-Path Certification Test",
    required_rows: &[
        "publication_family_equivalence",
        "publication_gap_classification",
        "typed_media_failures",
    ],
    required_assertion_classes: &[AssertionClass::Equality, AssertionClass::TypedFailure],
};

pub const ADVERSARIAL_CRASH_RECOVERY_SOURCE_PRECEDENCE_TEST: SuiteRequirement = SuiteRequirement {
    suite_name: "Adversarial Crash Recovery And Recovery Source Precedence Test",
    required_rows: &[
        "authoritative_truth_outranks_residue",
        "interrupted_snapshot_publication",
        "retained_without_ack_lane",
        "quiescent_second_restart",
        "quarantine_required_lane",
    ],
    required_assertion_classes: &[
        AssertionClass::Equality,
        AssertionClass::TypedFailure,
        AssertionClass::ExactCounter,
    ],
};

pub const SCHEMA_LINEAGE_CURSOR_DURABILITY_TEST: SuiteRequirement = SuiteRequirement {
    suite_name: "Schema/Lineage/Cursor Durability Test",
    required_rows: &[
        "support_artifact_restart_parity",
        "support_gap_backend_parity",
        "typed_support_gap_classification",
    ],
    required_assertion_classes: &[
        AssertionClass::Equality,
        AssertionClass::TypedFailure,
        AssertionClass::ExactCounter,
    ],
};

pub const ASPECT_LAYOUT_PHYSICAL_CERTIFICATION_TEST: SuiteRequirement = SuiteRequirement {
    suite_name: "Aspect Layout Physical Certification Test",
    required_rows: &[
        "admitted_layout_truth_parity",
        "admitted_layout_counter_contract_parity",
        "admitted_layout_artifact_parity",
        "authority_rebuild_preserves_layout_identity",
        "authority_rebuild_preserves_execution_surfaces",
        "dedup_control_overlap_branch_parity",
        "chunk_export_rebuild_parity",
        "sqlite_legacy_seed_migration_parity",
        "scope_shape_changes_physical_truth",
        "generalized_scope_requires_explicit_fallback",
        "commit_coupled_seed_corruption_requires_typed_failure",
        "chunk_export_corruption_requires_typed_failure",
        "chunk_export_boundary_mismatch_requires_typed_failure",
    ],
    required_assertion_classes: &[
        AssertionClass::Equality,
        AssertionClass::Inequality,
        AssertionClass::TypedFailure,
        AssertionClass::ExactCounter,
    ],
};

pub fn evaluate_completeness<T: serde::Serialize, E: serde::Serialize>(
    suite: &CertificationSuite<T, E>,
    requirement: &SuiteRequirement,
) -> CompletenessReport {
    let present_rows: BTreeSet<&str> = suite
        .canonical_rows()
        .iter()
        .map(|row| row.name())
        .chain(suite.rejection_rows().iter().map(|row| row.name()))
        .collect();
    let present_assertions: BTreeSet<AssertionClass> = suite
        .canonical_rows()
        .iter()
        .flat_map(|row| row.assertion_classes().iter().copied())
        .chain(
            suite
                .rejection_rows()
                .iter()
                .flat_map(|row| row.assertion_classes().iter().copied()),
        )
        .collect();

    let missing_rows = requirement
        .required_rows
        .iter()
        .filter(|row| !present_rows.contains(**row))
        .map(|row| (*row).to_string())
        .collect();
    let missing_assertion_classes = requirement
        .required_assertion_classes
        .iter()
        .copied()
        .filter(|class| !present_assertions.contains(class))
        .collect();

    CompletenessReport::new(
        missing_rows,
        missing_assertion_classes,
        suite.matrix_digest(),
    )
}
