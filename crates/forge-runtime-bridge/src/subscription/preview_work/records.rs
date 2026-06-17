use std::collections::BTreeSet;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::evidence::{
    BridgeSubscriptionPreviewWorkEvidence, BridgeSubscriptionPreviewWorkInput,
    BridgeSubscriptionPreviewWorkKind,
};
use super::rejection::{
    BridgeSubscriptionPreviewWorkTraceRejection, BridgeSubscriptionPreviewWorkTraceRejectionKind,
};
use crate::subscription::{
    BridgeAdmittedSubscriptionIdentity, BridgePreviewActiveSubscription,
    BridgePreviewActiveSubscriptionIdentity, BridgeSubscriptionPreviewBasisIdentity,
    BridgeSubscriptionPreviewResidueArtifactInput, BridgeSubscriptionPreviewResidueCategory,
    BridgeSubscriptionPreviewResidueScopeIdentity, BridgeSubscriptionPreviewScopeIdentity,
    BridgeSubscriptionPreviewWorkRecordIdentity, BridgeSubscriptionPreviewWorkTraceIdentity,
};

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
    evidence: BridgeSubscriptionPreviewWorkEvidence,
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
            input.kind().as_str(),
            input.evidence().digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            preview_work_record_identity:
                BridgeSubscriptionPreviewWorkRecordIdentity::admit_bridge_owned(format!(
                    "bridge-subscription-preview-work-record-id:sha256:{digest:x}"
                )),
            preview_active_subscription_identity: preview_active
                .preview_active_subscription_identity()
                .clone(),
            preview_basis_identity: preview_active.preview_basis_identity().clone(),
            preview_scope_identity: preview_active.preview_scope_identity().clone(),
            preview_residue_scope_identity: preview_active.preview_residue_scope_identity().clone(),
            admitted_subscription_identity: preview_active.admitted_subscription_identity().clone(),
            slot_index,
            kind: input.kind(),
            evidence: input.evidence().clone(),
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

    pub fn evidence(&self) -> &BridgeSubscriptionPreviewWorkEvidence {
        &self.evidence
    }

    pub fn evidence_digest(&self) -> &str {
        self.evidence.digest()
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
        validate_preview_work_inputs(preview_active, &inputs)?;
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
            preview_work_trace_identity:
                BridgeSubscriptionPreviewWorkTraceIdentity::admit_bridge_owned(format!(
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
            BridgeSubscriptionPreviewResidueCategory::AuthoritativeTruthSubscription,
            BridgeSubscriptionPreviewResidueCategory::BridgeSubscriptionRegistry,
            BridgeSubscriptionPreviewResidueCategory::ActiveDelivery,
            BridgeSubscriptionPreviewResidueCategory::FanoutConsumerContract,
            BridgeSubscriptionPreviewResidueCategory::Continuation,
            BridgeSubscriptionPreviewResidueCategory::CheckpointReplay,
            BridgeSubscriptionPreviewResidueCategory::SignalVisible,
        ]
        .into_iter()
        .map(|category| {
            BridgeSubscriptionPreviewResidueArtifactInput::zero_from_preview_work_trace(
                category, self,
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

fn validate_preview_work_inputs(
    preview_active: &BridgePreviewActiveSubscription,
    inputs: &[BridgeSubscriptionPreviewWorkInput],
) -> Result<(), BridgeSubscriptionPreviewWorkTraceRejection> {
    let mut seen = BTreeSet::new();
    for input in inputs {
        validate_input_evidence_scope(preview_active, input.evidence())?;
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
    Ok(())
}

fn validate_input_evidence_scope(
    preview_active: &BridgePreviewActiveSubscription,
    evidence: &BridgeSubscriptionPreviewWorkEvidence,
) -> Result<(), BridgeSubscriptionPreviewWorkTraceRejection> {
    if evidence.preview_active_subscription_identity()
        != preview_active.preview_active_subscription_identity()
        || evidence.preview_basis_identity() != preview_active.preview_basis_identity()
        || evidence.preview_scope_identity() != preview_active.preview_scope_identity()
        || evidence.preview_lifecycle_identity() != preview_active.preview_lifecycle_identity()
        || evidence.source_preview_digest() != preview_active.digest()
    {
        return Err(BridgeSubscriptionPreviewWorkTraceRejection::new(
            preview_active,
            BridgeSubscriptionPreviewWorkTraceRejectionKind::PreviewWorkEvidenceMismatch,
            format!(
                "preview-active={}|evidence-preview-active={}|kind={}",
                preview_active
                    .preview_active_subscription_identity()
                    .as_str(),
                evidence.preview_active_subscription_identity().as_str(),
                evidence.kind().as_str(),
            ),
        ));
    }
    Ok(())
}
