use crate::identity::hash_parts;
use crate::projection_consumption::ConsumedProjectionFactSet;
use serde_json::Value;

use super::fixtures::{certification_grouped_projection, certification_row_set};
use super::oracle_value_terms::canonical_aspect_value;

pub(super) fn row_set_control_expected_digest(row_count: usize) -> String {
    let row_set = certification_row_set(row_count);
    hash_parts(
        &row_set
            .rows()
            .iter()
            .flat_map(|row| {
                let entity_identity = row
                    .aspect_values()
                    .iter()
                    .find_map(|(field, value)| {
                        (field.as_str() == "identity.id").then(|| canonical_aspect_value(value))
                    })
                    .expect("identity.id should exist");
                let display_name = row
                    .aspect_values()
                    .iter()
                    .find_map(|(field, value)| {
                        (field.as_str() == "profile.display_name")
                            .then(|| canonical_aspect_value(value))
                    })
                    .expect("display name should exist");
                [
                    format!(
                        "entity_identity:{}:{}",
                        row.row_identity().as_str(),
                        entity_identity
                    ),
                    format!(
                        "display_field:{}:profile.display_name:{}",
                        row.row_identity().as_str(),
                        display_name
                    ),
                ]
            })
            .collect::<Vec<_>>(),
    )
}

pub(super) fn row_set_control_actual_digest(facts: &ConsumedProjectionFactSet) -> String {
    hash_parts(
        &facts
            .entity_identities()
            .iter()
            .map(|fact| {
                format!(
                    "entity_identity:{}:{}",
                    fact.source_row_identity(),
                    fact.entity_identity()
                )
            })
            .chain(facts.display_fields().iter().map(|fact| {
                format!(
                    "display_field:{}:{}:{}",
                    fact.source_row_identity(),
                    fact.field_key(),
                    canonical_json(fact.value())
                )
            }))
            .collect::<Vec<_>>(),
    )
}

pub(super) fn grouped_worth_expected_digest(row_count: usize) -> String {
    let grouped = certification_grouped_projection(row_count);
    hash_parts(
        &grouped
            .members()
            .iter()
            .flat_map(|member| {
                [
                    format!(
                        "membership:{}:{}:{}",
                        member.row_identity().as_str(),
                        grouped.contract().grouping_aspect().as_str(),
                        canonical_aspect_value(member.grouping_value())
                    ),
                    format!(
                        "relation_endpoint:{}:{}:{}",
                        member.row_identity().as_str(),
                        grouped.contract().grouping_aspect().as_str(),
                        canonical_aspect_value(member.grouping_value())
                    ),
                    format!(
                        "view_local_identity:{}:{}",
                        member.row_identity().as_str(),
                        member.row_identity().as_str()
                    ),
                ]
            })
            .collect::<Vec<_>>(),
    )
}

pub(super) fn grouped_worth_actual_digest(facts: &ConsumedProjectionFactSet) -> String {
    hash_parts(
        &facts
            .memberships()
            .iter()
            .map(|fact| {
                format!(
                    "membership:{}:{}:{}",
                    fact.source_row_identity(),
                    fact.grouping_aspect(),
                    canonical_json(fact.grouping_value())
                )
            })
            .chain(facts.relation_endpoints().iter().map(|fact| {
                format!(
                    "relation_endpoint:{}:{}:{}",
                    fact.source_row_identity().unwrap_or("none"),
                    fact.grouping_aspect().unwrap_or("none"),
                    canonical_json(fact.grouping_value().unwrap_or(&Value::Null))
                )
            }))
            .chain(facts.view_local_identities().iter().map(|fact| {
                format!(
                    "view_local_identity:{}:{}",
                    fact.source_row_identity(),
                    fact.view_local_identity()
                )
            }))
            .collect::<Vec<_>>(),
    )
}

fn canonical_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "<invalid-json>".to_string())
}
