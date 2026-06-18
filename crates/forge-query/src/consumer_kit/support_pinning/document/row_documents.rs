use super::semantic_admission::{
    admit_pinned_facade_family, admit_pinned_status, admit_pinned_teaching_posture,
};
use crate::consumer_kit::support_pinning::error::ForgeQuerySupportPinningError;
use crate::consumer_kit::support_pinning::observed_row::ForgeQueryObservedSupportPin;
use crate::consumer_kit::support_pinning::requirement::ForgeQuerySupportPinRequirement;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub(super) struct ForgeQuerySupportPinRequirementDocument {
    family: String,
    surface: String,
    required_status: String,
    required_teaching_posture: String,
    pinned_live_row_digest: String,
    pinned_snapshot_row_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub(super) struct ForgeQueryObservedSupportPinDocument {
    family: String,
    surface: String,
    observed_status: String,
    observed_teaching_posture: String,
    observed_live_row_digest: Option<String>,
}

impl ForgeQuerySupportPinRequirementDocument {
    pub(super) fn from_requirement(requirement: &ForgeQuerySupportPinRequirement) -> Self {
        Self {
            family: requirement.family().as_str().to_string(),
            surface: requirement.surface().to_string(),
            required_status: requirement.required_status().as_str().to_string(),
            required_teaching_posture: requirement.required_teaching_posture().as_str().to_string(),
            pinned_live_row_digest: requirement.pinned_live_row_digest().to_string(),
            pinned_snapshot_row_digest: requirement.pinned_snapshot_row_digest().to_string(),
        }
    }

    pub(super) fn validate(
        &self,
    ) -> Result<ForgeQuerySupportPinRequirement, ForgeQuerySupportPinningError> {
        Ok(ForgeQuerySupportPinRequirement::from_validated_parts(
            admit_pinned_facade_family(&self.family)?,
            self.surface.clone(),
            admit_pinned_status(&self.required_status)?,
            admit_pinned_teaching_posture(&self.required_teaching_posture)?,
            self.pinned_live_row_digest.clone(),
            self.pinned_snapshot_row_digest.clone(),
        ))
    }
}

impl ForgeQueryObservedSupportPinDocument {
    pub(super) fn from_observed(observed: &ForgeQueryObservedSupportPin) -> Self {
        Self {
            family: observed.family().as_str().to_string(),
            surface: observed.surface().to_string(),
            observed_status: observed.observed_status().to_string(),
            observed_teaching_posture: observed.observed_teaching_posture().to_string(),
            observed_live_row_digest: observed.observed_live_row_digest().map(str::to_string),
        }
    }

    pub(super) fn validate(
        &self,
    ) -> Result<ForgeQueryObservedSupportPin, ForgeQuerySupportPinningError> {
        admit_pinned_status(&self.observed_status)?;
        admit_pinned_teaching_posture(&self.observed_teaching_posture)?;
        Ok(ForgeQueryObservedSupportPin::from_validated_parts(
            admit_pinned_facade_family(&self.family)?,
            self.surface.clone(),
            self.observed_status.clone(),
            self.observed_teaching_posture.clone(),
            self.observed_live_row_digest.clone(),
        ))
    }
}
