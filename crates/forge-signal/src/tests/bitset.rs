use crate::facade::{BitsetFrontier, DenseBitset};

#[test]
fn dense_bitset_marks_and_merges_deterministically() {
    let mut a = DenseBitset::new();
    a.mark(5);
    a.mark(1);
    a.mark(70);

    let mut b = DenseBitset::new();
    b.mark(2);
    b.mark(70);
    a.merge(&b);

    assert_eq!(a.marked_indices(), vec![1, 2, 5, 70]);
}

#[test]
fn frontier_steps_in_sorted_order() {
    let mut f = BitsetFrontier::new();
    f.seed(8);
    f.seed(2);
    f.seed(5);
    assert_eq!(f.current_indices(), vec![2, 5, 8]);

    f.mark_next(9);
    f.mark_next(1);
    f.advance();
    assert_eq!(f.current_indices(), vec![1, 9]);
}
