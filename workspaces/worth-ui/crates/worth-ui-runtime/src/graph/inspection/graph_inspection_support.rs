use crate::declaration::{UiAspectName, UiDeclarationIdentity};
use crate::graph::{
    UiGraphAspectConsumerKind, UiGraphAspectPublisherKind, UiGraphEvidenceRef, UiGraphInspection,
    UiGraphInspectionTarget, UiGraphLookup, UiGraphMountedReceiptAuthorityRecord,
    UiGraphNodeIdentity, UiGraphNodeRecord, UiGraphPageParticipationMember,
    UiGraphParticipationAxis, UiGraphSnapshot, UiGraphTopologyRecord, UiMountedReceiptIdentity,
};

pub struct UiGraphInspectionSupport<'a> {
    snapshot: &'a UiGraphSnapshot,
}

impl<'a> UiGraphInspectionSupport<'a> {
    pub(crate) const fn new(snapshot: &'a UiGraphSnapshot) -> Self {
        Self { snapshot }
    }

    pub fn inspect_graph_node(
        self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<UiGraphInspection<UiGraphNodeRecord>> {
        self.snapshot
            .lookup()
            .graph_node(graph_node_identity)
            .map(|lookup| {
                scalar(
                    self.snapshot,
                    UiGraphInspectionTarget::GraphNode(graph_node_identity),
                    lookup,
                    vec![UiGraphEvidenceRef::GraphNode(graph_node_identity)],
                )
            })
    }

    pub fn inspect_topology_node(
        self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<UiGraphInspection<UiGraphTopologyRecord>> {
        self.snapshot
            .lookup()
            .topology_node(graph_node_identity)
            .map(|lookup| {
                scalar(
                    self.snapshot,
                    UiGraphInspectionTarget::TopologyNode(graph_node_identity),
                    lookup,
                    vec![UiGraphEvidenceRef::GraphNode(graph_node_identity)],
                )
            })
    }

    pub fn inspect_declaration_instances(
        self,
        declaration_identity: &UiDeclarationIdentity,
    ) -> UiGraphInspection<&'a [UiGraphNodeIdentity]> {
        let lookup = self
            .snapshot
            .lookup()
            .declaration_instances(declaration_identity);
        let mut evidence_refs = vec![UiGraphEvidenceRef::Declaration(
            declaration_identity.clone(),
        )];
        evidence_refs.extend(
            lookup
                .value()
                .iter()
                .copied()
                .map(UiGraphEvidenceRef::GraphNode),
        );

        scalar(
            self.snapshot,
            UiGraphInspectionTarget::DeclarationInstances(declaration_identity.clone()),
            lookup,
            evidence_refs,
        )
    }

    pub fn inspect_parent_child(
        self,
        parent_node_identity: UiGraphNodeIdentity,
    ) -> UiGraphInspection<&'a [UiGraphNodeIdentity]> {
        let lookup = self.snapshot.lookup().child_nodes(parent_node_identity);
        let mut evidence_refs = vec![UiGraphEvidenceRef::GraphNode(parent_node_identity)];
        evidence_refs.extend(
            lookup
                .value()
                .iter()
                .copied()
                .map(UiGraphEvidenceRef::GraphNode),
        );

        scalar(
            self.snapshot,
            UiGraphInspectionTarget::ParentChild(parent_node_identity),
            lookup,
            evidence_refs,
        )
    }

    pub fn inspect_slot_occupants(
        self,
        parent_node_identity: UiGraphNodeIdentity,
        slot_name: &str,
    ) -> UiGraphInspection<&'a [UiGraphNodeIdentity]> {
        let lookup = self
            .snapshot
            .lookup()
            .slot_occupants(parent_node_identity, slot_name);
        let mut evidence_refs = vec![UiGraphEvidenceRef::GraphNode(parent_node_identity)];
        evidence_refs.extend(
            lookup
                .value()
                .iter()
                .copied()
                .map(UiGraphEvidenceRef::GraphNode),
        );

