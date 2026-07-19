mod bridge_backed;
mod built;
mod denied;

pub use bridge_backed::{
    AdmittedQueryCausalInspectionArtifact, AdvisoryQueryCausalInspectionArtifact,
    QueryCausalEvidenceReferenceArtifact, QueryCausalInspectionArtifact,
};
pub(in crate::runtime::inspection::causal::materialization) use built::BuiltBridgeBackedArtifact;
pub use denied::DeniedQueryCausalInspectionArtifact;
