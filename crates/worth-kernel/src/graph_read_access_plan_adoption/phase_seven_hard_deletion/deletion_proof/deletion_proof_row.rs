use std::path::Path;

use crate::graph_read_access_plan_adoption::WorthGraphReadAccessSliceCutoverProof;

use super::super::stable_digest;
use super::deletion_status::WorthGraphReadAccessHardDeletionStatus;
use super::migrated_execution_target::MigratedExecutionTarget;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessHardDeletionProofRow {
    label: String,
    source_path: String,
    owner: String,
    blocker: Option<String>,
    removal_trigger: String,
    status: WorthGraphReadAccessHardDeletionStatus,
    row_digest: String,
}

impl WorthGraphReadAccessHardDeletionProofRow {
    pub(crate) fn from_phase_four_cutover_proof(
        cutover_proof: &WorthGraphReadAccessSliceCutoverProof,
    ) -> Self {
        Self::new(
            "phase_four_vertical_slice_cutover_proof".to_string(),
            cutover_proof.deletion_target_identity().to_string(),
            "worth-kernel.graph_read_access_plan_adoption.phase_four_vertical_slice".to_string(),
            None,
            "Phase 7 consumed the Phase 4 cutover proof as hard-deletion closeout evidence"
                .to_string(),
            WorthGraphReadAccessHardDeletionStatus::Deleted,
        )
    }

    #[cfg(test)]
    pub(in crate::graph_read_access_plan_adoption::phase_seven_hard_deletion) fn capped_residue_for_test(
        source_path: &str,
        owner: &str,
        blocker: &str,
        removal_trigger: &str,
    ) -> Self {
        Self::new(
            "test_capped_residue".to_string(),
            source_path.to_string(),
            owner.to_string(),
            Some(blocker.to_string()),
            removal_trigger.to_string(),
            WorthGraphReadAccessHardDeletionStatus::CappedResidue,
        )
    }

    pub(crate) fn from_target(target: MigratedExecutionTarget, workspace_root: &Path) -> Self {
        let absolute_source_path = workspace_root.join(target.source_path);
        let status = if absolute_source_path.exists() {
            WorthGraphReadAccessHardDeletionStatus::Unresolved
        } else {
            WorthGraphReadAccessHardDeletionStatus::Deleted
        };
        let blocker = if status == WorthGraphReadAccessHardDeletionStatus::Unresolved {
            Some("source path still exists after hard-deletion phase".to_string())
        } else {
            None
        };
        Self::new(
            target.label.to_string(),
            target.source_path.to_string(),
            target.owner.to_string(),
            blocker,
            target.removal_trigger.to_string(),
            status,
        )
    }

    fn new(
        label: String,
        source_path: String,
        owner: String,
        blocker: Option<String>,
        removal_trigger: String,
        status: WorthGraphReadAccessHardDeletionStatus,
    ) -> Self {
        let row_digest = stable_digest(&[
            "worth_graph_read_access_hard_deletion_proof_row_v1".to_string(),
            format!("label:{label}"),
            format!("source_path:{source_path}"),
            format!("owner:{owner}"),
            format!("blocker:{}", blocker.as_deref().unwrap_or("none")),
            format!("removal_trigger:{removal_trigger}"),
            format!("status:{}", status.as_str()),
        ]);
        Self {
            label,
            source_path,
            owner,
            blocker,
            removal_trigger,
            status,
            row_digest,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn blocker(&self) -> Option<&str> {
        self.blocker.as_deref()
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn status(&self) -> WorthGraphReadAccessHardDeletionStatus {
        self.status
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
