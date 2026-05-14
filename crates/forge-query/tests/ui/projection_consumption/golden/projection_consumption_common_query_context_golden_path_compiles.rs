use forge_query::facade::{
    AuthorizedProjectionArtifact, ProjectMaterializedFacts, ProjectionFactConsumptionPathError,
    QueryContextExecutionArtifact,
};

fn common_query_context_path(
    execution: &QueryContextExecutionArtifact,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<String, ProjectionFactConsumptionPathError> {
    let attempt = execution.consume_projection_facts(
        authorized_projection,
        ProjectMaterializedFacts::declare().display_field("profile.display_name"),
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

fn main() {
    let _ = common_query_context_path;
}
