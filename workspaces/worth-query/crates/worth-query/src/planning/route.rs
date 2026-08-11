#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlannedExecutionRoute {
    RuntimeSnapshotRead,
    RuntimeExpandedSnapshotRead,
    StoreSnapshotRead,
}

impl PlannedExecutionRoute {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::RuntimeSnapshotRead => "runtime_snapshot_read",
            Self::RuntimeExpandedSnapshotRead => "runtime_expanded_snapshot_read",
            Self::StoreSnapshotRead => "store_snapshot_read",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FallbackDisposition {
    Forbidden,
    AdmittedButUnused,
    AdmittedAndSelected,
}

impl FallbackDisposition {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Forbidden => "forbidden",
            Self::AdmittedButUnused => "admitted_but_unused",
            Self::AdmittedAndSelected => "admitted_and_selected",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExecutionCostMarker(String);

impl ExecutionCostMarker {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionMechanics {
    cost_markers: Vec<ExecutionCostMarker>,
}

impl ExecutionMechanics {
    pub fn cost_markers(&self) -> &[ExecutionCostMarker] {
        &self.cost_markers
    }
}

pub(in crate::planning) fn route_candidate_count(route: &PlannedExecutionRoute) -> usize {
    match route {
        PlannedExecutionRoute::StoreSnapshotRead => 1,
        PlannedExecutionRoute::RuntimeSnapshotRead
        | PlannedExecutionRoute::RuntimeExpandedSnapshotRead => 2,
    }
}

pub(in crate::planning) fn planned_read_surface_count(
    route: &PlannedExecutionRoute,
    projection_count: usize,
    traversal_count: usize,
    predicate_count: usize,
    ordering_count: usize,
) -> usize {
    match route {
        PlannedExecutionRoute::RuntimeSnapshotRead | PlannedExecutionRoute::StoreSnapshotRead => {
            projection_count.max(1)
        }
        PlannedExecutionRoute::RuntimeExpandedSnapshotRead => {
            (projection_count + traversal_count + predicate_count + ordering_count).max(1)
        }
    }
}
