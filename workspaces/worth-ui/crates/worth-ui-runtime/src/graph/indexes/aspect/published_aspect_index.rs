use std::collections::BTreeMap;

use crate::declaration::{UiAspectContract, UiAspectName};
use crate::graph::{
    UiGraphMountedReceiptAuthoritySeedStore, UiGraphMountedReceiptIndex, UiGraphNodeIdentity,
};

const EMPTY_PUBLISHERS: [UiGraphAspectPublisher; 0] = [];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphAspectPublisherKind {
    GraphNode(UiGraphNodeIdentity),
    MountedReceiptSlot(crate::graph::UiMountedReceiptIdentity),
    FutureReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphAspectPublisher {
    kind: UiGraphAspectPublisherKind,
}

impl UiGraphAspectPublisher {
    pub(crate) const fn new(kind: UiGraphAspectPublisherKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> UiGraphAspectPublisherKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphPublishedAspectIndex {
    publishers_by_aspect: BTreeMap<UiAspectName, Vec<UiGraphAspectPublisher>>,
}

impl UiGraphPublishedAspectIndex {
    pub(crate) fn build(
        node_aspects: &[(&UiAspectContract, UiGraphNodeIdentity)],
        mounted_receipts: &UiGraphMountedReceiptAuthoritySeedStore,
        mounted_receipt_index: &UiGraphMountedReceiptIndex,
    ) -> Self {
        let mut publishers_by_aspect = BTreeMap::<UiAspectName, Vec<UiGraphAspectPublisher>>::new();

        for (aspect_contract, graph_node_identity) in node_aspects {
            for aspect in aspect_contract.published().aspects() {
                publishers_by_aspect
                    .entry(aspect.clone())
                    .or_default()
                    .push(UiGraphAspectPublisher::new(
                        UiGraphAspectPublisherKind::GraphNode(*graph_node_identity),
                    ));

                if let Some(slot) =
                    mounted_receipt_index.slot_for_node(mounted_receipts, *graph_node_identity)
                {
                    publishers_by_aspect
                        .entry(aspect.clone())
                        .or_default()
                        .push(UiGraphAspectPublisher::new(
                            UiGraphAspectPublisherKind::MountedReceiptSlot(
                                slot.mounted_receipt_identity(),
                            ),
                        ));
                }
            }
        }

        Self {
            publishers_by_aspect,
        }
    }

    pub fn publishers_for(&self, aspect: &UiAspectName) -> &[UiGraphAspectPublisher] {
        self.publishers_by_aspect
            .get(aspect)
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY_PUBLISHERS)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&UiAspectName, &[UiGraphAspectPublisher])> {
        self.publishers_by_aspect
            .iter()
            .map(|(aspect, publishers)| (aspect, publishers.as_slice()))
    }
}
