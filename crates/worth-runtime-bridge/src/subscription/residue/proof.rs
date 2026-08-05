use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionPreviewDiscardResidueRejection,
    BridgeSubscriptionPreviewDiscardResidueRejectionContext,
    BridgeSubscriptionPreviewDiscardResidueRejectionKind,
    BridgeSubscriptionPreviewResidueArtifactRecord, BridgeSubscriptionPreviewResidueCategory,
    BridgeSubscriptionPreviewResidueCategoryCount, BridgeSubscriptionPreviewResidueScopeIndex,
};
use crate::subscription::{
    BridgePreviewActiveSubscription, BridgePreviewActiveSubscriptionIdentity,
    BridgeSubscriptionCounters, BridgeSubscriptionPreviewDiscardResidueProofIdentity,
    BridgeSubscriptionPreviewResidueScopeIdentity,
    BridgeSubscriptionPreviewResidueScopeIndexIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewDiscardResidueProof {
    proof_identity: BridgeSubscriptionPreviewDiscardResidueProofIdentity,
    preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
    preview_residue_scope_index_identity: BridgeSubscriptionPreviewResidueScopeIndexIdentity,
    preview_residue_scope_identity: BridgeSubscriptionPreviewResidueScopeIdentity,
    artifact_records: Arc<[BridgeSubscriptionPreviewResidueArtifactRecord]>,
    total_residue_count: usize,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewDiscardResidueProof {
    pub(crate) fn prove(
        preview_active: BridgePreviewActiveSubscription,
        residue_scope_index: BridgeSubscriptionPreviewResidueScopeIndex,
    ) -> Result<Self, BridgeSubscriptionPreviewDiscardResidueRejection> {
        if residue_scope_index.preview_active_subscription_identity()
            != preview_active.preview_active_subscription_identity()
        {
            return Err(BridgeSubscriptionPreviewDiscardResidueRejection::new(
                BridgeSubscriptionPreviewDiscardResidueRejectionKind::PreviewActiveMismatch,
                BridgeSubscriptionPreviewDiscardResidueRejectionContext::preview_active_mismatch(
                    preview_active
                        .preview_active_subscription_identity()
                        .clone(),
                    residue_scope_index
                        .preview_active_subscription_identity()
                        .clone(),
                ),
                false,
                0,
            ));
        }
        if residue_scope_index.preview_residue_scope_identity()
            != preview_active.preview_residue_scope_identity()
        {
            return Err(BridgeSubscriptionPreviewDiscardResidueRejection::new(
                BridgeSubscriptionPreviewDiscardResidueRejectionKind::PreviewResidueScopeMismatch,
                BridgeSubscriptionPreviewDiscardResidueRejectionContext::preview_residue_scope_mismatch(
                    preview_active.preview_residue_scope_identity().clone(),
                    residue_scope_index.preview_residue_scope_identity().clone(),
                ),
                false,
                0,
            ));
        }

        let residue_check_count = residue_scope_index.artifact_records().len();
        let mut category_counts =
            BTreeMap::<BridgeSubscriptionPreviewResidueCategory, usize>::new();
        let mut seen_categories = BTreeSet::<BridgeSubscriptionPreviewResidueCategory>::new();
        for record in residue_scope_index.artifact_records() {
            if !seen_categories.insert(record.category()) {
                return Err(BridgeSubscriptionPreviewDiscardResidueRejection::new(
                    BridgeSubscriptionPreviewDiscardResidueRejectionKind::DuplicateResidueCategory,
                    BridgeSubscriptionPreviewDiscardResidueRejectionContext::duplicate_category(
                        preview_active
                            .preview_active_subscription_identity()
                            .clone(),
                        record.category(),
                    ),
                    false,
                    residue_check_count,
                ));
            }
            *category_counts.entry(record.category()).or_default() += record.residue_count();
        }
        for required_category in BridgeSubscriptionPreviewResidueCategory::all() {
            if !seen_categories.contains(&required_category) {
                return Err(BridgeSubscriptionPreviewDiscardResidueRejection::new(
                    BridgeSubscriptionPreviewDiscardResidueRejectionKind::MissingResidueCategory,
                    BridgeSubscriptionPreviewDiscardResidueRejectionContext::missing_category(
                        preview_active
                            .preview_active_subscription_identity()
                            .clone(),
                        required_category,
                    ),
                    false,
                    residue_check_count,
                ));
            }
        }

        let total_residue_count = category_counts.values().sum::<usize>();
        if total_residue_count != 0 {
            let nonzero_categories = category_counts
                .iter()
                .filter(|&(_category, count)| *count != 0)
                .map(|(category, count)| {
                    BridgeSubscriptionPreviewResidueCategoryCount::new(*category, *count)
                })
                .collect::<Vec<_>>();
            return Err(BridgeSubscriptionPreviewDiscardResidueRejection::new(
                BridgeSubscriptionPreviewDiscardResidueRejectionKind::NonzeroResidue,
                BridgeSubscriptionPreviewDiscardResidueRejectionContext::nonzero_residue(
                    preview_active
                        .preview_active_subscription_identity()
                        .clone(),
                    nonzero_categories,
                ),
                true,
                residue_check_count,
            ));
        }

        let artifact_digest_list = residue_scope_index
            .artifact_records()
            .iter()
            .map(BridgeSubscriptionPreviewResidueArtifactRecord::digest)
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-discard-residue-proof|preview-active={}|preview-basis={}|scope-index={}|residue-scope={}|artifacts={}|total-residue={}",
            preview_active.preview_active_subscription_identity().as_str(),
            preview_active.preview_basis_identity().as_str(),
            residue_scope_index
                .preview_residue_scope_index_identity()
                .as_str(),
            preview_active.preview_residue_scope_identity().as_str(),
            artifact_digest_list,
            total_residue_count,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            proof_identity:
                BridgeSubscriptionPreviewDiscardResidueProofIdentity::admit_bridge_owned(format!(
                    "bridge-subscription-preview-discard-residue-proof-id:sha256:{digest:x}"
                )),
            preview_active_subscription_identity: preview_active
                .preview_active_subscription_identity()
                .clone(),
            preview_residue_scope_index_identity: residue_scope_index
                .preview_residue_scope_index_identity()
                .clone(),
            preview_residue_scope_identity: preview_active.preview_residue_scope_identity().clone(),
            artifact_records: Arc::from(residue_scope_index.artifact_records().to_vec()),
            total_residue_count,
            counters: BridgeSubscriptionCounters::from_subscription_preview_discard(
                residue_check_count,
            ),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-discard-residue-proof:sha256:{digest:x}"
            )),
        })
    }

    pub fn proof_identity(&self) -> &BridgeSubscriptionPreviewDiscardResidueProofIdentity {
        &self.proof_identity
    }

    pub fn preview_active_subscription_identity(&self) -> &BridgePreviewActiveSubscriptionIdentity {
        &self.preview_active_subscription_identity
    }

    pub fn preview_residue_scope_index_identity(
        &self,
    ) -> &BridgeSubscriptionPreviewResidueScopeIndexIdentity {
        &self.preview_residue_scope_index_identity
    }

    pub fn preview_residue_scope_identity(&self) -> &BridgeSubscriptionPreviewResidueScopeIdentity {
        &self.preview_residue_scope_identity
    }

    pub fn artifact_records(&self) -> &[BridgeSubscriptionPreviewResidueArtifactRecord] {
        &self.artifact_records
    }

    pub fn total_residue_count(&self) -> usize {
        self.total_residue_count
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
