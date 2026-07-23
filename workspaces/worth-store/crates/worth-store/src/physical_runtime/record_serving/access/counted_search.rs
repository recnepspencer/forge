use std::cmp::Ordering;

pub(in crate::physical_runtime::record_serving) fn binary_search_by<T>(
    values: &[T],
    mut compare: impl FnMut(&T) -> Ordering,
) -> (Result<usize, usize>, usize) {
    let mut comparisons = 0_usize;
    let result = values.binary_search_by(|value| {
        comparisons = comparisons.saturating_add(1);
        compare(value)
    });
    (result, comparisons)
}

pub(in crate::physical_runtime::record_serving) fn partition_point<T>(
    values: &[T],
    mut predicate: impl FnMut(&T) -> bool,
) -> (usize, usize) {
    let mut comparisons = 0_usize;
    let index = values.partition_point(|value| {
        comparisons = comparisons.saturating_add(1);
        predicate(value)
    });
    (index, comparisons)
}
