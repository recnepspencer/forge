use crate::view_shape::{
    runtime_backed_view_shape_future_support_profile, runtime_backed_view_shape_support_profile,
    ViewShapeComplexityStatus, ViewShapeFamily, ViewShapeTemporalAsyncSupportPosture,
};

#[test]
fn runtime_backed_view_shape_support_profile_closes_non_grouped_core_views() {
    let (admitted, deferred, statuses) = runtime_backed_view_shape_support_profile();

    assert!(deferred.is_empty());
    assert_eq!(
        admitted,
        vec![
            ViewShapeFamily::Table,
            ViewShapeFamily::Detail,
            ViewShapeFamily::InspectorDetailObserved,
            ViewShapeFamily::InspectorDetailFocused,
            ViewShapeFamily::KanbanGrouped,
        ]
    );
    assert_eq!(
        statuses,
        vec![
            (ViewShapeFamily::Table, ViewShapeComplexityStatus::Verified),
            (ViewShapeFamily::Detail, ViewShapeComplexityStatus::Verified),
            (
                ViewShapeFamily::InspectorDetailObserved,
                ViewShapeComplexityStatus::Verified,
            ),
            (
                ViewShapeFamily::InspectorDetailFocused,
                ViewShapeComplexityStatus::Verified,
            ),
            (
                ViewShapeFamily::KanbanGrouped,
                ViewShapeComplexityStatus::Verified,
            ),
        ]
    );
    assert_eq!(
        admitted,
        statuses
            .iter()
            .map(|(family, _)| *family)
            .collect::<Vec<_>>(),
        "support profile must publish one status row for every admitted family in the same order"
    );
}

#[test]
fn runtime_backed_view_shape_future_support_profile_distinguishes_inspector_and_grouped_posture() {
    let profile = runtime_backed_view_shape_future_support_profile();

    assert_eq!(
        profile,
        vec![
            (
                ViewShapeFamily::Table,
                ViewShapeTemporalAsyncSupportPosture::FuturePreserving,
            ),
            (
                ViewShapeFamily::Detail,
                ViewShapeTemporalAsyncSupportPosture::FuturePreserving,
            ),
            (
                ViewShapeFamily::InspectorDetailObserved,
                ViewShapeTemporalAsyncSupportPosture::VisibleButDeferred,
            ),
            (
                ViewShapeFamily::InspectorDetailFocused,
                ViewShapeTemporalAsyncSupportPosture::VisibleButDeferred,
            ),
            (
                ViewShapeFamily::KanbanGrouped,
                ViewShapeTemporalAsyncSupportPosture::FuturePreserving,
            ),
        ]
    );
}
