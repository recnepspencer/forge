use std::collections::BTreeSet;
use std::sync::Arc;

use crate::transactions::data::MergedCommitPlan;

pub(super) fn packet_partition_scope(
    merged_plan: Option<&MergedCommitPlan>,
) -> Arc<[crate::identity::data::PartitionId]> {
    let mut touched = BTreeSet::new();
    if let Some(plan) = merged_plan {
        for intent in &plan.merged_intents {
            intent.seed_touched_partitions(&mut touched);
        }
    }
    touched.into_iter().collect::<Vec<_>>().into()
}
