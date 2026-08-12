#[cfg(test)]
use super::{markers::ProofMarker, sets::AuthorityProves, sets::Proof};

/// Test-only convenience over [`Proof::mint`]. Production proof creation
/// goes through a checked constructor that validates before minting.
#[cfg(test)]
pub(crate) fn mint_proof<P, Auth>() -> Proof<P, Auth>
where
    P: ProofMarker,
    Auth: AuthorityProves<P>,
{
    Proof::mint()
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::proof::{
        mint_proof, AuthorityMarker, AuthorityProves, CanonicalOrder, Disjointness, Proof,
    };

    #[derive(Debug, PartialEq, Eq)]
    struct TestAuthority;
    impl AuthorityMarker for TestAuthority {}
    impl AuthorityProves<CanonicalOrder> for TestAuthority {}
    impl AuthorityProves<Disjointness> for TestAuthority {}

    #[test]
    fn crate_internal_minting_produces_zero_sized_proofs() {
        let canonical = mint_proof::<CanonicalOrder, TestAuthority>();
        let disjoint = mint_proof::<Disjointness, TestAuthority>();

        assert_eq!(canonical, Proof::<CanonicalOrder, TestAuthority>::mint());
        assert_eq!(disjoint, Proof::<Disjointness, TestAuthority>::mint());
        assert_eq!(size_of::<Proof<CanonicalOrder, TestAuthority>>(), 0);
    }
}
