use std::collections::BTreeSet;

use forge_query::facade::{
    forge_query_materialized_relation_field_key, ForgeQueryEntity, ForgeQueryEntityIdentity,
    RelationName,
};
use forge_runtime_bridge::facade::RelationalBridgeRecordIdentityKind;

use crate::projection::read_views::domain::error::TopologyReadError;
use crate::query_native_runtime_boundary::row_text_at;

#[derive(Debug, Clone, Copy)]
pub(crate) struct RetainedTopologyRows<'a> {
    rows: &'a [ForgeQueryEntity],
}

impl<'a> RetainedTopologyRows<'a> {
    pub(crate) fn new(rows: &'a [ForgeQueryEntity]) -> Self {
        Self { rows }
    }

    pub(crate) fn row(
        &self,
        identity: &str,
        label: &str,
    ) -> Result<RetainedTopologyRow<'a>, TopologyReadError> {
        self.rows
            .iter()
            .find(|row| retained_row_identity_label(row).as_deref() == Some(identity))
            .map(|row| RetainedTopologyRow { row })
            .ok_or_else(|| {
                let inventory = self
                    .identities()
                    .collect::<Vec<_>>()
                    .join(", ");
                TopologyReadError::read_family_execution_denied(format!(
                    "{label} rows did not retain anchor `{identity}`; retained identities: [{inventory}]"
                ))
            })
    }

    pub(crate) fn identities(&self) -> impl Iterator<Item = String> + 'a {
        self.rows
            .iter()
            .filter_map(|row| retained_row_identity_label(row))
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RetainedTopologyRow<'a> {
    row: &'a ForgeQueryEntity,
}

impl<'a> RetainedTopologyRow<'a> {
    pub(crate) fn relation_target_identity(
        &self,
        relation: &RelationName,
        label: &str,
    ) -> Result<&'a str, TopologyReadError> {
        relation_target_identity_from_native_fields(
            self.row,
            relation,
            label,
            "relation materialization",
            "relations",
        )
    }

    pub(crate) fn relation_record_identity(
        &self,
        relation: &RelationName,
        label: &str,
    ) -> Result<&'a str, TopologyReadError> {
        relation_target_identity_from_native_fields(
            self.row,
            relation,
            label,
            "relation identity materialization",
            "relation_identities",
        )
    }

    pub(crate) fn relation_target_identities(
        &self,
        relations: &[RelationName],
        label: &str,
    ) -> Result<Vec<String>, TopologyReadError> {
        relations
            .iter()
            .map(|relation| {
                self.relation_target_identity(relation, label)
                    .map(str::to_string)
            })
            .collect::<Result<BTreeSet<_>, _>>()
            .map(|identities| identities.into_iter().collect())
    }
}

fn relation_target_identity_from_native_fields<'a>(
    row: &'a ForgeQueryEntity,
    relation: &RelationName,
    label: &str,
    materialization_label: &str,
    field_root: &str,
) -> Result<&'a str, TopologyReadError> {
    let relation_slot = forge_query_materialized_relation_field_key(relation);
    row_text_at(row, [field_root, relation_slot.as_str()]).ok_or_else(|| {
        TopologyReadError::read_family_execution_denied(format!(
            "{label} rows did not retain `{}` {materialization_label}",
            relation.as_str()
        ))
    })
}

pub(crate) fn adjacent_row_identities_sharing_targets(
    rows: &RetainedTopologyRows<'_>,
    source_identity: &str,
    source_target_identities: &[String],
    relations: &[RelationName],
    label: &str,
) -> Result<Vec<String>, TopologyReadError> {
    rows.identities()
        .filter(|identity| identity != source_identity)
        .try_fold(BTreeSet::new(), |mut identities, identity| {
            let row = rows.row(&identity, label)?;
            let row_target_identities = row.relation_target_identities(relations, label)?;
            if row_target_identities
                .iter()
                .any(|target| source_target_identities.contains(target))
            {
                identities.insert(identity);
            }
            Ok(identities)
        })
        .map(|identities| identities.into_iter().collect())
}

pub(crate) fn filter_row_identities_by_edge_match(
    rows: &RetainedTopologyRows<'_>,
    source_identity: &str,
    source_edge_identity: &str,
    edge_relation: &RelationName,
    same_edge: bool,
    label: &str,
) -> Result<Vec<String>, TopologyReadError> {
    rows.identities()
        .filter(|identity| identity != source_identity)
        .try_fold(BTreeSet::new(), |mut identities, identity| {
            let row = rows.row(&identity, label)?;
            let edge_identity = row.relation_target_identity(edge_relation, label)?;
            if (edge_identity == source_edge_identity) == same_edge {
                identities.insert(identity);
            }
            Ok(identities)
        })
        .map(|identities| identities.into_iter().collect())
}

pub(crate) fn filter_identities_by_edge_mismatch(
    rows: &RetainedTopologyRows<'_>,
    candidate_identities: &[String],
    source_edge_identity: &str,
    edge_relation: &RelationName,
    label: &str,
) -> Result<Vec<String>, TopologyReadError> {
    candidate_identities
        .iter()
        .try_fold(BTreeSet::new(), |mut identities, identity| {
            let row = rows.row(identity, label)?;
            let edge_identity = row.relation_target_identity(edge_relation, label)?;
            if edge_identity != source_edge_identity {
                identities.insert(identity.clone());
            }
            Ok(identities)
        })
        .map(|identities| identities.into_iter().collect())
}

pub(crate) fn cycle_identities_from_successors(
    rows: &RetainedTopologyRows<'_>,
    start_identity: &str,
    count: usize,
    successor_relation: &RelationName,
    label: &str,
) -> Result<Vec<String>, TopologyReadError> {
    let mut cycle = Vec::with_capacity(count);
    let mut current = start_identity;
    for _ in 0..count {
        cycle.push(current.to_string());
        current = rows
            .row(current, label)?
            .relation_target_identity(successor_relation, label)?;
    }
    Ok(cycle)
}

pub(crate) fn edge_identity_by_row(
    rows: &RetainedTopologyRows<'_>,
    identity: &str,
    edge_relation: &RelationName,
    label: &str,
) -> Result<String, TopologyReadError> {
    rows.row(identity, label)?
        .relation_target_identity(edge_relation, label)
        .map(str::to_string)
}

fn retained_row_identity_label(row: &ForgeQueryEntity) -> Option<String> {
    row_projection_identity_label(row).or_else(|| query_identity_label(row.identity()))
}

fn row_projection_identity_label(row: &ForgeQueryEntity) -> Option<String> {
    row_text_at(row, ["identity", "id"])
        .or_else(|| row_text_at(row, ["id"]))
        .map(str::to_string)
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
