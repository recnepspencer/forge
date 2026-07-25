pub(super) struct UiMountedProjectionCostInput {
    pub work_class: super::super::UiMountWorkClass,
    pub considered: usize,
    pub index_entries: usize,
    pub projected_instances: usize,
    pub surface_instance_pairs: usize,
    pub changed_bindings: usize,
    pub reused: usize,
    pub retired: usize,
    pub coalesced: u64,
    pub overflowed: bool,
}

pub(super) fn begin_projection_cost(
    input: UiMountedProjectionCostInput,
) -> Result<super::super::UiMountStageCounters, super::UiMountedProjectionDenial> {
    let mut counters = super::super::UiMountStageCounters::begin(input.work_class);
    counters
        .consider(input.considered)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .touch_indexes(input.index_entries)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .project_surface_pairs(input.surface_instance_pairs)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .record_projected_instances(input.projected_instances)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .change_bindings(input.changed_bindings)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .reuse(input.reused)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .retire(input.retired)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .coalesce(input.coalesced)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .record_overflow(input.overflowed)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    Ok(counters)
}
