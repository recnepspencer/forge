use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgePreviewActiveSubscription, BridgePreviewActiveSubscriptionIdentity,
    BridgeSubscriptionCounters, BridgeSubscriptionPreviewDiscardResidueProofIdentity,
    BridgeSubscriptionPreviewResidueArtifactIdentity,
    BridgeSubscriptionPreviewResidueScopeIdentity,
    BridgeSubscriptionPreviewResidueScopeIndexIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionPreviewResidueCategory {
    AuthoritativeTruthSubscription,
    BridgeSubscriptionRegistry,
    ActiveDelivery,
    FanoutConsumerContract,
    Continuation,
    CheckpointReplay,
    SignalVisible,
}

impl BridgeSubscriptionPreviewResidueCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeTruthSubscription => "authoritative_truth_subscription",
            Self::BridgeSubscriptionRegistry => "bridge_subscription_registry",
            Self::ActiveDelivery => "active_delivery",
            Self::FanoutConsumerContract => "fanout_consumer_contract",
            Self::Continuation => "continuation",
            Self::CheckpointReplay => "checkpoint_replay",
            Self::SignalVisible => "signal_visible",
        }
    }

    const fn all() -> [Self; 7] {
        [
            Self::AuthoritativeTruthSubscription,
            Self::BridgeSubscriptionRegistry,
            Self::ActiveDelivery,
            Self::FanoutConsumerContract,
            Self::Continuation,
            Self::CheckpointReplay,
            Self::SignalVisible,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewResidueArtifactInput {
    category: BridgeSubscriptionPreviewResidueCategory,
    residue_count: usize,
    evidence_digest: Arc<str>,
}

impl BridgeSubscriptionPreviewResidueArtifactInput {
    pub fn new(
        category: BridgeSubscriptionPreviewResidueCategory,
        residue_count: usize,
        evidence_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            category,
            residue_count,
            evidence_digest: evidence_digest.into(),
        }
    }

    pub fn zero(
        category: BridgeSubscriptionPreviewResidueCategory,
        evidence_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self::new(category, 0, evidence_digest)
    }

    pub fn category(&self) -> BridgeSubscriptionPreviewResidueCategory {
        self.category
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewResidueArtifactRecord {
    artifact_identity: BridgeSubscriptionPreviewResidueArtifactIdentity,
    preview_residue_scope_identity: BridgeSubscriptionPreviewResidueScopeIdentity,
    category: BridgeSubscriptionPreviewResidueCategory,
    residue_count: usize,
    evidence_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewResidueArtifactRecord {
    fn from_input(
        preview_residue_scope_identity: &BridgeSubscriptionPreviewResidueScopeIdentity,
        input: BridgeSubscriptionPreviewResidueArtifactInput,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-residue-artifact|scope={}|category={}|residue-count={}|evidence={}",
            preview_residue_scope_identity.as_str(),
            input.category.as_str(),
            input.residue_count,
            input.evidence_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            artifact_identity: BridgeSubscriptionPreviewResidueArtifactIdentity::new(format!(
                "bridge-subscription-preview-residue-artifact-id:sha256:{digest:x}"
            )),
            preview_residue_scope_identity: preview_residue_scope_identity.clone(),
            category: input.category,
            residue_count: input.residue_count,
            evidence_digest: input.evidence_digest,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-residue-artifact:sha256:{digest:x}"
            )),
        }
    }

    pub fn artifact_identity(&self) -> &BridgeSubscriptionPreviewResidueArtifactIdentity {
        &self.artifact_identity
    }

    pub fn preview_residue_scope_identity(&self) -> &BridgeSubscriptionPreviewResidueScopeIdentity {
        &self.preview_residue_scope_identity
    }

    pub fn category(&self) -> BridgeSubscriptionPreviewResidueCategory {
        self.category
    }

    pub fn residue_count(&self) -> usize {
        self.residue_count
    }

    pub fn evidence_digest(&self) -> &str {
        self.evidence_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewResidueScopeIndex {
    preview_residue_scope_index_identity: BridgeSubscriptionPreviewResidueScopeIndexIdentity,
    preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
    preview_residue_scope_identity: BridgeSubscriptionPreviewResidueScopeIdentity,
    artifact_records: Arc<[BridgeSubscriptionPreviewResidueArtifactRecord]>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewResidueScopeIndex {
    pub(crate) fn build(
        preview_active: &BridgePreviewActiveSubscription,
        artifact_inputs: Vec<BridgeSubscriptionPreviewResidueArtifactInput>,
    ) -> Self {
        let mut artifact_records = artifact_inputs
            .into_iter()
            .map(|input| {
                BridgeSubscriptionPreviewResidueArtifactRecord::from_input(
                    preview_active.preview_residue_scope_identity(),
                    input,
                )
            })
            .collect::<Vec<_>>();
        artifact_records.sort_by(|left, right| {
            left.category()
                .cmp(&right.category())
                .then_with(|| left.digest().cmp(right.digest()))
        });
        let artifact_digest_list = artifact_records
            .iter()
            .map(BridgeSubscriptionPreviewResidueArtifactRecord::digest)
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-residue-scope-index|preview-active={}|scope={}|artifacts={}",
            preview_active.preview_active_subscription_identity().as_str(),
            preview_active.preview_residue_scope_identity().as_str(),
            artifact_digest_list,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            preview_residue_scope_index_identity:
                BridgeSubscriptionPreviewResidueScopeIndexIdentity::new(format!(
                    "bridge-subscription-preview-residue-scope-index-id:sha256:{digest:x}"
                )),
            preview_active_subscription_identity: preview_active
                .preview_active_subscription_identity()
                .clone(),
            preview_residue_scope_identity: preview_active.preview_residue_scope_identity().clone(),
            counters: BridgeSubscriptionCounters::from_subscription_preview_residue_scope_index(
                artifact_records.len(),
            ),
            artifact_records: Arc::from(artifact_records),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-residue-scope-index:sha256:{digest:x}"
            )),
        }
    }

    pub fn preview_residue_scope_index_identity(
        &self,
    ) -> &BridgeSubscriptionPreviewResidueScopeIndexIdentity {
        &self.preview_residue_scope_index_identity
    }

    pub fn preview_active_subscription_identity(&self) -> &BridgePreviewActiveSubscriptionIdentity {
        &self.preview_active_subscription_identity
    }

    pub fn preview_residue_scope_identity(&self) -> &BridgeSubscriptionPreviewResidueScopeIdentity {
        &self.preview_residue_scope_identity
    }

    pub fn artifact_records(&self) -> &[BridgeSubscriptionPreviewResidueArtifactRecord] {
        &self.artifact_records
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionPreviewDiscardResidueRejectionKind {
    PreviewActiveMismatch,
    PreviewResidueScopeMismatch,
    MissingResidueCategory,
    DuplicateResidueCategory,
    NonzeroResidue,
}

impl BridgeSubscriptionPreviewDiscardResidueRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewActiveMismatch => "preview_active_mismatch",
            Self::PreviewResidueScopeMismatch => "preview_residue_scope_mismatch",
            Self::MissingResidueCategory => "missing_residue_category",
            Self::DuplicateResidueCategory => "duplicate_residue_category",
            Self::NonzeroResidue => "nonzero_residue",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewDiscardResidueRejection {
    rejection_kind: BridgeSubscriptionPreviewDiscardResidueRejectionKind,
    rejection_context: Arc<str>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewDiscardResidueRejection {
    fn new(
        rejection_kind: BridgeSubscriptionPreviewDiscardResidueRejectionKind,
        rejection_context: impl Into<Arc<str>>,
        nonzero_residue: bool,
        residue_check_count: usize,
    ) -> Self {
        let rejection_context = rejection_context.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-discard-residue-rejection|kind={}|context={}",
            rejection_kind.as_str(),
            rejection_context.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            rejection_context,
            counters: BridgeSubscriptionCounters::from_subscription_preview_discard_rejection(
                nonzero_residue,
                residue_check_count,
            ),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-discard-residue-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionPreviewDiscardResidueRejectionKind {
        self.rejection_kind
    }

    pub fn rejection_context(&self) -> &str {
        self.rejection_context.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

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
                format!(
                    "preview-active={}|index-preview-active={}",
                    preview_active
                        .preview_active_subscription_identity()
                        .as_str(),
                    residue_scope_index
                        .preview_active_subscription_identity()
                        .as_str(),
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
                format!(
                    "preview-scope={}|index-scope={}",
                    preview_active.preview_residue_scope_identity().as_str(),
                    residue_scope_index
                        .preview_residue_scope_identity()
                        .as_str(),
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
                    format!(
                        "preview-active={}|duplicate-category={}",
                        preview_active
                            .preview_active_subscription_identity()
                            .as_str(),
                        record.category().as_str(),
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
                    format!(
                        "preview-active={}|missing-category={}",
                        preview_active
                            .preview_active_subscription_identity()
                            .as_str(),
                        required_category.as_str(),
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
                .filter_map(|(category, count)| {
                    (*count != 0).then(|| format!("{}={count}", category.as_str()))
                })
                .collect::<Vec<_>>()
                .join(",");
            return Err(BridgeSubscriptionPreviewDiscardResidueRejection::new(
                BridgeSubscriptionPreviewDiscardResidueRejectionKind::NonzeroResidue,
                format!(
                    "preview-active={}|nonzero={}",
                    preview_active
                        .preview_active_subscription_identity()
                        .as_str(),
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
            proof_identity: BridgeSubscriptionPreviewDiscardResidueProofIdentity::new(format!(
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
