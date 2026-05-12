use forge_proof::TransitionOutcome;

use crate::identities::{
    BoundaryArtifactId, BoundaryEpoch, BoundaryHandle, CanonicalDigestId, EquivalenceBasisId,
};

use super::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalIntegerWidth, CanonicalizationRuleVersion,
};

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
) -> TransitionOutcome<CanonicalBasisReadyArtifact, super::CanonicalBasisConstructionDenial> {
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

fn canonical_identity_entry(identity: CanonicalIdentityInput) -> CanonicalBasisEntry {
    match identity {
        CanonicalIdentityInput::BoundaryArtifact(id) => {
            numeric_identity_entry("boundary_artifact_id", u128::from(id.get()))
        }
        CanonicalIdentityInput::BoundaryHandle(handle) => {
            numeric_identity_entry("boundary_handle", u128::from(handle.get()))
        }
        CanonicalIdentityInput::EquivalenceBasis(id) => {
            numeric_identity_entry("equivalence_basis_id", u128::from(id.get()))
        }
        CanonicalIdentityInput::BoundaryEpoch(epoch) => {
            numeric_identity_entry("boundary_epoch", u128::from(epoch.get()))
        }
        CanonicalIdentityInput::CanonicalDigest(id) => CanonicalBasisEntry::new(
            CanonicalBasisDomain::Identity,
            CanonicalBasisLocus::Named("canonical_digest_id".into()),
            CanonicalBasisEntryKind::Identity,
            CanonicalBasisValue::BytesDigest(id),
        ),
    }
}

fn numeric_identity_entry(category: &'static str, value: u128) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Identity,
        CanonicalBasisLocus::Named(category.into()),
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value,
        },
    )
}
