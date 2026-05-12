use crate::artifact::Artifact;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForkOutputs2<L, R> {
    left: L,
    right: R,
}

impl<L, R> ForkOutputs2<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn into_parts(self) -> (L, R) {
        (self.left, self.right)
    }
}

pub fn fork_artifact_pair<P, T, S, A, L, R, LS, RS, LA, RA>(
    artifact: Artifact<P, T, S, A>,
    split: impl FnOnce(T, S, A) -> ForkOutputs2<(L, LS, LA), (R, RS, RA)>,
) -> ForkOutputs2<Artifact<P, L, LS, LA>, Artifact<P, R, RS, RA>> {
    let (payload, proofs, basis) = artifact.into_parts().into_parts();
    let outputs = split(payload, proofs, basis);
    let (left, right) = outputs.into_parts();
    let (left_payload, left_proofs, left_basis) = left;
    let (right_payload, right_proofs, right_basis) = right;

    ForkOutputs2::new(
        Artifact::with_state(left_payload, left_proofs, left_basis),
        Artifact::with_state(right_payload, right_proofs, right_basis),
    )
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::artifact::Artifact;
    use crate::assumption::{AssumptionBasis, NoAssumptionBasis};
    use crate::phase::PhaseMarker;
    use crate::proof::{mint_proof, NoProofs, ProofMarker};

    use super::{fork_artifact_pair, ForkOutputs2};

    struct RawPhase;
    impl PhaseMarker for RawPhase {}

    struct LeftProof;
    impl ProofMarker for LeftProof {}

    struct RightProof;
    impl ProofMarker for RightProof {}

    #[test]
    fn fork_outputs_preserve_explicit_positions() {
        let outputs = ForkOutputs2::new("left", "right");

        assert_eq!(outputs.left(), &"left");
        assert_eq!(outputs.right(), &"right");
        assert_eq!(outputs.into_parts(), ("left", "right"));
    }

    #[test]
    fn artifact_fork_requires_explicit_state_redistribution() {
        let artifact = Artifact::<RawPhase, _, _, _>::with_state(
            "anchor",
            NoProofs,
            AssumptionBasis::new(7_u8),
        );

        let outputs = fork_artifact_pair(artifact, |payload, _proofs, basis| {
            ForkOutputs2::new(
                (payload.len(), mint_proof::<LeftProof>(), NoAssumptionBasis),
                (
                    basis.value().to_owned(),
                    mint_proof::<RightProof>(),
                    NoAssumptionBasis,
                ),
            )
        });

        assert_eq!(outputs.left().payload(), &6_usize);
        assert_eq!(outputs.right().payload(), &7_u8);
    }

    #[test]
    fn fork_outputs_are_size_honest() {
        assert_eq!(size_of::<ForkOutputs2<u64, u16>>(), size_of::<(u64, u16)>());
    }
}
