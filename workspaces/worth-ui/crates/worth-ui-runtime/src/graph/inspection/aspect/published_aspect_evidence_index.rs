use std::collections::{BTreeMap, BTreeSet};

use super::aspect_evidence_neighborhood::{UiAspectEvidenceLookup, UiAspectEvidenceNeighborhood};
use super::super::aspect_evidence_record::{UiAspectEvidenceRecord, UiAspectEvidenceRecordKind};
use crate::evidence::{order_refs, UiEvidenceAuthorityGeneration, UiEvidenceRef};
use crate::graph::{
    UiGraphAspectPublisherKind, UiGraphNodeEvidenceIndex, UiGraphSnapshot, UiMountedReceiptIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiPublishedAspectEvidenceIndex {
    by_canonical_label: BTreeMap<Box<str>, UiAspectEvidenceNeighborhood>,
    canonical_label_by_ref_identity_digest: BTreeMap<u64, Box<str>>,
}

impl UiPublishedAspectEvidenceIndex {
    pub(crate) fn rebuild(
        graph_snapshot: &UiGraphSnapshot,
        graph_node_evidence_index: &UiGraphNodeEvidenceIndex,
    ) -> Self {
        let authority_generation =
            UiEvidenceAuthorityGeneration::new(graph_snapshot.generation().as_u64());
        let by_canonical_label: BTreeMap<Box<str>, UiAspectEvidenceNeighborhood> = graph_snapshot
            .core_indexes()
            .published_aspects()
            .iter()
            .map(|(aspect, publishers)| {
                (
                    aspect.canonical_label().into(),
                    neighborhood_for_publishers(
                        graph_snapshot,
                        graph_node_evidence_index,
                        aspect.canonical_label(),
                        publishers,
                        authority_generation,
                    ),
                )
            })
            .collect();
        let canonical_label_by_ref_identity_digest: BTreeMap<u64, Box<str>> = by_canonical_label
            .iter()
            .flat_map(|(canonical_label, neighborhood)| {
                neighborhood
                    .refs()
                    .iter()
                    .map(move |evidence_ref| (evidence_ref.identity().digest(), canonical_label.clone()))
            })
            .collect();

        Self {
            by_canonical_label,
            canonical_label_by_ref_identity_digest,
        }
    }

    pub(crate) fn lookup(&self, canonical_label: &str) -> Option<UiAspectEvidenceLookup<'_>> {
        self.by_canonical_label
            .get(canonical_label)
            .map(UiAspectEvidenceLookup::indexed_hit)
    }

    pub(crate) fn lookup_ref_identity_digest(&self, identity_digest: u64) -> Option<&str> {
        self.canonical_label_by_ref_identity_digest
            .get(&identity_digest)
            .map(Box::as_ref)
    }
}

fn neighborhood_for_publishers(
    graph_snapshot: &UiGraphSnapshot,
    graph_node_evidence_index: &UiGraphNodeEvidenceIndex,
    canonical_label: &str,
    publishers: &[crate::graph::UiGraphAspectPublisher],
    authority_generation: UiEvidenceAuthorityGeneration,
) -> UiAspectEvidenceNeighborhood {
    let mut declaration_artifact_indexes = BTreeSet::new();
    let mut refs = Vec::<UiEvidenceRef>::new();

    for publisher in publishers {
        match publisher.kind() {
            UiGraphAspectPublisherKind::GraphNode(graph_node_identity) => {
                declaration_artifact_indexes.insert(
                    graph_node_evidence_index
                        .lookup_graph_node_identity(graph_node_identity)
                        .expect("published aspect node should resolve through graph-node evidence index")
                        .neighborhood()
                        .declaration_artifact_index(),
                );
                refs.push(
                    UiAspectEvidenceRecord::new(
                        canonical_label,
                        UiAspectEvidenceRecordKind::PublishedGraphNode(
                            graph_node_identity.digest(),
                        ),
                        authority_generation,
                    )
                    .reference(),
                );
            }
            UiGraphAspectPublisherKind::MountedReceiptSlot(mounted_receipt_identity) => {
                declaration_artifact_indexes.insert(receipt_artifact_index(
                    graph_snapshot,
                    graph_node_evidence_index,
                    mounted_receipt_identity,
                ));
                refs.push(
                    UiAspectEvidenceRecord::new(
                        canonical_label,
                        UiAspectEvidenceRecordKind::PublishedMountedReceipt(
                            mounted_receipt_identity.digest(),
                        ),
                        authority_generation,
                    )
                    .reference(),
                );
            }
            UiGraphAspectPublisherKind::FutureReceipt => {}
        }
    }

    UiAspectEvidenceNeighborhood::new(
        declaration_artifact_indexes.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        order_refs(refs),
    )
}

fn receipt_artifact_index(
    graph_snapshot: &UiGraphSnapshot,
    graph_node_evidence_index: &UiGraphNodeEvidenceIndex,
    mounted_receipt_identity: UiMountedReceiptIdentity,
) -> usize {
    let mounted_receipt_slot = graph_snapshot
        .lookup()
        .mounted_receipt_slot(mounted_receipt_identity)
        .expect("published aspect receipt should resolve through mounted receipt index");
    graph_node_evidence_index
        .lookup_graph_node_identity(mounted_receipt_slot.value().graph_node_identity())
        .expect("mounted receipt graph node should resolve through graph-node evidence index")
        .neighborhood()
        .declaration_artifact_index()
}
