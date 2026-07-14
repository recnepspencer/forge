use crate::retention_reclaim::transitions::plan_reclaim::{
    construct_plan_counters, transition_plan_reclaim,
};
use crate::retention_reclaim::types::{BlobRetentionReclaimOutcome, BlobRetentionReclaimRequest};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct BlobRetentionSafeReclaimPlanner;

impl BlobRetentionSafeReclaimPlanner {
    pub fn new_store_owned() -> Self {
        Self
    }

    pub fn plan_reclaim(
        &mut self,
        request: BlobRetentionReclaimRequest,
    ) -> BlobRetentionReclaimOutcome {
        let counters = construct_plan_counters();
        let (admission, residue_kind) = request.into_parts();
        let permit = transition_plan_reclaim(admission, residue_kind, counters);
        BlobRetentionReclaimOutcome::Permitted(permit)
    }
}
