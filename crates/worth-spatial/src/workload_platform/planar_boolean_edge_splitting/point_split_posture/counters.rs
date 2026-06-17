#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanPointSplitPostureCounters {
    admitted_point_candidates: usize,
    postured_point_candidates: usize,
    interior_splits: usize,
    t_junction_promotions: usize,
    shared_endpoint_noops: usize,
    endpoint_noops: usize,
}

impl PlanarBooleanPointSplitPostureCounters {
    pub(crate) fn new(
        admitted_point_candidates: usize,
        postured_point_candidates: usize,
        interior_splits: usize,
        t_junction_promotions: usize,
        shared_endpoint_noops: usize,
        endpoint_noops: usize,
    ) -> Self {
        Self {
            admitted_point_candidates,
            postured_point_candidates,
            interior_splits,
            t_junction_promotions,
            shared_endpoint_noops,
            endpoint_noops,
        }
    }

    pub fn admitted_point_candidates(self) -> usize {
        self.admitted_point_candidates
    }

    pub fn postured_point_candidates(self) -> usize {
        self.postured_point_candidates
    }

    pub fn interior_splits(self) -> usize {
        self.interior_splits
    }

    pub fn t_junction_promotions(self) -> usize {
        self.t_junction_promotions
    }

    pub fn shared_endpoint_noops(self) -> usize {
        self.shared_endpoint_noops
    }

    pub fn endpoint_noops(self) -> usize {
        self.endpoint_noops
    }
}
