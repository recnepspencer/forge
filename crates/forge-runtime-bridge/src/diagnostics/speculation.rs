use crate::speculation::{
    BridgePreviewDiscardRecord, BridgePreviewExecutionRecord, BridgePreviewLifecycleStateKind,
    BridgePreviewPromotionRecord, BridgePreviewReplayBundle, BridgePreviewResidueClass,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePreviewExecutionExplanation {
    preview_session_identity: String,
    preview_execution_record_identity: String,
    preview_declaration_digest: String,
    branch_binding_digest: String,
    preview_artifact_count: usize,
    replay_bundle_width: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePreviewDiscardExplanation {
    preview_session_identity: String,
    preview_discard_record_identity: String,
    preview_execution_record_identity: String,
    cleanup_outcome: crate::speculation::BridgePreviewDiscardCleanupOutcome,
    residue_classes: Vec<BridgePreviewResidueClass>,
    authoritative_residue_count: usize,
    destroyed_artifact_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePreviewPromotionExplanation {
    preview_session_identity: String,
    preview_promotion_record_identity: String,
    preview_execution_record_identity: String,
    authoritative_commit_boundary_digest: String,
    authoritative_artifact_digest: String,
    promotion_proof_digest: String,
    promotion_proof_checks: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePreviewReplayExplanation {
    preview_session_identity: String,
    lifecycle_outcome: BridgePreviewLifecycleStateKind,
    replay_bundle_digest: String,
    replay_bundle_width: usize,
    has_discard_record: bool,
    has_promotion_record: bool,
}

impl BridgePreviewExecutionExplanation {
    pub fn from_record(record: &BridgePreviewExecutionRecord) -> Self {
        Self {
            preview_session_identity: record.preview_session_identity().to_owned(),
            preview_execution_record_identity: record.record_identity().as_str().to_owned(),
            preview_declaration_digest: record.preview_declaration_digest().to_owned(),
            branch_binding_digest: record.branch_binding_digest().to_owned(),
            preview_artifact_count: record.counters().preview_artifact_count(),
            replay_bundle_width: record.counters().replay_bundle_width(),
        }
    }

    pub fn preview_session_identity(&self) -> &str {
        &self.preview_session_identity
    }

    pub fn preview_execution_record_identity(&self) -> &str {
        &self.preview_execution_record_identity
    }

    pub fn preview_declaration_digest(&self) -> &str {
        &self.preview_declaration_digest
    }

    pub fn branch_binding_digest(&self) -> &str {
        &self.branch_binding_digest
    }

    pub fn preview_artifact_count(&self) -> usize {
        self.preview_artifact_count
    }

    pub fn replay_bundle_width(&self) -> usize {
        self.replay_bundle_width
    }
}

impl BridgePreviewDiscardExplanation {
    pub fn from_record(record: &BridgePreviewDiscardRecord) -> Self {
        Self {
            preview_session_identity: record.preview_session_identity().to_owned(),
            preview_discard_record_identity: record.record_identity().as_str().to_owned(),
            preview_execution_record_identity: record
                .preview_execution_record_identity()
                .as_str()
                .to_owned(),
            cleanup_outcome: record.cleanup_outcome(),
            residue_classes: record.residue_report().residue_classes().to_vec(),
            authoritative_residue_count: record
                .residue_report()
                .authoritative_residue_count(),
            destroyed_artifact_count: record.counters().destroyed_artifact_count(),
        }
    }

    pub fn preview_session_identity(&self) -> &str {
        &self.preview_session_identity
    }

    pub fn preview_discard_record_identity(&self) -> &str {
        &self.preview_discard_record_identity
    }

    pub fn preview_execution_record_identity(&self) -> &str {
        &self.preview_execution_record_identity
    }

    pub fn cleanup_outcome(&self) -> crate::speculation::BridgePreviewDiscardCleanupOutcome {
        self.cleanup_outcome
    }

    pub fn residue_classes(&self) -> &[BridgePreviewResidueClass] {
        &self.residue_classes
    }

    pub fn authoritative_residue_count(&self) -> usize {
        self.authoritative_residue_count
    }

    pub fn destroyed_artifact_count(&self) -> usize {
        self.destroyed_artifact_count
    }
}

impl BridgePreviewPromotionExplanation {
    pub fn from_record(record: &BridgePreviewPromotionRecord) -> Self {
        Self {
            preview_session_identity: record.preview_session_identity().to_owned(),
            preview_promotion_record_identity: record.record_identity().as_str().to_owned(),
            preview_execution_record_identity: record
                .preview_execution_record_identity()
                .as_str()
                .to_owned(),
            authoritative_commit_boundary_digest: record
                .authoritative_commit_boundary_digest()
                .to_owned(),
            authoritative_artifact_digest: record.authoritative_artifact_digest().to_owned(),
            promotion_proof_digest: record.promotion_proof_digest().to_owned(),
            promotion_proof_checks: record.counters().promotion_proof_checks(),
        }
    }

    pub fn preview_session_identity(&self) -> &str {
        &self.preview_session_identity
    }

    pub fn preview_promotion_record_identity(&self) -> &str {
        &self.preview_promotion_record_identity
    }

    pub fn preview_execution_record_identity(&self) -> &str {
        &self.preview_execution_record_identity
    }

    pub fn authoritative_commit_boundary_digest(&self) -> &str {
        &self.authoritative_commit_boundary_digest
    }

    pub fn authoritative_artifact_digest(&self) -> &str {
        &self.authoritative_artifact_digest
    }

    pub fn promotion_proof_digest(&self) -> &str {
        &self.promotion_proof_digest
    }

    pub fn promotion_proof_checks(&self) -> usize {
        self.promotion_proof_checks
    }
}

impl BridgePreviewReplayExplanation {
    pub fn from_bundle(bundle: &BridgePreviewReplayBundle) -> Self {
        Self {
            preview_session_identity: bundle
                .preview_execution_record()
                .preview_session_identity()
                .to_owned(),
            lifecycle_outcome: bundle.lifecycle_outcome(),
            replay_bundle_digest: bundle.digest().to_owned(),
            replay_bundle_width: bundle.counters().replay_bundle_width(),
            has_discard_record: bundle.preview_discard_record().is_some(),
            has_promotion_record: bundle.preview_promotion_record().is_some(),
        }
    }

    pub fn preview_session_identity(&self) -> &str {
        &self.preview_session_identity
    }

    pub fn lifecycle_outcome(&self) -> BridgePreviewLifecycleStateKind {
        self.lifecycle_outcome
    }

    pub fn replay_bundle_digest(&self) -> &str {
        &self.replay_bundle_digest
    }

    pub fn replay_bundle_width(&self) -> usize {
        self.replay_bundle_width
    }

    pub fn has_discard_record(&self) -> bool {
        self.has_discard_record
    }

    pub fn has_promotion_record(&self) -> bool {
        self.has_promotion_record
    }
}
