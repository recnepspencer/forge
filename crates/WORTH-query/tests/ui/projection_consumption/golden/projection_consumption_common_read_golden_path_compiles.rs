use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::{
    AuthorizedProjectionArtifact, CanonicalResultShapeArtifact, WorthQueryReadResult,
    ProjectMaterializedFacts, ProjectionFactConsumptionAttempt, ProjectionFactConsumptionPathError,
    ProjectionFactFieldPath,
};

fn common_read_path(
    read_result: &WorthQueryReadResult,
    result_shape: &CanonicalResultShapeArtifact,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<String, ProjectionFactConsumptionPathError> {
    let attempt = read_result.consume_projection_facts(
        result_shape,
        authorized_projection,
        ProjectMaterializedFacts::declare()
            .entity_identities()
            .display_field_path(profile_display_name_field_path()),
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
    let _ = common_read_path;
}
