use forge_query::facade::{
    AuthorizedProjectionArtifact, CanonicalResultShapeArtifact, ForgeQueryReadResult,
    ProjectMaterializedFacts, ProjectionFactConsumptionAttempt, ProjectionFactConsumptionPathError,
};

fn common_read_path(
    read_result: &ForgeQueryReadResult,
    result_shape: &CanonicalResultShapeArtifact,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<String, ProjectionFactConsumptionPathError> {
    let attempt = read_result.consume_projection_facts(
        result_shape,
        authorized_projection,
        ProjectMaterializedFacts::declare()
            .entity_identities()
            .display_field("profile.display_name"),
    )?;

    if let Some(completed) = attempt.completed() {
        let receipt = completed.receipt();
        let envelope = completed.projection_consumption_envelope();
        return Ok(format!(
            "{}:{}:{}",
            receipt.contract_digest(),
            envelope.envelope_digest(),
            completed.extracted_fact_count()
        ));
    }

    match attempt {
        ProjectionFactConsumptionAttempt::Denied(denied) => Ok(format!("{:?}", denied.reason())),
        ProjectionFactConsumptionAttempt::Deferred(deferred) => {
            Ok(format!("{:?}", deferred.reason()))
        }
        ProjectionFactConsumptionAttempt::SourceMismatch(mismatch) => {
            Ok(format!("{:?}", mismatch.source_family()))
        }
        ProjectionFactConsumptionAttempt::Admitted(_)
        | ProjectionFactConsumptionAttempt::AdmittedWithWarnings(_, _) => unreachable!(),
    }
}

fn main() {
    let _ = common_read_path;
}
