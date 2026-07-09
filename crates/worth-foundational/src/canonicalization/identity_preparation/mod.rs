mod entries;

use worth_proof::TransitionOutcome;

use crate::identities::{
    BoundaryArtifactId, BoundaryEpoch, BoundaryHandle, CanonicalDigestId, EquivalenceBasisId,
};

use super::{
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};

use self::entries::canonical_identity_entry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalIdentityInput {
    BoundaryArtifact(BoundaryArtifactId),
    BoundaryHandle(BoundaryHandle),
    EquivalenceBasis(EquivalenceBasisId),
    BoundaryEpoch(BoundaryEpoch),
    CanonicalDigest(CanonicalDigestId),
}

pub fn prepare_identity_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    identity: CanonicalIdentityInput,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::Identity,
        [canonical_identity_entry(identity)],
    )
}

pub fn identity_canonical_basis_entries(
    ready: &CanonicalBasisReadyArtifact,
) -> &[CanonicalBasisEntry] {
    ready.payload().entries()
}
