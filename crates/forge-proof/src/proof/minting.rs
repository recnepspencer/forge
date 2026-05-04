use super::{markers::ProofMarker, sets::Proof};

#[allow(dead_code)]
pub(crate) fn mint_proof<P>() -> Proof<P>
where
    P: ProofMarker,
{
    Proof::mint()
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::proof::{mint_proof, CanonicalOrder, Disjointness, Proof};

    #[test]
    fn crate_internal_minting_produces_zero_sized_proofs() {
        let canonical = mint_proof::<CanonicalOrder>();
        let disjoint = mint_proof::<Disjointness>();

        assert_eq!(canonical, Proof::<CanonicalOrder>::mint());
        assert_eq!(disjoint, Proof::<Disjointness>::mint());
        assert_eq!(size_of::<Proof<CanonicalOrder>>(), 0);
    }
}
