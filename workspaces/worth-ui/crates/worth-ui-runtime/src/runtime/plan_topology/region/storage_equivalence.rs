use super::{
    identity_trie, WorthUiPlanRegionSchema, WorthUiPlanRegionStorageCounters,
    WorthUiPlanRegionStore,
};

impl WorthUiPlanRegionStore {
    pub(crate) fn exactly_matches(&self, other: &Self) -> (bool, WorthUiPlanRegionStorageCounters) {
        let mut counters = WorthUiPlanRegionStorageCounters::default();
        if self.region_count != other.region_count {
            return (false, counters);
        }
        let mut left = Vec::with_capacity(self.region_count);
        let mut right = Vec::with_capacity(other.region_count);
        identity_trie::collect_records(&self.identity_root, &mut left);
        identity_trie::collect_records(&other.identity_root, &mut right);
        for (previous, next) in left.iter().zip(&right) {
            if previous.schema.identity() != next.schema.identity()
                || previous.handle != next.handle
                || !schemas_match(&previous.schema, &next.schema, &mut counters)
            {
                return (false, counters);
            }
        }
        (true, counters)
    }

    pub(crate) fn semantically_matches(
        &self,
        other: &Self,
    ) -> (bool, WorthUiPlanRegionStorageCounters) {
        let mut counters = WorthUiPlanRegionStorageCounters::default();
        if self.region_count != other.region_count {
            return (false, counters);
        }
        let mut left = Vec::with_capacity(self.region_count);
        let mut right = Vec::with_capacity(other.region_count);
        identity_trie::collect_records(&self.identity_root, &mut left);
        identity_trie::collect_records(&other.identity_root, &mut right);
        for (previous, next) in left.iter().zip(&right) {
            if previous.schema.identity() != next.schema.identity()
                || !schemas_match(&previous.schema, &next.schema, &mut counters)
            {
                return (false, counters);
            }
        }
        (true, counters)
    }
}

impl PartialEq for WorthUiPlanRegionStore {
    fn eq(&self, other: &Self) -> bool {
        self.region_count == other.region_count
            && self.next_stable_slot == other.next_stable_slot
            && self.semantic_digest == other.semantic_digest
            && self.exactly_matches(other).0
    }
}

impl Eq for WorthUiPlanRegionStore {}

pub(super) fn schemas_match(
    previous: &WorthUiPlanRegionSchema,
    next: &WorthUiPlanRegionSchema,
    counters: &mut WorthUiPlanRegionStorageCounters,
) -> bool {
    if previous.narrowing_fingerprint() != next.narrowing_fingerprint() {
        counters.record_fingerprint_rejection();
        return false;
    }
    counters.record_exact_comparison();
    previous.exactly_matches_after_narrowing(next)
}
