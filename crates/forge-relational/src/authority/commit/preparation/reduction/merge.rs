pub(crate) fn canonical_merge_indices<T>(
    items: &mut [T],
    compare: impl Fn(&T, &T) -> std::cmp::Ordering,
) {
    items.sort_by(compare);
}
