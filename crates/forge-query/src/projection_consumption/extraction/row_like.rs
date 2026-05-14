use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::grouped_truth::RelationalAuthoritativeRowSetArtifact;
use forge_runtime_bridge::facade::BridgeMaterializedRowSetArtifact;

use super::super::consumed::{
    ConsumedEntityIdentityFact, ConsumedFieldValueFact, ConsumedProjectionFactSet,
    ConsumedViewLocalIdentityFact, ProjectionFactExtractionCounters,
};
use super::super::contracts::{BoundProjectionFactFamily, MaterializedProjectionContract};
use super::super::facts::ProjectionFactKind;
use super::super::source::ProjectionSourceFamily;
use crate::memory_workspace::ForgeQueryEntity;
use crate::projection_consumption::ProjectionFactExtractionError;
use crate::runtime::ForgeQueryReadResult;

#[derive(Clone, Copy)]
enum RowIdentityExtractionMode {
    RowIdentityAsEntityIdentity,
    IdentityFieldBackedEntityIdentity,
}

pub(super) fn extract_relational_row_set_facts(
    contract: &MaterializedProjectionContract,
    row_set: &RelationalAuthoritativeRowSetArtifact,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    super::ensure_contract_family(contract, ProjectionSourceFamily::RelationalRowSet)?;
    super::ensure_source_identity(contract.source_identity(), row_set.digest().as_str())?;
    extract_field_map_rows(
        contract,
        row_set.rows().iter().map(|row| {
            (
                row.row_identity().as_str(),
                row.fields()
                    .iter()
                    .map(|(key, value)| (key.as_str(), value.value())),
            )
        }),
        RowIdentityExtractionMode::IdentityFieldBackedEntityIdentity,
    )
}

pub(super) fn extract_bridge_row_set_facts(
    contract: &MaterializedProjectionContract,
    row_set: &BridgeMaterializedRowSetArtifact,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    super::ensure_contract_family(contract, ProjectionSourceFamily::BridgeTruthViewRowSet)?;
    super::ensure_source_identity(contract.source_identity(), row_set.digest().as_str())?;
    extract_field_map_rows(
        contract,
        row_set.rows().iter().map(|row| {
            (
                row.row_identity().as_str(),
                row.fields()
                    .iter()
                    .map(|(key, value)| (key.as_ref(), value.value())),
            )
        }),
        RowIdentityExtractionMode::IdentityFieldBackedEntityIdentity,
    )
}

pub(super) fn extract_read_result_facts(
    contract: &MaterializedProjectionContract,
    result: &ForgeQueryReadResult,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    super::ensure_contract_family(contract, ProjectionSourceFamily::QueryReadReceipt)?;
    super::ensure_source_identity(
        contract.source_identity(),
        result.receipt().read_graph_digest(),
    )?;
    extract_json_rows(
        contract,
        result.rows(),
        RowIdentityExtractionMode::RowIdentityAsEntityIdentity,
    )
}

fn extract_field_map_rows<'a, Rows, Fields>(
    contract: &MaterializedProjectionContract,
    rows: Rows,
    row_identity_mode: RowIdentityExtractionMode,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError>
