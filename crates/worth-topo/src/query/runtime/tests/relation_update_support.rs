use forge_query::facade::ForgeQueryWorkspace;
use forge_relational::facade::identity::{EntityId, RelationId};
use serde_json::Value;
use worth_schema::facade::DerivedTopologyReadBasis;
use worth_schema::facade::WorthTopologyRelationKind;

use crate::query::domain::parity::{
    build_domain_query_view_parity_artifact, WorthTopologyDomainQueryParityKind,
    WorthTopologyDomainQueryViewParityArtifact, WorthTopologyDomainQueryViewRef,
};
use crate::query::domain::proof::WorthTopologyDomainQueryProofReport;
use crate::query::domain::report::WorthTopologyDomainQueryAggregateReport;
use crate::query::{WorthTopologyDomainQuery, WorthTopologyQueryAssembly};

pub(super) fn query_relation_id_from_row(
    row: &forge_query::facade::ForgeQueryEntity,
) -> RelationId {
    crate::query::query_relation_id_from_row(row).expect("query relation provenance should decode")
}

pub(super) fn query_entity_id_from_row(row: &forge_query::facade::ForgeQueryEntity) -> EntityId {
    crate::query::query_entity_id_from_row(row).expect("query entity provenance should decode")
}

pub(super) struct RelationUpdateQuerySupport {
    domain_query: WorthTopologyDomainQuery,
    entity_rows: Vec<forge_query::facade::ForgeQueryEntity>,
}

impl RelationUpdateQuerySupport {
    pub(super) fn load(
        workspace: &ForgeQueryWorkspace,
        assembly: &WorthTopologyQueryAssembly,
    ) -> Self {
        let entity_rows = workspace.read::<Value>(assembly.entities());
        Self {
            domain_query: WorthTopologyDomainQuery::load(&workspace, assembly)
                .expect("topology domain query should load for relation update support"),
            entity_rows,
        }
    }

    pub(super) fn first_source_identity_for_relation_kind(
        &self,
        relation_kind: WorthTopologyRelationKind,
    ) -> String {
        self.domain_query
            .first_source_identity_for_relation_kind(relation_kind)
            .expect("seeded topology should expose requested source relation")
    }

    pub(super) fn aggregate_report(&self) -> WorthTopologyDomainQueryAggregateReport {
        self.domain_query.aggregate_report()
    }

    pub(super) fn proof_report(&self) -> WorthTopologyDomainQueryProofReport {
        self.domain_query.proof_report()
    }

    pub(super) fn record_view_parity(
        &self,
        parity_kind: WorthTopologyDomainQueryParityKind,
        left: &WorthTopologyDomainQueryViewParityArtifact,
        right: &WorthTopologyDomainQueryViewParityArtifact,
    ) {
        let _ = self
            .domain_query
            .record_view_parity(parity_kind, left, right);
    }

    pub(super) fn find_entity_id_by_identity(&self, identity: &str) -> EntityId {
        self.domain_query
            .find_entity_id_by_identity(identity)
            .expect("requested identity should resolve to one entity")
    }

    pub(super) fn find_entity_identity_by_id(&self, entity_id: EntityId) -> String {
        self.domain_query
            .find_entity_identity_by_id(entity_id)
            .expect("requested entity id should resolve to one identity")
    }

    pub(super) fn alternate_same_edge_half_edge_id(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
        current_target_identity: &str,
    ) -> EntityId {
        self.domain_query
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

    pub(super) fn radial_current_target_identity(&self, source_identity: &str) -> String {
        self.domain_query
            .outgoing_target_identity(
                source_identity,
                WorthTopologyRelationKind::HalfEdgeRadialNext,
            )
            .expect("seeded topology should expose radial successor binding")
    }

    pub(super) fn different_edge_half_edge_id(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        source_identity: &str,
    ) -> EntityId {
        self.domain_query
            .radial_half_edge_neighborhood(workspace, source_identity)
            .expect("seeded topology should expose radial neighborhood")
            .different_edge_half_edge_identities
            .iter()
            .find(|identity| identity.as_str() != source_identity)
            .map(|identity| self.find_entity_id_by_identity(identity))
            .expect("seeded edge fan should provide a halfedge on a different edge")
    }

    pub(super) fn relation_id_for_source_kind(
        &self,
        source_identity: &str,
        relation_kind: WorthTopologyRelationKind,
    ) -> RelationId {
        self.domain_query
            .relation_id_for_source_kind(source_identity, relation_kind)
            .expect("seeded topology should expose requested source/kind relation")
    }

    pub(super) fn next_target_half_edge_id(&self, source_identity: &str) -> EntityId {
        let local_rewire = self
            .domain_query
            .local_rewire_neighborhood(source_identity, 2)
            .expect("seeded topology should expose local rewire neighborhood");
        self.find_entity_id_by_identity(&local_rewire.old_successor_identity)
    }

    pub(super) fn prev_target_half_edge_id(&self, source_identity: &str) -> EntityId {
        let local_rewire = self
            .domain_query
            .local_rewire_neighborhood(source_identity, 2)
            .expect("seeded topology should expose local rewire neighborhood");
        self.find_entity_id_by_identity(&local_rewire.old_predecessor_identity)
    }

    pub(super) fn half_edge_identities_for_different_loops(&self) -> (String, String) {
        let half_edges = self
            .entity_rows
            .iter()
            .map(|row| row.identity.as_str())
            .filter(|identity| {
                self.domain_query
                    .incoming_source_identity(identity, WorthTopologyRelationKind::LoopOwnsHalfEdge)
                    .is_ok()
            })
            .collect::<Vec<_>>();
        for left in &half_edges {
            let left_loop = self
                .domain_query
                .incoming_source_identity(left, WorthTopologyRelationKind::LoopOwnsHalfEdge)
                .expect("seeded topology should expose loop ownership");
            for right in &half_edges {
                if left == right {
                    continue;
                }
                let right_loop = self
                    .domain_query
                    .incoming_source_identity(right, WorthTopologyRelationKind::LoopOwnsHalfEdge)
                    .expect("seeded topology should expose loop ownership");
                if left_loop != right_loop {
                    return ((*left).to_string(), (*right).to_string());
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
        self.domain_query
            .loop_cycle(workspace, start_identity, count)
            .expect("seeded topology should expose a closed successor cycle")
            .cycle_identities
    }

    pub(super) fn local_rewire_parity_artifact(
        &self,
        read_basis: &DerivedTopologyReadBasis,
        moved_identity: &str,
        cycle_count: usize,
    ) -> WorthTopologyDomainQueryViewParityArtifact {
        let local_rewire = self
            .domain_query
            .local_rewire_neighborhood(moved_identity, cycle_count)
            .expect("seeded topology should expose local rewire neighborhood");
        build_domain_query_view_parity_artifact(
            read_basis,
            WorthTopologyDomainQueryViewRef::LocalRewire(&local_rewire),
        )
    }
}
