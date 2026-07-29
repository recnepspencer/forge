use crate::graph::{UiGraphMountEligibilityIdentity, UiGraphNodeIdentity};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiGraphFactConsumerIdentity {
    GraphNode(UiGraphNodeIdentity),
    MountEligibilitySlot(UiGraphMountEligibilityIdentity),
}
