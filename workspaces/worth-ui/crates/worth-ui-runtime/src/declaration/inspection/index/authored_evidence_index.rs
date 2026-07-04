use std::collections::BTreeMap;

use worth_ui_inspection::{
    UiAuthoredSourceProvenanceRef, UiEvidenceAuthorityGeneration, UiInspectionDeclarationIdentity,
};

use crate::admission::{UiAdmissionBoundary, UiAdmissionTarget, UiAdmissionWorld};
use crate::declaration::inspection::UiDeclarationEvidenceRecord;
use crate::declaration::UiDeclarationArtifact;
use crate::evidence::{order_refs, UiEvidenceRef};
use crate::graph::UiGraphSnapshot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiDeclarationAuthoredEvidenceIndex {
    by_declaration_identity:
        BTreeMap<UiInspectionDeclarationIdentity, UiDeclarationAuthoredEvidenceNeighborhood>,
    by_authored_provenance:
        BTreeMap<UiAuthoredSourceProvenanceRef, UiDeclarationAuthoredEvidenceNeighborhood>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiDeclarationAuthoredEvidenceNeighborhood {
    declaration_artifact_index: usize,
    refs: Box<[UiEvidenceRef]>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiDeclarationAuthoredLookupCost {
    declaration_identity_index_lookups: usize,
    authored_provenance_index_lookups: usize,
    declaration_artifact_scans: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiDeclarationAuthoredLookup<'a> {
    neighborhood: &'a UiDeclarationAuthoredEvidenceNeighborhood,
    cost: UiDeclarationAuthoredLookupCost,
}

impl UiDeclarationAuthoredEvidenceIndex {
    pub(crate) fn rebuild(
        declaration_artifacts: &[UiDeclarationArtifact],
        graph_snapshot: &UiGraphSnapshot,
    ) -> Self {
        let authority_generation = UiEvidenceAuthorityGeneration::new(graph_snapshot.generation().as_u64());
        let admission_boundary = UiAdmissionBoundary::new(declaration_artifacts, graph_snapshot);
        let by_declaration_identity = declaration_artifacts
            .iter()
            .enumerate()
            .map(|(declaration_artifact_index, artifact)| {
                (
                    artifact.identity().inspection_identity(),
                    authored_neighborhood_for_artifact(
                        declaration_artifact_index,
                        artifact,
                        authority_generation,
                        graph_snapshot,
                        &admission_boundary,
                    ),
                )
            })
            .collect();
        let by_authored_provenance = declaration_artifacts
            .iter()
            .enumerate()
            .map(|(declaration_artifact_index, artifact)| {
                (
                    artifact.provenance().inspection_authored_source_provenance_ref(),
                    authored_neighborhood_for_artifact(
                        declaration_artifact_index,
                        artifact,
                        authority_generation,
                        graph_snapshot,
                        &admission_boundary,
                    ),
                )
            })
            .collect();

        Self {
            by_declaration_identity,
            by_authored_provenance,
        }
    }

    pub(crate) fn lookup_declaration_identity(
        &self,
        identity: UiInspectionDeclarationIdentity,
    ) -> Option<UiDeclarationAuthoredLookup<'_>> {
        self.by_declaration_identity
            .get(&identity)
            .map(UiDeclarationAuthoredLookup::declaration_identity_hit)
    }

    pub(crate) fn lookup_authored_provenance(
        &self,
        provenance: &UiAuthoredSourceProvenanceRef,
    ) -> Option<UiDeclarationAuthoredLookup<'_>> {
        self.by_authored_provenance
            .get(provenance)
            .map(UiDeclarationAuthoredLookup::authored_provenance_hit)
    }
}

impl UiDeclarationAuthoredEvidenceNeighborhood {
    pub(crate) fn declaration_artifact_index(&self) -> usize {
        self.declaration_artifact_index
    }

    pub(crate) fn refs(&self) -> &[UiEvidenceRef] {
        &self.refs
    }
}

impl UiDeclarationAuthoredLookupCost {
    pub(crate) const fn index_lookups(self) -> usize {
        self.declaration_identity_index_lookups + self.authored_provenance_index_lookups
    }

    #[cfg(test)]
    pub(crate) const fn declaration_identity_index_lookups(self) -> usize {
        self.declaration_identity_index_lookups
    }

    #[cfg(test)]
    pub(crate) const fn authored_provenance_index_lookups(self) -> usize {
        self.authored_provenance_index_lookups
    }

    #[cfg(test)]
    pub(crate) const fn declaration_artifact_scans(self) -> usize {
        self.declaration_artifact_scans
    }
}

impl<'a> UiDeclarationAuthoredLookup<'a> {
    fn declaration_identity_hit(
        neighborhood: &'a UiDeclarationAuthoredEvidenceNeighborhood,
    ) -> Self {
        Self {
            neighborhood,
            cost: UiDeclarationAuthoredLookupCost {
                declaration_identity_index_lookups: 1,
                authored_provenance_index_lookups: 0,
                declaration_artifact_scans: 0,
            },
        }
    }

    fn authored_provenance_hit(
        neighborhood: &'a UiDeclarationAuthoredEvidenceNeighborhood,
    ) -> Self {
        Self {
            neighborhood,
            cost: UiDeclarationAuthoredLookupCost {
                declaration_identity_index_lookups: 0,
                authored_provenance_index_lookups: 1,
                declaration_artifact_scans: 0,
            },
        }
    }

    pub(crate) const fn neighborhood(self) -> &'a UiDeclarationAuthoredEvidenceNeighborhood {
        self.neighborhood
    }

    pub(crate) const fn cost(self) -> UiDeclarationAuthoredLookupCost {
        self.cost
    }
}

fn authored_neighborhood_for_artifact(
    declaration_artifact_index: usize,
    artifact: &UiDeclarationArtifact,
    authority_generation: UiEvidenceAuthorityGeneration,
    graph_snapshot: &UiGraphSnapshot,
    admission_boundary: &UiAdmissionBoundary<'_>,
) -> UiDeclarationAuthoredEvidenceNeighborhood {
    let declaration_ref =
        UiDeclarationEvidenceRecord::for_artifact(artifact).bind_ref(authority_generation);
    let mut refs = vec![declaration_ref];
    if let Some(admission_ref) =
        authored_admission_ref_for_artifact(artifact, graph_snapshot, admission_boundary)
    {
        refs.push(admission_ref);
    }

    UiDeclarationAuthoredEvidenceNeighborhood {
        declaration_artifact_index,
        refs: order_refs(refs),
    }
}

fn authored_admission_ref_for_artifact(
    artifact: &UiDeclarationArtifact,
    graph_snapshot: &UiGraphSnapshot,
    admission_boundary: &UiAdmissionBoundary<'_>,
) -> Option<UiEvidenceRef> {
    let graph_node_identity = graph_snapshot
        .core_indexes()
        .declaration_correspondence()
        .single_graph_node_for(artifact.identity())?;
    let report = admission_boundary.report(UiAdmissionTarget::graph_node(
        graph_node_identity,
        UiAdmissionWorld::authoritative(),
    ));

    Some(report.evidence_ref())
}
