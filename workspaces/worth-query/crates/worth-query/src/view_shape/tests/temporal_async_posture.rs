use crate::view_shape::{
    runtime_backed_view_shape_future_support_profile,
    runtime_backed_view_shape_temporal_async_support_posture, ViewShapeFamily,
    ViewShapeTemporalAsyncSupportPosture,
};

#[test]
fn temporal_async_profile_stays_lane_local_and_family_exact() {
    let profile = runtime_backed_view_shape_future_support_profile();

    assert_eq!(
        profile
            .iter()
            .map(|(family, _)| *family)
            .collect::<Vec<_>>(),
        vec![
            ViewShapeFamily::Table,
            ViewShapeFamily::Detail,
            ViewShapeFamily::InspectorDetailObserved,
            ViewShapeFamily::InspectorDetailFocused,
            ViewShapeFamily::KanbanGrouped,
        ]
    );
}

#[test]
fn temporal_async_posture_does_not_collapse_inspector_into_future_preserving_lane() {
    assert_eq!(
        runtime_backed_view_shape_temporal_async_support_posture(
            ViewShapeFamily::InspectorDetailObserved
        ),
        ViewShapeTemporalAsyncSupportPosture::VisibleButDeferred
    );
    assert_eq!(
        runtime_backed_view_shape_temporal_async_support_posture(
            ViewShapeFamily::InspectorDetailFocused
        ),
        ViewShapeTemporalAsyncSupportPosture::VisibleButDeferred
    );
    assert_eq!(
        runtime_backed_view_shape_temporal_async_support_posture(ViewShapeFamily::KanbanGrouped),
        ViewShapeTemporalAsyncSupportPosture::FuturePreserving
    );
}
