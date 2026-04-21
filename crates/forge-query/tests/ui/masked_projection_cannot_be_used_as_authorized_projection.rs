use forge_query::facade::{AuthorizedProjectionArtifact, MaskedProjectionArtifact};

fn requires_authorized_projection(_artifact: &AuthorizedProjectionArtifact) {}

fn main() {
    let masked_projection: Option<MaskedProjectionArtifact> = None;
    requires_authorized_projection(masked_projection.as_ref().unwrap());
}
