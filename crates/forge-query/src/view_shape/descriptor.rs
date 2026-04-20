use super::family::ViewShapeFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeDescriptor {
    family: ViewShapeFamily,
    focused_aspect: Option<String>,
    grouping_aspect: Option<String>,
}

impl ViewShapeDescriptor {
    pub fn table() -> Self {
        Self {
            family: ViewShapeFamily::Table,
            focused_aspect: None,
            grouping_aspect: None,
        }
    }

    pub fn detail() -> Self {
        Self {
            family: ViewShapeFamily::Detail,
            focused_aspect: None,
            grouping_aspect: None,
        }
    }

    pub fn inspector_detail_observed() -> Self {
        Self {
            family: ViewShapeFamily::InspectorDetailObserved,
            focused_aspect: None,
            grouping_aspect: None,
        }
    }

    pub fn inspector_detail_focused(focused_aspect: impl Into<String>) -> Self {
        Self {
            family: ViewShapeFamily::InspectorDetailFocused,
            focused_aspect: Some(focused_aspect.into()),
            grouping_aspect: None,
        }
    }

    pub fn kanban_grouped(grouping_aspect: impl Into<String>) -> Self {
        Self {
            family: ViewShapeFamily::KanbanGrouped,
            focused_aspect: None,
            grouping_aspect: Some(grouping_aspect.into()),
        }
    }

    pub fn family(&self) -> ViewShapeFamily {
        self.family
    }

    pub fn focused_aspect(&self) -> Option<&str> {
        self.focused_aspect.as_deref()
    }

    pub fn grouping_aspect(&self) -> Option<&str> {
        self.grouping_aspect.as_deref()
    }

    #[cfg(test)]
    pub(crate) fn inspector_detail_focused_missing_for_test() -> Self {
        Self {
            family: ViewShapeFamily::InspectorDetailFocused,
            focused_aspect: None,
            grouping_aspect: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn kanban_grouped_missing_for_test() -> Self {
        Self {
            family: ViewShapeFamily::KanbanGrouped,
            focused_aspect: None,
            grouping_aspect: None,
        }
    }
}
