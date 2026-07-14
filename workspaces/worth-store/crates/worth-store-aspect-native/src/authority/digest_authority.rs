use worth_foundational::canonicalization_api::lower_lane::{
    basis::CanonicalBasisReadyArtifact,
    digest::{
        admit_canonical_sequence_digest_derivation, derive_canonical_digest,
        CanonicalDerivedDigest, CanonicalDigestAlgorithmId, CanonicalDigestDerivationDenial,
        CanonicalSingleSequenceDigestAlgorithmSlot,
    },
};
use worth_proof::TransitionOutcome;

use crate::canonical_basis::canonical_basis_domains::validate_store_native_basis_domain;
use crate::{
    canonical_basis_source_owner_for_family, StoreCanonicalBasisDomainMismatch,
    StoreCanonicalBasisFamily, StoreCanonicalBasisSourceDenial, StoreCanonicalBasisSourceKind,
    StoreDigestEquivalenceBasis, StoreDigestEquivalenceOutcome, StoreEquivalenceBasisIdentity,
};

pub type StoreDigestAuthorityOutcome =
    TransitionOutcome<StoreDigestEvidence, StoreDigestAuthorityDenial>;

#[derive(Debug, Clone)]
pub struct StoreDigestAuthority {
    family: StoreCanonicalBasisFamily,
    native_basis: CanonicalBasisReadyArtifact,
}

impl StoreDigestAuthority {
    pub const fn for_native_basis(
        family: StoreCanonicalBasisFamily,
        native_basis: CanonicalBasisReadyArtifact,
    ) -> Self {
        Self {
            family,
            native_basis,
        }
    }

    pub fn derive(self, algorithm_id: CanonicalDigestAlgorithmId) -> StoreDigestAuthorityOutcome {
        if let Err(mismatch) = validate_store_native_basis_domain(self.family, &self.native_basis) {
            return TransitionOutcome::denied(
                StoreDigestAuthorityDenial::NativeBasisFamilyDomainMismatch(mismatch),
            );
        }
        let source_kind = match primary_source_kind_for_family(self.family) {
            Ok(source_kind) => source_kind,
            Err(denial) => return TransitionOutcome::denied(denial),
        };
        let equivalence_basis = StoreDigestEquivalenceBasis::exact_native_basis(self.family);
        let slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
            algorithm_id,
            self.native_basis.payload().domain(),
            self.native_basis.payload().version().clone(),
        );
        let ready = match admit_canonical_sequence_digest_derivation(self.native_basis, slot) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(denial) => {
                return TransitionOutcome::denied(StoreDigestAuthorityDenial::Foundational(denial));
            }
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };

        TransitionOutcome::success(StoreDigestEvidence {
            family: self.family,
            source_kind,
            equivalence_basis: equivalence_basis.identity(),
            digest: derive_canonical_digest(ready),
        })
    }

    pub fn compare_native_basis(
        equivalence_basis: StoreDigestEquivalenceBasis,
        left: CanonicalBasisReadyArtifact,
        right: CanonicalBasisReadyArtifact,
    ) -> StoreDigestEquivalenceOutcome {
        equivalence_basis.compare_native_basis(left, right)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreDigestEvidence {
    family: StoreCanonicalBasisFamily,
    source_kind: StoreCanonicalBasisSourceKind,
    equivalence_basis: StoreEquivalenceBasisIdentity,
    digest: CanonicalDerivedDigest,
}

impl StoreDigestEvidence {
    pub const fn family(&self) -> StoreCanonicalBasisFamily {
        self.family
    }

    pub const fn source_kind(&self) -> StoreCanonicalBasisSourceKind {
        self.source_kind
    }

    pub const fn equivalence_basis_identity(&self) -> StoreEquivalenceBasisIdentity {
        self.equivalence_basis
    }

    pub const fn canonical_digest(&self) -> &CanonicalDerivedDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreDigestAuthorityDenial {
    Source(StoreCanonicalBasisSourceDenial),
    NativeBasisFamilyDomainMismatch(StoreCanonicalBasisDomainMismatch),
    MissingSourceRole { family: StoreCanonicalBasisFamily },
    Foundational(CanonicalDigestDerivationDenial),
}

impl From<StoreCanonicalBasisSourceDenial> for StoreDigestAuthorityDenial {
    fn from(value: StoreCanonicalBasisSourceDenial) -> Self {
        Self::Source(value)
    }
}

fn primary_source_kind_for_family(
    family: StoreCanonicalBasisFamily,
) -> Result<StoreCanonicalBasisSourceKind, StoreDigestAuthorityDenial> {
    let owner = canonical_basis_source_owner_for_family(family)?;
    owner
        .primary_source_kind()
        .ok_or(StoreDigestAuthorityDenial::MissingSourceRole { family })
}
