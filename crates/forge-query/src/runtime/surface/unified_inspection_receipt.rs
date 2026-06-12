use crate::intent_admission::ForgeQueryGenericInspectionRequestLabel;
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::{
    ForgeQueryInspection, ForgeQueryIntentConsumerInspection, ForgeQueryIntentExecutionProvenance,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryUnifiedInspectionReceipt {
    target_label: ForgeQueryGenericInspectionRequestLabel,
    result_digest: String,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    snapshot_evidence_identity: crate::evidence_identity::ForgeQueryEvidenceIdentity,
    pub(super) decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
    pub(super) execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
}

impl ForgeQueryUnifiedInspectionReceipt {
    pub(in crate::runtime) fn from_inspection(
        target_label: ForgeQueryGenericInspectionRequestLabel,
        inspection: &ForgeQueryInspection,
        snapshot_identity: ForgeQuerySnapshotIdentity,
    ) -> Self {
        let snapshot_evidence_identity = snapshot_identity.evidence_identity();
        Self {
            target_label,
            result_digest: inspection_result_digest(inspection).to_string(),
            snapshot_identity,
            snapshot_evidence_identity,
            decision_trace_envelope: None,
            execution_provenance: None,
        }
    }

    pub fn target_label(&self) -> &str {
        self.target_label.as_str()
    }

    pub fn typed_target_label(&self) -> &ForgeQueryGenericInspectionRequestLabel {
        &self.target_label
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn snapshot_evidence_identity(
        &self,
    ) -> &crate::evidence_identity::ForgeQueryEvidenceIdentity {
        &self.snapshot_evidence_identity
    }

    pub fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.decision_trace_envelope.as_ref()
    }

    pub fn execution_provenance(&self) -> Option<&ForgeQueryIntentExecutionProvenance> {
        self.execution_provenance.as_ref()
    }

    pub fn execution_provenance_chain_digest(&self) -> Option<&str> {
        self.execution_provenance
            .as_ref()
            .map(ForgeQueryIntentExecutionProvenance::execution_provenance_chain_digest)
    }

    pub fn consumer_inspection(&self) -> Option<ForgeQueryIntentConsumerInspection<'_>> {
        Some(ForgeQueryIntentConsumerInspection::from_unified_inspection_receipt(self))
    }
}

pub(super) fn inspection_result_digest(inspection: &ForgeQueryInspection) -> &str {
    match inspection {
        ForgeQueryInspection::LiveView(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::DerivedView(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::Effect(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::WriteReceipt(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::BatchWriteReceipt(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::IntentReceipt(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::IntentDenial(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::EffectIntentReceipt(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::PreviewBinding(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::PreviewOutcome(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::PreviewIntentReceipt(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::BranchIntentReceipt(inspection) => inspection.inspection_digest(),
        ForgeQueryInspection::BasisLifecycle(inspection) => inspection.inspection_digest(),
    }
}
