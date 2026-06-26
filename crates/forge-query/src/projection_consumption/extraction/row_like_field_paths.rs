use std::collections::{BTreeMap, BTreeSet};

use forge_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use forge_runtime_bridge::facade::BridgeMaterializedFieldProjection;

use super::super::contracts::MaterializedProjectionContract;
use super::super::facts::{
    projection_fact_field_path_from_segments, ProjectionFactFieldPath, ProjectionFactKind,
};
use super::super::identity::compose_scoped_row_source_identity;
use super::consumed_scalar_value::consumed_scalar_value_from_entity_path;
use super::ProjectionFactExtractionError;
use crate::memory_workspace::ForgeQueryEntity;

#[derive(Clone, Debug)]
pub(super) struct ProjectionMaterializedField {
    field_path: ProjectionFactFieldPath,
    value: AspectValue,
}

impl ProjectionMaterializedField {
    pub(super) fn from_relational_projected_aspect_key(
        contract: &MaterializedProjectionContract,
        row_identity: &str,
        aspect_key: &AspectKey,
        value: AspectValue,
    ) -> Result<Self, ProjectionFactExtractionError> {
        projection_fact_field_path_from_relational_projected_aspect_key(aspect_key)
            .map(|field_path| Self { field_path, value })
            .map_err(
                |_message| ProjectionFactExtractionError::InvalidDeclaredFieldValueShape {
                    source_family: contract.source_family(),
                    source_identity: compose_scoped_row_source_identity(
                        contract.source_identity(),
                        row_identity,
                    ),
                    field_key: aspect_key.as_str().to_string(),
                    fact_kind: ProjectionFactKind::DerivedScalarField,
                    expected_shape: "relational projected aspect key",
                },
            )
    }

    pub(super) fn from_bridge_field_value(
        contract: &MaterializedProjectionContract,
        row_identity: &str,
        projection: &BridgeMaterializedFieldProjection,
        value: AspectValue,
    ) -> Result<Self, ProjectionFactExtractionError> {
        projection_fact_field_path_from_bridge_projection(projection)
            .map(|field_path| Self { field_path, value })
            .map_err(
                |_message| ProjectionFactExtractionError::InvalidDeclaredFieldValueShape {
                    source_family: contract.source_family(),
                    source_identity: compose_scoped_row_source_identity(
                        contract.source_identity(),
                        row_identity,
                    ),
                    field_key: projection.field_identity().as_str().to_string(),
                    fact_kind: ProjectionFactKind::DerivedScalarField,
                    expected_shape: "bridge projection field locator",
                },
            )
    }

    fn into_parts(self) -> (ProjectionFactFieldPath, AspectValue) {
        (self.field_path, self.value)
    }
}

pub(super) fn lower_materialized_fields<Fields>(
    fields: Fields,
) -> Result<BTreeMap<ProjectionFactFieldPath, AspectValue>, ProjectionFactExtractionError>
where
    Fields: Iterator<Item = ProjectionMaterializedField>,
{
    Ok(fields
        .map(ProjectionMaterializedField::into_parts)
        .collect())
}

pub(super) fn query_read_result_row_fields(
    contract: &MaterializedProjectionContract,
    row: &ForgeQueryEntity,
) -> Result<BTreeMap<ProjectionFactFieldPath, AspectValue>, ProjectionFactExtractionError> {
    let mut row_fields = row
        .aspect_values()
        .map(|(aspect_key, value)| {
            projection_fact_field_path_from_aspect_key(aspect_key)
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
                        field_key: aspect_key.as_str().to_string(),
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
    projection_fact_field_path_from_segments([
        FieldKey::new("identity").expect("identity field key should admit"),
        FieldKey::new("id").expect("id field key should admit"),
    ])
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

fn projection_fact_field_path_from_relational_projected_aspect_key(
    aspect_key: &AspectKey,
) -> Result<ProjectionFactFieldPath, String> {
    let path = aspect_key.as_str();
    let fields = path
        .split('.')
        .map(|segment| {
            FieldKey::new(segment)
                .ok_or_else(|| format!("`{path}` is not a relational projected aspect key path"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let path = CanonicalFieldPath::new(fields)
        .ok_or_else(|| format!("`{path}` is not a relational projected aspect key path"))?;
    Ok(ProjectionFactFieldPath::from_canonical_field_path(path))
}

fn projection_fact_field_path_from_aspect_key(
    aspect_key: &AspectKey,
) -> Result<ProjectionFactFieldPath, String> {
    let aspect_key_text = aspect_key.as_str();
    let field = FieldKey::new(aspect_key_text.to_string())
        .ok_or_else(|| format!("`{aspect_key_text}` is not a projection fact aspect key"))?;
    let path = CanonicalFieldPath::new([field])
        .ok_or_else(|| format!("`{aspect_key_text}` is not a projection fact field path"))?;
    Ok(ProjectionFactFieldPath::from_canonical_field_path(path))
}

fn projection_fact_field_path_from_bridge_projection(
    projection: &BridgeMaterializedFieldProjection,
) -> Result<ProjectionFactFieldPath, String> {
    if let Some(field_locator) = projection.field_locator() {
        return Ok(ProjectionFactFieldPath::from_canonical_field_path(
            field_locator.field_path().clone(),
        ));
    }
    projection_fact_field_path_from_aspect_key(projection.aspect_key())
}
