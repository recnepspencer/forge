use forge_query::facade::{ForgeQueryEntityIdentity, ForgeQueryWorkspace};
use forge_relational::facade::identity::{EntityId, RelationId};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityKind;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use serde_json::Value;

use crate::certification::support::read_proof_harness::TopologyReadProofHarness;
use crate::projection::read_views::domain::closeout::TopologyReadCloseoutReport;
use crate::projection::read_views::domain::parity::{
    build_topology_read_view_parity_artifact, TopologyReadParityKind,
    TopologyReadViewParityArtifact, TopologyReadViewRef,
};
use crate::projection::read_views::domain::read_proof::TopologyReadProofReport;
use crate::projection::read_views::domain::report::TopologyReadAggregateReport;
use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
use crate::projection::TopologyQueryRowLookup;

pub(super) fn query_relation_id_from_row(
    row: &forge_query::facade::ForgeQueryEntity,
) -> RelationId {
    crate::projection::query_relation_id_from_row(row)
        .expect("query relation provenance should decode")
}

pub(super) fn query_entity_id_from_row(row: &forge_query::facade::ForgeQueryEntity) -> EntityId {
    crate::projection::query_entity_id_from_row(row).expect("query entity provenance should decode")
}

pub(super) fn row_text<'a>(
    row: &'a forge_query::facade::ForgeQueryEntity,
    path: [&'static str; 2],
) -> Option<&'a str> {
    crate::query_native_runtime_boundary::row_text_at(row, path)
}

pub(super) struct QueryRuntimeSupport {
    topology_read: TopologyReadProofHarness,
    entity_rows: Vec<forge_query::facade::ForgeQueryEntity>,
    relation_rows: Vec<forge_query::facade::ForgeQueryEntity>,
}

impl QueryRuntimeSupport {
    pub(super) fn load(
        workspace: &mut ForgeQueryWorkspace,
        surfaces: &TopologyDeclaredQuerySurfaces,
    ) -> Self {
        let entity_rows = workspace.read::<Value>(surfaces.entities());
        let relation_rows = workspace.read::<Value>(surfaces.relations());
        Self {
            topology_read: TopologyReadProofHarness::current_head(),
            entity_rows,
            relation_rows,
        }
    }

    pub(super) fn first_source_identity_for_relation_kind(
        &self,
        relation_kind: TopologyRelationKind,
    ) -> String {
        self.lookup()
            .first_source_identity_for_relation_kind(relation_kind)
            .expect("seeded topology should expose requested source relation")
    }

    pub(super) fn aggregate_report(&self) -> TopologyReadAggregateReport {
        self.topology_read.aggregate_report()
    }

    pub(super) fn proof_report(&self) -> TopologyReadProofReport {
        self.topology_read.proof_report()
    }

    pub(super) fn closeout_report(&self) -> TopologyReadCloseoutReport {
        self.topology_read.closeout_report()
    }

    pub(super) fn record_view_parity(
        &self,
        parity_kind: TopologyReadParityKind,
        left: &TopologyReadViewParityArtifact,
        right: &TopologyReadViewParityArtifact,
    ) {
        let _ = self
            .topology_read
            .record_view_parity(parity_kind, left, right);
    }

    pub(super) fn find_entity_id_by_identity(&self, identity: &str) -> EntityId {
        self.lookup()
            .find_entity_id_by_identity(identity)
            .expect("requested identity should resolve to one entity")
    }

    pub(super) fn find_entity_identity_by_id(&self, entity_id: EntityId) -> String {
        self.lookup()
            .find_entity_identity_by_id(entity_id)
            .expect("requested entity id should resolve to one identity")
    }

    pub(super) fn alternate_same_edge_half_edge_id(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
        current_target_identity: &str,
    ) -> EntityId {
        self.topology_read
            .radial_half_edge_neighborhood(workspace, source_identity)
            .expect("seeded topology should expose radial neighborhood")
            .same_edge_half_edge_identities
            .iter()
            .find(|identity| {
                identity.as_str() != source_identity && identity.as_str() != current_target_identity
            })
            .map(|identity| self.find_entity_id_by_identity(identity))
            .expect("seeded edge fan should provide an alternate halfedge on the same edge")
    }

