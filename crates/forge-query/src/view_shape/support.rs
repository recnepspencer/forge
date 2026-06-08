use super::{ViewShapeComplexityStatus, ViewShapeFamily};

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
        vec![
            (ViewShapeFamily::Table, ViewShapeComplexityStatus::Debt),
            (ViewShapeFamily::Detail, ViewShapeComplexityStatus::Debt),
            (
                ViewShapeFamily::InspectorDetailObserved,
                ViewShapeComplexityStatus::Debt,
            ),
            (
                ViewShapeFamily::InspectorDetailFocused,
                ViewShapeComplexityStatus::Debt,
            ),
            (
                ViewShapeFamily::KanbanGrouped,
                ViewShapeComplexityStatus::Debt,
            ),
        ],
    )
}

pub fn runtime_backed_view_shape_future_support_profile(
) -> Vec<(ViewShapeFamily, ViewShapeTemporalAsyncSupportPosture)> {
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
            ViewShapeTemporalAsyncSupportPosture::VisibleButDeferred,
        ),
    ]
}
