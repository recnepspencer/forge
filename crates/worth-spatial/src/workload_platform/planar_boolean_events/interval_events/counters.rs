#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanIntervalEventExtractionCounters {
    inspected_collinear_relations: usize,
    skipped_disjoint_relations: usize,
    skipped_endpoint_touch_relations: usize,
    emitted_partial_overlap_events: usize,
    emitted_containment_overlap_events: usize,
    emitted_identical_same_direction_events: usize,
    emitted_identical_anti_parallel_events: usize,
    missing_interval_basis_relations: usize,
    collapsed_interval_denials: usize,
}

impl PlanarBooleanIntervalEventExtractionCounters {
    pub(crate) fn inspect_collinear_relation(&mut self) {
        self.inspected_collinear_relations += 1;
    }

    pub(crate) fn skip_disjoint_relation(&mut self) {
        self.skipped_disjoint_relations += 1;
    }

    pub(crate) fn skip_endpoint_touch_relation(&mut self) {
        self.skipped_endpoint_touch_relations += 1;
    }

    pub(crate) fn emit_interval_event(&mut self, kind: super::PlanarBooleanIntervalEventKind) {
        match kind {
            super::PlanarBooleanIntervalEventKind::PartialOverlap => {
                self.emitted_partial_overlap_events += 1
            }
            super::PlanarBooleanIntervalEventKind::ContainmentOverlap => {
                self.emitted_containment_overlap_events += 1
            }
            super::PlanarBooleanIntervalEventKind::IdenticalSameDirection => {
                self.emitted_identical_same_direction_events += 1
            }
            super::PlanarBooleanIntervalEventKind::IdenticalAntiParallel => {
                self.emitted_identical_anti_parallel_events += 1
            }
        }
    }

    pub(crate) fn missing_interval_basis_relation(&mut self) {
        self.missing_interval_basis_relations += 1;
    }

    pub(crate) fn collapsed_interval_denial(&mut self) {
        self.collapsed_interval_denials += 1;
    }

    pub fn inspected_collinear_relations(&self) -> usize {
        self.inspected_collinear_relations
    }

    pub fn skipped_disjoint_relations(&self) -> usize {
        self.skipped_disjoint_relations
    }

    pub fn skipped_endpoint_touch_relations(&self) -> usize {
        self.skipped_endpoint_touch_relations
    }

    pub fn emitted_partial_overlap_events(&self) -> usize {
        self.emitted_partial_overlap_events
    }

    pub fn emitted_containment_overlap_events(&self) -> usize {
        self.emitted_containment_overlap_events
    }

    pub fn emitted_identical_same_direction_events(&self) -> usize {
        self.emitted_identical_same_direction_events
    }

    pub fn emitted_identical_anti_parallel_events(&self) -> usize {
        self.emitted_identical_anti_parallel_events
    }

    pub fn emitted_interval_events(&self) -> usize {
        self.emitted_partial_overlap_events
            + self.emitted_containment_overlap_events
            + self.emitted_identical_same_direction_events
            + self.emitted_identical_anti_parallel_events
    }

    pub fn missing_interval_basis_relations(&self) -> usize {
        self.missing_interval_basis_relations
    }

    pub fn collapsed_interval_denials(&self) -> usize {
        self.collapsed_interval_denials
    }
}
