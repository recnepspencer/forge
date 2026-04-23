use super::posture::QuerySubscriptionAllocationPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSliceBudget {
    projected_slice_width_limit: usize,
    ordering_slice_width_limit: usize,
    grouping_slice_width_limit: usize,
    relation_scope_slice_width_limit: usize,
    metadata_slice_width_limit: usize,
    deduplication_input_width_limit: usize,
    deduplicated_output_width_limit: usize,
    sort_comparison_limit: usize,
    grouping_slice_support: bool,
    bounded_materialization_slice_support: bool,
    delivery_intent_support: bool,
    masked_slice_request_detected: bool,
    allocation_posture: QuerySubscriptionAllocationPosture,
}

impl QuerySubscriptionSliceBudget {
    pub fn no_allocation(
        projected_slice_width_limit: usize,
        ordering_slice_width_limit: usize,
        grouping_slice_width_limit: usize,
        relation_scope_slice_width_limit: usize,
        metadata_slice_width_limit: usize,
        deduplication_input_width_limit: usize,
        deduplicated_output_width_limit: usize,
        sort_comparison_limit: usize,
    ) -> Self {
        Self::new(
            projected_slice_width_limit,
            ordering_slice_width_limit,
            grouping_slice_width_limit,
            relation_scope_slice_width_limit,
            metadata_slice_width_limit,
            deduplication_input_width_limit,
            deduplicated_output_width_limit,
            sort_comparison_limit,
            true,
            true,
            true,
            false,
            QuerySubscriptionAllocationPosture::NoAllocation,
        )
    }

    pub fn scratch_buffer_only(
        projected_slice_width_limit: usize,
        ordering_slice_width_limit: usize,
        grouping_slice_width_limit: usize,
        relation_scope_slice_width_limit: usize,
        metadata_slice_width_limit: usize,
        deduplication_input_width_limit: usize,
        deduplicated_output_width_limit: usize,
        sort_comparison_limit: usize,
    ) -> Self {
        Self::new(
            projected_slice_width_limit,
            ordering_slice_width_limit,
            grouping_slice_width_limit,
            relation_scope_slice_width_limit,
            metadata_slice_width_limit,
            deduplication_input_width_limit,
            deduplicated_output_width_limit,
            sort_comparison_limit,
            true,
            true,
            true,
            false,
            QuerySubscriptionAllocationPosture::ScratchBufferOnly,
        )
    }

    fn new(
        projected_slice_width_limit: usize,
        ordering_slice_width_limit: usize,
        grouping_slice_width_limit: usize,
        relation_scope_slice_width_limit: usize,
        metadata_slice_width_limit: usize,
        deduplication_input_width_limit: usize,
        deduplicated_output_width_limit: usize,
        sort_comparison_limit: usize,
        grouping_slice_support: bool,
        bounded_materialization_slice_support: bool,
        delivery_intent_support: bool,
        masked_slice_request_detected: bool,
        allocation_posture: QuerySubscriptionAllocationPosture,
    ) -> Self {
        Self {
            projected_slice_width_limit,
            ordering_slice_width_limit,
            grouping_slice_width_limit,
            relation_scope_slice_width_limit,
            metadata_slice_width_limit,
            deduplication_input_width_limit,
            deduplicated_output_width_limit,
            sort_comparison_limit,
            grouping_slice_support,
            bounded_materialization_slice_support,
            delivery_intent_support,
            masked_slice_request_detected,
            allocation_posture,
        }
    }

    pub fn projected_slice_width_limit(&self) -> usize {
        self.projected_slice_width_limit
    }

    pub fn ordering_slice_width_limit(&self) -> usize {
        self.ordering_slice_width_limit
    }

    pub fn grouping_slice_width_limit(&self) -> usize {
        self.grouping_slice_width_limit
    }

    pub fn relation_scope_slice_width_limit(&self) -> usize {
        self.relation_scope_slice_width_limit
    }

    pub fn metadata_slice_width_limit(&self) -> usize {
        self.metadata_slice_width_limit
    }

    pub fn deduplication_input_width_limit(&self) -> usize {
        self.deduplication_input_width_limit
    }

    pub fn deduplicated_output_width_limit(&self) -> usize {
        self.deduplicated_output_width_limit
    }

    pub fn sort_comparison_limit(&self) -> usize {
        self.sort_comparison_limit
    }

    pub(super) fn grouping_slice_support(&self) -> bool {
        self.grouping_slice_support
    }

    pub(super) fn bounded_materialization_slice_support(&self) -> bool {
        self.bounded_materialization_slice_support
    }

    pub(super) fn delivery_intent_support(&self) -> bool {
        self.delivery_intent_support
    }

    pub(super) fn masked_slice_request_detected(&self) -> bool {
        self.masked_slice_request_detected
    }

    pub fn allocation_posture(&self) -> &QuerySubscriptionAllocationPosture {
        &self.allocation_posture
    }

    #[cfg(test)]
    pub(crate) fn without_grouping_slice_support(mut self) -> Self {
        self.grouping_slice_support = false;
        self
    }

    #[cfg(test)]
    pub(crate) fn without_bounded_materialization_slice_support(mut self) -> Self {
        self.bounded_materialization_slice_support = false;
        self
    }

    #[cfg(test)]
    pub(crate) fn without_delivery_intent_support(mut self) -> Self {
        self.delivery_intent_support = false;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_masked_slice_request_detected(mut self) -> Self {
        self.masked_slice_request_detected = true;
        self
    }
}
