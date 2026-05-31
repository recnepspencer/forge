use forge_foundational::facade::{CanonicalBasisReadyArtifact, CanonicalBasisSequence};

pub(crate) fn foundational_canonical_basis_terms(
    ready: &CanonicalBasisReadyArtifact,
) -> &CanonicalBasisSequence {
    ready.payload()
}
