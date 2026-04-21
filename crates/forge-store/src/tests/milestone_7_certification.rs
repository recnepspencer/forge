use crate::{
    backend::records::{
        EmbeddedCheckpointClassification as StoredCheckpointClassification,
        EmbeddedCheckpointRecord,
    },
    DurableCursorAcknowledgeRequest, ForgeStore, ForgeStoreBuilder, RecoveryOperatorDisposition,
    SupportArtifactRecoveryReport,
};

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_rejection_payloads_present},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult, RejectionRow},
        requirements::{evaluate_completeness, SCHEMA_LINEAGE_CURSOR_DURABILITY_TEST},
    },
    corruption::{
        local_file::{
            force_commit_support_summary_key_mismatch, force_cursor_checkpoint_gap,
            force_cursor_identity_key_mismatch, force_embedded_checkpoint_key_mismatch,
            force_first_lineage_support_gap, force_lineage_support_key_mismatch,
            force_schema_support_key_mismatch, force_subscriber_checkpoint_key_mismatch,
        },
        sqlite::delete_first_sqlite_lineage_support_record,
    },
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
};

fn canonical_support_truth(bundle: &crate::Milestone7CertificationBundle) -> serde_json::Value {
    serde_json::json!({
        "backend_family": format!("{:?}", bundle.backend_family),
        "history_digest": bundle.history_digest,
        "artifact_digest": bundle.artifact_digest,
        "replay_digest": bundle.replay_digest,
        "support_truth_digest": bundle.support_truth_digest,
        "support_artifact_recovery_report": bundle.support_artifact_recovery_report,
        "certification_summary": bundle.certification_summary,
        "access_structure_contract": bundle.access_structure_contract,
        "access_structure_verification": bundle.access_structure_verification,
        "complexity_status": bundle.complexity_status,
    })
}

fn support_gap_surface(
    report: &SupportArtifactRecoveryReport,
    disposition: RecoveryOperatorDisposition,
    gap_count: u64,
) -> serde_json::Value {
    serde_json::json!({
        "support_artifact_recovery_report": report,
        "operator_disposition": format!("{disposition:?}"),
        "support_artifact_recovery_gap_count": gap_count,
    })
}

fn assert_complexity_debt(
    path: &crate::Milestone7ComplexityPathStatus,
    verification: &crate::Milestone7AccessStructureVerificationPath,
) {
    assert!(!verification.verified_at_open);
    assert!(
        verification
            .verification_gap
            .as_deref()
            .unwrap_or_default()
            .contains("stored key")
            || verification
                .verification_gap
                .as_deref()
                .unwrap_or_default()
                .contains("map key")
    );
    assert_eq!(path.status, crate::ComplexityStatus::Debt);
    assert!(path.proof_basis.is_none());
    assert!(
        path.debt_reason
            .as_deref()
            .unwrap_or_default()
            .contains("stored key")
            || path
                .debt_reason
                .as_deref()
                .unwrap_or_default()
                .contains("map key")
    );
}

#[path = "milestone_7_certification/access_structure_debt.rs"]
mod access_structure_debt;
#[path = "milestone_7_certification/parity.rs"]
mod parity;
#[path = "milestone_7_certification/suite.rs"]
mod suite;
#[path = "milestone_7_certification/support_gap_recovery.rs"]
mod support_gap_recovery;
