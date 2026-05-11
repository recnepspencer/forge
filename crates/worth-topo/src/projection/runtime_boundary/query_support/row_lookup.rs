use forge_query::facade::ForgeQueryEntity;
use forge_relational::facade::identity::{EntityId, RelationId};
use schema::facade::TopologyRelationKind;

use super::query_rows::{
    query_entity_id_from_row, query_relation_id_from_row, relation_kind_name,
    topology_source_identity, topology_target_identity, TopologyQueryRowError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyQueryRowLookupError {
    detail: String,
}

impl TopologyQueryRowLookupError {
    fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}

impl From<TopologyQueryRowError> for TopologyQueryRowLookupError {
    fn from(error: TopologyQueryRowError) -> Self {
        Self::new(error.to_string())
    }
}

impl std::fmt::Display for TopologyQueryRowLookupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.detail)
    }
}

impl std::error::Error for TopologyQueryRowLookupError {}

pub(crate) struct TopologyQueryRowLookup<'a> {
    entity_rows: &'a [ForgeQueryEntity],
    relation_rows: &'a [ForgeQueryEntity],
}

impl<'a> TopologyQueryRowLookup<'a> {
    pub(crate) fn new(
        entity_rows: &'a [ForgeQueryEntity],
        relation_rows: &'a [ForgeQueryEntity],
    ) -> Self {
        Self {
            entity_rows,
            relation_rows,
        }
    }

    pub(crate) fn first_source_identity_for_relation_kind(
        &self,
        relation_kind: TopologyRelationKind,
    ) -> Result<String, TopologyQueryRowLookupError> {
        self.relation_rows
            .iter()
            .find_map(|row| {
                (relation_kind_name(row) == relation_kind.kind_name())
                    .then(|| topology_source_identity(row).map(str::to_string))
                    .flatten()
            })
            .ok_or_else(|| {
                TopologyQueryRowLookupError::new(format!(
                    "query rows should expose `{}` source identities",
                    relation_kind.kind_name()
                ))
            })
    }

    #[cfg(test)]
    pub(crate) fn find_entity_id_by_identity(
        &self,
        identity: &str,
    ) -> Result<EntityId, TopologyQueryRowLookupError> {
        self.entity_rows
            .iter()
            .find(|row| row.identity == identity)
            .ok_or_else(|| {
                TopologyQueryRowLookupError::new(format!(
                    "query identity `{identity}` should resolve to one entity"
                ))
            })
            .and_then(|row| query_entity_id_from_row(row).map_err(Into::into))
    }

    pub(crate) fn find_entity_identity_by_id(
        &self,
        entity_id: EntityId,
    ) -> Result<String, TopologyQueryRowLookupError> {
        self.entity_rows
            .iter()
            .find_map(|row| {
                query_entity_id_from_row(row)
                    .ok()
                    .filter(|candidate| *candidate == entity_id)
                    .map(|_| row.identity.clone())
            })
            .ok_or_else(|| {
                TopologyQueryRowLookupError::new(format!(
                    "entity id `{:?}` should resolve to one query identity",
                    entity_id
                ))
            })
    }

    #[cfg(test)]
    pub(crate) fn outgoing_target_identity(
        &self,
        source_identity: &str,
        relation_kind: TopologyRelationKind,
    ) -> Result<String, TopologyQueryRowLookupError> {
        self.relation_rows
            .iter()
            .find_map(|row| {
                (relation_kind_name(row) == relation_kind.kind_name()
                    && topology_source_identity(row) == Some(source_identity))
                .then(|| topology_target_identity(row).map(str::to_string))
                .flatten()
            })
            .ok_or_else(|| {
                TopologyQueryRowLookupError::new(format!(
                    "query rows should expose target identity for `{}` from `{source_identity}`",
                    relation_kind.kind_name()
                ))
            })
    }

    #[cfg(test)]
    pub(crate) fn incoming_source_identity(
        &self,
        target_identity: &str,
        relation_kind: TopologyRelationKind,
    ) -> Result<String, TopologyQueryRowLookupError> {
        self.relation_rows
            .iter()
            .find_map(|row| {
                (relation_kind_name(row) == relation_kind.kind_name()
                    && topology_target_identity(row) == Some(target_identity))
                .then(|| topology_source_identity(row).map(str::to_string))
                .flatten()
            })
            .ok_or_else(|| {
                TopologyQueryRowLookupError::new(format!(
                    "query rows should expose source identity for incoming `{}` to `{target_identity}`",
                    relation_kind.kind_name()
                ))
            })
    }

    #[cfg(test)]
    pub(crate) fn relation_id_for_source_kind(
        &self,
        source_identity: &str,
        relation_kind: TopologyRelationKind,
    ) -> Result<RelationId, TopologyQueryRowLookupError> {
        self.relation_rows
            .iter()
            .find(|row| {
                relation_kind_name(row) == relation_kind.kind_name()
                    && topology_source_identity(row) == Some(source_identity)
            })
            .ok_or_else(|| {
                TopologyQueryRowLookupError::new(format!(
                    "query rows should expose `{}` relation from `{source_identity}`",
                    relation_kind.kind_name()
                ))
            })
            .and_then(|row| query_relation_id_from_row(row).map_err(Into::into))
    }

    #[cfg(test)]
    pub(crate) fn edge_identity_of_half_edge(
        &self,
        half_edge_identity: &str,
    ) -> Result<String, TopologyQueryRowLookupError> {
        self.outgoing_target_identity(half_edge_identity, TopologyRelationKind::HalfEdgeUsesEdge)
    }

    #[cfg(test)]
    pub(crate) fn half_edge_vertex_identities(
        &self,
        half_edge_identity: &str,
    ) -> Result<Vec<String>, TopologyQueryRowLookupError> {
        Ok(vec![
            self.outgoing_target_identity(
                half_edge_identity,
                TopologyRelationKind::HalfEdgeStartsAtVertex,
            )?,
            self.outgoing_target_identity(
                half_edge_identity,
                TopologyRelationKind::HalfEdgeEndsAtVertex,
            )?,
        ])
    }
}
