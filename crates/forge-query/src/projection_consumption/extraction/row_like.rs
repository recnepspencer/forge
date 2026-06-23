use std::collections::BTreeSet;

use forge_foundational::facade::{AspectValue, ContractValidatedAspectValueView};
use forge_relational::facade::grouped_truth::RelationalAuthoritativeRowSetArtifact;
use forge_runtime_bridge::facade::BridgeMaterializedRowSetArtifact;

use super::super::consumed::{
    ConsumedEntityIdentityFact, ConsumedFieldValueFact, ConsumedProjectionFactSet,
    ConsumedViewLocalIdentityFact, ProjectionFactExtractionCounters,
};
use super::super::contracts::{BoundProjectionFactFamily, MaterializedProjectionContract};
use super::super::facts::{ProjectionFactFieldPath, ProjectionFactKind};
use super::super::identity::compose_scoped_row_source_identity;
use super::super::source::ProjectionSourceFamily;
use super::row_like_field_paths::{
    identity_field_path, lower_materialized_fields, query_read_result_row_fields,
    ProjectionMaterializedField,
};
use super::row_like_values::consumed_aspect_value_as_str;
use crate::memory_workspace::{ForgeQueryEntity, ForgeQueryEntityIdentity};
use crate::projection_consumption::ProjectionFactExtractionError;
use crate::runtime::{ForgeQueryLiveReadResult, ForgeQueryReadResult};

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
    let materialized_rows = row_set
        .rows()
        .iter()
        .map(|row| {
            Ok((
                row.row_identity().as_str(),
                row.projected_aspect_values()
                    .iter()
                    .map(|(key, value)| {
                        ProjectionMaterializedField::from_relational_projected_aspect_key(
                            contract,
                            row.row_identity().as_str(),
                            key,
                            value.clone(),
                        )
                    })
                    .collect::<Result<Vec<_>, ProjectionFactExtractionError>>()?,
            ))
        })
        .collect::<Result<Vec<_>, ProjectionFactExtractionError>>()?;
    extract_field_map_rows(
        contract,
        materialized_rows
            .iter()
            .map(|(row_identity, fields)| (*row_identity, fields.iter().cloned())),
        RowIdentityExtractionMode::IdentityFieldBackedEntityIdentity,
    )
}

