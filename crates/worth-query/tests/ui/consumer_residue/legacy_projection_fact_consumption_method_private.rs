use worth_query::facade::foundation::{AuthorizedProjectionArtifact, CanonicalResultShapeArtifact, ProjectMaterializedFacts};
use worth_query::facade::runtime::WorthQueryReadResult;

fn old_path(
    result: &WorthQueryReadResult,
    shape: &CanonicalResultShapeArtifact,
    projection: &AuthorizedProjectionArtifact,
) {
    let _ = result.consume_projection_facts(
        shape,
        projection,
        ProjectMaterializedFacts::declare().entity_identities(),
    );
}

fn main() {}
