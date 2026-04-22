use std::collections::BTreeSet;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeAdmittedSubscriptionIdentity, BridgePreviewActiveSubscription,
    BridgePreviewActiveSubscriptionIdentity, BridgeSubscriptionPreviewBasisIdentity,
    BridgeSubscriptionPreviewResidueArtifactInput, BridgeSubscriptionPreviewResidueCategory,
    BridgeSubscriptionPreviewResidueScopeIdentity, BridgeSubscriptionPreviewScopeIdentity,
    BridgeSubscriptionPreviewWorkRecordIdentity, BridgeSubscriptionPreviewWorkTraceIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionPreviewWorkKind {
    Routing,
    Delivery,
    Diagnostics,
    Continuation,
}

impl BridgeSubscriptionPreviewWorkKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Routing => "routing",
            Self::Delivery => "delivery",
            Self::Diagnostics => "diagnostics",
            Self::Continuation => "continuation",
        }
    }

    const fn all() -> [Self; 4] {
        [
            Self::Routing,
            Self::Delivery,
            Self::Diagnostics,
            Self::Continuation,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewWorkInput {
    kind: BridgeSubscriptionPreviewWorkKind,
    evidence_digest: Arc<str>,
}

impl BridgeSubscriptionPreviewWorkInput {
    pub fn routing(evidence_digest: impl Into<Arc<str>>) -> Self {
        Self::new(BridgeSubscriptionPreviewWorkKind::Routing, evidence_digest)
    }

    pub fn delivery(evidence_digest: impl Into<Arc<str>>) -> Self {
        Self::new(BridgeSubscriptionPreviewWorkKind::Delivery, evidence_digest)
    }

    pub fn diagnostics(evidence_digest: impl Into<Arc<str>>) -> Self {
        Self::new(
            BridgeSubscriptionPreviewWorkKind::Diagnostics,
            evidence_digest,
        )
    }

    pub fn continuation(evidence_digest: impl Into<Arc<str>>) -> Self {
        Self::new(
            BridgeSubscriptionPreviewWorkKind::Continuation,
            evidence_digest,
        )
    }

    fn new(kind: BridgeSubscriptionPreviewWorkKind, evidence_digest: impl Into<Arc<str>>) -> Self {
        Self {
            kind,
            evidence_digest: evidence_digest.into(),
        }
    }

    pub fn kind(&self) -> BridgeSubscriptionPreviewWorkKind {
        self.kind
    }

    pub fn evidence_digest(&self) -> &str {
        self.evidence_digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionPreviewWorkTraceRejectionKind {
    EmptyEvidenceDigest,
    DuplicateWorkKind,
    MissingWorkKind,
}

impl BridgeSubscriptionPreviewWorkTraceRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyEvidenceDigest => "empty_evidence_digest",
            Self::DuplicateWorkKind => "duplicate_work_kind",
            Self::MissingWorkKind => "missing_work_kind",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewWorkTraceRejection {
    rejection_kind: BridgeSubscriptionPreviewWorkTraceRejectionKind,
    rejection_context: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewWorkTraceRejection {
    fn new(
        preview_active: &BridgePreviewActiveSubscription,
        rejection_kind: BridgeSubscriptionPreviewWorkTraceRejectionKind,
        rejection_context: impl Into<Arc<str>>,
    ) -> Self {
        let rejection_context = rejection_context.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-work-trace-rejection|preview-active={}|scope={}|kind={}|context={}",
            preview_active.preview_active_subscription_identity().as_str(),
            preview_active.preview_scope_identity().as_str(),
            rejection_kind.as_str(),
            rejection_context.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            rejection_context,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-work-trace-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionPreviewWorkTraceRejectionKind {
        self.rejection_kind
    }

    pub fn rejection_context(&self) -> &str {
        self.rejection_context.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewWorkRecord {
    preview_work_record_identity: BridgeSubscriptionPreviewWorkRecordIdentity,
    preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
    preview_basis_identity: BridgeSubscriptionPreviewBasisIdentity,
    preview_scope_identity: BridgeSubscriptionPreviewScopeIdentity,
    preview_residue_scope_identity: BridgeSubscriptionPreviewResidueScopeIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    slot_index: usize,
    kind: BridgeSubscriptionPreviewWorkKind,
    evidence_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewWorkRecord {
    fn from_input(
        preview_active: &BridgePreviewActiveSubscription,
        slot_index: usize,
        input: BridgeSubscriptionPreviewWorkInput,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-work-record|preview-active={}|preview-basis={}|scope={}|residue-scope={}|admitted={}|slot={slot_index}|kind={}|evidence={}",
            preview_active.preview_active_subscription_identity().as_str(),
            preview_active.preview_basis_identity().as_str(),
            preview_active.preview_scope_identity().as_str(),
            preview_active.preview_residue_scope_identity().as_str(),
            preview_active.admitted_subscription_identity().as_str(),
            input.kind.as_str(),
            input.evidence_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            preview_work_record_identity: BridgeSubscriptionPreviewWorkRecordIdentity::new(
                format!("bridge-subscription-preview-work-record-id:sha256:{digest:x}"),
            ),
            preview_active_subscription_identity: preview_active
                .preview_active_subscription_identity()
                .clone(),
            preview_basis_identity: preview_active.preview_basis_identity().clone(),
            preview_scope_identity: preview_active.preview_scope_identity().clone(),
            preview_residue_scope_identity: preview_active.preview_residue_scope_identity().clone(),
            admitted_subscription_identity: preview_active.admitted_subscription_identity().clone(),
            slot_index,
            kind: input.kind,
            evidence_digest: input.evidence_digest,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-work-record:sha256:{digest:x}"
            )),
        }
    }

    pub fn preview_work_record_identity(&self) -> &BridgeSubscriptionPreviewWorkRecordIdentity {
        &self.preview_work_record_identity
    }

    pub fn preview_active_subscription_identity(&self) -> &BridgePreviewActiveSubscriptionIdentity {
        &self.preview_active_subscription_identity
    }

    pub fn preview_scope_identity(&self) -> &BridgeSubscriptionPreviewScopeIdentity {
        &self.preview_scope_identity
    }

    pub fn preview_residue_scope_identity(&self) -> &BridgeSubscriptionPreviewResidueScopeIdentity {
        &self.preview_residue_scope_identity
    }

    pub fn slot_index(&self) -> usize {
        self.slot_index
    }

    pub fn kind(&self) -> BridgeSubscriptionPreviewWorkKind {
        self.kind
    }

    pub fn evidence_digest(&self) -> &str {
        self.evidence_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewWorkTrace {
    preview_work_trace_identity: BridgeSubscriptionPreviewWorkTraceIdentity,
    preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
    preview_basis_identity: BridgeSubscriptionPreviewBasisIdentity,
    preview_scope_identity: BridgeSubscriptionPreviewScopeIdentity,
    preview_residue_scope_identity: BridgeSubscriptionPreviewResidueScopeIdentity,
    admitted_subscription_identity: BridgeAdmittedSubscriptionIdentity,
    records: Arc<[BridgeSubscriptionPreviewWorkRecord]>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewWorkTrace {
    pub(crate) fn record(
        preview_active: &BridgePreviewActiveSubscription,
        inputs: Vec<BridgeSubscriptionPreviewWorkInput>,
    ) -> Result<Self, BridgeSubscriptionPreviewWorkTraceRejection> {
        let mut seen = BTreeSet::new();
        for input in &inputs {
            if input.evidence_digest().is_empty() {
                return Err(BridgeSubscriptionPreviewWorkTraceRejection::new(
                    preview_active,
                    BridgeSubscriptionPreviewWorkTraceRejectionKind::EmptyEvidenceDigest,
                    format!("kind={}", input.kind().as_str()),
                ));
            }
            if !seen.insert(input.kind()) {
                return Err(BridgeSubscriptionPreviewWorkTraceRejection::new(
                    preview_active,
                    BridgeSubscriptionPreviewWorkTraceRejectionKind::DuplicateWorkKind,
                    format!("kind={}", input.kind().as_str()),
                ));
            }
        }
        for required_kind in BridgeSubscriptionPreviewWorkKind::all() {
            if !seen.contains(&required_kind) {
                return Err(BridgeSubscriptionPreviewWorkTraceRejection::new(
                    preview_active,
                    BridgeSubscriptionPreviewWorkTraceRejectionKind::MissingWorkKind,
                    format!("kind={}", required_kind.as_str()),
                ));
            }
        }

        let records = inputs
            .into_iter()
            .enumerate()
            .map(|(slot_index, input)| {
                BridgeSubscriptionPreviewWorkRecord::from_input(preview_active, slot_index, input)
            })
            .collect::<Vec<_>>();
        let record_digest_basis = records
            .iter()
            .map(BridgeSubscriptionPreviewWorkRecord::digest)
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-work-trace|preview-active={}|preview-basis={}|scope={}|residue-scope={}|admitted={}|records={}",
            preview_active.preview_active_subscription_identity().as_str(),
            preview_active.preview_basis_identity().as_str(),
            preview_active.preview_scope_identity().as_str(),
            preview_active.preview_residue_scope_identity().as_str(),
            preview_active.admitted_subscription_identity().as_str(),
            record_digest_basis,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            preview_work_trace_identity: BridgeSubscriptionPreviewWorkTraceIdentity::new(format!(
                "bridge-subscription-preview-work-trace-id:sha256:{digest:x}"
            )),
            preview_active_subscription_identity: preview_active
                .preview_active_subscription_identity()
                .clone(),
            preview_basis_identity: preview_active.preview_basis_identity().clone(),
            preview_scope_identity: preview_active.preview_scope_identity().clone(),
            preview_residue_scope_identity: preview_active.preview_residue_scope_identity().clone(),
            admitted_subscription_identity: preview_active.admitted_subscription_identity().clone(),
            records: Arc::from(records),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-work-trace:sha256:{digest:x}"
            )),
        })
    }

    pub fn preview_work_trace_identity(&self) -> &BridgeSubscriptionPreviewWorkTraceIdentity {
        &self.preview_work_trace_identity
    }

    pub fn preview_active_subscription_identity(&self) -> &BridgePreviewActiveSubscriptionIdentity {
        &self.preview_active_subscription_identity
    }

    pub fn preview_scope_identity(&self) -> &BridgeSubscriptionPreviewScopeIdentity {
        &self.preview_scope_identity
    }

    pub fn preview_residue_scope_identity(&self) -> &BridgeSubscriptionPreviewResidueScopeIdentity {
        &self.preview_residue_scope_identity
    }

    pub fn records(&self) -> &[BridgeSubscriptionPreviewWorkRecord] {
        &self.records
    }

    pub fn record_digest_for(&self, kind: BridgeSubscriptionPreviewWorkKind) -> &str {
        self.records
            .iter()
            .find(|record| record.kind() == kind)
            .expect("preview work trace construction requires every work kind")
            .digest()
    }

    pub fn zero_residue_inputs(&self) -> Vec<BridgeSubscriptionPreviewResidueArtifactInput> {
        [
            (
                BridgeSubscriptionPreviewResidueCategory::AuthoritativeTruthSubscription,
                self.record_digest_for(BridgeSubscriptionPreviewWorkKind::Routing),
            ),
            (
                BridgeSubscriptionPreviewResidueCategory::BridgeSubscriptionRegistry,
                self.record_digest_for(BridgeSubscriptionPreviewWorkKind::Routing),
            ),
            (
                BridgeSubscriptionPreviewResidueCategory::ActiveDelivery,
                self.record_digest_for(BridgeSubscriptionPreviewWorkKind::Delivery),
            ),
            (
                BridgeSubscriptionPreviewResidueCategory::FanoutConsumerContract,
                self.record_digest_for(BridgeSubscriptionPreviewWorkKind::Delivery),
            ),
            (
                BridgeSubscriptionPreviewResidueCategory::Continuation,
                self.record_digest_for(BridgeSubscriptionPreviewWorkKind::Continuation),
            ),
            (
                BridgeSubscriptionPreviewResidueCategory::CheckpointReplay,
                self.record_digest_for(BridgeSubscriptionPreviewWorkKind::Continuation),
            ),
            (
                BridgeSubscriptionPreviewResidueCategory::SignalVisible,
                self.record_digest_for(BridgeSubscriptionPreviewWorkKind::Diagnostics),
            ),
        ]
        .into_iter()
        .map(|(category, record_digest)| {
            BridgeSubscriptionPreviewResidueArtifactInput::zero(
                category,
                format!(
                    "preview-work-zero-residue|trace={}|scope={}|record={record_digest}|category={}",
                    self.digest(),
                    self.preview_residue_scope_identity.as_str(),
                    category.as_str(),
                ),
            )
        })
        .collect()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
