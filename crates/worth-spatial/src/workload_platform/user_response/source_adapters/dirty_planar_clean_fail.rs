use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::{
    dirty_planar_clean_fail::{DirtyPlanarCleanFailError, DirtyPlanarCleanFailReceipt},
    user_response::{
        source::WorthUserResponseSourceKind, WorthUserOutcomeCauseKind, WorthUserResponseSource,
    },
};

impl WorthUserResponseSource {
    pub fn from_dirty_planar_clean_fail(receipt: &DirtyPlanarCleanFailReceipt) -> Self {
        let message = format!(
            "Dirty input is {}; no automatic boolean option exists in M6.5, so inspect topology before overlap or recovery admission.",
            receipt.dirty_case().human_name()
        );
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: WorthUserOutcomeCauseKind::DirtyInput,
                message,
                evidence_digest: receipt.clean_fail_digest().to_string(),
                source_identity: receipt.workload_identity().to_string(),
            },
        }
    }

    pub fn from_dirty_planar_clean_fail_error(error: DirtyPlanarCleanFailError) -> Self {
        let cause_kind = match error {
            DirtyPlanarCleanFailError::StableTopologyIdentityHidDirtyGeometry { .. }
            | DirtyPlanarCleanFailError::MismatchedDirtyKind { .. }
            | DirtyPlanarCleanFailError::CleanFailBoundaryDidNotConsumeTopologyReceipt => {
                WorthUserOutcomeCauseKind::IntegrityMismatch
            }
            DirtyPlanarCleanFailError::RecoveryAttemptedTruthUpgrade
            | DirtyPlanarCleanFailError::CleanFailAttemptedRepair
            | DirtyPlanarCleanFailError::CleanFailAttemptedBoundedConversion => {
                WorthUserOutcomeCauseKind::DirtyInput
            }
            DirtyPlanarCleanFailError::UserResponseDidNotExplainDirtyNoOptions
            | DirtyPlanarCleanFailError::UserResponseDidNotConsumeCleanFailBoundary => {
                WorthUserOutcomeCauseKind::MissingEvidence
            }
            _ => WorthUserOutcomeCauseKind::MissingEvidence,
        };
        let human_reason = error.human_reason();
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "dirty-planar-clean-fail-error".to_string(),
                format!("{error:?}"),
                human_reason.clone(),
            ],
        );
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind,
                message: human_reason,
                evidence_digest: evidence_digest.clone(),
                source_identity: evidence_digest,
            },
        }
    }
}
