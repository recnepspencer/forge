use schema::facade::platform::authority::touched_graph_conflict::ConflictIndependencePlannerRouteFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ConflictIndependencePlannerRouteFamilyCatalog {
    conflict_route_family: ConflictIndependencePlannerRouteFamily,
    independence_route_family: ConflictIndependencePlannerRouteFamily,
}

pub(crate) fn current_conflict_independence_planner_route_family_catalog(
) -> ConflictIndependencePlannerRouteFamilyCatalog {
    ConflictIndependencePlannerRouteFamilyCatalog {
        conflict_route_family: ConflictIndependencePlannerRouteFamily::ConflictRoute,
        independence_route_family: ConflictIndependencePlannerRouteFamily::IndependenceRoute,
    }
}

impl ConflictIndependencePlannerRouteFamilyCatalog {
    pub(crate) const fn conflict_route_family(self) -> ConflictIndependencePlannerRouteFamily {
        self.conflict_route_family
    }

    pub(crate) const fn independence_route_family(self) -> ConflictIndependencePlannerRouteFamily {
        self.independence_route_family
    }
}
