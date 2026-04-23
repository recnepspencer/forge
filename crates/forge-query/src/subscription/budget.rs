use super::posture::QuerySubscriptionAllocationPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionWorkBudget {
    pub(super) max_admitted_slice_count: usize,
    pub(super) authorized_projection_width_limit: usize,
    pub(super) view_shape_metadata_width_limit: usize,
    pub(super) policy_tenant_digest_width_limit: usize,
    pub(super) bridge_family_map_lookup_limit: usize,
    pub(super) allocation_posture: QuerySubscriptionAllocationPosture,
}

impl QuerySubscriptionWorkBudget {
    pub fn no_allocation(
        max_admitted_slice_count: usize,
        authorized_projection_width_limit: usize,
        view_shape_metadata_width_limit: usize,
        policy_tenant_digest_width_limit: usize,
        bridge_family_map_lookup_limit: usize,
    ) -> Self {
        Self::new(
            max_admitted_slice_count,
            authorized_projection_width_limit,
            view_shape_metadata_width_limit,
            policy_tenant_digest_width_limit,
            bridge_family_map_lookup_limit,
            QuerySubscriptionAllocationPosture::NoAllocation,
        )
    }

    pub fn scratch_buffer_only(
        max_admitted_slice_count: usize,
        authorized_projection_width_limit: usize,
        view_shape_metadata_width_limit: usize,
        policy_tenant_digest_width_limit: usize,
        bridge_family_map_lookup_limit: usize,
    ) -> Self {
        Self::new(
            max_admitted_slice_count,
            authorized_projection_width_limit,
            view_shape_metadata_width_limit,
            policy_tenant_digest_width_limit,
            bridge_family_map_lookup_limit,
            QuerySubscriptionAllocationPosture::ScratchBufferOnly,
        )
    }

    fn new(
        max_admitted_slice_count: usize,
        authorized_projection_width_limit: usize,
        view_shape_metadata_width_limit: usize,
        policy_tenant_digest_width_limit: usize,
        bridge_family_map_lookup_limit: usize,
        allocation_posture: QuerySubscriptionAllocationPosture,
    ) -> Self {
        Self {
            max_admitted_slice_count,
            authorized_projection_width_limit,
            view_shape_metadata_width_limit,
            policy_tenant_digest_width_limit,
            bridge_family_map_lookup_limit,
            allocation_posture,
        }
    }

    pub fn max_admitted_slice_count(&self) -> usize {
        self.max_admitted_slice_count
    }

    pub fn authorized_projection_width_limit(&self) -> usize {
        self.authorized_projection_width_limit
    }

    pub fn view_shape_metadata_width_limit(&self) -> usize {
        self.view_shape_metadata_width_limit
    }

    pub fn policy_tenant_digest_width_limit(&self) -> usize {
        self.policy_tenant_digest_width_limit
    }

    pub fn bridge_family_map_lookup_limit(&self) -> usize {
        self.bridge_family_map_lookup_limit
    }

    pub fn allocation_posture(&self) -> &QuerySubscriptionAllocationPosture {
        &self.allocation_posture
    }
}
