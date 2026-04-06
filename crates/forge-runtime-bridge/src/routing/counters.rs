use crate::clone_budget::CheapClone;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BridgeRoutingCounters {
    patch_item_count: usize,
    normalized_patch_item_count: usize,
    truth_delta_surface_count: usize,
    normalized_truth_delta_surface_count: usize,
    planned_slice_match_count: usize,
    slice_fallback_count: usize,
    slice_suppression_count: usize,
    routing_entry_count: usize,
    invalidation_target_count: usize,
    mapping_lookup_count: usize,
    mapping_fallback_count: usize,
    snapshot_read_count: usize,
    snapshot_read_packet_count: usize,
    validation_record_count: usize,
    digest_computation_count: usize,
    digest_input_bytes: usize,
    sort_input_width: usize,
    snapshot_identity_mismatch_count: usize,
    route_replay_mismatch_count: usize,
}

impl CheapClone for BridgeRoutingCounters {}

impl BridgeRoutingCounters {
    pub(crate) fn from_patch_counts(
        patch_item_count: usize,
        normalized_patch_item_count: usize,
    ) -> Self {
        Self {
            patch_item_count,
            normalized_patch_item_count,
            ..Self::default()
        }
    }

    pub fn patch_item_count(&self) -> usize {
        self.patch_item_count
    }

    pub fn normalized_patch_item_count(&self) -> usize {
        self.normalized_patch_item_count
    }

    pub fn truth_delta_surface_count(&self) -> usize {
        self.truth_delta_surface_count
    }

    pub fn normalized_truth_delta_surface_count(&self) -> usize {
        self.normalized_truth_delta_surface_count
    }

    pub fn planned_slice_match_count(&self) -> usize {
        self.planned_slice_match_count
    }

    pub fn slice_fallback_count(&self) -> usize {
        self.slice_fallback_count
    }

    pub fn slice_suppression_count(&self) -> usize {
        self.slice_suppression_count
    }

    pub fn routing_entry_count(&self) -> usize {
        self.routing_entry_count
    }

    pub fn invalidation_target_count(&self) -> usize {
        self.invalidation_target_count
    }

    pub fn mapping_lookup_count(&self) -> usize {
        self.mapping_lookup_count
    }

    pub fn mapping_fallback_count(&self) -> usize {
        self.mapping_fallback_count
    }

    pub fn snapshot_read_count(&self) -> usize {
        self.snapshot_read_count
    }

    pub fn snapshot_read_packet_count(&self) -> usize {
        self.snapshot_read_packet_count
    }

    pub fn validation_record_count(&self) -> usize {
        self.validation_record_count
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

    pub fn snapshot_identity_mismatch_count(&self) -> usize {
        self.snapshot_identity_mismatch_count
    }

    pub fn route_replay_mismatch_count(&self) -> usize {
        self.route_replay_mismatch_count
    }

    pub(crate) fn with_mapping_lookup(mut self) -> Self {
        self.mapping_lookup_count += 1;
        self
    }

    pub(crate) fn with_truth_delta_surface_counts(
        mut self,
        truth_delta_surface_count: usize,
        normalized_truth_delta_surface_count: usize,
    ) -> Self {
        self.truth_delta_surface_count = truth_delta_surface_count;
        self.normalized_truth_delta_surface_count = normalized_truth_delta_surface_count;
        self
    }

    pub(crate) fn with_planned_slice_match(mut self) -> Self {
        self.planned_slice_match_count += 1;
        self
    }

    pub(crate) fn with_slice_fallback(mut self) -> Self {
        self.slice_fallback_count += 1;
        self
    }

    pub(crate) fn with_slice_suppression(mut self) -> Self {
        self.slice_suppression_count += 1;
        self
    }

    pub(crate) fn with_mapping_fallback(mut self) -> Self {
        self.mapping_fallback_count += 1;
        self
    }

    pub(crate) fn with_routing_entry_count(mut self, routing_entry_count: usize) -> Self {
        self.routing_entry_count = routing_entry_count;
        self
    }

    pub(crate) fn with_invalidation_target_count(
        mut self,
        invalidation_target_count: usize,
    ) -> Self {
        self.invalidation_target_count = invalidation_target_count;
        self
    }

    pub(crate) fn with_snapshot_packet(mut self, snapshot_read_count: usize) -> Self {
        self.snapshot_read_packet_count = usize::from(snapshot_read_count > 0);
        self.snapshot_read_count = snapshot_read_count;
        self
    }

    pub(crate) fn with_validation_record_count(mut self, validation_record_count: usize) -> Self {
        self.validation_record_count = validation_record_count;
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

    pub(crate) fn with_snapshot_identity_mismatch(mut self) -> Self {
        self.snapshot_identity_mismatch_count += 1;
        self
    }

    pub(crate) fn with_route_replay_mismatch(mut self) -> Self {
        self.route_replay_mismatch_count += 1;
        self
    }

}