        scalar(
            self.snapshot,
            UiGraphInspectionTarget::SlotOccupancy {
                parent_node_identity,
                slot_name: slot_name.into(),
            },
            lookup,
            evidence_refs,
        )
    }

    pub fn inspect_page_participation(
        self,
        page_node_identity: UiGraphNodeIdentity,
        axis: UiGraphParticipationAxis,
    ) -> UiGraphInspection<&'a [UiGraphPageParticipationMember]> {
        let lookup = self
            .snapshot
            .lookup()
            .page_participation(page_node_identity, axis);
        let mut evidence_refs = vec![UiGraphEvidenceRef::Page(page_node_identity)];
        evidence_refs.extend(
            lookup
                .value()
                .iter()
                .map(|member| UiGraphEvidenceRef::GraphNode(member.member_node_identity())),
        );

        scalar(
            self.snapshot,
            UiGraphInspectionTarget::PageParticipation {
                page_node_identity,
                axis,
            },
            lookup,
            evidence_refs,
        )
    }

    pub fn inspect_aspect_publishers(
        self,
        aspect: &UiAspectName,
    ) -> UiGraphInspection<&'a [crate::graph::UiGraphAspectPublisher]> {
        let lookup = self.snapshot.lookup().published_aspect(aspect);
        let mut evidence_refs = vec![UiGraphEvidenceRef::Aspect(aspect.clone())];
        evidence_refs.extend(
            lookup
                .value()
                .iter()
                .flat_map(|publisher| match publisher.kind() {
                    UiGraphAspectPublisherKind::GraphNode(node_identity) => {
                        [Some(UiGraphEvidenceRef::GraphNode(node_identity)), None]
                    }
                    UiGraphAspectPublisherKind::MountedReceiptSlot(receipt_identity) => [
                        Some(UiGraphEvidenceRef::MountedReceipt(receipt_identity)),
                        None,
                    ],
                    UiGraphAspectPublisherKind::FutureReceipt => [None, None],
                })
                .flatten(),
        );

        scalar(
            self.snapshot,
            UiGraphInspectionTarget::PublishedAspect(aspect.clone()),
            lookup,
            evidence_refs,
        )
    }

    pub fn inspect_aspect_consumers(
        self,
        aspect: &UiAspectName,
    ) -> UiGraphInspection<&'a [crate::graph::UiGraphAspectConsumer]> {
        let lookup = self.snapshot.lookup().consumed_aspect(aspect);
        let mut evidence_refs = vec![UiGraphEvidenceRef::Aspect(aspect.clone())];
        evidence_refs.extend(lookup.value().iter().map(|consumer| match consumer.kind() {
            UiGraphAspectConsumerKind::GraphNode(node_identity) => {
                UiGraphEvidenceRef::GraphNode(node_identity)
            }
            UiGraphAspectConsumerKind::MountedReceiptSlot(receipt_identity) => {
                UiGraphEvidenceRef::MountedReceipt(receipt_identity)
            }
        }));

        scalar(
            self.snapshot,
            UiGraphInspectionTarget::ConsumedAspect(aspect.clone()),
            lookup,
            evidence_refs,
        )
    }

    pub fn inspect_mounted_receipt_slot(
        self,
        mounted_receipt_identity: UiMountedReceiptIdentity,
    ) -> Option<UiGraphInspection<UiGraphMountedReceiptAuthorityRecord>> {
        self.snapshot
            .lookup()
            .mounted_receipt_slot(mounted_receipt_identity)
            .map(|lookup| {
                let slot = lookup.value();
                scalar(
                    self.snapshot,
                    UiGraphInspectionTarget::MountedReceipt(mounted_receipt_identity),
                    lookup,
                    vec![
                        UiGraphEvidenceRef::MountedReceipt(mounted_receipt_identity),
                        UiGraphEvidenceRef::GraphNode(slot.graph_node_identity()),
                    ],
                )
            })
    }
}

fn scalar<'a, T>(
    snapshot: &'a UiGraphSnapshot,
    target: UiGraphInspectionTarget,
    lookup: UiGraphLookup<T>,
    evidence_refs: Vec<UiGraphEvidenceRef>,
) -> UiGraphInspection<T> {
    UiGraphInspection::new(snapshot.generation(), target, lookup, evidence_refs)
}
