use super::SnapshotLookupCounters;

/// Result of an index-backed typed snapshot lookup.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotLookupReport<T> {
    value: Option<T>,
    counters: SnapshotLookupCounters,
}

impl<T> SnapshotLookupReport<T> {
    pub(crate) fn new(value: Option<T>, counters: SnapshotLookupCounters) -> Self {
        Self { value, counters }
    }

    pub fn is_found(&self) -> bool {
        self.value.is_some()
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn into_value(self) -> Option<T> {
        self.value
    }

    pub fn counters(&self) -> SnapshotLookupCounters {
        self.counters
    }
}
