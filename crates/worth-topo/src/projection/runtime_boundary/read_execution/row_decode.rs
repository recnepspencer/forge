use std::collections::BTreeSet;

use forge_query::facade::{ForgeQueryEntity, RelationName};
use serde_json::Value;

use crate::projection::read_views::domain::error::TopologyDomainQueryError;

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
    ) -> Result<RetainedTopologyRow<'a>, TopologyDomainQueryError> {
        self.rows
            .iter()
            .find(|row| row.identity == identity)
            .map(|row| RetainedTopologyRow { row })
            .ok_or_else(|| {
                TopologyDomainQueryError::read_family_execution_denied(format!(
                    "{label} rows did not retain anchor `{identity}`"
                ))
            })
    }

    pub(crate) fn identities(&self) -> impl Iterator<Item = &'a str> {
        self.rows.iter().map(|row| row.identity.as_str())
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
    ) -> Result<&'a str, TopologyDomainQueryError> {
        relation_target_identity_from_payload(
            self.row.payload.get("relations"),
            relation,
            label,
            "relation materialization",
        )
    }

    pub(crate) fn relation_record_identity(
        &self,
        relation: &RelationName,
        label: &str,
    ) -> Result<&'a str, TopologyDomainQueryError> {
        relation_target_identity_from_payload(
            self.row.payload.get("relation_identities"),
            relation,
            label,
            "relation identity materialization",
        )
    }

    pub(crate) fn relation_target_identities(
        &self,
        relations: &[RelationName],
        label: &str,
    ) -> Result<Vec<String>, TopologyDomainQueryError> {
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

fn relation_target_identity_from_payload<'a>(
    payload: Option<&'a Value>,
    relation: &RelationName,
    label: &str,
    materialization_label: &str,
) -> Result<&'a str, TopologyDomainQueryError> {
    payload
        .and_then(|relations| relations.get(relation.as_str()))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            TopologyDomainQueryError::read_family_execution_denied(format!(
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
) -> Result<Vec<String>, TopologyDomainQueryError> {
    rows.identities()
        .filter(|identity| *identity != source_identity)
        .try_fold(BTreeSet::new(), |mut identities, identity| {
            let row = rows.row(identity, label)?;
            let row_target_identities = row.relation_target_identities(relations, label)?;
            if row_target_identities
                .iter()
                .any(|target| source_target_identities.contains(target))
            {
                identities.insert(identity.to_string());
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
) -> Result<Vec<String>, TopologyDomainQueryError> {
    rows.identities()
        .filter(|identity| *identity != source_identity)
        .try_fold(BTreeSet::new(), |mut identities, identity| {
            let row = rows.row(identity, label)?;
            let edge_identity = row.relation_target_identity(edge_relation, label)?;
            if (edge_identity == source_edge_identity) == same_edge {
                identities.insert(identity.to_string());
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
) -> Result<Vec<String>, TopologyDomainQueryError> {
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
) -> Result<Vec<String>, TopologyDomainQueryError> {
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
) -> Result<String, TopologyDomainQueryError> {
    rows.row(identity, label)?
        .relation_target_identity(edge_relation, label)
        .map(str::to_string)
}
