use crate::planning::FallbackDisposition;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ViewShapeComplexityStatus {
    Verified,
    Debt,
}

impl ViewShapeComplexityStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Debt => "debt",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ViewShapeCostClass {
    OrderedCollectionTable,
    DetailProjection,
    InspectorObservedNarrow,
    InspectorFocusedNarrow,
    KanbanGroupedDeltaBound,
}

impl ViewShapeCostClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OrderedCollectionTable => "ordered_collection_table",
            Self::DetailProjection => "detail_projection",
            Self::InspectorObservedNarrow => "inspector_observed_narrow",
            Self::InspectorFocusedNarrow => "inspector_focused_narrow",
            Self::KanbanGroupedDeltaBound => "kanban_grouped_delta_bound",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewShapeComplexityReport {
    status: ViewShapeComplexityStatus,
    cost_class: ViewShapeCostClass,
    fallback: FallbackDisposition,
}

impl ViewShapeComplexityReport {
    pub(crate) fn new(
        status: ViewShapeComplexityStatus,
        cost_class: ViewShapeCostClass,
        fallback: FallbackDisposition,
    ) -> Self {
        Self {
            status,
            cost_class,
            fallback,
        }
    }

    pub fn status(&self) -> ViewShapeComplexityStatus {
        self.status
    }

    pub fn cost_class(&self) -> ViewShapeCostClass {
        self.cost_class
    }

    pub fn fallback(&self) -> &FallbackDisposition {
        &self.fallback
    }
}
