use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::{
    ForgeQueryComputedInspectionEvidence, ForgeQueryIntentConsumerInspection,
    ForgeQueryIntentExecutionProvenance,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDerivedInspectionReceipt {
    view_name: String,
    dependency_digest: String,
    materialization_digest: String,
    inspection_digest: String,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    snapshot_evidence_identity: crate::evidence_identity::ForgeQueryEvidenceIdentity,
    row_count: usize,
    pub(super) decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
    pub(super) execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
}

impl ForgeQueryDerivedInspectionReceipt {
    pub(in crate::runtime) fn from_evidence(
        evidence: &ForgeQueryComputedInspectionEvidence,
        snapshot_identity: ForgeQuerySnapshotIdentity,
    ) -> Self {
        let snapshot_evidence_identity = snapshot_identity.evidence_identity();
        Self {
            view_name: evidence.name().to_string(),
            dependency_digest: evidence.dependency_digest().to_string(),
            materialization_digest: evidence.materialization_digest().to_string(),
            inspection_digest: evidence.inspection_digest().to_string(),
            snapshot_identity,
            snapshot_evidence_identity,
            row_count: evidence.materialized_row_count(),
            decision_trace_envelope: None,
            execution_provenance: None,
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn dependency_digest(&self) -> &str {
        &self.dependency_digest
    }

    pub fn materialization_digest(&self) -> &str {
        &self.materialization_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.inspection_digest
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn snapshot_evidence_identity(
        &self,
    ) -> &crate::evidence_identity::ForgeQueryEvidenceIdentity {
        &self.snapshot_evidence_identity
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn decision_trace_envelope(&self) -> Option<&ForgeQueryIntentDecisionTraceEnvelope> {
        self.decision_trace_envelope.as_ref()
    }

    pub fn execution_provenance(&self) -> Option<&ForgeQueryIntentExecutionProvenance> {
        self.execution_provenance.as_ref()
    }

    pub fn consumer_inspection(&self) -> Option<ForgeQueryIntentConsumerInspection<'_>> {
        Some(ForgeQueryIntentConsumerInspection::from_derived_inspection_receipt(self))
    }
}
