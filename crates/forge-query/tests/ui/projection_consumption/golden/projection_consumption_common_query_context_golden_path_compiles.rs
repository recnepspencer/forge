use forge_foundational::facade::{CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    AuthorizedProjectionArtifact, ProjectMaterializedFacts, ProjectionFactConsumptionPathError,
    ProjectionFactFieldPath, QueryContextExecutionArtifact,
};

fn common_query_context_path(
    execution: &QueryContextExecutionArtifact,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<String, ProjectionFactConsumptionPathError> {
    let attempt = execution.consume_projection_facts(
        authorized_projection,
        ProjectMaterializedFacts::declare().display_field_path(profile_display_name_field_path()),
    )?;

    if let Some(completed) = attempt.completed() {
        let warnings = attempt
            .warnings()
            .map(|warnings| warnings.warning_kinds().len())
            .unwrap_or(0);
        return Ok(format!(
            "{}:{}:{}",
            completed.receipt().receipt_digest(),
            completed.warning_kinds().len(),
            warnings
        ));
    }

    if let Some(denied) = attempt.denied() {
        return Ok(format!("{:?}", denied.reason()));
    }
    if let Some(deferred) = attempt.deferred() {
        return Ok(format!("{:?}", deferred.reason()));
    }
    if let Some(mismatch) = attempt.source_mismatch() {
        return Ok(format!("{:?}", mismatch.source_family()));
    }

    unreachable!()
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
