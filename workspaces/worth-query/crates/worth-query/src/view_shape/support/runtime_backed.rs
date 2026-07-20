use crate::view_shape::{ViewShapeComplexityStatus, ViewShapeFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ViewShapeTemporalAsyncSupportPosture {
    FuturePreserving,
    VisibleButDeferred,
}

impl ViewShapeTemporalAsyncSupportPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FuturePreserving => "future_preserving",
            Self::VisibleButDeferred => "visible_but_deferred",
        }
    }
}

pub(crate) fn runtime_backed_view_shape_complexity_status(
    family: ViewShapeFamily,
) -> ViewShapeComplexityStatus {
    runtime_backed_view_shape_status_rows()
        .into_iter()
        .find(|(row_family, _)| *row_family == family)
        .map(|(_, status)| status)
        .expect("runtime-backed support rows should cover every admitted view family")
}

pub fn runtime_backed_view_shape_support_profile() -> (
    Vec<ViewShapeFamily>,
    Vec<ViewShapeFamily>,
    Vec<(ViewShapeFamily, ViewShapeComplexityStatus)>,
) {
    (
        vec![
            ViewShapeFamily::Table,
            ViewShapeFamily::Detail,
            ViewShapeFamily::InspectorDetailObserved,
            ViewShapeFamily::InspectorDetailFocused,
            ViewShapeFamily::KanbanGrouped,
        ],
        Vec::new(),
        runtime_backed_view_shape_status_rows(),
    )
}

pub fn runtime_backed_view_shape_future_support_profile(
) -> Vec<(ViewShapeFamily, ViewShapeTemporalAsyncSupportPosture)> {
    vec![
        (
            ViewShapeFamily::Table,
            runtime_backed_view_shape_temporal_async_support_posture(ViewShapeFamily::Table),
        ),
        (
            ViewShapeFamily::Detail,
            runtime_backed_view_shape_temporal_async_support_posture(ViewShapeFamily::Detail),
        ),
        (
            ViewShapeFamily::InspectorDetailObserved,
            runtime_backed_view_shape_temporal_async_support_posture(
                ViewShapeFamily::InspectorDetailObserved,
            ),
        ),
        (
            ViewShapeFamily::InspectorDetailFocused,
            runtime_backed_view_shape_temporal_async_support_posture(
                ViewShapeFamily::InspectorDetailFocused,
            ),
        ),
        (
            ViewShapeFamily::KanbanGrouped,
            runtime_backed_view_shape_temporal_async_support_posture(
                ViewShapeFamily::KanbanGrouped,
            ),
        ),
    ]
}

pub(crate) fn runtime_backed_view_shape_temporal_async_support_posture(
    family: ViewShapeFamily,
) -> ViewShapeTemporalAsyncSupportPosture {
    match family {
        ViewShapeFamily::Table | ViewShapeFamily::Detail | ViewShapeFamily::KanbanGrouped => {
            ViewShapeTemporalAsyncSupportPosture::FuturePreserving
        }
        ViewShapeFamily::InspectorDetailObserved | ViewShapeFamily::InspectorDetailFocused => {
            ViewShapeTemporalAsyncSupportPosture::VisibleButDeferred
        }
    }
}

fn runtime_backed_view_shape_status_rows() -> Vec<(ViewShapeFamily, ViewShapeComplexityStatus)> {
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
}
