use std::collections::{BTreeMap, BTreeSet};

use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::{EntityId, RelationId};
use worth_schema::facade::WorthTopologyRelationKind;

use super::snapshot_rows::{
    query_entity_id_from_row, query_relation_id_from_row, relation_kind_name,
    topology_source_identity, topology_target_identity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorthTopologyQuerySnapshotIndexError {
    detail: String,
}

impl WorthTopologyQuerySnapshotIndexError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}

impl std::fmt::Display for WorthTopologyQuerySnapshotIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.detail)
    }
}

impl std::error::Error for WorthTopologyQuerySnapshotIndexError {}

pub(crate) struct WorthTopologyQuerySnapshotIndex {
    entity_id_by_identity: BTreeMap<String, EntityId>,
    entity_identity_by_id: BTreeMap<EntityId, String>,
    relation_id_by_source_kind: BTreeMap<(String, String), RelationId>,
    relation_id_by_kind_endpoints: BTreeMap<(String, String, String), RelationId>,
    target_by_source_kind: BTreeMap<(String, String), String>,
    #[allow(dead_code)]
    sources_by_target_kind: BTreeMap<(String, String), Vec<String>>,
    first_source_identity_by_kind: BTreeMap<String, String>,
    half_edge_edge_by_identity: BTreeMap<String, String>,
    half_edge_vertices_by_identity: BTreeMap<String, Vec<String>>,
    half_edges_by_vertex_identity: BTreeMap<String, Vec<String>>,
}

impl WorthTopologyQuerySnapshotIndex {
    pub(crate) fn new(
        entity_rows: &[ForgeQueryEntity],
        relation_rows: &[ForgeQueryEntity],
    ) -> Result<Self, WorthTopologyQuerySnapshotIndexError> {
        let mut entity_id_by_identity = BTreeMap::new();
        let mut entity_identity_by_id = BTreeMap::new();
        let mut half_edge_identities = Vec::new();

        for row in entity_rows {
            let entity_id = query_entity_id_from_row(row)?;
            entity_id_by_identity.insert(row.identity.clone(), entity_id);
            entity_identity_by_id.insert(entity_id, row.identity.clone());
            if relation_kind_name(row) == "worth.half_edge" {
                half_edge_identities.push(row.identity.clone());
            }
        }

        let mut relation_id_by_source_kind = BTreeMap::new();
        let mut relation_id_by_kind_endpoints = BTreeMap::new();
        let mut target_by_source_kind = BTreeMap::new();
        let mut sources_by_target_kind = BTreeMap::new();
        let mut first_source_identity_by_kind = BTreeMap::new();

        for row in relation_rows {
            let Some(source_identity) = topology_source_identity(row) else {
                continue;
            };
            let Some(target_identity) = topology_target_identity(row) else {
                continue;
            };
            let kind_name = relation_kind_name(row).to_string();
            let relation_id = query_relation_id_from_row(row)?;
            relation_id_by_source_kind
                .entry((source_identity.to_string(), kind_name.clone()))
                .or_insert(relation_id);
            relation_id_by_kind_endpoints
                .entry((
                    source_identity.to_string(),
                    target_identity.to_string(),
                    kind_name.clone(),
                ))
                .or_insert(relation_id);
            target_by_source_kind
                .entry((source_identity.to_string(), kind_name.clone()))
                .or_insert_with(|| target_identity.to_string());
            sources_by_target_kind
                .entry((target_identity.to_string(), kind_name.clone()))
                .or_insert_with(Vec::new)
                .push(source_identity.to_string());
            first_source_identity_by_kind
                .entry(kind_name)
                .or_insert_with(|| source_identity.to_string());
        }

        let mut half_edge_edge_by_identity = BTreeMap::new();
        let mut half_edge_vertices_by_identity = BTreeMap::new();
        let mut half_edges_by_vertex_identity = BTreeMap::new();

        for half_edge_identity in &half_edge_identities {
            let edge_identity = target_by_source_kind
                .get(&(
                    half_edge_identity.clone(),
                    WorthTopologyRelationKind::HalfEdgeUsesEdge
                        .kind_name()
                        .to_string(),
                ))
                .cloned()
                .ok_or_else(|| {
                    WorthTopologyQuerySnapshotIndexError::new(format!(
                        "half-edge `{half_edge_identity}` should expose `{}`",
                        WorthTopologyRelationKind::HalfEdgeUsesEdge.kind_name()
                    ))
                })?;
            half_edge_edge_by_identity.insert(half_edge_identity.clone(), edge_identity.clone());
            let _ = edge_identity;

            let mut vertices = Vec::with_capacity(2);
            for relation_kind in [
                WorthTopologyRelationKind::HalfEdgeStartsAtVertex,
                WorthTopologyRelationKind::HalfEdgeEndsAtVertex,
            ] {
                let vertex_identity = target_by_source_kind
                    .get(&(
                        half_edge_identity.clone(),
                        relation_kind.kind_name().to_string(),
                    ))
                    .cloned()
                    .ok_or_else(|| {
                        WorthTopologyQuerySnapshotIndexError::new(format!(
                            "half-edge `{half_edge_identity}` should expose `{}`",
                            relation_kind.kind_name()
                        ))
                    })?;
                half_edges_by_vertex_identity
                    .entry(vertex_identity.clone())
                    .or_insert_with(Vec::new)
                    .push(half_edge_identity.clone());
                vertices.push(vertex_identity);
            }
            half_edge_vertices_by_identity.insert(half_edge_identity.clone(), vertices);
        }

        Ok(Self {
            entity_id_by_identity,
            entity_identity_by_id,
            relation_id_by_source_kind,
            relation_id_by_kind_endpoints,
            target_by_source_kind,
            sources_by_target_kind,
            first_source_identity_by_kind,
            half_edge_edge_by_identity,
            half_edge_vertices_by_identity,
            half_edges_by_vertex_identity,
        })
    }

