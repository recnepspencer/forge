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

    pub(crate) fn count_relation_contracts_evaluated(&self, count: usize) {
        self.access.count_relation_integrity_contracts_evaluated(count);
    }

    pub(crate) fn count_relation_endpoint_kind_checks(&self, count: usize) {
        self.access.count_relation_endpoint_kind_checks(count);
    }

    pub(crate) fn count_relation_cardinality_checks(&self, count: usize) {
        self.access.count_relation_cardinality_checks(count);
    }

    pub(crate) fn count_relation_uniqueness_checks(&self, count: usize) {
        self.access.count_relation_uniqueness_checks(count);
    }

    pub(crate) fn count_relation_uniqueness_candidates(&self, count: usize) {
        self.access.count_relation_uniqueness_candidates(count);
    }

    pub(crate) fn count_relation_symmetry_checks(&self, count: usize) {
        self.access.count_relation_symmetry_checks(count);
    }

    pub(crate) fn count_relation_endpoint_deletion_checks(&self, count: usize) {
        self.access.count_relation_endpoint_deletion_checks(count);
    }
}
