use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::{
    AuthorizedProjectionArtifact, CanonicalResultShapeArtifact, WorthQueryDerivedArtifactBinding,
    WorthQueryEvidenceIdentity, WorthQueryLiveArtifactBinding, ProjectMaterializedFacts,
    ProjectionFactConsumptionPathError, ProjectionFactFieldPath,
};

fn retained_live_ordinary_path(
    retained: &WorthQueryDerivedArtifactBinding,
    live: &WorthQueryLiveArtifactBinding,
    result_shape: &CanonicalResultShapeArtifact,
    result_shape_identity: &WorthQueryEvidenceIdentity,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<(usize, usize), ProjectionFactConsumptionPathError> {
    let retained_attempt = retained.consume_projection_facts(
        result_shape,
        authorized_projection,
        ProjectMaterializedFacts::declare()
            .view_local_identities()
            .display_field_path(profile_display_name_field_path())
            .source_references(),
    )?;
    let live_attempt = live.consume_projection_facts(
        result_shape_identity,
        authorized_projection,
        ProjectMaterializedFacts::declare()
            .entity_identities()
            .view_local_identities()
            .display_field_path(profile_display_name_field_path())
            .source_references(),
    )?;

    Ok((
        retained_attempt
            .completed()
            .map(|completed| completed.extracted_fact_count())
            .unwrap_or(0),
        live_attempt
            .completed()
            .map(|completed| completed.extracted_fact_count())
            .unwrap_or(0),
    ))
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
    let _ = retained_live_ordinary_path;
}
