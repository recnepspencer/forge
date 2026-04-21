mod descriptor;
mod evidence;
mod expansion;

pub use descriptor::QueryScopeDescriptor;
pub use evidence::BasisScopeEvidence;
pub(crate) use expansion::expand_scopes;
pub(crate) use expansion::validate_basis_evidence_for_canonical_query;
pub use expansion::ExpandedScopeArtifact;
