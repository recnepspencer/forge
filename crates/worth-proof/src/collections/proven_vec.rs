use crate::proof::{CanonicalOrder, Proof, StructuralProofAuthority, Uniqueness};

pub type CanonicalOrderProof = Proof<CanonicalOrder, StructuralProofAuthority>;
pub type UniquenessProof = Proof<Uniqueness, StructuralProofAuthority>;

#[derive(Debug, PartialEq, Eq)]
pub struct CanonicalVec<T> {
    items: Vec<T>,
    proof: CanonicalOrderProof,
}

impl<T> Clone for CanonicalVec<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            proof: Proof::<CanonicalOrder, StructuralProofAuthority>::mint(),
        }
    }
}

impl<T> CanonicalVec<T> {
    pub(crate) fn new(items: Vec<T>, proof: CanonicalOrderProof) -> Self {
        Self { items, proof }
    }

    pub fn try_from_sorted(items: Vec<T>) -> Result<Self, Vec<T>>
    where
        T: Ord,
    {
        if items.windows(2).all(|window| window[0] <= window[1]) {
            Ok(Self::new(
                items,
                Proof::<CanonicalOrder, StructuralProofAuthority>::mint(),
            ))
        } else {
            Err(items)
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    pub fn proof(&self) -> &CanonicalOrderProof {
        &self.proof
    }

    pub fn into_parts(self) -> (Vec<T>, CanonicalOrderProof) {
        (self.items, self.proof)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UniqueVec<T> {
    items: Vec<T>,
    proof: UniquenessProof,
}

impl<T> Clone for UniqueVec<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            proof: Proof::<Uniqueness, StructuralProofAuthority>::mint(),
        }
    }
}

impl<T> UniqueVec<T> {
    pub(crate) fn new(items: Vec<T>, proof: UniquenessProof) -> Self {
        Self { items, proof }
    }

    pub fn try_from_unique(mut items: Vec<T>) -> Result<Self, Vec<T>>
    where
        T: Ord,
    {
        items.sort();
        if items.windows(2).all(|window| window[0] != window[1]) {
            Ok(Self::new(
                items,
                Proof::<Uniqueness, StructuralProofAuthority>::mint(),
            ))
        } else {
            Err(items)
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    pub fn proof(&self) -> &UniquenessProof {
        &self.proof
    }

    pub fn into_parts(self) -> (Vec<T>, UniquenessProof) {
        (self.items, self.proof)
    }

    /// Establishes equality uniqueness without reordering the owner-defined
    /// sequence. The hash table is only an index; `Eq` remains decisive.
    pub fn try_from_unique_preserving_order(items: Vec<T>) -> Result<Self, Vec<T>>
    where
        T: Eq + std::hash::Hash + Clone,
    {
        let mut seen = std::collections::HashSet::with_capacity(items.len());
        if items.iter().cloned().all(|item| seen.insert(item)) {
            Ok(Self::new(
                items,
                Proof::<Uniqueness, StructuralProofAuthority>::mint(),
            ))
        } else {
            Err(items)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::proof::{mint_proof, CanonicalOrder, Uniqueness};

    use super::{CanonicalVec, UniqueVec};

    #[test]
    fn canonical_vec_carries_items_and_explicit_order_proof() {
        let items = CanonicalVec::new(
            vec![1, 2, 3],
            mint_proof::<CanonicalOrder, crate::proof::StructuralProofAuthority>(),
        );

        assert_eq!(items.as_slice(), &[1, 2, 3]);
        let (items, _proof) = items.into_parts();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn unique_vec_carries_items_and_explicit_uniqueness_proof() {
        let items = UniqueVec::new(
            vec![1, 2, 3],
            mint_proof::<Uniqueness, crate::proof::StructuralProofAuthority>(),
        );

        assert_eq!(items.as_slice(), &[1, 2, 3]);
        let (items, _proof) = items.into_parts();
        assert_eq!(items, vec![1, 2, 3]);
    }

    #[test]
    fn order_preserving_uniqueness_is_a_linear_admission() {
        let unique = UniqueVec::try_from_unique_preserving_order(vec!["b", "a"]).unwrap();
        assert_eq!(unique.as_slice(), &["b", "a"]);
        assert!(UniqueVec::try_from_unique_preserving_order(vec!["a", "a"]).is_err());
    }

    #[test]
    fn proven_vecs_are_size_honest_for_zero_sized_proofs() {
        assert_eq!(size_of::<CanonicalVec<u64>>(), size_of::<Vec<u64>>());
        assert_eq!(size_of::<UniqueVec<u64>>(), size_of::<Vec<u64>>());
    }
}
