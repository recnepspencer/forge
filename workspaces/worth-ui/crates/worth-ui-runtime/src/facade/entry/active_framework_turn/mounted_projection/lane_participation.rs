pub(super) fn mounted_lanes(
    plan: &crate::runtime::WorthUiActiveExecutionPlan,
    virtualized_range_present: bool,
) -> crate::mounting::UiMountedLaneAssembly {
    crate::mounting::UiMountedLaneAssembly {
        ordinary: matches!(
            plan.ordinary_availability(),
            crate::runtime::WorthUiOrdinaryPlanAvailability::Executable
        ),
        virtualized: virtualized_range_present
            && matches!(
                plan.virtualized_availability(),
                crate::runtime::WorthUiVirtualizedPlanAvailability::Executable
            ),
        canvas: matches!(
            plan.canvas_spatial_availability(),
            crate::runtime::WorthUiCanvasSpatialPlanAvailability::Executable
        ),
        realtime: matches!(
            plan.realtime_availability(),
            crate::runtime::WorthUiRealtimePlanAvailability::Executable
        ),
        preview: false,
    }
}
