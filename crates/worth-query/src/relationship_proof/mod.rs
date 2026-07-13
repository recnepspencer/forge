mod admission;
mod counters;
mod descriptors;
mod errors;
mod support;

pub use admission::admit_relationship_proofs;
pub(crate) use admission::admit_relationship_proofs_for_query_identity;
pub use counters::RelationshipProofCounters;
pub use descriptors::{
    RelationshipProofAdmission, RelationshipProofAdmissionIdentity, RelationshipProofBudget,
    RelationshipProofDescriptor, RelationshipProofDescriptorSet, RelationshipProofTopologyClass,
};
pub use errors::{RelationshipProofError, RelationshipProofFailureClass};
pub use support::{
    runtime_backed_relationship_proof_support_profile, RelationshipProofSupportProfile,
    RelationshipProofSupportStatus, RelationshipProofSurface,
};

#[cfg(test)]
mod tests;
