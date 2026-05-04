use crate::proof::{CanonicalOrder, Proof, Uniqueness};

#[derive(Debug, PartialEq, Eq)]
pub struct CanonicalVec<T> {
    items: Vec<T>,
    proof: Proof<CanonicalOrder>,
}

impl<T> CanonicalVec<T> {
    #[allow(dead_code)]
    pub(crate) fn new(items: Vec<T>, proof: Proof<CanonicalOrder>) -> Self {
        Self { items, proof }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    pub fn proof(&self) -> &Proof<CanonicalOrder> {
        &self.proof
    }

    pub fn into_parts(self) -> (Vec<T>, Proof<CanonicalOrder>) {
        (self.items, self.proof)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UniqueVec<T> {
    items: Vec<T>,
    proof: Proof<Uniqueness>,
}

impl<T> UniqueVec<T> {
    #[allow(dead_code)]
    pub(crate) fn new(items: Vec<T>, proof: Proof<Uniqueness>) -> Self {
        Self { items, proof }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    pub fn proof(&self) -> &Proof<Uniqueness> {
        &self.proof
    }

    pub fn into_parts(self) -> (Vec<T>, Proof<Uniqueness>) {
        (self.items, self.proof)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::proof::{mint_proof, CanonicalOrder, Uniqueness};

    use super::{CanonicalVec, UniqueVec};

    #[test]
    fn canonical_vec_carries_items_and_explicit_order_proof() {
        let items = CanonicalVec::new(vec![1, 2, 3], mint_proof::<CanonicalOrder>());

        assert_eq!(items.as_slice(), &[1, 2, 3]);
        let (items, _proof) = items.into_parts();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn unique_vec_carries_items_and_explicit_uniqueness_proof() {
        let items = UniqueVec::new(vec![1, 2, 3], mint_proof::<Uniqueness>());

        assert_eq!(items.as_slice(), &[1, 2, 3]);
        let (items, _proof) = items.into_parts();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn proven_vecs_are_size_honest_for_zero_sized_proofs() {
        assert_eq!(size_of::<CanonicalVec<u64>>(), size_of::<Vec<u64>>());
        assert_eq!(size_of::<UniqueVec<u64>>(), size_of::<Vec<u64>>());
    }
}
