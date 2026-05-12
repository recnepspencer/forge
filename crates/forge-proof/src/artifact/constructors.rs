use std::marker::PhantomData;

use super::carrier::Artifact;
use crate::assumption::{
    AssumptionBasis, CurrentValidity, FreshnessScopedBasis, NoAssumptionBasis,
};
use crate::proof::{AuthorityMarker, AuthorityWitness, NoProofs};

impl<P, T> Artifact<P, T, NoProofs, NoAssumptionBasis> {
    pub fn new(payload: T) -> Self {
        Self {
            payload,
            proofs: NoProofs,
            basis: NoAssumptionBasis,
            phase: PhantomData,
        }
    }
}

impl<P, T, B> Artifact<P, T, NoProofs, FreshnessScopedBasis<CurrentValidity, AssumptionBasis<B>>> {
    pub fn with_current_basis<Auth>(
        payload: T,
        basis: B,
        _authority: AuthorityWitness<Auth>,
    ) -> Self
    where
        Auth: AuthorityMarker,
    {
        Self {
            payload,
            proofs: NoProofs,
            basis: FreshnessScopedBasis::new(AssumptionBasis::new(basis)),
            phase: PhantomData,
        }
    }
}

impl<P, T, S, A> Artifact<P, T, S, A> {
    #[allow(dead_code)]
    pub(crate) fn with_state(payload: T, proofs: S, basis: A) -> Self {
        Self {
            payload,
            proofs,
            basis,
            phase: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::artifact::{Artifact, ArtifactParts};
    use crate::assumption::{AssumptionBasis, NoAssumptionBasis};
    use crate::phase::PhaseMarker;
    use crate::proof::{
        mint_authority_witness, mint_proof, AuthorityMarker, NoProofs, ProofMarker,
    };

    struct RawPhase;
    impl PhaseMarker for RawPhase {}

    struct ValidatedProof;
    impl ProofMarker for ValidatedProof {}

    struct CurrentBasisAuthority;
    impl AuthorityMarker for CurrentBasisAuthority {}

    #[test]
    fn artifact_new_uses_empty_proof_and_basis_defaults() {
        let artifact = Artifact::<RawPhase, _>::new("payload");

        assert_eq!(artifact.payload(), &"payload");
        assert_eq!(artifact.proofs(), &NoProofs);
        assert_eq!(artifact.basis(), &NoAssumptionBasis);
    }

    #[test]
    fn artifact_with_state_preserves_explicit_proof_and_basis() {
        let artifact = Artifact::<RawPhase, _, _, _>::with_state(
            "payload",
            mint_proof::<ValidatedProof>(),
            AssumptionBasis::new(7_u8),
        );

        assert_eq!(artifact.payload(), &"payload");
        assert_eq!(artifact.basis().value(), &7_u8);
    }

    #[test]
    fn artifact_current_basis_constructor_requires_authority_witness() {
        let artifact = Artifact::<RawPhase, _, _, _>::with_current_basis(
            "payload",
            13_u8,
            mint_authority_witness::<CurrentBasisAuthority>(),
        );

        assert_eq!(artifact.payload(), &"payload");
        assert_eq!(artifact.basis().basis().value(), &13_u8);
    }

    #[test]
    fn zero_sized_phase_and_proof_markers_do_not_change_payload_only_size() {
        assert_eq!(
            size_of::<Artifact<RawPhase, u64, NoProofs, NoAssumptionBasis>>(),
            size_of::<u64>()
        );
    }

    #[test]
    fn artifact_into_parts_preserves_owned_state_without_clone_pressure() {
        let artifact = Artifact::<RawPhase, _, _, _>::with_state(
            String::from("payload"),
            mint_proof::<ValidatedProof>(),
            AssumptionBasis::new(11_u8),
        );

        let parts = artifact.into_parts();

        assert_eq!(parts.payload(), "payload");
        assert_eq!(parts.basis().value(), &11_u8);

        let (payload, _proofs, basis) = parts.into_parts();
        assert_eq!(payload, "payload");
        assert_eq!(basis.value(), &11_u8);
    }

    #[test]
    fn artifact_into_parts_is_size_honest_for_underlying_owned_state() {
        assert_eq!(
            size_of::<ArtifactParts<u64, NoProofs, NoAssumptionBasis>>(),
            size_of::<(u64, NoProofs, NoAssumptionBasis)>()
        );
    }
}
