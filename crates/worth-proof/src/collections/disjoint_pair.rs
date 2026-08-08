use crate::proof::{Disjointness, Proof, StructuralProofAuthority};

use super::Pair;

#[derive(Debug, PartialEq, Eq)]
pub struct DisjointPair<T> {
    pair: Pair<T>,
    proof: Proof<Disjointness, StructuralProofAuthority>,
}

impl<T> DisjointPair<T> {
    pub(crate) fn new(
        left: T,
        right: T,
        proof: Proof<Disjointness, StructuralProofAuthority>,
    ) -> Self {
        Self {
            pair: Pair::new(left, right),
            proof,
        }
    }

    /// Admit two values as disjoint, minting the proof on success.
    ///
    /// The checked door, matching [`super::CanonicalVec::try_from_sorted`] and
    /// [`super::UniqueVec::try_from_unique`]: a caller supplies raw values, this
    /// establishes the fact, and only then does the proof exist. There is no
    /// path that produces a `DisjointPair` without disjointness having been
    /// checked here.
    pub fn try_from_disjoint(left: T, right: T) -> Result<Self, Pair<T>>
    where
        T: PartialEq,
    {
        if left == right {
            return Err(Pair::new(left, right));
        }
        Ok(Self::new(
            left,
            right,
            Proof::<Disjointness, StructuralProofAuthority>::mint(),
        ))
    }

    pub fn left(&self) -> &T {
        self.pair.left()
    }

    pub fn right(&self) -> &T {
        self.pair.right()
    }

    pub fn pair(&self) -> &Pair<T> {
        &self.pair
    }

    pub fn proof(&self) -> &Proof<Disjointness, StructuralProofAuthority> {
        &self.proof
    }

    pub fn into_parts(self) -> (Pair<T>, Proof<Disjointness, StructuralProofAuthority>) {
        (self.pair, self.proof)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::proof::{mint_proof, Disjointness};

    use super::DisjointPair;
    use crate::collections::Pair;

    #[test]
    fn disjoint_pair_carries_pair_and_explicit_proof() {
        let pair = DisjointPair::new(
            "left",
            "right",
            mint_proof::<Disjointness, crate::proof::StructuralProofAuthority>(),
        );

        assert_eq!(pair.left(), &"left");
        assert_eq!(pair.right(), &"right");
        let (pair, _proof) = pair.into_parts();
        assert_eq!(pair.into_array(), ["left", "right"]);
    }

    #[test]
    fn disjoint_pair_is_size_honest_for_zero_sized_proof() {
        assert_eq!(size_of::<DisjointPair<u64>>(), size_of::<Pair<u64>>());
    }
}
