use super::{ViewShapeComplexityStatus, ViewShapeFamily};

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
