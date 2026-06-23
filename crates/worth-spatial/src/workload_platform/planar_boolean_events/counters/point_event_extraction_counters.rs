#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanPointEventExtractionCounters {
    inspected_bound_pairs: usize,
    candidate_point_relations: usize,
    emitted_point_events: usize,
    skipped_non_point_relations: usize,
    shared_endpoint_candidates: usize,
    emitted_shared_endpoint_events: usize,
    duplicate_point_reports_suppressed: usize,
    high_valence_point_groups_detected: usize,
    ambiguous_relations: usize,
}

impl PlanarBooleanPointEventExtractionCounters {
    pub(crate) fn inspect_bound_pair(&mut self) {
        self.inspected_bound_pairs += 1;
    }

    pub(crate) fn candidate_point_relation(&mut self) {
        self.candidate_point_relations += 1;
    }

    pub(crate) fn emitted_point_event(&mut self) {
        self.emitted_point_events += 1;
    }

    pub(crate) fn skipped_non_point_relation(&mut self) {
        self.skipped_non_point_relations += 1;
    }

    pub(crate) fn shared_endpoint_candidate(&mut self) {
        self.shared_endpoint_candidates += 1;
    }

    pub(crate) fn emitted_shared_endpoint_event(&mut self) {
        self.emitted_shared_endpoint_events += 1;
    }

    pub(crate) fn suppress_duplicate_point_reports(&mut self, reports: usize) {
        self.duplicate_point_reports_suppressed += reports;
    }

    pub(crate) fn detect_high_valence_point_groups(&mut self, groups: usize) {
        self.high_valence_point_groups_detected += groups;
    }

    pub(crate) fn ambiguous_relation(&mut self) {
        self.ambiguous_relations += 1;
    }

    pub fn inspected_bound_pairs(&self) -> usize {
        self.inspected_bound_pairs
    }

    pub fn candidate_point_relations(&self) -> usize {
        self.candidate_point_relations
    }

    pub fn emitted_point_events(&self) -> usize {
        self.emitted_point_events
    }

    pub fn skipped_non_point_relations(&self) -> usize {
        self.skipped_non_point_relations
    }

    pub fn shared_endpoint_candidates(&self) -> usize {
        self.shared_endpoint_candidates
    }

    pub fn emitted_shared_endpoint_events(&self) -> usize {
        self.emitted_shared_endpoint_events
    }

    pub fn duplicate_point_reports_suppressed(&self) -> usize {
        self.duplicate_point_reports_suppressed
    }

    pub fn high_valence_point_groups_detected(&self) -> usize {
        self.high_valence_point_groups_detected
    }

    pub fn ambiguous_relations(&self) -> usize {
        self.ambiguous_relations
    }
}
