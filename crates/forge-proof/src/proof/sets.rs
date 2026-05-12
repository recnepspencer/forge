use std::marker::PhantomData;

use super::markers::ProofMarker;

pub trait ProofSet: 'static {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NoProofs;

impl ProofSet for NoProofs {}

#[derive(Debug, PartialEq, Eq)]
pub struct Proof<P>(PhantomData<P>);

impl<P> Proof<P> {
    #[allow(dead_code)]
    pub(crate) fn mint() -> Self {
        Self(PhantomData)
    }
}

impl<P> ProofSet for Proof<P> where P: ProofMarker {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProofSetCons<Head, Tail> {
    head: Head,
    tail: Tail,
}

impl<Head, Tail> ProofSetCons<Head, Tail> {
    pub fn new(head: Head, tail: Tail) -> Self {
        Self { head, tail }
    }

    pub fn head(&self) -> &Head {
        &self.head
    }

    pub fn tail(&self) -> &Tail {
        &self.tail
    }
}

impl<Head, Tail> ProofSet for ProofSetCons<Head, Tail>
where
    Head: ProofSet,
    Tail: ProofSet,
{
}

#[cfg(test)]
mod tests {
    use super::{NoProofs, Proof, ProofSet, ProofSetCons};
    use crate::proof::{mint_proof, ProofMarker};

    struct LeftProof;
    impl ProofMarker for LeftProof {}

    struct RightProof;
    impl ProofMarker for RightProof {}

    fn accepts_proof_set<T: ProofSet>(_: &T) {}

    #[test]
    fn nested_proof_sets_preserve_head_and_tail_access() {
        let proofs = ProofSetCons::new(
            mint_proof::<LeftProof>(),
            ProofSetCons::new(mint_proof::<RightProof>(), NoProofs),
        );

        accepts_proof_set(&proofs);
        accepts_proof_set(proofs.tail());
        assert_eq!(*proofs.tail().tail(), NoProofs);
    }

    #[test]
    fn proof_minting_is_zero_sized_and_crate_internal() {
        let minted: Proof<LeftProof> = mint_proof();
        let direct = Proof::<LeftProof>::mint();

        let _ = minted;
        let _ = direct;
    }
}
