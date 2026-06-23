#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanCollinearRelationCounters {
    inspected_bound_pairs: usize,
    skipped_non_collinear_pairs: usize,
    emitted_disjoint_relations: usize,
    emitted_endpoint_touch_relations: usize,
    emitted_partial_overlap_relations: usize,
    emitted_containment_overlap_relations: usize,
    emitted_identical_same_direction_relations: usize,
    emitted_identical_anti_parallel_relations: usize,
    unsupported_degenerate_collinear_relations: usize,
}

impl PlanarBooleanCollinearRelationCounters {
    pub(crate) fn inspect_bound_pair(&mut self) {
        self.inspected_bound_pairs += 1;
    }

    pub(crate) fn skip_non_collinear_pair(&mut self) {
        self.skipped_non_collinear_pairs += 1;
    }

    pub(crate) fn emit_disjoint_relation(&mut self) {
        self.emitted_disjoint_relations += 1;
    }

    pub(crate) fn emit_endpoint_touch_relation(&mut self) {
        self.emitted_endpoint_touch_relations += 1;
    }

    pub(crate) fn emit_partial_overlap_relation(&mut self) {
        self.emitted_partial_overlap_relations += 1;
    }

    pub(crate) fn emit_containment_overlap_relation(&mut self) {
        self.emitted_containment_overlap_relations += 1;
    }

    pub(crate) fn emit_identical_same_direction_relation(&mut self) {
        self.emitted_identical_same_direction_relations += 1;
    }

    pub(crate) fn emit_identical_anti_parallel_relation(&mut self) {
        self.emitted_identical_anti_parallel_relations += 1;
    }

    pub(crate) fn unsupported_degenerate_collinear_relation(&mut self) {
        self.unsupported_degenerate_collinear_relations += 1;
    }

    pub fn inspected_bound_pairs(&self) -> usize {
        self.inspected_bound_pairs
    }

    pub fn skipped_non_collinear_pairs(&self) -> usize {
        self.skipped_non_collinear_pairs
    }

    pub fn emitted_disjoint_relations(&self) -> usize {
        self.emitted_disjoint_relations
    }

    pub fn emitted_endpoint_touch_relations(&self) -> usize {
        self.emitted_endpoint_touch_relations
    }

    pub fn emitted_partial_overlap_relations(&self) -> usize {
        self.emitted_partial_overlap_relations
    }

    pub fn emitted_containment_overlap_relations(&self) -> usize {
        self.emitted_containment_overlap_relations
    }

    pub fn emitted_identical_same_direction_relations(&self) -> usize {
        self.emitted_identical_same_direction_relations
    }

    pub fn emitted_identical_anti_parallel_relations(&self) -> usize {
        self.emitted_identical_anti_parallel_relations
    }

    pub fn unsupported_degenerate_collinear_relations(&self) -> usize {
        self.unsupported_degenerate_collinear_relations
    }
}
