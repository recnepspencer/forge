use worth_proof::{Artifact, AuthorityWitness, TransitionOutcome};

use super::basis::CanonicalEquivalenceBasis;
use crate::canonicalization::basis::CanonicalBasisConstructionAuthority;
use crate::canonicalization::{
    CanonicalBasisReadyArtifact, CanonicalComparisonReadinessProofs, CanonicalComparisonReady,
};

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
    worth_proof::FreshnessScopedBasis<
        worth_proof::CurrentValidity,
        worth_proof::AssumptionBasis<CanonicalEquivalenceBasis>,
    >,
>;

pub fn prepare_canonical_comparison(
    equivalence_basis: CanonicalEquivalenceBasis,
    left: CanonicalBasisReadyArtifact,
    right: CanonicalBasisReadyArtifact,
) -> TransitionOutcome<CanonicalComparisonReadyArtifact> {
    let input = CanonicalComparisonInput::new(left, right, equivalence_basis);
    let authority =
        AuthorityWitness::from_authority_marker(CanonicalBasisConstructionAuthority::new());
    let proofs = CanonicalComparisonReadinessProofs::new(
        worth_proof::Proof::from_authority_witness(&authority),
        worth_proof::Proof::from_authority_witness(&authority),
    );

    TransitionOutcome::success(Artifact::with_proofs_and_current_basis(
        input,
        proofs,
        equivalence_basis,
        authority,
    ))
}
