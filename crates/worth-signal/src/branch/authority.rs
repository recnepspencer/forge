use std::marker::PhantomData;

use worth_proof::{AuthorityProves, AuthorityWitness, Proof, ProofMarker};

worth_proof::authority_marker!(pub SignalBranchBasisAuthorityMarker);

impl Clone for SignalBranchBasisAuthorityMarker {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for SignalBranchBasisAuthorityMarker {}

pub type SignalBranchBasisAuthority = AuthorityWitness<SignalBranchBasisAuthorityMarker>;

/// The owner proof carried by a Signal branch-basis artifact.
///
/// This marker is intentionally public-but-sealed so the public artifact type
/// can name its concrete proof carrier while only this owner module can mint
/// one through `SignalBranchBasisAuthority`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalBranchBasisOwnerProof(PhantomData<()>);

impl ProofMarker for SignalBranchBasisOwnerProof {}

impl AuthorityProves<SignalBranchBasisOwnerProof> for SignalBranchBasisAuthorityMarker {}

pub type SignalBranchBasisProof =
    Proof<SignalBranchBasisOwnerProof, SignalBranchBasisAuthorityMarker>;

pub(crate) fn signal_branch_basis_proof(
    authority: &SignalBranchBasisAuthority,
) -> SignalBranchBasisProof {
    Proof::from_authority_witness(authority)
}

pub(crate) fn mint_signal_branch_authority() -> SignalBranchBasisAuthority {
    SignalBranchBasisAuthorityMarker::witness()
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_foundational::{FoundationalBranchReferenceGeneration, FoundationalBranchTarget};

    #[test]
    fn owner_mints_concrete_branch_basis_authority() {
        let reference = crate::branch::signal_branch_observation(
            "graph-a",
            1,
            "main",
            FoundationalBranchTarget::empty(),
            FoundationalBranchReferenceGeneration::initial(),
        )
        .expect("valid branch");
        let admitted = crate::branch::admit_signal_branch_observation(
            reference.clone(),
            mint_signal_branch_authority(),
        );
        assert_eq!(admitted.observation(), &reference);
    }
}
