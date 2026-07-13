use super::{BTreeLookupReadinessOutcome, BTreeLookupReady, LoweredBTreeLookup};

pub(in crate::access::execution) fn lower(
    selected: crate::planning::SelectedBTreeLookup,
) -> LoweredBTreeLookup {
    LoweredBTreeLookup::issue(selected)
}

pub(in crate::access::execution) fn admit_ready(
    lowered: LoweredBTreeLookup,
    frontier: crate::CurrentMaterializationFrontier,
) -> BTreeLookupReadinessOutcome {
    BTreeLookupReady::issue(lowered, frontier)
}
