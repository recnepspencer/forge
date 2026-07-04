use std::collections::BTreeMap;

use crate::admission::{UiAdmissionBoundary, UiAdmissionTarget, UiAdmissionWorld};
use crate::declaration::UiDeclarationEvidenceRecord;
use crate::declaration::UiDeclarationArtifact;
use crate::evidence::{order_refs, UiEvidenceAuthorityGeneration, UiEvidenceRef};
use crate::graph::{UiGraphEvidenceRecord, UiGraphNodeIdentity, UiGraphSnapshot};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiGraphNodeEvidenceIndex {
    by_graph_node_identity: BTreeMap<UiGraphNodeIdentity, UiGraphNodeEvidenceNeighborhood>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiGraphNodeEvidenceNeighborhood {
    declaration_artifact_index: usize,
    refs: Box<[UiEvidenceRef]>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiGraphNodeEvidenceLookupCost {
    graph_node_identity_index_lookups: usize,
    graph_node_scan_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiGraphNodeEvidenceLookup<'a> {
    neighborhood: &'a UiGraphNodeEvidenceNeighborhood,
    cost: UiGraphNodeEvidenceLookupCost,
}

impl UiGraphNodeEvidenceIndex {
    pub(crate) fn rebuild(
        declaration_artifacts: &[UiDeclarationArtifact],
        graph_snapshot: &UiGraphSnapshot,
    ) -> Self {
        let authority_generation = UiEvidenceAuthorityGeneration::new(graph_snapshot.generation().as_u64());
        let admission_boundary = UiAdmissionBoundary::new(declaration_artifacts, graph_snapshot);
        let artifact_indexes_by_declaration = declaration_artifacts
            .iter()
            .enumerate()
            .map(|(artifact_index, artifact)| (artifact.identity().digest().raw(), artifact_index))
            .collect::<BTreeMap<_, _>>();
        let by_graph_node_identity = graph_snapshot
            .nodes()
            .iter()
            .map(|node| {
                let declaration_artifact_index = *artifact_indexes_by_declaration
                    .get(&node.declaration_identity().digest().raw())
                    .expect("graph-backed inspection requires every admitted graph node to map to one declaration artifact");
                let artifact = &declaration_artifacts[declaration_artifact_index];
                let graph_node_identity = node.graph_node_identity();

                (
                    graph_node_identity,
                    graph_node_neighborhood(
                        declaration_artifact_index,
                        artifact,
                        graph_node_identity,
                        authority_generation,
                        graph_snapshot,
                        &admission_boundary,
                    ),
                )
            })
            .collect();

        Self {
            by_graph_node_identity,
        }
    }

    pub(crate) fn lookup_graph_node_identity(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<UiGraphNodeEvidenceLookup<'_>> {
        self.by_graph_node_identity
            .get(&graph_node_identity)
            .map(UiGraphNodeEvidenceLookup::graph_node_identity_hit)
    }
}

impl UiGraphNodeEvidenceNeighborhood {
    pub(crate) fn declaration_artifact_index(&self) -> usize {
        self.declaration_artifact_index
    }

    pub(crate) fn refs(&self) -> &[UiEvidenceRef] {
        &self.refs
    }
}

impl UiGraphNodeEvidenceLookupCost {
    pub(crate) const fn index_lookups(self) -> usize {
        self.graph_node_identity_index_lookups
    }

    #[cfg(test)]
    pub(crate) const fn graph_node_identity_index_lookups(self) -> usize {
        self.graph_node_identity_index_lookups
    }

    #[cfg(test)]
    pub(crate) const fn graph_node_scan_count(self) -> usize {
        self.graph_node_scan_count
    }
}

impl<'a> UiGraphNodeEvidenceLookup<'a> {
    fn graph_node_identity_hit(neighborhood: &'a UiGraphNodeEvidenceNeighborhood) -> Self {
        Self {
            neighborhood,
            cost: UiGraphNodeEvidenceLookupCost {
                graph_node_identity_index_lookups: 1,
                graph_node_scan_count: 0,
            },
        }
    }

    pub(crate) const fn neighborhood(self) -> &'a UiGraphNodeEvidenceNeighborhood {
        self.neighborhood
    }

    pub(crate) const fn cost(self) -> UiGraphNodeEvidenceLookupCost {
        self.cost
    }
}

fn graph_node_neighborhood(
    declaration_artifact_index: usize,
    artifact: &UiDeclarationArtifact,
    graph_node_identity: UiGraphNodeIdentity,
    authority_generation: UiEvidenceAuthorityGeneration,
    graph_snapshot: &UiGraphSnapshot,
    admission_boundary: &UiAdmissionBoundary<'_>,
) -> UiGraphNodeEvidenceNeighborhood {
    let graph_ref = UiGraphEvidenceRecord::for_snapshot(graph_snapshot, graph_node_identity.digest()).reference();
    let declaration_ref =
        UiDeclarationEvidenceRecord::for_artifact(artifact).bind_ref(authority_generation);
    let admission_report = admission_boundary.report(UiAdmissionTarget::graph_node(
        graph_node_identity,
        UiAdmissionWorld::authoritative(),
    ));
    let admission_ref = admission_report.evidence_ref();
    let obligation_refs = admission_report
        .evidence_index()
        .records()
        .iter()
        .filter(|record| record.graph_node_digest() == graph_node_identity.digest())
        .map(|record| record.evidence_ref(authority_generation));
    let refs = std::iter::once(graph_ref)
        .chain(std::iter::once(declaration_ref))
        .chain(std::iter::once(admission_ref))
        .chain(obligation_refs)
        .collect::<Vec<_>>();

    UiGraphNodeEvidenceNeighborhood {
        declaration_artifact_index,
        refs: order_refs(refs),
    }
}
