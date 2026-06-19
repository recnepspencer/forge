use super::ForgeServerProductOperationRuntimeArtifactRequirements;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductOperationRuntimeSupportRow {
    requirements: ForgeServerProductOperationRuntimeArtifactRequirements,
    readiness_label: String,
    canonical_digest: String,
}

impl ForgeServerProductOperationRuntimeSupportRow {
    pub(crate) fn new(
        requirements: ForgeServerProductOperationRuntimeArtifactRequirements,
    ) -> Self {
        let readiness_label = if requirements.is_ready() {
            "product-operation-runtime-ready"
        } else {
            "product-operation-runtime-blocked"
        }
        .to_string();
        let canonical_digest = format!(
            "forge-server-product-operation-runtime-support-row-v1|label={readiness_label}|requirements={}",
            requirements.canonical_digest()
        );
        Self {
            requirements,
            readiness_label,
            canonical_digest,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.requirements.is_ready()
    }

    pub fn readiness_label(&self) -> &str {
        &self.readiness_label
    }

    pub fn requirements(&self) -> &ForgeServerProductOperationRuntimeArtifactRequirements {
        &self.requirements
    }

    pub fn blocking_artifact_names(&self) -> Vec<&str> {
        self.requirements.blocking_artifact_names()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
