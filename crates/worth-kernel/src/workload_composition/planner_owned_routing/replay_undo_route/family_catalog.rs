use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ReplayUndoPlannerRouteFamilyRow {
    family: ReplayUndoPlannerRouteFamily,
}

impl ReplayUndoPlannerRouteFamilyRow {
    pub(crate) const fn new(family: ReplayUndoPlannerRouteFamily) -> Self {
        Self { family }
    }

    pub(crate) const fn family(self) -> ReplayUndoPlannerRouteFamily {
        self.family
    }
}

pub(crate) fn current_replay_undo_planner_route_family_catalog(
) -> [ReplayUndoPlannerRouteFamilyRow; 3] {
    [
        ReplayUndoPlannerRouteFamilyRow::new(ReplayUndoPlannerRouteFamily::Replay),
        ReplayUndoPlannerRouteFamilyRow::new(ReplayUndoPlannerRouteFamily::Undo),
        ReplayUndoPlannerRouteFamilyRow::new(ReplayUndoPlannerRouteFamily::Transaction),
    ]
}

pub(crate) fn current_replay_undo_planner_route_family_row(
    family: ReplayUndoPlannerRouteFamily,
) -> ReplayUndoPlannerRouteFamilyRow {
    current_replay_undo_planner_route_family_catalog()
        .into_iter()
        .find(|row| row.family() == family)
        .expect("route family catalog is exhaustive")
}
