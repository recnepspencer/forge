//! Checked and ordinary collection introductions.

pub(crate) fn disjoint_pair() -> worth_proof::DisjointPair<u8> {
    worth_proof::DisjointPair::try_from_disjoint(1, 2).expect("the witness values differ")
}

pub(crate) fn exactly_one() -> worth_proof::ExactlyOne<u8> {
    worth_proof::ExactlyOne::new(1)
}

pub(crate) fn non_empty() -> worth_proof::NonEmpty<u8> {
    worth_proof::NonEmpty::new(1, vec![2])
}

pub(crate) fn pair() -> worth_proof::Pair<u8> {
    worth_proof::Pair::new(1, 2)
}

pub(crate) fn canonical_vec() -> worth_proof::CanonicalVec<u8> {
    worth_proof::CanonicalVec::try_from_sorted(vec![1, 2]).expect("the witness is sorted")
}

pub(crate) fn unique_vec() -> worth_proof::UniqueVec<u8> {
    worth_proof::UniqueVec::try_from_unique(vec![1, 2]).expect("the witness is unique")
}