pub(super) fn extract_bridge_row_set_facts(
    contract: &MaterializedProjectionContract,
    row_set: &BridgeMaterializedRowSetArtifact,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    super::ensure_contract_family(contract, ProjectionSourceFamily::BridgeTruthViewRowSet)?;
    super::ensure_source_identity(contract.source_identity(), row_set.digest().as_str())?;
    let materialized_rows = row_set
        .rows()
        .iter()
        .map(|row| {
            Ok((
                row.row_identity().as_str(),
                row.fields()
                    .iter()
                    .map(|(_key, bridge_field)| {
                        let value = match bridge_field.validated_value().payload().view() {
                            ContractValidatedAspectValueView::Scalar(value) => value.clone(),
                            ContractValidatedAspectValueView::Struct(_) => {
                                return Err(
                                    ProjectionFactExtractionError::InvalidDeclaredFieldValueShape {
                                        source_family: contract.source_family(),
                                        source_identity: compose_scoped_row_source_identity(
                                            contract.source_identity(),
                                            row.row_identity().as_str(),
                                        ),
                                        field_key: bridge_field
                                            .projection()
                                            .field_identity()
                                            .as_str()
                                            .to_string(),
                                        fact_kind: ProjectionFactKind::DerivedScalarField,
                                        expected_shape: "foundational scalar",
                                    },
                                );
                            }
                        };
                        ProjectionMaterializedField::from_bridge_field_value(
                            contract,
                            row.row_identity().as_str(),
                            bridge_field.projection(),
                            value,
                        )
                    })
                    .collect::<Result<Vec<_>, ProjectionFactExtractionError>>()?,
            ))
        })
        .collect::<Result<Vec<_>, ProjectionFactExtractionError>>()?;
    extract_field_map_rows(
        contract,
        materialized_rows
            .iter()
            .map(|(row_identity, fields)| (*row_identity, fields.iter().cloned())),
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
    extract_entity_rows(
        contract,
        result.rows(),
        RowIdentityExtractionMode::RowIdentityAsEntityIdentity,
    )
}

pub(super) fn extract_live_read_result_facts(
    contract: &MaterializedProjectionContract,
    result: &ForgeQueryLiveReadResult,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    super::ensure_contract_family(contract, ProjectionSourceFamily::QueryLiveReadReceipt)?;
    super::ensure_source_identity(
        contract.source_identity(),
        result.receipt().installation_digest(),
    )?;
    extract_entity_rows(
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
    Fields: Iterator<Item = ProjectionMaterializedField>,
{
    let materialized_rows = rows
        .map(|(row_identity, fields)| {
            Ok((
                row_identity.to_string(),
                None,
                lower_materialized_fields(fields)?,
            ))
        })
        .collect::<Result<Vec<_>, ProjectionFactExtractionError>>()?;
    extract_materialized_rows(
        contract,
        &materialized_rows,
        |row_identity, field_map, field_key, fact_kind| {
            field_map.get(field_key).ok_or_else(|| {
                ProjectionFactExtractionError::MissingDeclaredFieldEvidence {
                    source_family: contract.source_family(),
                    source_identity: compose_scoped_row_source_identity(
                        contract.source_identity(),
                        &row_identity,
                    ),
                    field_key: field_key.terminal_projection_for_boundary().to_string(),
                    fact_kind,
                }
            })
        },
        row_identity_mode,
    )
}

fn extract_entity_rows(
    contract: &MaterializedProjectionContract,
    rows: &[ForgeQueryEntity],
    row_identity_mode: RowIdentityExtractionMode,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    let materialized_rows = rows
        .iter()
        .map(|row| {
            Ok((
                row.identity()
                    .evidence_identity()
                    .reporting_projection()
                    .to_string(),
                Some(row.identity().clone()),
                query_read_result_row_fields(contract, row)?,
            ))
        })
        .collect::<Result<Vec<_>, ProjectionFactExtractionError>>()?;
    extract_materialized_rows(
        contract,
        &materialized_rows,
        |row_identity, row_fields, field_key, fact_kind| {
            row_fields.get(field_key).ok_or_else(|| {
                ProjectionFactExtractionError::MissingDeclaredFieldEvidence {
                    source_family: contract.source_family(),
                    source_identity: compose_scoped_row_source_identity(
                        contract.source_identity(),
                        &row_identity,
                    ),
                    field_key: field_key.terminal_projection_for_boundary().to_string(),
                    fact_kind,
                }
            })
        },
        row_identity_mode,
    )
}

fn extract_materialized_rows<RowData, Lookup>(
    contract: &MaterializedProjectionContract,
    rows: &[(String, Option<ForgeQueryEntityIdentity>, RowData)],
    lookup: Lookup,
    row_identity_mode: RowIdentityExtractionMode,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError>
where
    Lookup: for<'a> Fn(
        &'a str,
        &'a RowData,
        &'a ProjectionFactFieldPath,
        ProjectionFactKind,
    ) -> Result<&'a AspectValue, ProjectionFactExtractionError>,
{
    let requested_field_keys = contract
        .fact_families()
        .iter()
        .filter_map(|fact: &BoundProjectionFactFamily| match fact.kind() {
            ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => fact
                .field_path()
                .map(|field_path| field_path.terminal_projection_for_boundary().to_string()),
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

    for (row_identity, typed_entity_identity, row_data) in rows {
        for fact_family in contract.fact_families() {
            match fact_family.kind() {
                ProjectionFactKind::EntityIdentity => {
                    let entity_identity = match row_identity_mode {
                        RowIdentityExtractionMode::RowIdentityAsEntityIdentity => {
                            typed_entity_identity.clone().unwrap_or_else(|| {
                                crate::memory_workspace::admit_authored_entity_label(row_identity)
                            })
                        }
                        RowIdentityExtractionMode::IdentityFieldBackedEntityIdentity => {
                            let identity_field_path = identity_field_path();
                            let value = lookup(
                                row_identity.as_str(),
                                row_data,
                                &identity_field_path,
                                ProjectionFactKind::EntityIdentity,
                            )?;
                            crate::memory_workspace::admit_authored_entity_label(
                                consumed_aspect_value_as_str(value).ok_or_else(|| {
                                    ProjectionFactExtractionError::InvalidDeclaredFieldValueShape {
                                        source_family: contract.source_family(),
                                        source_identity: compose_scoped_row_source_identity(
                                            contract.source_identity(),
                                            row_identity.as_str(),
                                        ),
                                        field_key: "identity.id".to_string(),
                                        fact_kind: ProjectionFactKind::EntityIdentity,
                                        expected_shape: "string",
                                    }
                                })?,
                            )
                        }
                    };
                    entity_identities.push(ConsumedEntityIdentityFact::new(
                        row_identity.as_str(),
                        entity_identity,
                    ));
                }
                ProjectionFactKind::ViewLocalIdentity => {
                    view_local_identities.push(ConsumedViewLocalIdentityFact::new(
                        row_identity.as_str(),
                        row_identity.as_str(),
                    ));
                }
                ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => {
                    let field_path = fact_family
                        .field_path()
                        .expect("field path required")
                        .clone();
                    let value = lookup(
                        row_identity.as_str(),
                        row_data,
                        &field_path,
                        fact_family.kind(),
                    )?;
                    let fact = ConsumedFieldValueFact::new(
                        row_identity.as_str(),
                        field_path.clone(),
                        value.clone(),
                    );
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
        contract.source_identity_handle().clone(),
        contract.support_posture().clone(),
        contract.materialized_fact_posture().cloned(),
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
