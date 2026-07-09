use super::CanonicalIdentityInput;
use crate::canonicalization::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth,
};

pub(super) fn canonical_identity_entry(identity: CanonicalIdentityInput) -> CanonicalBasisEntry {
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
