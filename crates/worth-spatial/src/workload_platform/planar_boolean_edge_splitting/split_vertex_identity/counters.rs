#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSplitVertexIdentityCounters {
    schedules_inspected: usize,
    point_cuts_inspected: usize,
    interval_endpoint_candidates_inspected: usize,
    endpoint_contact_decisions_inspected: usize,
    split_vertices_minted: usize,
    split_vertices_coalesced: usize,
    coordinate_only_attempts_rejected: usize,
    interval_point_endpoint_collisions_resolved: usize,
}

impl PlanarBooleanSplitVertexIdentityCounters {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        schedules_inspected: usize,
        point_cuts_inspected: usize,
        interval_endpoint_candidates_inspected: usize,
        endpoint_contact_decisions_inspected: usize,
        split_vertices_minted: usize,
        split_vertices_coalesced: usize,
        coordinate_only_attempts_rejected: usize,
        interval_point_endpoint_collisions_resolved: usize,
    ) -> Self {
        Self {
            schedules_inspected,
            point_cuts_inspected,
            interval_endpoint_candidates_inspected,
            endpoint_contact_decisions_inspected,
            split_vertices_minted,
            split_vertices_coalesced,
            coordinate_only_attempts_rejected,
            interval_point_endpoint_collisions_resolved,
        }
    }

    pub fn schedules_inspected(self) -> usize {
        self.schedules_inspected
    }
    pub fn point_cuts_inspected(self) -> usize {
        self.point_cuts_inspected
    }
    pub fn interval_endpoint_candidates_inspected(self) -> usize {
        self.interval_endpoint_candidates_inspected
    }
    pub fn endpoint_contact_decisions_inspected(self) -> usize {
        self.endpoint_contact_decisions_inspected
    }
    pub fn split_vertices_minted(self) -> usize {
        self.split_vertices_minted
    }
    pub fn split_vertices_coalesced(self) -> usize {
        self.split_vertices_coalesced
    }
    pub fn coordinate_only_attempts_rejected(self) -> usize {
        self.coordinate_only_attempts_rejected
    }
    pub fn interval_point_endpoint_collisions_resolved(self) -> usize {
        self.interval_point_endpoint_collisions_resolved
    }
}
