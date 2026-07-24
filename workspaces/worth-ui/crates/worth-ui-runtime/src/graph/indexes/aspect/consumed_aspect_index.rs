use std::collections::BTreeMap;

use crate::declaration::{UiAspectContract, UiAspectName};
use crate::graph::{
    UiGraphMountEligibilityIndex, UiGraphMountEligibilityStore, UiGraphNodeIdentity,
};

const EMPTY_CONSUMERS: [UiGraphAspectConsumer; 0] = [];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphAspectConsumerKind {
    GraphNode(UiGraphNodeIdentity),
    MountEligibilitySlot(crate::graph::UiGraphMountEligibilityIdentity),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphAspectConsumer {
    kind: UiGraphAspectConsumerKind,
}

impl UiGraphAspectConsumer {
    pub(crate) const fn new(kind: UiGraphAspectConsumerKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> UiGraphAspectConsumerKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphConsumedAspectIndex {
    consumers_by_aspect: BTreeMap<UiAspectName, Vec<UiGraphAspectConsumer>>,
}

impl UiGraphConsumedAspectIndex {
    pub(crate) fn build(
        node_aspects: &[(&UiAspectContract, UiGraphNodeIdentity)],
        mount_eligibilities: &UiGraphMountEligibilityStore,
        mount_eligibility_index: &UiGraphMountEligibilityIndex,
    ) -> Self {
        let mut consumers_by_aspect = BTreeMap::<UiAspectName, Vec<UiGraphAspectConsumer>>::new();

        for (aspect_contract, graph_node_identity) in node_aspects {
            for aspect in aspect_contract.consumed().aspects() {
                consumers_by_aspect.entry(aspect.clone()).or_default().push(
                    UiGraphAspectConsumer::new(UiGraphAspectConsumerKind::GraphNode(
                        *graph_node_identity,
                    )),
                );

                if let Some(slot) =
                    mount_eligibility_index.slot_for_node(mount_eligibilities, *graph_node_identity)
                {
                    consumers_by_aspect.entry(aspect.clone()).or_default().push(
                        UiGraphAspectConsumer::new(
                            UiGraphAspectConsumerKind::MountEligibilitySlot(
                                slot.mount_eligibility_identity(),
                            ),
                        ),
                    );
                }
            }
        }

        Self {
            consumers_by_aspect,
        }
    }

    pub fn consumers_for(&self, aspect: &UiAspectName) -> &[UiGraphAspectConsumer] {
        self.consumers_by_aspect
            .get(aspect)
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY_CONSUMERS)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&UiAspectName, &[UiGraphAspectConsumer])> {
        self.consumers_by_aspect
            .iter()
            .map(|(aspect, consumers)| (aspect, consumers.as_slice()))
    }
}
