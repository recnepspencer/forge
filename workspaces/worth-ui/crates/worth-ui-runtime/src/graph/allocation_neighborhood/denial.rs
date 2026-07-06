use crate::graph::UiGraphNodeIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationNeighborhoodDenial {
    TouchTargetMismatch {
        expected: UiGraphNodeIdentity,
        observed: UiGraphNodeIdentity,
    },
    WrongWorld,
    UnknownRootGraphNode {
        graph_node_identity: UiGraphNodeIdentity,
    },
    RootNotLayoutParticipant {
        graph_node_identity: UiGraphNodeIdentity,
    },
}
