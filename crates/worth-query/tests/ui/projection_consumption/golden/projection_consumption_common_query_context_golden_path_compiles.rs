use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::{
    AuthorizedProjectionArtifact, ProjectionAuthorityContract, ProjectionAuthorityOutcome,
    ProjectionFactConsumptionPathError, ProjectionFactFieldPath, QueryContextExecutionArtifact,
};

fn common_query_context_path(
    execution: &QueryContextExecutionArtifact,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
    execution.consume_projection_authority(
        authorized_projection,
        ProjectionAuthorityContract::declare()
            .require_settled_consumption()
            .require_source_authority()
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
    let _ = common_query_context_path;
}
