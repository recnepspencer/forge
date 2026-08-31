use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};

pub(super) fn base_value_if_live<'a, K, V, Q>(
    base: &'a BTreeMap<K, V>,
    retired_intervals: &im::OrdMap<K, K>,
    key: &Q,
) -> Option<&'a V>
where
    K: Clone + Ord + std::borrow::Borrow<Q>,
    Q: Ord + ?Sized,
{
    let (base_key, value) = base.get_key_value(key)?;
    retired_interval_end(retired_intervals, base_key)
        .is_none()
        .then_some(value)
}

pub(super) fn retired_interval_end<'a, K: Clone + Ord>(
    retired_intervals: &'a im::OrdMap<K, K>,
    key: &K,
) -> Option<&'a K> {
    retired_intervals.get(key).or_else(|| {
        retired_intervals
            .get_prev(key)
            .and_then(|(_, end)| (end >= key).then_some(end))
    })
}

pub(super) fn record_base_retirement<K: Clone + Ord, V>(
    base: &BTreeMap<K, V>,
    retired_intervals: &mut im::OrdMap<K, K>,
    key: &K,
) {
    let left_start = base
        .range(..key)
        .next_back()
        .and_then(|(predecessor, _)| containing_interval(retired_intervals, predecessor))
        .map(|(start, _)| start);
    let right_start = base
        .range((Excluded(key), Unbounded))
        .next()
        .and_then(|(successor, _)| retired_intervals.get(successor).map(|_| successor.clone()));
    let start = left_start.clone().unwrap_or_else(|| key.clone());
    let end = right_start
        .as_ref()
        .and_then(|right| retired_intervals.get(right))
        .cloned()
        .unwrap_or_else(|| key.clone());

    if let Some(left_start) = left_start {
        retired_intervals.remove(&left_start);
    }
    if let Some(right_start) = right_start {
        retired_intervals.remove(&right_start);
    }
    retired_intervals.insert(start, end);
}

pub(super) fn record_base_readmission<K: Clone + Ord, V>(
    base: &BTreeMap<K, V>,
    retired_intervals: &mut im::OrdMap<K, K>,
    key: &K,
) {
    let Some((start, end)) = containing_interval(retired_intervals, key) else {
        return;
    };
    retired_intervals.remove(&start);
    if start < *key {
        let predecessor = base
            .range(..key)
            .next_back()
            .map(|(predecessor, _)| predecessor.clone())
            .expect("non-start interval key must have a base predecessor");
        retired_intervals.insert(start, predecessor);
    }
    if *key < end {
        let successor = base
            .range((Excluded(key), Unbounded))
            .next()
            .map(|(successor, _)| successor.clone())
            .expect("non-end interval key must have a base successor");
        retired_intervals.insert(successor, end);
    }
}

pub(super) fn next_live_key_after<K: Clone + Ord, V>(
    base: &BTreeMap<K, V>,
    live_change_keys: &im::OrdSet<K>,
    retired_intervals: &im::OrdMap<K, K>,
    removed_key: &K,
) -> Option<K> {
    let base_key = next_live_base_key_after(base, retired_intervals, removed_key);
    let changed_key = live_change_keys.get_next(removed_key).cloned();
    match (base_key, changed_key) {
        (Some(base_key), Some(changed_key)) => Some(base_key.min(changed_key)),
        (Some(base_key), None) => Some(base_key),
        (None, Some(changed_key)) => Some(changed_key),
        (None, None) => None,
    }
}

fn next_live_base_key_after<K: Clone + Ord, V>(
    base: &BTreeMap<K, V>,
    retired_intervals: &im::OrdMap<K, K>,
    removed_key: &K,
) -> Option<K> {
    let mut search_key = removed_key.clone();
    loop {
        let candidate = base
            .range((Excluded(&search_key), Unbounded))
            .next()
            .map(|(key, _)| key.clone())?;
        let Some((_, retired_end)) = containing_interval(retired_intervals, &candidate) else {
            return Some(candidate);
        };
        search_key = retired_end;
    }
}

fn containing_interval<K: Clone + Ord>(
    retired_intervals: &im::OrdMap<K, K>,
    key: &K,
) -> Option<(K, K)> {
    retired_intervals
        .get(key)
        .map(|end| (key.clone(), end.clone()))
        .or_else(|| {
            retired_intervals
                .get_prev(key)
                .filter(|(_, end)| *end >= key)
                .map(|(start, end)| (start.clone(), end.clone()))
        })
}
