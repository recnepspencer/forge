use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::intent_admission::ForgeQueryIntentDecisionTraceEnvelope;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::projection_consumption::ProjectionMaterializedFactPosture;
use crate::relationship_proof::{
    RelationshipProofAdmission, RelationshipProofSupportProfile, RelationshipProofSupportStatus,
};
use crate::runtime::{
    ForgeQueryAuthoritativeMutationObligationDispatch, ForgeQueryIntentExecutionProvenance,
};

use super::read_receipt_support::relationship_proof_support_surface_count;
use super::{
    ForgeQueryReadBreadth, ForgeQueryReadBuiltInOperator, ForgeQueryReadExecutionEngine,
    ForgeQueryReadFallbackClass, ForgeQueryReadGraphFamily, ForgeQueryReadOperatorFamily,
    ForgeQueryReadReceipt, ForgeQueryReadRelationshipProofPosture, ForgeQueryReadScopeClass,
};

impl ForgeQueryReadReceipt {
    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn graph_family(&self) -> &ForgeQueryReadGraphFamily {
        &self.graph_family
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn snapshot_evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        self.snapshot_identity.evidence_identity()
    }

    pub fn scope_class(&self) -> &ForgeQueryReadScopeClass {
        &self.scope_class
    }

    pub fn execution_engine(&self) -> &ForgeQueryReadExecutionEngine {
        &self.execution_engine
    }

    pub fn fallback_class(&self) -> &ForgeQueryReadFallbackClass {
        &self.fallback_class
    }

    pub fn fallback_count(&self) -> usize {
        self.fallback_count
    }

    pub fn operator_families(&self) -> &[ForgeQueryReadOperatorFamily] {
        &self.operator_families
    }

    pub fn built_in_operator_coverage(&self) -> &[ForgeQueryReadBuiltInOperator] {
        &self.built_in_operator_coverage
    }

    pub fn relationship_proof_posture(&self) -> &ForgeQueryReadRelationshipProofPosture {
        &self.relationship_proof_posture
    }

    pub fn relationship_proof_admission_identity(&self) -> Option<&str> {
        self.relationship_proof_admission
            .as_ref()
            .map(|admission| admission.identity().as_str())
    }

    pub fn relationship_proof_descriptor_count(&self) -> usize {
        self.relationship_proof_admission
            .as_ref()
            .map(RelationshipProofAdmission::descriptor_count)
            .unwrap_or(0)
    }

    pub fn relationship_proof_support_profile(&self) -> Option<&RelationshipProofSupportProfile> {
        self.relationship_proof_support_profile.as_ref()
    }

    pub fn relationship_proof_support_profile_digest(&self) -> Option<&str> {
        self.relationship_proof_support_profile
            .as_ref()
            .map(RelationshipProofSupportProfile::profile_digest)
    }

    pub fn relationship_proof_verified_surface_count(&self) -> usize {
        relationship_proof_support_surface_count(
            self.relationship_proof_support_profile(),
            RelationshipProofSupportStatus::Verified,
        )
    }

    pub fn relationship_proof_deferred_surface_count(&self) -> usize {
        relationship_proof_support_surface_count(
            self.relationship_proof_support_profile(),
            RelationshipProofSupportStatus::Deferred,
        )
    }

    pub fn relationship_proof_forbidden_surface_count(&self) -> usize {
        relationship_proof_support_surface_count(
            self.relationship_proof_support_profile(),
            RelationshipProofSupportStatus::Forbidden,
        )
    }

    pub fn breadth(&self) -> &ForgeQueryReadBreadth {
        &self.breadth
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

    pub(in crate::runtime) fn with_materialized_fact_posture(
        mut self,
        posture: Option<ProjectionMaterializedFactPosture>,
    ) -> Self {
        self.materialized_fact_posture = posture;
        self
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

    #[cfg(test)]
    pub(crate) fn test_only(
        read_graph_digest: impl Into<String>,
        query_digest: impl Into<String>,
        basis_digest: impl Into<String>,
        result_digest: impl Into<String>,
        execution_engine: ForgeQueryReadExecutionEngine,
    ) -> Self {
        Self {
            read_graph_digest: read_graph_digest.into(),
            graph_family: ForgeQueryReadGraphFamily::Collection,
            query_digest: query_digest.into(),
            basis_digest: basis_digest.into(),
            result_digest: result_digest.into(),
            snapshot_identity: crate::memory_workspace::admit_external_snapshot_label(
                "snapshot:test",
            ),
            scope_class: ForgeQueryReadScopeClass::ExplicitBroadSearch,
            execution_engine,
            fallback_class: ForgeQueryReadFallbackClass::None,
            fallback_count: 0,
            operator_families: Vec::new(),
            built_in_operator_coverage: Vec::new(),
            relationship_proof_posture: ForgeQueryReadRelationshipProofPosture::NotRequired,
            relationship_proof_admission: None,
            relationship_proof_support_profile: None,
            breadth: ForgeQueryReadBreadth {
                planned_read_surface_count: 0,
                planned_traversal_clause_count: 0,
                planned_traversal_depth_limit: 0,
                execution_read_operation_count: 0,
                execution_records_examined_count: 0,
                execution_records_emitted_count: 0,
                execution_page_width: 0,
                execution_page_truncation_count: 0,
                execution_cursor_advance_count: 0,
                execution_materialized_relation_count: 0,
            },
            materialized_fact_posture: None,
            graph_obligation_dispatch: None,
            decision_trace_envelope: None,
            execution_provenance: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn test_only_with_materialized_fact_posture(
        mut self,
        posture: ProjectionMaterializedFactPosture,
    ) -> Self {
        self.materialized_fact_posture = Some(posture);
        self
    }
}
