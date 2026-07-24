pub(super) struct UiMountedProjectionCostInput {
    pub has_published_frame: bool,
    pub plan_rows: usize,
    pub allocation_receipts: usize,
    pub mounted_instances: usize,
    pub surface_bindings: usize,
    pub projected_instances: usize,
    pub projected_surfaces: usize,
}

pub(super) fn begin_projection_cost(
    input: UiMountedProjectionCostInput,
) -> Result<super::super::UiMountStageCounters, super::UiMountedProjectionDenial> {
    let work_class = if input.has_published_frame {
        super::super::UiMountWorkClass::ComparisonRequired
    } else {
        super::super::UiMountWorkClass::InitialMount
    };
    let mut counters = super::super::UiMountStageCounters::begin(work_class);
    let index_entries = input
        .plan_rows
        .checked_add(input.allocation_receipts)
        .ok_or(super::UiMountedProjectionDenial::CostCounterOverflow)?;
    let considered = index_entries
        .checked_add(input.mounted_instances)
        .and_then(|count| count.checked_add(input.surface_bindings))
        .ok_or(super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .consider(considered)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .touch_indexes(index_entries)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .project_surface_pairs(input.projected_instances)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    counters
        .record_projected_instances(input.projected_instances)
        .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    if matches!(work_class, super::super::UiMountWorkClass::InitialMount) {
        counters
            .change_bindings(input.projected_surfaces)
            .map_err(|_| super::UiMountedProjectionDenial::CostCounterOverflow)?;
    }
    Ok(counters)
}
