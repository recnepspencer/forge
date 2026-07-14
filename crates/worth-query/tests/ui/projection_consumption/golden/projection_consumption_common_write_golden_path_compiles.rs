use worth_query::facade::foundation::{AuthorizedProjectionArtifact, ProjectionAuthorityContract, ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError};
use worth_query::facade::runtime::WorthQueryWriteReceipt;

fn common_write_path(
    write_receipt: &WorthQueryWriteReceipt,
    authorized_projection: &AuthorizedProjectionArtifact,
) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
    write_receipt.consume_projection_authority(
        "result-shape:test",
        authorized_projection,
        ProjectionAuthorityContract::declare()
            .require_settled_consumption()
            .require_source_authority()
            .require_target_identity()
            .require_source_references(),
    )
}

fn main() {
    let _ = common_write_path;
}
