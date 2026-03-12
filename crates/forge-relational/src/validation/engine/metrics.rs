use crate::performance::logic::PerformanceAccess;

pub(crate) struct InvariantMetrics<'runtime> {
    access: PerformanceAccess<'runtime>,
}

impl<'runtime> InvariantMetrics<'runtime> {
    pub(crate) fn new(access: PerformanceAccess<'runtime>) -> Self {
        Self { access }
    }

    pub(crate) fn count_entity_slot_scans(&self, slots: usize) {
        self.access.count_invariant_entity_slot_scans(slots);
    }

    pub(crate) fn count_relation_slot_scans(&self, slots: usize) {
        self.access.count_invariant_relation_slot_scans(slots);
    }
}
