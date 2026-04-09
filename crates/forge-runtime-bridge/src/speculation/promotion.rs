use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, PreviewPromotionRecordIdentityTag};

use super::contracts::BridgePromotionAdmissibilityProof;
use super::counters::BridgeSpeculationCounters;
use super::execution::BridgePreviewExecutionRecord;
use super::session::{BridgePreviewSession, PreviewExecutionRecordIdentity};
use super::taxonomy::PreviewActive;

pub type BridgePreviewPromotionRecordIdentity =
    BridgeIdentity<PreviewPromotionRecordIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePreviewPromotionRecord {
    record_identity: BridgePreviewPromotionRecordIdentity,
    preview_session_identity: Arc<str>,
    preview_execution_record_identity: PreviewExecutionRecordIdentity,
    promotion_proof_digest: Arc<str>,
    authoritative_commit_boundary_digest: Arc<str>,
    authoritative_artifact_digest: Arc<str>,
    counters: BridgeSpeculationCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePreviewPromotionRecord {
    pub fn from_active_session(
        session: &BridgePreviewSession<PreviewActive>,
        execution_record: &BridgePreviewExecutionRecord,
        proof: &BridgePromotionAdmissibilityProof,
        authoritative_commit_boundary_digest: impl Into<Arc<str>>,
        authoritative_artifact_digest: impl Into<Arc<str>>,
        counters: BridgeSpeculationCounters,
    ) -> Self {
        let execution_record_identity = session
            .execution_record_identity()
            .expect("active preview sessions must carry execution record identity")
            .clone();
        let authoritative_commit_boundary_digest = authoritative_commit_boundary_digest.into();
        let authoritative_artifact_digest = authoritative_artifact_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "preview-promotion-record|session={}|execution-record={}|execution-digest={}|proof={}|commit-boundary={}|authoritative-artifact={}|proof-width={}|proof-checks={}|replay-width={}",
            session.session_identity().as_str(),
            execution_record_identity.as_str(),
            execution_record.digest(),
            proof.digest(),
            authoritative_commit_boundary_digest.as_ref(),
            authoritative_artifact_digest.as_ref(),
            counters.admissibility_proof_width(),
            counters.promotion_proof_checks(),
            counters.replay_bundle_width(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            record_identity: BridgePreviewPromotionRecordIdentity::new(format!(
                "preview-promotion-record:sha256:{digest:x}"
            )),
            preview_session_identity: Arc::from(session.session_identity().as_str()),
            preview_execution_record_identity: execution_record_identity,
            promotion_proof_digest: Arc::from(proof.digest()),
            authoritative_commit_boundary_digest,
            authoritative_artifact_digest,
            counters,
            canonical_basis,
            digest: Arc::from(format!("preview-promotion-record:sha256:{digest:x}")),
        }
    }

    pub fn record_identity(&self) -> &BridgePreviewPromotionRecordIdentity {
        &self.record_identity
    }

    pub fn preview_session_identity(&self) -> &str {
        self.preview_session_identity.as_ref()
    }

    pub fn preview_execution_record_identity(&self) -> &PreviewExecutionRecordIdentity {
        &self.preview_execution_record_identity
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

    pub fn counters(&self) -> &BridgeSpeculationCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
