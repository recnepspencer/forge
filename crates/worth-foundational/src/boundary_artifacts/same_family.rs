use worth_proof::TransitionOutcome;

use super::basis::prepare_materialized_boundary_artifact_for_canonical_basis;
use super::materialization::FoundationalMaterializedBoundaryArtifact;
use super::roles::FoundationalBoundaryArtifactRole;
use crate::canonicalization::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest,
    prepare_canonical_basis_sequence, CanonicalBasisConstructionDenial, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact,
    CanonicalBasisSequence, CanonicalBasisValue, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalDigestDerivationDenial,
    CanonicalSingleSequenceDigestAlgorithmSlot, CanonicalizationRuleVersion,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalSameFamilyBoundaryFamily(String);

impl FoundationalSameFamilyBoundaryFamily {
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, FoundationalSameFamilyBoundaryFamilyDenial> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(FoundationalSameFamilyBoundaryFamilyDenial::FamilyMustNotBeBlank);
        }
        if value.contains(char::is_whitespace) {
            return Err(FoundationalSameFamilyBoundaryFamilyDenial::FamilyMustNotContainWhitespace);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalSameFamilyBoundaryFamilyDenial {
    FamilyMustNotBeBlank,
    FamilyMustNotContainWhitespace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalSameFamilyBoundaryArtifactDenial {
    BoundaryRoleMustRemainDescriptive,
    BasisConstructionDenied(CanonicalBasisConstructionDenial),
    DigestDerivationDenied(CanonicalDigestDerivationDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalSameFamilyBoundaryArtifact<Surface> {
    artifact: FoundationalMaterializedBoundaryArtifact<Surface>,
    family: FoundationalSameFamilyBoundaryFamily,
}

impl<Surface> FoundationalSameFamilyBoundaryArtifact<Surface> {
    fn new(
        artifact: FoundationalMaterializedBoundaryArtifact<Surface>,
        family: FoundationalSameFamilyBoundaryFamily,
    ) -> Self {
        Self { artifact, family }
    }

    pub const fn artifact(&self) -> &FoundationalMaterializedBoundaryArtifact<Surface> {
        &self.artifact
    }

    pub const fn surface(&self) -> &Surface {
        self.artifact.surface()
    }

    pub fn family(&self) -> &FoundationalSameFamilyBoundaryFamily {
        &self.family
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalSameFamilyBoundaryIdentity {
    family: FoundationalSameFamilyBoundaryFamily,
    basis: CanonicalBasisSequence,
    digest: CanonicalDerivedDigest,
}

impl FoundationalSameFamilyBoundaryIdentity {
    pub fn family(&self) -> &FoundationalSameFamilyBoundaryFamily {
        &self.family
    }

    pub fn basis(&self) -> &CanonicalBasisSequence {
        &self.basis
    }

    pub fn digest(&self) -> &CanonicalDerivedDigest {
        &self.digest
    }
}

impl PartialEq for FoundationalSameFamilyBoundaryIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.family == other.family
            && self.digest == other.digest
            && self.basis.version() == other.basis.version()
            && self.basis.domain() == other.basis.domain()
            && self.basis.entries() == other.basis.entries()
    }
}

impl Eq for FoundationalSameFamilyBoundaryIdentity {}

pub fn admit_same_family_boundary_artifact<Surface>(
    artifact: FoundationalMaterializedBoundaryArtifact<Surface>,
    family: FoundationalSameFamilyBoundaryFamily,
) -> Result<
    FoundationalSameFamilyBoundaryArtifact<Surface>,
    FoundationalSameFamilyBoundaryArtifactDenial,
> {
    match artifact.role() {
        FoundationalBoundaryArtifactRole::DerivedProjection
        | FoundationalBoundaryArtifactRole::SupportOnly
        | FoundationalBoundaryArtifactRole::PlannedWork => {}
        FoundationalBoundaryArtifactRole::AuthoritativeCurrent
        | FoundationalBoundaryArtifactRole::ReceiptEvidence => {
            return Err(
                FoundationalSameFamilyBoundaryArtifactDenial::BoundaryRoleMustRemainDescriptive,
            );
        }
    }

    Ok(FoundationalSameFamilyBoundaryArtifact::new(
        artifact, family,
    ))
}

pub fn prepare_same_family_boundary_artifact_for_canonical_basis<Surface>(
    version: CanonicalizationRuleVersion,
    artifact: &FoundationalSameFamilyBoundaryArtifact<Surface>,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial> {
    let base = match prepare_materialized_boundary_artifact_for_canonical_basis(
        version.clone(),
        artifact.artifact(),
    ) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("boundary basis preparation uses only denied")
        }
    };

    let mut entries = vec![same_family_text_entry(
        "same_family.family",
        artifact.family().as_str(),
    )];
    entries.extend(base.payload().entries().iter().cloned());

    prepare_canonical_basis_sequence(version, CanonicalBasisDomain::BoundaryArtifact, entries)
}

pub fn derive_same_family_boundary_identity<Surface>(
    version: CanonicalizationRuleVersion,
    artifact: &FoundationalSameFamilyBoundaryArtifact<Surface>,
) -> TransitionOutcome<
    FoundationalSameFamilyBoundaryIdentity,
    FoundationalSameFamilyBoundaryArtifactDenial,
> {
    let basis = match prepare_same_family_boundary_artifact_for_canonical_basis(
        version.clone(),
        artifact,
    ) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            return TransitionOutcome::denied(
                FoundationalSameFamilyBoundaryArtifactDenial::BasisConstructionDenied(denial),
            );
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("same-family basis preparation uses only denied")
        }
    };
    let basis_sequence = basis.payload().clone();

    let slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
        CanonicalDigestAlgorithmId::test_stable_fixture(),
        CanonicalBasisDomain::BoundaryArtifact,
        version,
    );
    let derivation = match admit_canonical_sequence_digest_derivation(basis, slot) {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            return TransitionOutcome::denied(
                FoundationalSameFamilyBoundaryArtifactDenial::DigestDerivationDenied(denial),
            );
        }
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => {
            unreachable!("same-family digest admission uses only denied")
        }
    };

    TransitionOutcome::success(FoundationalSameFamilyBoundaryIdentity {
        family: artifact.family().clone(),
        basis: basis_sequence,
        digest: derive_canonical_digest(derivation),
    })
}

fn same_family_text_entry(locus: &'static str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::BoundaryArtifact,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::BoundaryArtifact,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}
