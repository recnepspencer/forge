pub(crate) fn zero_preview_lifecycle_residue_inputs(
    work_trace: &crate::facade::BridgeSubscriptionPreviewWorkTrace,
) -> Vec<crate::facade::BridgeSubscriptionPreviewLifecycleResidueInput> {
    crate::facade::BridgeSubscriptionPreviewLifecycleResidueKind::all()
        .into_iter()
        .map(|kind| {
            crate::facade::BridgeSubscriptionPreviewLifecycleResidueInput::from_preview_work_trace(
                kind, 0, work_trace,
            )
        })
        .collect()
}

pub(crate) fn preview_lifecycle_residue_inputs_with_count(
    work_trace: &crate::facade::BridgeSubscriptionPreviewWorkTrace,
    nonzero_kind: crate::facade::BridgeSubscriptionPreviewLifecycleResidueKind,
    residue_count: usize,
) -> Vec<crate::facade::BridgeSubscriptionPreviewLifecycleResidueInput> {
    crate::facade::BridgeSubscriptionPreviewLifecycleResidueKind::all()
        .into_iter()
        .map(|kind| {
            crate::facade::BridgeSubscriptionPreviewLifecycleResidueInput::from_preview_work_trace(
                kind,
                usize::from(kind == nonzero_kind) * residue_count,
                work_trace,
            )
        })
        .collect()
}
