use forge_query::facade::{
    AuthorizedProjectionArtifact, ForgeQueryWriteReceipt, ProjectMaterializedFacts,
    ProjectionFactConsumptionPathError,
};

fn common_write_path(
    write_receipt: &ForgeQueryWriteReceipt,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<String, ProjectionFactConsumptionPathError> {
    let attempt = write_receipt.consume_projection_facts(
        "result-shape:test",
        authorized_projection,
        ProjectMaterializedFacts::declare()
            .target_identity()
            .source_references(),
    )?;

    if let Some(completed) = attempt.completed() {
        let transition_rules = completed.transition_rules();
        return Ok(format!(
            "{}:{}:{}",
            completed.source_identity(),
            transition_rules.rules_digest(),
            completed
                .projection_consumption_envelope()
                .boundary_digest()
        ));
    }

    if let Some(denied) = attempt.denied() {
        return Ok(format!("{:?}", denied.reason()));
    }
    if let Some(deferred) = attempt.deferred() {
        return Ok(format!("{:?}", deferred.reason()));
    }
    if let Some(mismatch) = attempt.source_mismatch() {
        return Ok(format!("{:?}", mismatch.requested_fact_kind()));
    }

    unreachable!()
}

fn main() {
    let _ = common_write_path;
}
