use std::collections::HashSet;

use worth_ui_host_contract::{UiMountedLogicalDamage, UiMountedPaintOrderIdentity};

use super::{
    UiNativeRetainedDrawList, UiNativeRetainedDrawListDenial, UiNativeRetainedMutationCounters,
    UiNativeRetainedReplayPlan,
};
use crate::native::presentation::damage_regions::normalize_damage;

impl UiNativeRetainedDrawList {
    pub(super) fn replay_plan(
        &self,
        regions: &[UiMountedLogicalDamage],
        draw_mutations: usize,
        order_mutations: usize,
    ) -> Result<UiNativeRetainedReplayPlan, UiNativeRetainedDrawListDenial> {
        let mut affected = HashSet::new();
        let mut counters = UiNativeRetainedMutationCounters {
            draw_mutations: exact_u64(draw_mutations)?,
            order_mutations: exact_u64(order_mutations)?,
            damage_regions: exact_u64(regions.len())?,
            ..Default::default()
        };
        let clear_regions = normalize_damage(regions)?;
        for region in &clear_regions {
            let query = self.damage.intersecting(region.bounds())?;
            counters.damage_cell_probes = add(counters.damage_cell_probes, query.cell_probes)?;
            counters.damage_candidate_probes =
                add(counters.damage_candidate_probes, query.candidate_probes)?;
            affected.extend(query.identities);
        }
        let replay = self.order.ordered_subset(
            affected
                .into_iter()
                .map(UiMountedPaintOrderIdentity::for_command),
        )?;
        counters.replayed_commands = exact_u64(replay.len())?;
        Ok(UiNativeRetainedReplayPlan {
            baseline_rgba8: self.baseline.transparent_rgba8(),
            clear_regions: clear_regions.into_boxed_slice(),
            replay: replay
                .into_iter()
                .map(UiMountedPaintOrderIdentity::command)
                .collect(),
            counters,
        })
    }
}

fn exact_u64(value: usize) -> Result<u64, UiNativeRetainedDrawListDenial> {
    u64::try_from(value).map_err(|_| UiNativeRetainedDrawListDenial::CounterOverflow)
}

fn add(total: u64, value: usize) -> Result<u64, UiNativeRetainedDrawListDenial> {
    total
        .checked_add(exact_u64(value)?)
        .ok_or(UiNativeRetainedDrawListDenial::CounterOverflow)
}
