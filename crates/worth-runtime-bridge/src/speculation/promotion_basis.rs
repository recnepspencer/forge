use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::contracts::BridgePromotionAdmissibilityProof;
use super::execution::BridgePreviewExecutionRecord;
use super::session::{BridgePreviewSession, PreviewExecutionRecordIdentity};
use super::taxonomy::PreviewActive;
use crate::error::{BridgeSpeculationError, BridgeSpeculationErrorKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePreviewPromotionAuthorityBasis {
    preview_session_identity: Arc<str>,
    preview_execution_record_identity: PreviewExecutionRecordIdentity,
    preview_execution_record_digest: Arc<str>,
    promotion_proof_digest: Arc<str>,
    authoritative_commit_boundary_digest: Arc<str>,
    authoritative_artifact_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePreviewPromotionAuthorityBasis {
    pub fn from_active_session(
        session: &BridgePreviewSession<PreviewActive>,
        execution_record: &BridgePreviewExecutionRecord,
        proof: &BridgePromotionAdmissibilityProof,
    ) -> Result<Self, BridgeSpeculationError> {
        let execution_record_identity = session
            .execution_record_identity()
            .expect("active preview sessions must carry execution record identity")
            .clone();

        if execution_record.record_identity() != &execution_record_identity {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PreviewBranchBindingMismatch,
                format!(
                    "Promotion authority basis rejected execution record `{}` for active preview session `{}`.",
                    execution_record.record_identity().as_str(),
                    session.session_identity().as_str(),
                ),
            ));
        }

        if !proof.matches_active_session(session) {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PromotionAdmissibilityMismatch,
                format!(
                    "Promotion authority basis rejected proof `{}` for active preview session `{}`.",
                    proof.proof_identity().as_str(),
                    session.session_identity().as_str(),
                ),
            ));
        }

        let declaration = session.declaration().declaration();
        let canonical_basis = Arc::<str>::from(format!(
            "preview-promotion-authority-basis|session={}|execution-record={}|execution-digest={}|declaration={}|binding={}|truth-view={}|structural-basis={}|source-capability={}|request-shape={}|artifact-schema={}|proof={}",
            session.session_identity().as_str(),
            execution_record_identity.as_str(),
            execution_record.digest(),
            session.declaration().digest(),
            declaration.branch_binding().digest(),
            declaration.truth_view_basis_digest(),
            declaration.structural_basis_digest().unwrap_or("none"),
            declaration.source_capability_digest(),
            declaration.request_shape_digest(),
            declaration.retained_artifact_schema_digest(),
            proof.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let digest_text = Arc::<str>::from(format!(
            "preview-promotion-authority-basis:sha256:{digest:x}"
        ));
        let authoritative_commit_boundary_digest = Arc::<str>::from(format!(
            "preview-promotion-commit-boundary:sha256:{digest:x}"
        ));
        let authoritative_artifact_digest = Arc::<str>::from(format!(
            "preview-promotion-authoritative-artifact:sha256:{digest:x}"
        ));

        Ok(Self {
            preview_session_identity: Arc::from(session.session_identity().as_str()),
            preview_execution_record_identity: execution_record_identity,
            preview_execution_record_digest: Arc::from(execution_record.digest()),
            promotion_proof_digest: Arc::from(proof.digest()),
            authoritative_commit_boundary_digest,
            authoritative_artifact_digest,
            canonical_basis,
            digest: digest_text,
        })
    }

    pub fn preview_session_identity(&self) -> &str {
        self.preview_session_identity.as_ref()
    }

    pub fn preview_execution_record_identity(&self) -> &PreviewExecutionRecordIdentity {
        &self.preview_execution_record_identity
    }

    pub fn preview_execution_record_digest(&self) -> &str {
        self.preview_execution_record_digest.as_ref()
    }

    pub fn promotion_proof_digest(&self) -> &str {
        self.promotion_proof_digest.as_ref()
    }

    pub fn authoritative_commit_boundary_digest(&self) -> &str {
        self.authoritative_commit_boundary_digest.as_ref()
    }

    pub fn authoritative_artifact_digest(&self) -> &str {
        self.authoritative_artifact_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
