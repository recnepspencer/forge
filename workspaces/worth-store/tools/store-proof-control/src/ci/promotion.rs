use std::collections::BTreeSet;

use super::CiPartitionEvidence;

pub(super) fn promoted_lane_evidence<'a>(
    bundles: &[&'a CiPartitionEvidence],
) -> Result<Vec<&'a CiPartitionEvidence>, String> {
    if bundles.is_empty() {
        return Err("required lane has no evidence".to_owned());
    }
    let shard_plans: Vec<_> = bundles
        .iter()
        .filter_map(|bundle| bundle.shard_plan.as_ref())
        .collect();
    if shard_plans.is_empty() {
        return bundles
            .iter()
            .rev()
            .find(|bundle| bundle.closeout_eligible)
            .map(|bundle| vec![*bundle])
            .ok_or_else(|| "required lane has no closeout-eligible attempt".to_owned());
    }
    if shard_plans.len() != bundles.len() {
        return Err("CI lane mixes sharded and unsharded evidence".to_owned());
    }
    let plan_identities: BTreeSet<_> = shard_plans
        .iter()
        .map(|plan| plan.plan_identity.as_str())
        .collect();
    if plan_identities.len() != 1 {
        return Err("CI lane mixes incompatible shard plans".to_owned());
    }
    let shard_count = shard_plans[0].shard_count;
    let mut promoted = Vec::new();
    for shard_index in 0..shard_count {
        let candidate = bundles.iter().rev().find(|bundle| {
            bundle.closeout_eligible
                && bundle
                    .shard_plan
                    .as_ref()
                    .is_some_and(|plan| plan.selected_shard == shard_index)
        });
        match candidate {
            Some(candidate) => promoted.push(*candidate),
            None => return Err(format!("CI lane is missing successful shard {shard_index}")),
        }
    }
    Ok(promoted)
}
