use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::{
    AuthorizedProjectionArtifact, CanonicalResultShapeArtifact, WorthQueryDerivedArtifactBinding,
    WorthQueryEvidenceIdentity, WorthQueryLiveArtifactBinding,
    ProjectionAuthorityContract, ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError,
    ProjectionFactFieldPath,
};

fn retained_live_ordinary_path(
    retained: &WorthQueryDerivedArtifactBinding,
    live: &WorthQueryLiveArtifactBinding,
    result_shape: &CanonicalResultShapeArtifact,
    result_shape_identity: &WorthQueryEvidenceIdentity,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<(usize, usize), ProjectionFactConsumptionPathError> {
    let (retained, live) = retained_live_authority_path(
        retained,
        live,
        result_shape,
        result_shape_identity,
        authorized_projection,
    )?;
    Ok((
        retained
            .authority()
            .map(|authority| authority.counters().consumed_fact_visits())
            .unwrap_or(0),
        live.authority()
            .map(|authority| authority.counters().consumed_fact_visits())
            .unwrap_or(0),
    ))
}

fn retained_live_authority_path(
    retained: &WorthQueryDerivedArtifactBinding,
    live: &WorthQueryLiveArtifactBinding,
    result_shape: &CanonicalResultShapeArtifact,
    result_shape_identity: &WorthQueryEvidenceIdentity,
    projection: &AuthorizedProjectionArtifact,
) -> Result<(ProjectionAuthorityOutcome, ProjectionAuthorityOutcome), ProjectionFactConsumptionPathError>
{
    let retained_contract = ProjectionAuthorityContract::declare()
        .require_settled_consumption()
        .require_source_authority()
        .require_source_references()
        .require_display_field(profile_display_name_field_path());
    let live_contract = ProjectionAuthorityContract::declare()
        .require_settled_consumption()
        .require_source_authority()
        .require_source_references()
        .require_entity_identities()
        .require_display_field(profile_display_name_field_path());
    Ok((
        retained.consume_projection_authority(result_shape, projection, retained_contract)?,
        live.consume_projection_authority(result_shape_identity, projection, live_contract)?,
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
    let _ = retained_live_authority_path;
}
