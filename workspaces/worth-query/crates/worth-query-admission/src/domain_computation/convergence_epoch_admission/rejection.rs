use worth_query_installation::facade::WorthQueryInstalledArtifactContractAuthority;

use super::WorthQueryConvergenceAdmissionDenial;

pub struct WorthQueryConvergenceAdmissionRejection {
    denial: WorthQueryConvergenceAdmissionDenial,
    artifact: WorthQueryInstalledArtifactContractAuthority,
}

impl WorthQueryConvergenceAdmissionRejection {
    pub(super) fn new(
        denial: WorthQueryConvergenceAdmissionDenial,
        artifact: WorthQueryInstalledArtifactContractAuthority,
    ) -> Self {
        Self { denial, artifact }
    }

    pub fn denial(&self) -> &WorthQueryConvergenceAdmissionDenial {
        &self.denial
    }

    pub fn into_artifact(self) -> WorthQueryInstalledArtifactContractAuthority {
        self.artifact
    }
}

impl std::fmt::Debug for WorthQueryConvergenceAdmissionRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryConvergenceAdmissionRejection")
            .field("denial", &self.denial)
            .field("artifact_admission", &self.artifact.admission_identity())
            .field(
                "artifact_contract",
                &self.artifact.contract().identity().as_str(),
            )
            .finish()
    }
}
