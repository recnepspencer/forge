use worth_query::facade::foundation::{AuthorizedProjectionArtifact, ProjectionAuthorityContract, ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError};
use worth_query::facade::runtime::WorthQueryWriteReceipt;

fn consume_authority(
    receipt: &WorthQueryWriteReceipt,
    projection: &AuthorizedProjectionArtifact,
) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
    receipt.consume_projection_authority(
        "result-shape:test",
        projection,
        ProjectionAuthorityContract::declare()
            .require_settled_consumption()
            .require_source_authority()
            .require_target_identity()
            .require_source_references(),
    )
}

fn main() {
    let _ = consume_authority;
}
