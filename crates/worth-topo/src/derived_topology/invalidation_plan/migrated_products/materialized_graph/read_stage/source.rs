use forge_relational::facade::identity::EntityId;
use serde::Serialize;

use super::relations::relation_rows_from_topology;
use crate::brep::topology_graph::TopologyView;
use crate::derived_topology::invalidation_plan::migrated_products::materialized_graph::MaterializedGraphMigrationError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphReadEntityRow {
    entity_id: EntityId,
    topology_kind: &'static str,
    row_digest: String,
}

impl MaterializedGraphReadEntityRow {
    fn new(entity_id: EntityId, topology_kind: &'static str) -> Self {
        let row_digest = super::super::super::super::catalog::catalog_digest([
            "worth-topo:materialized-graph-read-entity-row:v1".to_string(),
            format!("entity:{entity_id:?}"),
            format!("kind:{topology_kind}"),
        ]);
        Self {
            entity_id,
            topology_kind,
            row_digest,
        }
    }

    pub const fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub const fn topology_kind(&self) -> &'static str {
        self.topology_kind
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphReadRelationRow {
    relation_kind: &'static str,
    source_entity_id: EntityId,
    target_entity_id: EntityId,
    row_digest: String,
}

impl MaterializedGraphReadRelationRow {
    pub(super) fn new(
        relation_kind: &'static str,
        source_entity_id: EntityId,
        target_entity_id: EntityId,
    ) -> Self {
        let row_digest = super::super::super::super::catalog::catalog_digest([
            "worth-topo:materialized-graph-read-relation-row:v1".to_string(),
            format!("kind:{relation_kind}"),
            format!("source:{source_entity_id:?}"),
            format!("target:{target_entity_id:?}"),
        ]);
        Self {
            relation_kind,
            source_entity_id,
            target_entity_id,
            row_digest,
        }
    }

    pub const fn relation_kind(&self) -> &'static str {
        self.relation_kind
    }

    pub const fn source_entity_id(&self) -> EntityId {
        self.source_entity_id
    }

    pub const fn target_entity_id(&self) -> EntityId {
        self.target_entity_id
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphReadSource {
    selected_entities: Vec<MaterializedGraphReadEntityRow>,
    selected_relations: Vec<MaterializedGraphReadRelationRow>,
    available_entity_count: usize,
    available_relation_count: usize,
    source_digest: String,
}

impl MaterializedGraphReadSource {
    pub(crate) fn from_topology_view_with_selected_prefix(
        topology: &TopologyView,
        selected_entity_count: usize,
        selected_relation_count: usize,
    ) -> Result<Self, MaterializedGraphMigrationError> {
        let entity_rows = entity_rows_from_topology(topology);
        let relation_rows = relation_rows_from_topology(topology);
        let available_entity_count = entity_rows.len();
        let available_relation_count = relation_rows.len();
        if selected_entity_count > entity_rows.len()
            || selected_relation_count > relation_rows.len()
        {
            return Err(MaterializedGraphMigrationError::ReadStageSelectedRowsExceedAvailableRows);
        }
        Self::from_rows(
            entity_rows
                .into_iter()
                .take(selected_entity_count)
                .collect(),
            relation_rows
                .into_iter()
                .take(selected_relation_count)
                .collect(),
            available_entity_count,
            available_relation_count,
        )
    }

    pub(crate) fn from_rows(
        selected_entities: Vec<MaterializedGraphReadEntityRow>,
        selected_relations: Vec<MaterializedGraphReadRelationRow>,
        available_entity_count: usize,
        available_relation_count: usize,
    ) -> Result<Self, MaterializedGraphMigrationError> {
        if selected_entities.len() > available_entity_count
            || selected_relations.len() > available_relation_count
        {
            return Err(MaterializedGraphMigrationError::ReadStageSelectedRowsExceedAvailableRows);
        }
        let source_digest = read_source_digest(
            &selected_entities,
            &selected_relations,
            available_entity_count,
            available_relation_count,
        );
        Ok(Self {
            selected_entities,
            selected_relations,
            available_entity_count,
            available_relation_count,
            source_digest,
        })
    }

    pub fn selected_entities(&self) -> &[MaterializedGraphReadEntityRow] {
        &self.selected_entities
    }

    pub fn selected_relations(&self) -> &[MaterializedGraphReadRelationRow] {
        &self.selected_relations
    }

    pub const fn available_entity_count(&self) -> usize {
        self.available_entity_count
    }

    pub const fn available_relation_count(&self) -> usize {
        self.available_relation_count
    }

    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }
}

fn entity_rows_from_topology(topology: &TopologyView) -> Vec<MaterializedGraphReadEntityRow> {
    let mut rows = Vec::new();
    rows.extend(
        topology
            .models
            .iter()
            .map(|row| MaterializedGraphReadEntityRow::new(row.entity_id, "model")),
    );
    rows.extend(
        topology
            .bodies
            .iter()
            .map(|row| MaterializedGraphReadEntityRow::new(row.entity_id, "body")),
    );
    rows.extend(
        topology
            .lumps
            .iter()
            .map(|row| MaterializedGraphReadEntityRow::new(row.entity_id, "lump")),
    );
    rows.extend(
        topology
            .regions
            .iter()
            .map(|row| MaterializedGraphReadEntityRow::new(row.entity_id, "region")),
    );
    rows.extend(
        topology
            .shells
            .iter()
            .map(|row| MaterializedGraphReadEntityRow::new(row.entity_id, "shell")),
    );
    rows.extend(
        topology
            .faces
            .iter()
            .map(|row| MaterializedGraphReadEntityRow::new(row.entity_id, "face")),
    );
    rows.extend(
        topology
            .loops
            .iter()
            .map(|row| MaterializedGraphReadEntityRow::new(row.entity_id, "loop")),
    );
    rows.extend(
        topology
            .wires
            .iter()
            .map(|row| MaterializedGraphReadEntityRow::new(row.entity_id, "wire")),
    );
    rows.extend(
        topology
            .half_edges
            .iter()
            .map(|row| MaterializedGraphReadEntityRow::new(row.entity_id, "half_edge")),
    );
    rows.extend(
        topology
            .edges
            .iter()
            .map(|row| MaterializedGraphReadEntityRow::new(row.entity_id, "edge")),
    );
    rows.extend(
        topology
            .vertices
            .iter()
            .map(|row| MaterializedGraphReadEntityRow::new(row.entity_id, "vertex")),
    );
    rows
}

fn read_source_digest(
    selected_entities: &[MaterializedGraphReadEntityRow],
    selected_relations: &[MaterializedGraphReadRelationRow],
    available_entity_count: usize,
    available_relation_count: usize,
) -> String {
    let mut parts = vec![
        "worth-topo:materialized-graph-read-source:v1".to_string(),
        format!("selected-entities:{}", selected_entities.len()),
        format!("selected-relations:{}", selected_relations.len()),
        format!("available-entities:{available_entity_count}"),
        format!("available-relations:{available_relation_count}"),
    ];
    parts.extend(
        selected_entities
            .iter()
            .map(|row| format!("entity:{}", row.row_digest())),
    );
    parts.extend(
        selected_relations
            .iter()
            .map(|row| format!("relation:{}", row.row_digest())),
    );
    super::super::super::super::catalog::catalog_digest(parts)
}
