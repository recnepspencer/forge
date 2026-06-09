use forge_query::facade::{
    AuthorizedProjectionArtifact, ForgeQueryDerivedArtifactBinding, ForgeQueryLiveArtifactBinding,
    ProjectMaterializedFacts, ProjectionFactConsumptionPathError,
};

fn retained_live_ordinary_path(
    retained: &ForgeQueryDerivedArtifactBinding,
    live: &ForgeQueryLiveArtifactBinding,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<(usize, usize), ProjectionFactConsumptionPathError> {
    let retained_attempt = retained.consume_projection_facts(
        "result-shape:test",
        authorized_projection,
        ProjectMaterializedFacts::declare()
            .view_local_identities()
            .display_field("profile.display_name")
            .source_references(),
    )?;
    let live_attempt = live.consume_projection_facts(
        "result-shape:test",
        authorized_projection,
        ProjectMaterializedFacts::declare()
            .entity_identities()
            .view_local_identities()
            .display_field("profile.display_name")
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

fn main() {
    let _ = retained_live_ordinary_path;
}
