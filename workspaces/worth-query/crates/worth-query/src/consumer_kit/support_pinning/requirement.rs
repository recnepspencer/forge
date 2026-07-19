use crate::runtime::WorthQueryRuntimeFacadeFamily;

use super::super::support_snapshot::WorthQuerySupportSnapshotRow;
use super::error::{WorthQuerySupportPinningError, WorthQuerySupportPinningErrorKind};
use super::status::{WorthQueryPinnedSupportStatus, WorthQueryPinnedTeachingPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySupportPinRequirement {
    family: WorthQueryRuntimeFacadeFamily,
    surface: String,
    required_status: WorthQueryPinnedSupportStatus,
    required_teaching_posture: WorthQueryPinnedTeachingPosture,
    pinned_live_row_digest: String,
    pinned_snapshot_row_digest: String,
}

impl WorthQuerySupportPinRequirement {
    pub(crate) fn from_validated_parts(
        family: WorthQueryRuntimeFacadeFamily,
        surface: String,
        required_status: WorthQueryPinnedSupportStatus,
        required_teaching_posture: WorthQueryPinnedTeachingPosture,
        pinned_live_row_digest: String,
        pinned_snapshot_row_digest: String,
    ) -> Self {
        Self {
            family,
            surface,
            required_status,
            required_teaching_posture,
            pinned_live_row_digest,
            pinned_snapshot_row_digest,
        }
    }

    pub(crate) fn from_draft(
        draft: WorthQuerySupportPinRequirementDraft,
    ) -> Result<Self, WorthQuerySupportPinningError> {
        Ok(Self {
            family: draft.family,
            surface: draft.surface,
            required_status: draft.required_status.ok_or_else(|| {
                WorthQuerySupportPinningError::with_family(
                    WorthQuerySupportPinningErrorKind::MissingRequiredStatus,
                    "support pin required row is missing an expected support status",
                    draft.family.as_str(),
                )
            })?,
            required_teaching_posture: draft.required_teaching_posture.ok_or_else(|| {
                WorthQuerySupportPinningError::with_family(
                    WorthQuerySupportPinningErrorKind::MissingRequiredTeachingPosture,
                    "support pin required row is missing an expected teaching posture",
                    draft.family.as_str(),
                )
            })?,
            pinned_live_row_digest: if draft.bind_live_row_digest {
                draft.live_row_digest
            } else {
                return Err(WorthQuerySupportPinningError::with_family(
                    WorthQuerySupportPinningErrorKind::MissingLiveRowDigestBinding,
                    "support pin required row must explicitly bind the live row digest",
                    draft.family.as_str(),
                ));
            },
            pinned_snapshot_row_digest: draft.snapshot_row_digest,
        })
    }

    pub fn family(&self) -> WorthQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn surface(&self) -> &str {
        &self.surface
    }

    pub fn required_status(&self) -> WorthQueryPinnedSupportStatus {
        self.required_status
    }

    pub fn required_teaching_posture(&self) -> WorthQueryPinnedTeachingPosture {
        self.required_teaching_posture
    }

    pub fn pinned_live_row_digest(&self) -> &str {
        &self.pinned_live_row_digest
    }

    pub fn pinned_snapshot_row_digest(&self) -> &str {
        &self.pinned_snapshot_row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySupportPinRequirementDraft {
    family: WorthQueryRuntimeFacadeFamily,
    surface: String,
    required_status: Option<WorthQueryPinnedSupportStatus>,
    required_teaching_posture: Option<WorthQueryPinnedTeachingPosture>,
    bind_live_row_digest: bool,
    live_row_digest: String,
    snapshot_row_digest: String,
}

impl WorthQuerySupportPinRequirementDraft {
    pub(crate) fn from_snapshot_row(
        family: WorthQueryRuntimeFacadeFamily,
        row: &WorthQuerySupportSnapshotRow,
    ) -> Self {
        Self {
            family,
            surface: row.surface().to_string(),
            required_status: None,
            required_teaching_posture: None,
            bind_live_row_digest: false,
            live_row_digest: row.live_row_digest().to_string(),
            snapshot_row_digest: row.snapshot_row_digest().to_string(),
        }
    }

    pub fn status(mut self, status: WorthQueryPinnedSupportStatus) -> Self {
        self.required_status = Some(status);
        self
    }

    pub fn teaching_posture(mut self, posture: WorthQueryPinnedTeachingPosture) -> Self {
        self.required_teaching_posture = Some(posture);
        self
    }

    pub fn bind_live_row_digest(mut self) -> Self {
        self.bind_live_row_digest = true;
        self
    }
}
