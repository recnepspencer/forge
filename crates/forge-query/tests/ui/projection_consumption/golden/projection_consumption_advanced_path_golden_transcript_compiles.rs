use forge_query::facade::{
    evaluate_projection_consumption_eligibility, AuthorizedProjectionArtifact,
    CanonicalResultShapeArtifact, ForgeQueryReadReceipt, ForgeQueryReadResult,
    ProjectMaterializedFacts, ProjectionConsumptionDeclarationError, ProjectionConsumptionEligibility,
    ProjectionFactExtractionError,
};

fn advanced_read_path(
    read_receipt: &ForgeQueryReadReceipt,
    read_result: &ForgeQueryReadResult,
    result_shape: &CanonicalResultShapeArtifact,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<String, ProjectionFactExtractionOrDeclarationError> {
    let declaration = read_receipt
        .declare_projection_fact_consumption(
            result_shape,
            authorized_projection,
            ProjectMaterializedFacts::declare()
                .entity_identities()
                .display_field("profile.display_name"),
        )
        .map_err(ProjectionFactExtractionOrDeclarationError::Declaration)?;

    match evaluate_projection_consumption_eligibility(&declaration) {
        ProjectionConsumptionEligibility::Admitted(admitted) => {
            let contract = admitted.bind_contract();
            let fact_set = contract
                .extract_from_read_result(read_result)
                .map_err(ProjectionFactExtractionOrDeclarationError::Extraction)?;
            let receipt = fact_set.issue_receipt();
            let envelope = receipt.projection_consumption_envelope();
            Ok(format!(
                "{}:{}:{}",
                declaration.declaration_digest(),
                receipt.receipt_digest(),
                envelope.envelope_digest()
            ))
        }
        ProjectionConsumptionEligibility::AdmittedWithWarnings(admitted, warnings) => {
            let contract = admitted.bind_contract();
            let fact_set = contract
                .extract_from_read_result(read_result)
                .map_err(ProjectionFactExtractionOrDeclarationError::Extraction)?;
            Ok(format!(
                "{}:{}",
                fact_set.fact_set_digest(),
                warnings.warning_kinds().len()
            ))
        }
        ProjectionConsumptionEligibility::Denied(denied) => Ok(format!("{:?}", denied.reason())),
        ProjectionConsumptionEligibility::Deferred(deferred) => {
            Ok(format!("{:?}", deferred.reason()))
        }
        ProjectionConsumptionEligibility::SourceMismatch(mismatch) => {
            Ok(format!("{:?}", mismatch.source_family()))
        }
    }
}

#[derive(Debug)]
enum ProjectionFactExtractionOrDeclarationError {
    Declaration(ProjectionConsumptionDeclarationError),
    Extraction(ProjectionFactExtractionError),
}

fn main() {
    let _ = advanced_read_path;
}