    pub(crate) fn first_source_identity_for_relation_kind(
        &self,
        relation_kind: WorthTopologyRelationKind,
    ) -> Result<String, WorthTopologyQuerySnapshotIndexError> {
        self.first_source_identity_by_kind
            .get(relation_kind.kind_name())
            .cloned()
            .ok_or_else(|| {
                WorthTopologyQuerySnapshotIndexError::new(format!(
                    "snapshot should expose `{}` source identities",
                    relation_kind.kind_name()
                ))
            })
    }

    pub(crate) fn find_entity_id_by_identity(
        &self,
        identity: &str,
    ) -> Result<EntityId, WorthTopologyQuerySnapshotIndexError> {
        self.entity_id_by_identity
            .get(identity)
            .copied()
            .ok_or_else(|| {
                WorthTopologyQuerySnapshotIndexError::new(format!(
                    "query identity `{identity}` should resolve to one entity"
                ))
            })
    }

    pub(crate) fn find_entity_identity_by_id(
        &self,
        entity_id: EntityId,
    ) -> Result<String, WorthTopologyQuerySnapshotIndexError> {
        self.entity_identity_by_id
            .get(&entity_id)
            .cloned()
            .ok_or_else(|| {
                WorthTopologyQuerySnapshotIndexError::new(format!(
                    "entity id `{:?}` should resolve to one query identity",
                    entity_id
                ))
            })
    }

    pub(crate) fn outgoing_target_identity(
        &self,
        source_identity: &str,
        relation_kind: WorthTopologyRelationKind,
    ) -> Result<String, WorthTopologyQuerySnapshotIndexError> {
        self.target_by_source_kind
            .get(&(
                source_identity.to_string(),
                relation_kind.kind_name().to_string(),
            ))
            .cloned()
            .ok_or_else(|| {
                WorthTopologyQuerySnapshotIndexError::new(format!(
                    "seeded topology should expose target identity for requested `{}` relation",
                    relation_kind.kind_name()
                ))
            })
    }

    #[allow(dead_code)]
    pub(crate) fn incoming_source_identity(
        &self,
        target_identity: &str,
        relation_kind: WorthTopologyRelationKind,
    ) -> Result<String, WorthTopologyQuerySnapshotIndexError> {
        self.sources_by_target_kind
            .get(&(target_identity.to_string(), relation_kind.kind_name().to_string()))
            .and_then(|sources| sources.first())
            .cloned()
            .ok_or_else(|| {
                WorthTopologyQuerySnapshotIndexError::new(format!(
                    "seeded topology should expose source identity for requested incoming `{}` relation",
                    relation_kind.kind_name()
                ))
            })
    }

