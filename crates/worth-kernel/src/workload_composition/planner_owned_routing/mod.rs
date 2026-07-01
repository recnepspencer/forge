mod admitted_public_proof_input;
mod selected_route;

pub use admitted_public_proof_input::{
    admit_worth_touched_graph_conflict_public_proof_input,
    current_worth_touched_graph_conflict_public_proof_input,
    WorthTouchedGraphConflictAdmittedPublicProofInput,
};
pub use selected_route::{
    current_worth_touched_graph_conflict_selected_route_packet,
    WorthTouchedGraphConflictSelectedRoutePacket,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlannerOwnedRoutingErrorKind {
    CurrentProofUnavailable,
    IncompleteSelectedRoutePacket,
    MismatchedSelectedRouteSupport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannerOwnedRoutingError {
    kind: PlannerOwnedRoutingErrorKind,
    detail: String,
}

impl PlannerOwnedRoutingError {
    pub(crate) fn new(kind: PlannerOwnedRoutingErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> PlannerOwnedRoutingErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
