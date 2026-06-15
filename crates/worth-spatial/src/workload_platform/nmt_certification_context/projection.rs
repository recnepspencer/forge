use std::collections::BTreeSet;

use topology::facade::NmtTopologyScopeReceipt;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{NmtCertificationDenial, NmtCertificationDenialInput, NmtCertificationDenialKind};
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NmtScopeProjectionCounters {
    scope_projected_faces: usize,
    scope_projected_edges: usize,
    scope_projected_loops: usize,
    scope_local_basis_parts: usize,
    parent_projected_entities_read: usize,
    scope_projected_entities_consumed: usize,
}

impl NmtScopeProjectionCounters {
    fn new(
        scope_projected_faces: usize,
        scope_projected_edges: usize,
        scope_projected_loops: usize,
        scope_local_basis_parts: usize,
        parent_projected_entities_read: usize,
    ) -> Self {
        Self {
            scope_projected_faces,
            scope_projected_edges,
            scope_projected_loops,
            scope_local_basis_parts,
            parent_projected_entities_read,
            scope_projected_entities_consumed: scope_projected_faces
                + scope_projected_edges
                + scope_projected_loops,
        }
    }

    pub fn scope_projected_faces(self) -> usize {
        self.scope_projected_faces
    }

    pub fn scope_projected_edges(self) -> usize {
        self.scope_projected_edges
    }

    pub fn scope_projected_loops(self) -> usize {
        self.scope_projected_loops
    }

    pub fn scope_local_basis_parts(self) -> usize {
        self.scope_local_basis_parts
    }

    pub fn parent_projected_entities_read(self) -> usize {
        self.parent_projected_entities_read
    }

    pub fn scope_projected_entities_consumed(self) -> usize {
        self.scope_projected_entities_consumed
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NmtScopeProjectionReceipt {
    parent_projection_identity: String,
    scope_identity: String,
    scope_projection_identity: String,
    local_frame_identity: String,
    consumed_projected_entities: Vec<String>,
    counters: NmtScopeProjectionCounters,
}

impl NmtScopeProjectionReceipt {
    pub(crate) fn from_projected_workload_scope(
        projected: &ProjectedPlanarWorkload,
        scope: &NmtTopologyScopeReceipt,
    ) -> Result<Self, NmtCertificationDenial> {
        let parent_projection_identity = projected.receipts().stage_identity().receipt_identity();
        let parent_local_frame_identity = projected
            .receipts()
            .local_frame_receipt()
            .local_basis_identity()
            .to_string();
        let faces = projected
            .projected_faces()
            .iter()
            .filter(|face| {
                scope.contains_topology_entity(face.identity().topology_entity_identity())
            })
            .map(|face| face.identity().projected_fact_identity().to_string())
            .collect::<Vec<_>>();
        let edges = projected
            .projected_edges()
            .edges()
            .iter()
            .filter(|edge| {
                scope.contains_topology_entity(edge.identity().topology_entity_identity())
            })
            .map(|edge| edge.identity().projected_fact_identity().to_string())
            .collect::<Vec<_>>();
        let loops = projected
            .projected_loops()
            .iter()
            .filter(|loop_row| {
                scope.contains_topology_entity(loop_row.identity().topology_entity_identity())
            })
            .map(|loop_row| {
                if loop_row.boundary().is_none() {
                    return Err(denial(
                        NmtCertificationDenialKind::MissingScopeProjection,
                        scope,
                        parent_projection_identity.clone(),
                        "NMT scope projection requires loop boundary geometry from the production projected workload.",
                    ));
                }
                Ok(loop_row.identity().projected_fact_identity().to_string())
            })
            .collect::<Result<Vec<_>, _>>()?;

        if faces.len() != scope.face_identities().len()
            || edges.len() != scope.edge_identities().len()
            || loops.len() != scope.loop_identities().len()
        {
            return Err(denial(
                NmtCertificationDenialKind::MissingScopeProjection,
                scope,
                parent_projection_identity,
                format!(
                    "{} projection consumed faces {}/{}, edges {}/{}, loops {}/{}.",
                    scope.kind().human_name(),
                    faces.len(),
                    scope.face_identities().len(),
                    edges.len(),
                    scope.edge_identities().len(),
                    loops.len(),
                    scope.loop_identities().len()
                ),
            ));
        }

        let mut consumed_projected_entities = faces;
        consumed_projected_entities.extend(edges);
        consumed_projected_entities.extend(loops);
        require_unique_entities(
            scope,
            &consumed_projected_entities,
            &parent_projection_identity,
        )?;
        let counters = NmtScopeProjectionCounters::new(
            scope.face_identities().len(),
            scope.edge_identities().len(),
            scope.loop_identities().len(),
            projected.receipts().counters().local_basis_parts(),
            projected
                .receipts()
                .counters()
                .projected_topology_entities(),
        );
        let scope_projection_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-scope-projection".to_string(),
                parent_projection_identity.clone(),
                scope.scope_identity().to_string(),
                consumed_projected_entities.join("|"),
            ],
        );
        let local_frame_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-scope-local-frame".to_string(),
                parent_local_frame_identity,
                scope.scope_identity().to_string(),
                scope_projection_identity.clone(),
            ],
        );
        Ok(Self {
            parent_projection_identity,
            scope_identity: scope.scope_identity().to_string(),
            scope_projection_identity,
            local_frame_identity,
            consumed_projected_entities,
            counters,
        })
    }

    pub fn parent_projection_identity(&self) -> &str {
        &self.parent_projection_identity
    }

    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }

    pub fn scope_projection_identity(&self) -> &str {
        &self.scope_projection_identity
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn consumed_projected_entities(&self) -> &[String] {
        &self.consumed_projected_entities
    }

    pub fn counters(&self) -> NmtScopeProjectionCounters {
        self.counters
    }
}

fn require_unique_entities(
    scope: &NmtTopologyScopeReceipt,
    consumed: &[String],
    parent_projection_identity: &str,
) -> Result<(), NmtCertificationDenial> {
    let mut seen = BTreeSet::new();
    if consumed.iter().all(|identity| seen.insert(identity)) {
        Ok(())
    } else {
        Err(denial(
            NmtCertificationDenialKind::AggregateReceiptWithoutScopeProof,
            scope,
            parent_projection_identity.to_string(),
            "NMT scope projection cannot certify duplicate projected entity identities.",
        ))
    }
}

fn denial(
    kind: NmtCertificationDenialKind,
    scope: &NmtTopologyScopeReceipt,
    evidence: String,
    human_reason: impl Into<String>,
) -> NmtCertificationDenial {
    NmtCertificationDenial::new(NmtCertificationDenialInput {
        kind,
        target_scope_identity: Some(scope.scope_identity().to_string()),
        source_scope_identity: None,
        target_scope_kind: Some(scope.kind()),
        consumed_evidence_digest: evidence,
        human_reason: human_reason.into(),
        counters: super::NmtScopeAttackCounters::new(
            1,
            scope.counters().scope_entity_count(),
            0,
            0,
            0,
            1,
        ),
    })
}
