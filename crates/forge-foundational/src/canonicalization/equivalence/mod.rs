use forge_proof::{Artifact, AuthorityWitness, TransitionOutcome};

use super::basis::CanonicalBasisConstructionAuthority;
use super::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisReadyArtifact,
    CanonicalComparisonReadinessProofs, CanonicalComparisonReady, CanonicalMismatchBasis,
    CanonicalMismatchKind, CanonicalizationRuleVersion,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalEquivalenceBasis {
    ExactCanonicalBasis,
    DeclaredAspectEquivalence,
    CompatibilityLoweredNativeEquivalence,
    ProjectionEquivalent,
    DigestEquivalent,
}

pub struct CanonicalComparisonInput {
    left: CanonicalBasisReadyArtifact,
    right: CanonicalBasisReadyArtifact,
    equivalence_basis: CanonicalEquivalenceBasis,
}

impl CanonicalComparisonInput {
    pub(crate) fn new(
        left: CanonicalBasisReadyArtifact,
        right: CanonicalBasisReadyArtifact,
        equivalence_basis: CanonicalEquivalenceBasis,
    ) -> Self {
        Self {
            left,
            right,
            equivalence_basis,
        }
    }

    pub fn left(&self) -> &CanonicalBasisReadyArtifact {
        &self.left
    }

    pub fn right(&self) -> &CanonicalBasisReadyArtifact {
        &self.right
    }

    pub const fn equivalence_basis(&self) -> CanonicalEquivalenceBasis {
        self.equivalence_basis
    }
}

pub type CanonicalComparisonReadyArtifact = Artifact<
    CanonicalComparisonReady,
    CanonicalComparisonInput,
    CanonicalComparisonReadinessProofs,
    forge_proof::FreshnessScopedBasis<
        forge_proof::CurrentValidity,
        forge_proof::AssumptionBasis<CanonicalEquivalenceBasis>,
    >,
>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalComparisonOutcome {
    Equivalent(CanonicalEquivalentBasis),
    Mismatched(CanonicalMismatchBasis),
    Unsupported(CanonicalMismatchBasis),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalEquivalentBasis {
    equivalence_basis: CanonicalEquivalenceBasis,
    left_version: CanonicalizationRuleVersion,
    right_version: CanonicalizationRuleVersion,
    domain: CanonicalBasisDomain,
    entry_count: u32,
}

impl CanonicalEquivalentBasis {
    fn new(input: &CanonicalComparisonInput) -> Self {
        Self {
            equivalence_basis: input.equivalence_basis(),
            left_version: input.left().payload().version().clone(),
            right_version: input.right().payload().version().clone(),
            domain: input.left().payload().domain(),
            entry_count: input.left().payload().entries().len() as u32,
        }
    }

    pub const fn equivalence_basis(&self) -> CanonicalEquivalenceBasis {
        self.equivalence_basis
    }

    pub fn left_version(&self) -> &CanonicalizationRuleVersion {
        &self.left_version
    }

    pub fn right_version(&self) -> &CanonicalizationRuleVersion {
        &self.right_version
    }

    pub const fn domain(&self) -> CanonicalBasisDomain {
        self.domain
    }

    pub const fn entry_count(&self) -> u32 {
        self.entry_count
    }
}

pub fn prepare_canonical_comparison(
    equivalence_basis: CanonicalEquivalenceBasis,
    left: CanonicalBasisReadyArtifact,
    right: CanonicalBasisReadyArtifact,
) -> TransitionOutcome<CanonicalComparisonReadyArtifact> {
    let input = CanonicalComparisonInput::new(left, right, equivalence_basis);
    let authority =
        AuthorityWitness::from_authority_marker(CanonicalBasisConstructionAuthority::new());
    let proofs = CanonicalComparisonReadinessProofs::new(
        forge_proof::Proof::from_authority_witness(&authority),
        forge_proof::Proof::from_authority_witness(&authority),
    );

    TransitionOutcome::success(Artifact::with_proofs_and_current_basis(
        input,
        proofs,
        equivalence_basis,
        authority,
    ))
}

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

fn first_mismatch(input: &CanonicalComparisonInput) -> Option<CanonicalMismatchBasis> {
    let mut left_iter = input.left().payload().entries().iter().peekable();
    let mut right_iter = input.right().payload().entries().iter().peekable();

    loop {
        match (left_iter.peek(), right_iter.peek()) {
            (Some(left), Some(right))
                if same_entry_locus(left, right) && left.kind() != right.kind() =>
            {
                return Some(CanonicalMismatchBasis::from_input(
                    input,
                    CanonicalMismatchKind::EntryKindMismatch,
                    left_iter.next(),
                    right_iter.next(),
                ));
            }
            (Some(left), Some(right)) => match compare_entry_keys(left, right) {
                std::cmp::Ordering::Equal => {
                    let left = left_iter.next().expect("peeked left");
                    let right = right_iter.next().expect("peeked right");
                    if left.value() != right.value() {
                        return Some(CanonicalMismatchBasis::from_input(
                            input,
                            CanonicalMismatchKind::ValueMismatch,
                            Some(left),
                            Some(right),
                        ));
                    }
                }
                std::cmp::Ordering::Less => {
                    return Some(CanonicalMismatchBasis::from_input(
                        input,
                        CanonicalMismatchKind::AdditionalEntry,
                        left_iter.next(),
                        None,
                    ));
                }
                std::cmp::Ordering::Greater => {
                    return Some(CanonicalMismatchBasis::from_input(
                        input,
                        CanonicalMismatchKind::MissingEntry,
                        None,
                        right_iter.next(),
                    ));
                }
            },
            (Some(_), None) => {
                return Some(CanonicalMismatchBasis::from_input(
                    input,
                    CanonicalMismatchKind::AdditionalEntry,
                    left_iter.next(),
                    None,
                ));
            }
            (None, Some(_)) => {
                return Some(CanonicalMismatchBasis::from_input(
                    input,
                    CanonicalMismatchKind::MissingEntry,
                    None,
                    right_iter.next(),
                ));
            }
            (None, None) => return None,
        }
    }
}

fn same_entry_locus(left: &CanonicalBasisEntry, right: &CanonicalBasisEntry) -> bool {
    left.domain() == right.domain() && left.locus() == right.locus()
}

fn compare_entry_keys(
    left: &CanonicalBasisEntry,
    right: &CanonicalBasisEntry,
) -> std::cmp::Ordering {
    (left.domain(), left.locus(), left.kind()).cmp(&(right.domain(), right.locus(), right.kind()))
}
