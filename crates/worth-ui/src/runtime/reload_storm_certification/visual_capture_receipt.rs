use crate::runtime::{
    WorthUiProjectionRebindBatchDigest, WorthUiReloadProjectionBreadthCertification,
    WorthUiRuntimeChangeEvidenceDigest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHotReloadVisualCaptureReceipt {
    runtime_change_digest: WorthUiRuntimeChangeEvidenceDigest,
    projection_rebind_digest: WorthUiProjectionRebindBatchDigest,
    image_artifact_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHotReloadVisualCaptureDenial {
    EmptyImageArtifactDigest,
}

impl WorthUiHotReloadVisualCaptureReceipt {
    pub fn from_certified_capture(
        breadth_certification: &WorthUiReloadProjectionBreadthCertification,
        image_artifact_digest: impl Into<String>,
    ) -> Result<Self, WorthUiHotReloadVisualCaptureDenial> {
        let image_artifact_digest = image_artifact_digest.into();
        if image_artifact_digest.trim().is_empty() {
            return Err(WorthUiHotReloadVisualCaptureDenial::EmptyImageArtifactDigest);
        }
        Ok(Self {
            runtime_change_digest: breadth_certification.change_evidence_digest(),
            projection_rebind_digest: breadth_certification.projection_rebind_batch_digest(),
            image_artifact_digest,
        })
    }

    pub fn runtime_change_digest(&self) -> WorthUiRuntimeChangeEvidenceDigest {
        self.runtime_change_digest
    }

    pub fn projection_rebind_digest(&self) -> WorthUiProjectionRebindBatchDigest {
        self.projection_rebind_digest
    }

    pub fn image_artifact_digest(&self) -> &str {
        &self.image_artifact_digest
    }
}
