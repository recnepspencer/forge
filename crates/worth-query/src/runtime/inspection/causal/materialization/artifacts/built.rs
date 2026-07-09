use super::super::super::identity::CausalInspectionArtifactIdentity;
use super::super::{
    CausalBridgeReadmissionProof, CausalInspectionBoundaryEnvelopeCategory,
    CausalInspectionPerformanceEnvelope, CausalMaterializationReceipt,
};
use super::bridge_backed::QueryCausalEvidenceReferenceArtifact;

pub(in crate::runtime::inspection::causal::materialization) struct BuiltBridgeBackedArtifact {
    pub(in crate::runtime::inspection::causal::materialization) boundary_categories:
        Vec<CausalInspectionBoundaryEnvelopeCategory>,
    pub(in crate::runtime::inspection::causal::materialization) evidence_references:
        Vec<QueryCausalEvidenceReferenceArtifact>,
    pub(in crate::runtime::inspection::causal::materialization) performance:
        CausalInspectionPerformanceEnvelope,
    pub(in crate::runtime::inspection::causal::materialization) receipt:
        CausalMaterializationReceipt,
    pub(in crate::runtime::inspection::causal::materialization) readmission_proof:
        CausalBridgeReadmissionProof,
    pub(in crate::runtime::inspection::causal::materialization) causal_identity:
        CausalInspectionArtifactIdentity,
    pub(in crate::runtime::inspection::causal::materialization) artifact_identity:
        CausalInspectionArtifactIdentity,
}
