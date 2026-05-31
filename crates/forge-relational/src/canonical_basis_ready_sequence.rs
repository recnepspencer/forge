use forge_foundational::facade::{CanonicalBasisReadyArtifact, CanonicalBasisSequence};

pub(crate) fn canonical_basis_ready_sequence(
    ready: &CanonicalBasisReadyArtifact,
) -> &CanonicalBasisSequence {
    ready.payload()
}
