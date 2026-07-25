use worth_query::facade::domain::{
    WorthQueryArtifactProductionAdmission, WorthQueryArtifactProviderResource,
};

fn bypass_stage_workspace<R: WorthQueryArtifactProviderResource>(
    admission: WorthQueryArtifactProductionAdmission,
    resource: R,
) {
    let _ = admission.register(resource);
}

fn main() {}
