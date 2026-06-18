use super::{RetainedArtifactSet, UnsupportedReplayReasonCode, UnsupportedReplayWorkload};
use crate::planar_contracts::projection_consumed_facts::ProjectionConsumedPlanarFactsReceipt;
use crate::planar_contracts::retained_planar_facts::RetainedPlanarFactsReceipt;

pub struct RetainedWorkload {
    declaration: String,
    retained_planar_facts: RetainedPlanarFactsReceipt,
    projection_consumed_facts: Option<ProjectionConsumedPlanarFactsReceipt>,
}

impl RetainedWorkload {
    pub fn from_retained_planar_facts(retained_planar_facts: RetainedPlanarFactsReceipt) -> Self {
        Self {
            declaration: "retained artifact capture workload".to_string(),
            retained_planar_facts,
            projection_consumed_facts: None,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_projection_consumed_facts(
        mut self,
        projection_consumed_facts: ProjectionConsumedPlanarFactsReceipt,
    ) -> Self {
        self.projection_consumed_facts = Some(projection_consumed_facts);
        self
    }

    pub fn capture(self) -> Result<CapturedRetainedWorkload, UnsupportedReplayWorkload> {
        reject_blank_retained_capture_declaration(&self.declaration)?;
        let projection_consumed_facts =
            require_projection_consumed_capture_artifact(self.projection_consumed_facts)?;
        let retained_artifacts =
            captured_retained_artifacts(self.retained_planar_facts, projection_consumed_facts)?;
        let capture_receipt =
            RetainedArtifactCaptureReceipt::from_artifacts(self.declaration, &retained_artifacts);
        Ok(CapturedRetainedWorkload::new(
            retained_artifacts,
            capture_receipt,
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedArtifactCaptureReceipt {
    capture_identity: String,
    retained_artifact_identity: String,
    retained_basis_identity: String,
    replay_checkpoint_identity: String,
    retained_artifact_rows: usize,
}

impl RetainedArtifactCaptureReceipt {
    pub(crate) fn from_artifacts(
        declaration: impl Into<String>,
        retained_artifacts: &RetainedArtifactSet,
    ) -> Self {
        let declaration = declaration.into();
        let retained_artifact_identity = retained_artifacts.retained_artifact_identity();
        let retained_basis_identity = retained_artifacts.retained_basis_identity();
        let replay_checkpoint_identity = retained_artifacts.replay_checkpoint_identity();
        let capture_identity = format!(
            "retained-artifact-capture:{declaration}:{retained_artifact_identity}:{retained_basis_identity}:{replay_checkpoint_identity}"
        );
        Self {
            capture_identity,
            retained_artifact_identity,
            retained_basis_identity,
            replay_checkpoint_identity,
            retained_artifact_rows: retained_artifacts.retained_artifact_rows(),
        }
    }

    pub fn capture_identity(&self) -> &str {
        &self.capture_identity
    }

    pub fn retained_artifact_identity(&self) -> &str {
        &self.retained_artifact_identity
    }

    pub fn retained_basis_identity(&self) -> &str {
        &self.retained_basis_identity
    }

    pub fn replay_checkpoint_identity(&self) -> &str {
        &self.replay_checkpoint_identity
    }

    pub fn retained_artifact_rows(&self) -> usize {
        self.retained_artifact_rows
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CapturedRetainedWorkload {
    retained_artifacts: RetainedArtifactSet,
    capture_receipt: RetainedArtifactCaptureReceipt,
}

impl CapturedRetainedWorkload {
    pub(crate) fn new(
        retained_artifacts: RetainedArtifactSet,
        capture_receipt: RetainedArtifactCaptureReceipt,
    ) -> Self {
        Self {
            retained_artifacts,
            capture_receipt,
        }
    }

    pub fn capture_receipt(&self) -> &RetainedArtifactCaptureReceipt {
        &self.capture_receipt
    }

    pub(crate) fn into_retained_artifacts(self) -> RetainedArtifactSet {
        self.retained_artifacts
    }
}

fn reject_blank_retained_capture_declaration(
    declaration: &str,
) -> Result<(), UnsupportedReplayWorkload> {
    if declaration.trim().is_empty() {
        Err(UnsupportedReplayWorkload::new(
            UnsupportedReplayReasonCode::MissingDeclaration,
            "Retained artifact capture requires a human-readable declaration.",
        ))
    } else {
        Ok(())
    }
}

fn require_projection_consumed_capture_artifact(
    projection_consumed_facts: Option<ProjectionConsumedPlanarFactsReceipt>,
) -> Result<ProjectionConsumedPlanarFactsReceipt, UnsupportedReplayWorkload> {
    projection_consumed_facts.ok_or_else(|| {
        UnsupportedReplayWorkload::new(
            UnsupportedReplayReasonCode::MissingProjectionConsumedFacts,
            "Retained artifact capture requires projection-consumed facts from the retained planar artifact.",
        )
    })
}

fn captured_retained_artifacts(
    retained_planar_facts: RetainedPlanarFactsReceipt,
    projection_consumed_facts: ProjectionConsumedPlanarFactsReceipt,
) -> Result<RetainedArtifactSet, UnsupportedReplayWorkload> {
    let retained_artifacts = RetainedArtifactSet::from_retained_planar_facts(retained_planar_facts)
        .with_projection_consumed_facts(projection_consumed_facts);
    retained_artifacts.require_projection_consumed_facts()?;
    Ok(retained_artifacts)
}
