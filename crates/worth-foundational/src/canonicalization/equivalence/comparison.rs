use super::basis::CanonicalEquivalenceBasis;
use super::mismatch_search::first_mismatch;
use super::outcome::{CanonicalComparisonOutcome, CanonicalEquivalentBasis};
use super::readiness::{CanonicalComparisonInput, CanonicalComparisonReadyArtifact};
use crate::canonicalization::{
    CanonicalBasisDomain, CanonicalMismatchBasis, CanonicalMismatchKind,
};

pub fn compare_canonical_basis(
    ready: &CanonicalComparisonReadyArtifact,
) -> CanonicalComparisonOutcome {
    let input = ready.payload();

    if input.left().payload().version() != input.right().payload().version() {
        return CanonicalComparisonOutcome::Unsupported(CanonicalMismatchBasis::from_input(
            input,
            CanonicalMismatchKind::VersionMismatch,
            input.left().payload().entries().first(),
            input.right().payload().entries().first(),
        ));
    }

    if !equivalence_basis_admits_domain_pair(input) {
        return CanonicalComparisonOutcome::Unsupported(CanonicalMismatchBasis::from_input(
            input,
            CanonicalMismatchKind::UnsupportedComparison,
            input.left().payload().entries().first(),
            input.right().payload().entries().first(),
        ));
    }

    match first_mismatch(input) {
        Some(mismatch) => CanonicalComparisonOutcome::Mismatched(mismatch),
        None => CanonicalComparisonOutcome::Equivalent(CanonicalEquivalentBasis::new(input)),
    }
}

fn equivalence_basis_admits_domain_pair(input: &CanonicalComparisonInput) -> bool {
    let left_domain = input.left().payload().domain();
    let right_domain = input.right().payload().domain();

    match input.equivalence_basis() {
        CanonicalEquivalenceBasis::ExactCanonicalBasis
        | CanonicalEquivalenceBasis::DeclaredAspectEquivalence => left_domain == right_domain,
        CanonicalEquivalenceBasis::CompatibilityLoweredNativeEquivalence => {
            matches!(left_domain, CanonicalBasisDomain::AuthoritativeState)
                && matches!(right_domain, CanonicalBasisDomain::AuthoritativeState)
        }
        CanonicalEquivalenceBasis::ProjectionEquivalent
        | CanonicalEquivalenceBasis::DigestEquivalent => false,
    }
}
