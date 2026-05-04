use crate::proof::{Disjointness, Proof};

use super::Pair;

#[derive(Debug, PartialEq, Eq)]
pub struct DisjointPair<T> {
    pair: Pair<T>,
    proof: Proof<Disjointness>,
}

impl<T> DisjointPair<T> {
    #[allow(dead_code)]
    pub(crate) fn new(left: T, right: T, proof: Proof<Disjointness>) -> Self {
        Self {
            pair: Pair::new(left, right),
            proof,
        }
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

    pub fn proof(&self) -> &Proof<Disjointness> {
        &self.proof
    }

    pub fn into_parts(self) -> (Pair<T>, Proof<Disjointness>) {
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
        let pair = DisjointPair::new("left", "right", mint_proof::<Disjointness>());

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
