use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::foundation::{AuthorizedProjectionArtifact, CanonicalResultShapeArtifact, ProjectionAuthorityContract, ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError, ProjectionFactFieldPath};
use worth_query::facade::runtime::WorthQueryReadResult;

fn advanced_read_path(
    read_result: &WorthQueryReadResult,
    result_shape: &CanonicalResultShapeArtifact,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
    read_result.consume_projection_authority(
        result_shape,
        authorized_projection,
        ProjectionAuthorityContract::declare()
            .require_settled_consumption()
            .require_source_authority()
            .require_basis_generation()
            .require_entity_identities()
            .require_display_field(profile_display_name_field_path()),
    )
}

fn profile_display_name_field_path() -> ProjectionFactFieldPath {
    ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("profile").expect("test field segment must be valid"),
            FieldKey::new("display_name").expect("test field segment must be valid"),
        ])
        .expect("test field path must be valid"),
    )
}

fn main() {
    let _ = advanced_read_path;
}
