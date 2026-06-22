use std::collections::{BTreeMap, BTreeSet};

use forge_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};

use super::super::contracts::MaterializedProjectionContract;
use super::super::facts::{
    projection_fact_field_path_from_segments, ProjectionFactFieldPath, ProjectionFactKind,
};
use super::super::identity::compose_scoped_row_source_identity;
use super::consumed_scalar_value::consumed_scalar_value_from_entity_path;
use super::ProjectionFactExtractionError;
use crate::memory_workspace::ForgeQueryEntity;

pub(super) fn lower_materialized_fields<'a, Fields>(
    contract: &MaterializedProjectionContract,
    row_identity: &str,
    fields: Fields,
) -> Result<BTreeMap<ProjectionFactFieldPath, AspectValue>, ProjectionFactExtractionError>
where
    Fields: Iterator<Item = (&'a str, AspectValue)>,
{
    fields
        .map(|(field_key, value)| {
            projection_fact_field_path_from_external_boundary(field_key)
                .map(|field_path| (field_path, value))
                .map_err(
                    |_message| ProjectionFactExtractionError::InvalidDeclaredFieldValueShape {
                        source_family: contract.source_family(),
                        source_identity: compose_scoped_row_source_identity(
                            contract.source_identity(),
                            row_identity,
                        ),
                        field_key: field_key.to_string(),
                        fact_kind: ProjectionFactKind::DerivedScalarField,
                        expected_shape: "projection fact field path",
                    },
                )
        })
        .collect()
}

pub(super) fn query_read_result_row_fields(
    contract: &MaterializedProjectionContract,
    row: &ForgeQueryEntity,
) -> Result<BTreeMap<ProjectionFactFieldPath, AspectValue>, ProjectionFactExtractionError> {
    let mut row_fields = row
        .aspect_values()
        .map(|(aspect_key, value)| {
            let aspect_path = aspect_key.as_str();
            projection_fact_field_path_from_aspect_label(aspect_path)
                .map(|field_path| (field_path, value.clone()))
                .map_err(
                    |_message| ProjectionFactExtractionError::InvalidDeclaredFieldValueShape {
                        source_family: contract.source_family(),
                        source_identity: compose_scoped_row_source_identity(
                            contract.source_identity(),
                            row.identity()
                                .evidence_identity()
                                .reporting_projection()
                                .as_ref(),
                        ),
                        field_key: aspect_path.to_string(),
                        fact_kind: ProjectionFactKind::DerivedScalarField,
                        expected_shape: "projection fact field path",
                    },
                )
        })
        .collect::<Result<BTreeMap<_, _>, ProjectionFactExtractionError>>()?;

    for field_path in contract_declared_field_paths(contract) {
        if row_fields.contains_key(&field_path) {
            continue;
        }
        if let Some(value) =
            consumed_scalar_value_from_entity_path(row, field_path.canonical_field_path()).map_err(
                |_message| ProjectionFactExtractionError::InvalidDeclaredFieldValueShape {
                    source_family: contract.source_family(),
                    source_identity: compose_scoped_row_source_identity(
                        contract.source_identity(),
                        row.identity()
                            .evidence_identity()
                            .reporting_projection()
                            .as_ref(),
                    ),
                    field_key: field_path.terminal_projection_for_boundary().to_string(),
                    fact_kind: ProjectionFactKind::DerivedScalarField,
                    expected_shape: "foundational scalar",
                },
            )?
        {
            row_fields.insert(field_path, value);
        }
    }

    Ok(row_fields)
}

pub(super) fn identity_field_path() -> ProjectionFactFieldPath {
    projection_fact_field_path_from_segments(["identity", "id"])
}

fn contract_declared_field_paths(
    contract: &MaterializedProjectionContract,
) -> BTreeSet<ProjectionFactFieldPath> {
    let mut field_paths = contract
        .fact_families()
        .iter()
        .filter_map(|fact| match fact.kind() {
            ProjectionFactKind::EntityIdentity => Some(identity_field_path()),
            ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => {
                fact.field_path().cloned()
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    field_paths.insert(identity_field_path());
    field_paths
}

fn projection_fact_field_path_from_external_boundary(
    path: &str,
) -> Result<ProjectionFactFieldPath, String> {
    let fields = path
        .split('.')
        .map(|segment| {
            FieldKey::new(segment.to_string())
                .ok_or_else(|| format!("`{path}` is not a projection fact field path"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let path = CanonicalFieldPath::new(fields)
        .ok_or_else(|| format!("`{path}` is not a projection fact field path"))?;
    Ok(ProjectionFactFieldPath::from_canonical_field_path(path))
}

fn projection_fact_field_path_from_aspect_label(
    aspect: &str,
) -> Result<ProjectionFactFieldPath, String> {
    let field = FieldKey::new(aspect.to_string())
        .ok_or_else(|| format!("`{aspect}` is not a projection fact aspect label"))?;
    let path = CanonicalFieldPath::new([field])
        .ok_or_else(|| format!("`{aspect}` is not a projection fact field path"))?;
    Ok(ProjectionFactFieldPath::from_canonical_field_path(path))
}
