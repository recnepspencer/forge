use forge_relational::facade::identity::EntityId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorthTopologyLoopWiringViolationKind {
    EmptyLoop,
    MissingLoopHalfEdge,
    MismatchedHalfEdgeLoopMembership,
    MissingNextLink,
    MissingPrevLink,
    MissingNextHalfEdge,
    MissingPrevHalfEdge,
    UnreciprocatedNextLink,
    UnreciprocatedPrevLink,
    DuplicateHalfEdgeInLoop,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyLoopWiringWitnessRow {
    violation_kind: WorthTopologyLoopWiringViolationKind,
    validator: &'static str,
    touched_loop_id: Option<EntityId>,
    touched_half_edge_id: Option<EntityId>,
    related_half_edge_id: Option<EntityId>,
    message: String,
    witness_digest: String,
}

impl WorthTopologyLoopWiringWitnessRow {
    pub(in crate::validator_invariant_catalog) fn violation(
        violation_kind: WorthTopologyLoopWiringViolationKind,
        touched_loop_id: Option<EntityId>,
        touched_half_edge_id: Option<EntityId>,
        related_half_edge_id: Option<EntityId>,
        message: impl Into<String>,
    ) -> Self {
        let validator = "loop_wiring";
        let message = message.into();
        let witness_digest = format!(
            "worth-topo-loop-wiring-witness-row-v1|{:?}|{:?}|{:?}|{:?}|{}",
            violation_kind, touched_loop_id, touched_half_edge_id, related_half_edge_id, message
        );
        Self {
            violation_kind,
            validator,
            touched_loop_id,
            touched_half_edge_id,
            related_half_edge_id,
            message,
            witness_digest,
        }
    }

    pub const fn violation_kind(&self) -> WorthTopologyLoopWiringViolationKind {
        self.violation_kind
    }

    pub fn validator(&self) -> &'static str {
        self.validator
    }

    pub const fn touched_loop_id(&self) -> Option<EntityId> {
        self.touched_loop_id
    }

    pub const fn touched_half_edge_id(&self) -> Option<EntityId> {
        self.touched_half_edge_id
    }

    pub const fn related_half_edge_id(&self) -> Option<EntityId> {
        self.related_half_edge_id
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn witness_digest(&self) -> &str {
        &self.witness_digest
    }
}