    pub(super) fn radial_current_target_identity(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> String {
        self.topology_read
            .radial_half_edge_neighborhood(workspace, source_identity)
            .expect("seeded topology should expose radial neighborhood")
            .current_target_half_edge_identity
    }

    pub(super) fn different_edge_half_edge_id(
        &self,
        _workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> EntityId {
        let source_edge_identity = self
            .lookup()
            .edge_identity_of_half_edge(source_identity)
            .expect("source half-edge should expose edge identity");
        self.lookup()
            .find_entity_id_by_identity(
                &self
                    .entity_rows
                    .iter()
                    .filter_map(|row| query_identity_label(row.identity()))
                    .find(|identity| {
                        identity.as_str() != source_identity
                            && self
                                .lookup()
                                .edge_identity_of_half_edge(identity.as_str())
                                .is_ok_and(|edge_identity| edge_identity != source_edge_identity)
                    })
                    .expect("seeded edge fan should expose an illegal radial target on a different edge"),
            )
            .expect("seeded edge fan should provide a halfedge on a different edge")
    }

    pub(super) fn relation_id_for_source_kind(
        &self,
        source_identity: &str,
        relation_kind: TopologyRelationKind,
    ) -> RelationId {
        self.lookup()
            .relation_id_for_source_kind(source_identity, relation_kind)
            .expect("seeded topology should expose requested source/kind relation")
    }

    pub(super) fn next_target_half_edge_id(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> EntityId {
        let local_rewire = self
            .topology_read
            .local_rewire_neighborhood(workspace, source_identity, 2)
            .expect("seeded topology should expose local rewire neighborhood");
        self.find_entity_id_by_identity(&local_rewire.old_successor_identity)
    }

    pub(super) fn prev_target_half_edge_id(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> EntityId {
        let local_rewire = self
            .topology_read
            .local_rewire_neighborhood(workspace, source_identity, 2)
            .expect("seeded topology should expose local rewire neighborhood");
        self.find_entity_id_by_identity(&local_rewire.old_predecessor_identity)
    }

    pub(super) fn half_edge_identities_for_different_loops(&self) -> (String, String) {
        let half_edges = self
            .entity_rows
            .iter()
            .filter_map(|row| query_identity_label(row.identity()))
            .filter(|identity| {
                self.lookup()
                    .incoming_source_identity(
                        identity.as_str(),
                        TopologyRelationKind::LoopOwnsHalfEdge,
                    )
                    .is_ok()
            })
            .collect::<Vec<_>>();
        for left in &half_edges {
            let left_loop = self
                .lookup()
                .incoming_source_identity(left.as_str(), TopologyRelationKind::LoopOwnsHalfEdge)
                .expect("seeded topology should expose loop ownership");
            for right in &half_edges {
                if left == right {
                    continue;
                }
                let right_loop = self
                    .lookup()
                    .incoming_source_identity(
                        right.as_str(),
                        TopologyRelationKind::LoopOwnsHalfEdge,
                    )
                    .expect("seeded topology should expose loop ownership");
                if left_loop != right_loop {
                    return (left.clone(), right.clone());
                }
            }
        }
        panic!("seeded topology should expose halfedges on different loops");
    }

    pub(super) fn successor_cycle_identities(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        start_identity: &str,
        count: usize,
    ) -> Vec<String> {
        self.topology_read
            .loop_cycle(workspace, start_identity, count)
            .expect("seeded topology should expose a closed successor cycle")
            .cycle_identities
    }

    pub(super) fn local_rewire_parity_artifact(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        read_basis: &DerivedTopologyReadBasis,
        moved_identity: &str,
        cycle_count: usize,
    ) -> TopologyReadViewParityArtifact {
        let local_rewire = self
            .topology_read
            .local_rewire_neighborhood(workspace, moved_identity, cycle_count)
            .expect("seeded topology should expose local rewire neighborhood");
        build_topology_read_view_parity_artifact(
            read_basis,
            TopologyReadViewRef::LocalRewire(&local_rewire),
        )
    }

    fn lookup(&self) -> TopologyQueryRowLookup<'_> {
        TopologyQueryRowLookup::new(&self.entity_rows, &self.relation_rows)
    }
}

fn query_identity_label(identity: &ForgeQueryEntityIdentity) -> Option<String> {
    let parts = identity.relational_record_parts()?;
    let kind = match parts.kind() {
        RelationalBridgeRecordIdentityKind::Entity => "entity",
        RelationalBridgeRecordIdentityKind::Relation => "relation",
    };
    Some(format!(
        "{kind}:{}:{}:{}",
        parts.partition_id(),
        parts.local_slot(),
        parts.generation()
    ))
}
