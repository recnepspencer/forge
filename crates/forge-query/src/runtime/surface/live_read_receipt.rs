use crate::identity::CanonicalResultShapeDigest;
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::projection_consumption::ProjectionMaterializedFactPosture;
use crate::runtime::{
    ForgeQueryAuthoritativeMutationObligationDispatch, ForgeQueryIntentConsumerInspection,
    ForgeQueryIntentExecutionProvenance, ForgeQueryRuntimeLiveSubscriptionInstallation,
};

use super::read_receipt_support::materialized_result_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveReadReceipt {
    view_name: String,
    installation_digest: String,
    query_digest: String,
    canonical_result_shape_digest: CanonicalResultShapeDigest,
    canonical_result_shape_identity: crate::evidence_identity::ForgeQueryEvidenceIdentity,
    subscription_family_digest: String,
    result_digest: String,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    snapshot_evidence_identity: crate::evidence_identity::ForgeQueryEvidenceIdentity,
    row_count: usize,
    materialized_fact_posture: Option<ProjectionMaterializedFactPosture>,
    pub(super) graph_obligation_dispatch: Option<ForgeQueryAuthoritativeMutationObligationDispatch>,
    pub(super) decision_trace_envelope: Option<ForgeQueryIntentDecisionTraceEnvelope>,
    pub(super) execution_provenance: Option<ForgeQueryIntentExecutionProvenance>,
}

impl ForgeQueryLiveReadReceipt {
    pub(in crate::runtime) fn from_rows(
        installation: &ForgeQueryRuntimeLiveSubscriptionInstallation,
        snapshot_identity: ForgeQuerySnapshotIdentity,
        materialized_fact_posture: Option<ProjectionMaterializedFactPosture>,
        rows: &[crate::memory_workspace::ForgeQueryEntity],
    ) -> Self {
        let snapshot_evidence_identity = snapshot_identity.evidence_identity();
        Self {
            view_name: installation.view_name().to_string(),
            installation_digest: installation.installation_projection().label().to_string(),
            query_digest: installation.query_projection().label().to_string(),
            canonical_result_shape_digest: installation.canonical_result_shape_digest().clone(),
            canonical_result_shape_identity: installation.canonical_result_shape_identity().clone(),
            subscription_family_digest: installation
                .subscription_family_projection()
                .label()
                .to_string(),
            result_digest: materialized_result_digest(
                installation.query_projection().label(),
                installation.basis_binding_projection().label(),
                rows,
            )
            .as_str()
            .to_string(),
            snapshot_identity,
            snapshot_evidence_identity,
            row_count: rows.len(),
            materialized_fact_posture,
            graph_obligation_dispatch: None,
            decision_trace_envelope: None,
            execution_provenance: None,
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn installation_digest(&self) -> &str {
        &self.installation_digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn view_shape_digest(&self) -> &str {
        self.canonical_result_shape_digest.as_str()
    }

    pub fn canonical_result_shape_digest(&self) -> &CanonicalResultShapeDigest {
        &self.canonical_result_shape_digest
    }

    pub fn canonical_result_shape_identity(
        &self,
    ) -> &crate::evidence_identity::ForgeQueryEvidenceIdentity {
        &self.canonical_result_shape_identity
    }

    pub fn subscription_family_digest(&self) -> &str {
        &self.subscription_family_digest
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

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn materialized_fact_posture(&self) -> Option<&ProjectionMaterializedFactPosture> {
        self.materialized_fact_posture.as_ref()
    }

    pub fn graph_obligation_dispatch(
        &self,
    ) -> Option<&ForgeQueryAuthoritativeMutationObligationDispatch> {
        self.graph_obligation_dispatch.as_ref()
    }

    pub fn graph_obligation_envelope_digest(&self) -> Option<&str> {
        self.graph_obligation_dispatch
            .as_ref()
            .and_then(ForgeQueryAuthoritativeMutationObligationDispatch::envelope_digest)
    }

    pub fn graph_obligation_evidence(
        &self,
    ) -> Option<crate::runtime::ForgeQueryGraphObligationAttachmentEvidence> {
        self.graph_obligation_dispatch
            .as_ref()
            .map(|dispatch| dispatch.attachment_evidence())
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
            .map(|provenance| provenance.execution_provenance_chain_digest())
    }

    pub fn consumer_inspection(&self) -> Option<ForgeQueryIntentConsumerInspection<'_>> {
        Some(ForgeQueryIntentConsumerInspection::from_live_read_receipt(
            self,
        ))
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        view_name: impl Into<String>,
        installation_digest: impl Into<String>,
        query_digest: impl Into<String>,
        canonical_result_shape_digest: CanonicalResultShapeDigest,
        subscription_family_digest: impl Into<String>,
        result_digest: impl Into<String>,
        snapshot_identity: ForgeQuerySnapshotIdentity,
        row_count: usize,
    ) -> Self {
        let snapshot_evidence_identity = snapshot_identity.evidence_identity();
        Self {
            view_name: view_name.into(),
            installation_digest: installation_digest.into(),
            query_digest: query_digest.into(),
            canonical_result_shape_identity: canonical_result_shape_digest.evidence_identity(),
            canonical_result_shape_digest,
            subscription_family_digest: subscription_family_digest.into(),
            result_digest: result_digest.into(),
            snapshot_identity,
            snapshot_evidence_identity,
            row_count,
            materialized_fact_posture: None,
            graph_obligation_dispatch: None,
            decision_trace_envelope: None,
            execution_provenance: None,
        }
    }
}
