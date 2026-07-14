use super::super::DerivedIndexParityBasis;

/// Untrusted, canonically shaped candidate data read from derived-index storage.
///
/// Constructing this declaration does not establish rebuild or parity authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedIndexCandidateDeclaration {
    pub(super) basis: DerivedIndexParityBasis,
}

impl DerivedIndexCandidateDeclaration {
    pub const fn from_canonical_basis(basis: DerivedIndexParityBasis) -> Self {
        Self { basis }
    }

    pub const fn basis(&self) -> &DerivedIndexParityBasis {
        &self.basis
    }
}
