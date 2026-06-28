use forge_foundational::canonicalization_api::lower_lane::{
    basis::CanonicalBasisReadyArtifact,
    comparison::{
        compare_canonical_basis, prepare_canonical_comparison, CanonicalComparisonOutcome,
        CanonicalEquivalenceBasis,
    },
};
use forge_proof::TransitionOutcome;

use crate::canonical_basis_domains::validate_store_native_basis_domain;
use crate::{StoreCanonicalBasisDomainMismatch, StoreCanonicalBasisFamily};

pub type StoreDigestEquivalenceOutcome =
    TransitionOutcome<StoreDigestEquivalenceDecision, StoreDigestEquivalenceDenial>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreEquivalenceBasisIdentity {
    family: StoreCanonicalBasisFamily,
    foundational_basis: CanonicalEquivalenceBasis,
}

impl StoreEquivalenceBasisIdentity {
    const fn new(
        family: StoreCanonicalBasisFamily,
        foundational_basis: CanonicalEquivalenceBasis,
    ) -> Self {
        Self {
            family,
            foundational_basis,
        }
    }

    pub const fn family(self) -> StoreCanonicalBasisFamily {
        self.family
    }

    pub const fn foundational_basis(self) -> CanonicalEquivalenceBasis {
        self.foundational_basis
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreDigestEquivalenceBasis {
    identity: StoreEquivalenceBasisIdentity,
}

impl StoreDigestEquivalenceBasis {
    pub const fn exact_native_basis(family: StoreCanonicalBasisFamily) -> Self {
        Self {
            identity: StoreEquivalenceBasisIdentity::new(
                family,
                CanonicalEquivalenceBasis::ExactCanonicalBasis,
            ),
        }
    }

    pub fn from_foundational_basis(
        family: StoreCanonicalBasisFamily,
        foundational_basis: CanonicalEquivalenceBasis,
    ) -> Result<Self, StoreDigestEquivalenceDenial> {
        if foundational_basis != CanonicalEquivalenceBasis::ExactCanonicalBasis {
            return Err(StoreDigestEquivalenceDenial::NonNativeEquivalenceRejected {
                family,
                foundational_basis,
            });
        }

        Ok(Self {
            identity: StoreEquivalenceBasisIdentity::new(family, foundational_basis),
        })
    }

    pub const fn identity(self) -> StoreEquivalenceBasisIdentity {
        self.identity
    }

    pub const fn family(self) -> StoreCanonicalBasisFamily {
        self.identity.family()
    }

    pub const fn foundational_basis(self) -> CanonicalEquivalenceBasis {
        self.identity.foundational_basis()
    }

    pub fn compare_native_basis(
        self,
        left: CanonicalBasisReadyArtifact,
        right: CanonicalBasisReadyArtifact,
    ) -> StoreDigestEquivalenceOutcome {
        if let Err(mismatch) = validate_store_native_basis_domain(self.family(), &left) {
            return TransitionOutcome::denied(
                StoreDigestEquivalenceDenial::NativeBasisFamilyDomainMismatch(mismatch),
            );
        }
        if let Err(mismatch) = validate_store_native_basis_domain(self.family(), &right) {
            return TransitionOutcome::denied(
                StoreDigestEquivalenceDenial::NativeBasisFamilyDomainMismatch(mismatch),
            );
        }

        let ready = match prepare_canonical_comparison(self.foundational_basis(), left, right) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(value) => match value {},
            TransitionOutcome::Deferred(value) => match value {},
            TransitionOutcome::Stale(value) => match value {},
            TransitionOutcome::RebindRequired(value) => match value {},
            TransitionOutcome::Failed(value) => match value {},
        };
        let outcome = compare_canonical_basis(&ready);

        TransitionOutcome::success(StoreDigestEquivalenceDecision {
            basis: self.identity,
            outcome,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreDigestEquivalenceDecision {
    basis: StoreEquivalenceBasisIdentity,
    outcome: CanonicalComparisonOutcome,
}

impl StoreDigestEquivalenceDecision {
    pub const fn basis(&self) -> StoreEquivalenceBasisIdentity {
        self.basis
    }

    pub const fn outcome(&self) -> &CanonicalComparisonOutcome {
        &self.outcome
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreDigestEquivalenceOperation {
    Reuse,
    Parity,
    Suppression,
    DigestComparison,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreDigestEquivalenceDenial {
    BasisRequired {
        operation: StoreDigestEquivalenceOperation,
    },
    NativeBasisFamilyDomainMismatch(StoreCanonicalBasisDomainMismatch),
    NonNativeEquivalenceRejected {
        family: StoreCanonicalBasisFamily,
        foundational_basis: CanonicalEquivalenceBasis,
    },
}

pub const fn deny_basis_free_reuse() -> StoreDigestEquivalenceDenial {
    StoreDigestEquivalenceDenial::BasisRequired {
        operation: StoreDigestEquivalenceOperation::Reuse,
    }
}

pub const fn deny_basis_free_parity() -> StoreDigestEquivalenceDenial {
    StoreDigestEquivalenceDenial::BasisRequired {
        operation: StoreDigestEquivalenceOperation::Parity,
    }
}

pub const fn deny_basis_free_suppression() -> StoreDigestEquivalenceDenial {
    StoreDigestEquivalenceDenial::BasisRequired {
        operation: StoreDigestEquivalenceOperation::Suppression,
    }
}

pub const fn deny_basis_free_digest_comparison() -> StoreDigestEquivalenceDenial {
    StoreDigestEquivalenceDenial::BasisRequired {
        operation: StoreDigestEquivalenceOperation::DigestComparison,
    }
}
