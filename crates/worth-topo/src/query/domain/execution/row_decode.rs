use std::collections::BTreeSet;

use forge_query::facade::{ForgeQueryEntity, RelationName};
use serde_json::Value;

use super::super::error::TopologyDomainQueryError;

pub(in crate::query::domain) fn row_payload<'a>(
    rows: &'a [ForgeQueryEntity],
    identity: &str,
    label: &str,
) -> Result<&'a Value, TopologyDomainQueryError> {
    rows.iter()
        .find(|row| row.identity == identity)
        .map(|row| &row.payload)
        .ok_or_else(|| {
            TopologyDomainQueryError::read_family_execution_denied(format!(
                "{label} rows did not retain anchor `{identity}`"
            ))
        })
}

pub(in crate::query::domain) fn relation_identity<'a>(
    payload: Option<&'a Value>,
    relation: &RelationName,
    label: &str,
) -> Result<&'a str, TopologyDomainQueryError> {
    payload
        .and_then(|payload| payload.get("relations"))
        .and_then(|relations| relations.get(relation.as_str()))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            TopologyDomainQueryError::read_family_execution_denied(format!(
                "{label} rows did not retain `{}` relation materialization",
                relation.as_str()
            ))
        })
}

pub(in crate::query::domain) fn relation_record_identity<'a>(
    payload: Option<&'a Value>,
    relation: &RelationName,
    label: &str,
) -> Result<&'a str, TopologyDomainQueryError> {
    payload
        .and_then(|payload| payload.get("relation_identities"))
        .and_then(|relations| relations.get(relation.as_str()))
        .and_then(Value::as_str)
        .ok_or_else(|| {
            TopologyDomainQueryError::read_family_execution_denied(format!(
                "{label} rows did not retain `{}` relation identity materialization",
                relation.as_str()
            ))
        })
}

pub(in crate::query::domain) fn relation_identities(
    payload: Option<&Value>,
    relations: &[RelationName],
    label: &str,
) -> Result<Vec<String>, TopologyDomainQueryError> {
    relations
        .iter()
        .map(|relation| relation_identity(payload, relation, label).map(str::to_string))
        .collect::<Result<BTreeSet<_>, _>>()
        .map(|identities| identities.into_iter().collect())
}