where
    Rows: Iterator<Item = (&'a str, Fields)>,
    Fields: Iterator<Item = (&'a str, &'a serde_json::Value)>,
{
    let materialized_rows = rows
        .map(|(row_identity, fields)| {
            (
                row_identity,
                fields.collect::<BTreeMap<&str, &serde_json::Value>>(),
            )
        })
        .collect::<Vec<_>>();
    extract_materialized_rows(
        contract,
        &materialized_rows,
        |row_identity, field_map, field_key, fact_kind| {
            field_map.get(field_key).copied().ok_or_else(|| {
                ProjectionFactExtractionError::MissingDeclaredFieldEvidence {
                    source_family: contract.source_family(),
                    source_identity: format!("{}::{row_identity}", contract.source_identity()),
                    field_key: field_key.to_string(),
                    fact_kind,
                }
            })
        },
        row_identity_mode,
    )
}

fn extract_json_rows(
    contract: &MaterializedProjectionContract,
    rows: &[ForgeQueryEntity],
    row_identity_mode: RowIdentityExtractionMode,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    let materialized_rows = rows
        .iter()
        .map(|row| (row.identity.as_str(), &row.payload))
        .collect::<Vec<_>>();
    extract_materialized_rows(
        contract,
        &materialized_rows,
        |row_identity, payload, field_key, fact_kind| {
            json_path_value(payload, field_key).ok_or_else(|| {
                ProjectionFactExtractionError::MissingDeclaredFieldEvidence {
                    source_family: contract.source_family(),
                    source_identity: format!("{}::{row_identity}", contract.source_identity()),
                    field_key: field_key.to_string(),
                    fact_kind,
                }
            })
        },
        row_identity_mode,
    )
}

fn extract_materialized_rows<RowData, Lookup>(
    contract: &MaterializedProjectionContract,
    rows: &[(&str, RowData)],
    lookup: Lookup,
    row_identity_mode: RowIdentityExtractionMode,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError>
where
    Lookup: for<'a> Fn(
        &'a str,
        &'a RowData,
        &'a str,
        ProjectionFactKind,
    ) -> Result<&'a serde_json::Value, ProjectionFactExtractionError>,
{
    let requested_field_keys = contract
        .fact_families()
        .iter()
        .filter_map(|fact: &BoundProjectionFactFamily| match fact.kind() {
            ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => {
                fact.field_key().map(str::to_string)
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let extracts_entity_identity = contract
        .fact_families()
        .iter()
        .any(|fact: &BoundProjectionFactFamily| fact.kind() == ProjectionFactKind::EntityIdentity);
    let extracts_view_local_identity =
        contract
            .fact_families()
            .iter()
            .any(|fact: &BoundProjectionFactFamily| {
                fact.kind() == ProjectionFactKind::ViewLocalIdentity
            });
    let mut entity_identities = Vec::new();
    let mut view_local_identities = Vec::new();
    let mut display_fields = Vec::new();
    let mut derived_scalar_fields = Vec::new();

    for (row_identity, row_data) in rows {
        for fact_family in contract.fact_families() {
            match fact_family.kind() {
                ProjectionFactKind::EntityIdentity => {
                    let entity_identity = match row_identity_mode {
                        RowIdentityExtractionMode::RowIdentityAsEntityIdentity => {
                            (*row_identity).to_string()
                        }
                        RowIdentityExtractionMode::IdentityFieldBackedEntityIdentity => {
                            let value = lookup(
                                *row_identity,
                                row_data,
                                "identity.id",
                                ProjectionFactKind::EntityIdentity,
                            )?;
                            value.as_str().map(str::to_string).ok_or_else(|| {
                                ProjectionFactExtractionError::InvalidDeclaredFieldValueShape {
                                    source_family: contract.source_family(),
                                    source_identity: format!(
                                        "{}::{row_identity}",
                                        contract.source_identity()
                                    ),
                                    field_key: "identity.id".to_string(),
                                    fact_kind: ProjectionFactKind::EntityIdentity,
                                    expected_shape: "string",
                                }
                            })?
                        }
                    };
                    entity_identities.push(ConsumedEntityIdentityFact::new(
                        *row_identity,
                        entity_identity,
                    ));
                }
                ProjectionFactKind::ViewLocalIdentity => {
                    view_local_identities.push(ConsumedViewLocalIdentityFact::new(
                        *row_identity,
                        *row_identity,
                    ));
                }
                ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => {
                    let field_key = fact_family.field_key().expect("field key required");
                    let value = lookup(*row_identity, row_data, field_key, fact_family.kind())?;
                    let fact = ConsumedFieldValueFact::new(*row_identity, field_key, value.clone());
                    if fact_family.kind() == ProjectionFactKind::DisplayField {
                        display_fields.push(fact);
                    } else {
                        derived_scalar_fields.push(fact);
                    }
                }
                ProjectionFactKind::TargetIdentity
                | ProjectionFactKind::SourceReference
                | ProjectionFactKind::EffectContinuity
                | ProjectionFactKind::Membership
                | ProjectionFactKind::RelationEndpoint => {}
            }
        }
    }

    let row_identity_surface_count = match row_identity_mode {
        RowIdentityExtractionMode::RowIdentityAsEntityIdentity => {
            usize::from(extracts_entity_identity || extracts_view_local_identity)
        }
        RowIdentityExtractionMode::IdentityFieldBackedEntityIdentity => {
            usize::from(extracts_view_local_identity)
        }
    };
    let entity_identity_field_surface_count = match row_identity_mode {
        RowIdentityExtractionMode::RowIdentityAsEntityIdentity => 0,
        RowIdentityExtractionMode::IdentityFieldBackedEntityIdentity => {
            usize::from(extracts_entity_identity)
        }
    };
    let row_width_per_row = requested_field_keys.len()
        + row_identity_surface_count
        + entity_identity_field_surface_count;
    let source_row_width_consumed = rows.len() * row_width_per_row;
    let extracted_fact_count = entity_identities.len()
        + view_local_identities.len()
        + display_fields.len()
        + derived_scalar_fields.len();

    Ok(ConsumedProjectionFactSet::new(
        contract.declaration_digest(),
        contract.contract_digest(),
        contract.source_family(),
        contract.source_identity(),
        contract.support_posture().clone(),
        ProjectionFactExtractionCounters::new(
            contract.fact_families().len(),
            contract.fact_families().len(),
            extracted_fact_count,
            source_row_width_consumed,
            0,
        ),
        entity_identities,
        view_local_identities,
        Vec::new(),
        display_fields,
        derived_scalar_fields,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ))
}

fn json_path_value<'a>(
    payload: &'a serde_json::Value,
    field_key: &str,
) -> Option<&'a serde_json::Value> {
    let mut current = payload;
    for segment in field_key.split('.') {
        current = current.get(segment)?;
    }
    Some(current)
}
