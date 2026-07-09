#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BridgeContinuityCounters {
    continuity_request_count: usize,
    continuity_prior_slice_count: usize,
    lineage_resolution_request_count: usize,
    lineage_resolution_candidate_count: usize,
    continuity_single_successor_count: usize,
    continuity_split_successor_count: usize,
    continuity_merge_like_successor_count: usize,
    continuity_rejection_count: usize,
    continuity_ambiguity_count: usize,
    continuity_replay_mismatch_count: usize,
    digest_computation_count: usize,
    digest_input_bytes: usize,
    sort_input_width: usize,
}

impl BridgeContinuityCounters {
    pub(crate) fn from_request_set(request_count: usize, prior_slice_count: usize) -> Self {
        Self {
            continuity_request_count: request_count,
            continuity_prior_slice_count: prior_slice_count,
            lineage_resolution_request_count: request_count,
            ..Self::default()
        }
    }

    pub fn continuity_request_count(&self) -> usize {
        self.continuity_request_count
    }

    pub fn continuity_prior_slice_count(&self) -> usize {
        self.continuity_prior_slice_count
    }

    pub fn lineage_resolution_request_count(&self) -> usize {
        self.lineage_resolution_request_count
    }

    pub fn lineage_resolution_candidate_count(&self) -> usize {
        self.lineage_resolution_candidate_count
    }

    pub fn continuity_single_successor_count(&self) -> usize {
        self.continuity_single_successor_count
    }

    pub fn continuity_split_successor_count(&self) -> usize {
        self.continuity_split_successor_count
    }

    pub fn continuity_merge_like_successor_count(&self) -> usize {
        self.continuity_merge_like_successor_count
    }

    pub fn continuity_rejection_count(&self) -> usize {
        self.continuity_rejection_count
    }

    pub fn continuity_ambiguity_count(&self) -> usize {
        self.continuity_ambiguity_count
    }

    pub fn continuity_replay_mismatch_count(&self) -> usize {
        self.continuity_replay_mismatch_count
    }

    pub fn digest_computation_count(&self) -> usize {
        self.digest_computation_count
    }

    pub fn digest_input_bytes(&self) -> usize {
        self.digest_input_bytes
    }

    pub fn sort_input_width(&self) -> usize {
        self.sort_input_width
    }

    pub(crate) fn with_lineage_resolution_candidate_count(
        mut self,
        lineage_resolution_candidate_count: usize,
    ) -> Self {
        self.lineage_resolution_candidate_count += lineage_resolution_candidate_count;
        self
    }

    pub(crate) fn with_single_successor(mut self) -> Self {
        self.continuity_single_successor_count += 1;
        self
    }

    pub(crate) fn with_split_successor(mut self) -> Self {
        self.continuity_split_successor_count += 1;
        self
    }

    pub(crate) fn with_merge_like_successor(mut self) -> Self {
        self.continuity_merge_like_successor_count += 1;
        self
    }

    pub(crate) fn with_rejection(mut self) -> Self {
        self.continuity_rejection_count += 1;
        self
    }

    pub(crate) fn with_ambiguity(mut self) -> Self {
        self.continuity_ambiguity_count += 1;
        self
    }

    pub(crate) fn with_continuity_replay_mismatch(mut self) -> Self {
        self.continuity_replay_mismatch_count += 1;
        self
    }

    pub(crate) fn with_digest_computations(mut self, digest_computation_count: usize) -> Self {
        self.digest_computation_count += digest_computation_count;
        self
    }

    pub(crate) fn with_digest_input_bytes(mut self, digest_input_bytes: usize) -> Self {
        self.digest_input_bytes += digest_input_bytes;
        self
    }

    pub(crate) fn with_sort_input_width(mut self, sort_input_width: usize) -> Self {
        self.sort_input_width += sort_input_width;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::BridgeContinuityCounters;

    #[test]
    fn continuity_counters_track_replay_mismatch_count() {
        let counters = BridgeContinuityCounters::default().with_continuity_replay_mismatch();

        assert_eq!(counters.continuity_replay_mismatch_count(), 1);
    }
}