    pub(crate) fn relation_id_for_source_kind(
        &self,
        source_identity: &str,
        relation_kind: WorthTopologyRelationKind,
    ) -> Result<RelationId, WorthTopologyQuerySnapshotIndexError> {
        self.relation_id_by_source_kind
            .get(&(
                source_identity.to_string(),
                relation_kind.kind_name().to_string(),
            ))
            .copied()
            .ok_or_else(|| {
                WorthTopologyQuerySnapshotIndexError::new(format!(
                    "seeded topology should expose requested source/kind relation"
                ))
            })
    }

    pub(crate) fn relation_id_by_kind_and_endpoints(
        &self,
        source_identity: &str,
        target_identity: &str,
        relation_kind: WorthTopologyRelationKind,
    ) -> Result<RelationId, WorthTopologyQuerySnapshotIndexError> {
        self.relation_id_by_kind_endpoints
            .get(&(
                source_identity.to_string(),
                target_identity.to_string(),
                relation_kind.kind_name().to_string(),
            ))
            .copied()
            .ok_or_else(|| {
                WorthTopologyQuerySnapshotIndexError::new(format!(
                    "query rows should expose `{}` relation between requested endpoints",
                    relation_kind.kind_name()
                ))
            })
    }

    pub(crate) fn half_edge_vertex_identities(
        &self,
        half_edge_identity: &str,
    ) -> Result<Vec<String>, WorthTopologyQuerySnapshotIndexError> {
        self.half_edge_vertices_by_identity
            .get(half_edge_identity)
            .cloned()
            .ok_or_else(|| {
                WorthTopologyQuerySnapshotIndexError::new(format!(
                    "half-edge `{half_edge_identity}` should expose vertex identities"
                ))
            })
    }

    pub(crate) fn edge_identity_of_half_edge(
        &self,
        half_edge_identity: &str,
    ) -> Result<String, WorthTopologyQuerySnapshotIndexError> {
        self.half_edge_edge_by_identity
            .get(half_edge_identity)
            .cloned()
            .ok_or_else(|| {
                WorthTopologyQuerySnapshotIndexError::new(format!(
                    "half-edge `{half_edge_identity}` should expose edge identity"
                ))
            })
    }

    pub(crate) fn half_edge_identities_sharing_vertex(
        &self,
        source_identity: &str,
    ) -> Result<Vec<String>, WorthTopologyQuerySnapshotIndexError> {
        let vertices = self.half_edge_vertex_identities(source_identity)?;
        let mut identities = BTreeSet::new();
        for vertex_identity in vertices {
            if let Some(half_edges) = self.half_edges_by_vertex_identity.get(&vertex_identity) {
                identities.extend(
                    half_edges
                        .iter()
                        .filter(|identity| identity.as_str() != source_identity)
                        .cloned(),
                );
            }
        }
        Ok(identities.into_iter().collect())
    }

    pub(crate) fn half_edge_identities_on_different_edge(
        &self,
        source_identity: &str,
    ) -> Result<Vec<String>, WorthTopologyQuerySnapshotIndexError> {
        let source_edge_identity = self.edge_identity_of_half_edge(source_identity)?;
        Ok(self
            .half_edge_edge_by_identity
            .iter()
            .filter(|(identity, edge_identity)| {
                identity.as_str() != source_identity
                    && edge_identity.as_str() != source_edge_identity
            })
            .map(|(identity, _)| identity.clone())
            .collect())
    }

    pub(crate) fn half_edge_identities_on_same_edge(
        &self,
        source_identity: &str,
    ) -> Result<Vec<String>, WorthTopologyQuerySnapshotIndexError> {
        let source_edge_identity = self.edge_identity_of_half_edge(source_identity)?;
        Ok(self
            .half_edge_edge_by_identity
            .iter()
            .filter(|(identity, edge_identity)| {
                identity.as_str() != source_identity
                    && edge_identity.as_str() == source_edge_identity
            })
            .map(|(identity, _)| identity.clone())
            .collect())
    }

    pub(crate) fn successor_cycle_identities(
        &self,
        start_identity: &str,
        count: usize,
    ) -> Result<Vec<String>, WorthTopologyQuerySnapshotIndexError> {
        let mut identities = Vec::with_capacity(count);
        let mut current_identity = start_identity.to_string();
        for _ in 0..count {
            identities.push(current_identity.clone());
            current_identity = self.outgoing_target_identity(
                &current_identity,
                WorthTopologyRelationKind::HalfEdgeNext,
            )?;
            let _ = self.find_entity_id_by_identity(&current_identity)?;
        }
        Ok(identities)
    }
}
