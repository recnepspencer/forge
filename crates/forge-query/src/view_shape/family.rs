#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ViewShapeFamily {
    Table,
    Detail,
    InspectorDetailObserved,
    InspectorDetailFocused,
    KanbanGrouped,
}

impl ViewShapeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Table => "table",
            Self::Detail => "detail",
            Self::InspectorDetailObserved => "inspector_detail_observed",
            Self::InspectorDetailFocused => "inspector_detail_focused",
            Self::KanbanGrouped => "kanban_grouped",
        }
    }
}
