use worth_query::facade::foundation::{AuthorizedProjectionArtifact, CanonicalResultShapeArtifact, ProjectMaterializedFacts};
use worth_query::facade::runtime::WorthQueryReadReceipt;

fn old_path(
    receipt: &WorthQueryReadReceipt,
    shape: &CanonicalResultShapeArtifact,
    projection: &AuthorizedProjectionArtifact,
) {
    let _ = receipt.declare_projection_fact_consumption(
        shape,
        projection,
        ProjectMaterializedFacts::declare().entity_identities(),
    );
}

fn main() {}
