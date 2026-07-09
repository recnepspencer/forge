use crate::live::LiveQueryFamily;
use crate::view_shape::ViewShapeFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LiveViewShapeFamily {
    Table,
    Detail,
    InspectorDetailObserved,
    InspectorDetailFocused,
    KanbanGrouped,
}

impl LiveViewShapeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Table => "table",
            Self::Detail => "detail",
            Self::InspectorDetailObserved => "inspector_detail_observed",
            Self::InspectorDetailFocused => "inspector_detail_focused",
            Self::KanbanGrouped => "kanban_grouped",
        }
    }

    pub fn underlying_live_family(&self) -> LiveQueryFamily {
        match self {
            Self::Table | Self::KanbanGrouped => LiveQueryFamily::OrderedCollection,
            Self::Detail | Self::InspectorDetailObserved | Self::InspectorDetailFocused => {
                LiveQueryFamily::Detail
            }
        }
    }
}

impl From<ViewShapeFamily> for LiveViewShapeFamily {
    fn from(value: ViewShapeFamily) -> Self {
        match value {
            ViewShapeFamily::Table => Self::Table,
            ViewShapeFamily::Detail => Self::Detail,
            ViewShapeFamily::InspectorDetailObserved => Self::InspectorDetailObserved,
            ViewShapeFamily::InspectorDetailFocused => Self::InspectorDetailFocused,
            ViewShapeFamily::KanbanGrouped => Self::KanbanGrouped,
        }
    }
}
