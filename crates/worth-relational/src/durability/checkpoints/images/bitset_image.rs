use crate::durability::data::DurableBitSet;
use crate::storage::partition::DenseSlotBitSet;

pub(super) fn restore_bitset(image: DurableBitSet) -> DenseSlotBitSet {
    if image.sparse_words.is_empty() {
        DenseSlotBitSet::from_words(image.words)
    } else {
        DenseSlotBitSet::from_sparse_words(image.sparse_words)
    }
}
