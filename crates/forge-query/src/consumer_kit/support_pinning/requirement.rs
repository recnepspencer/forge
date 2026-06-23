use crate::runtime::ForgeQueryRuntimeFacadeFamily;

use super::super::support_snapshot::ForgeQuerySupportSnapshotRow;
use super::error::{ForgeQuerySupportPinningError, ForgeQuerySupportPinningErrorKind};
use super::status::{ForgeQueryPinnedSupportStatus, ForgeQueryPinnedTeachingPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySupportPinRequirement {
    family: ForgeQueryRuntimeFacadeFamily,
    surface: String,
    required_status: ForgeQueryPinnedSupportStatus,
    required_teaching_posture: ForgeQueryPinnedTeachingPosture,
    pinned_live_row_digest: String,
    pinned_snapshot_row_digest: String,
}

impl ForgeQuerySupportPinRequirement {
    pub(crate) fn from_validated_parts(
        family: ForgeQueryRuntimeFacadeFamily,
        surface: String,
        required_status: ForgeQueryPinnedSupportStatus,
        required_teaching_posture: ForgeQueryPinnedTeachingPosture,
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
        draft: ForgeQuerySupportPinRequirementDraft,
    ) -> Result<Self, ForgeQuerySupportPinningError> {
        Ok(Self {
            family: draft.family,
            surface: draft.surface,
            required_status: draft.required_status.ok_or_else(|| {
                ForgeQuerySupportPinningError::with_family(
                    ForgeQuerySupportPinningErrorKind::MissingRequiredStatus,
                    "support pin required row is missing an expected support status",
                    draft.family.as_str(),
                )
            })?,
            required_teaching_posture: draft.required_teaching_posture.ok_or_else(|| {
                ForgeQuerySupportPinningError::with_family(
                    ForgeQuerySupportPinningErrorKind::MissingRequiredTeachingPosture,
                    "support pin required row is missing an expected teaching posture",
                    draft.family.as_str(),
                )
            })?,
            pinned_live_row_digest: if draft.bind_live_row_digest {
                draft.live_row_digest
            } else {
                return Err(ForgeQuerySupportPinningError::with_family(
                    ForgeQuerySupportPinningErrorKind::MissingLiveRowDigestBinding,
                    "support pin required row must explicitly bind the live row digest",
                    draft.family.as_str(),
                ));
            },
            pinned_snapshot_row_digest: draft.snapshot_row_digest,
        })
    }

    pub fn family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn surface(&self) -> &str {
        &self.surface
    }

    pub fn required_status(&self) -> ForgeQueryPinnedSupportStatus {
        self.required_status
    }

    pub fn required_teaching_posture(&self) -> ForgeQueryPinnedTeachingPosture {
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
pub struct ForgeQuerySupportPinRequirementDraft {
    family: ForgeQueryRuntimeFacadeFamily,
    surface: String,
    required_status: Option<ForgeQueryPinnedSupportStatus>,
    required_teaching_posture: Option<ForgeQueryPinnedTeachingPosture>,
    bind_live_row_digest: bool,
    live_row_digest: String,
    snapshot_row_digest: String,
}

impl ForgeQuerySupportPinRequirementDraft {
    pub(crate) fn from_snapshot_row(
        family: ForgeQueryRuntimeFacadeFamily,
        row: &ForgeQuerySupportSnapshotRow,
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

    pub fn status(mut self, status: ForgeQueryPinnedSupportStatus) -> Self {
        self.required_status = Some(status);
        self
    }

    pub fn teaching_posture(mut self, posture: ForgeQueryPinnedTeachingPosture) -> Self {
        self.required_teaching_posture = Some(posture);
        self
    }

    pub fn bind_live_row_digest(mut self) -> Self {
        self.bind_live_row_digest = true;
        self
    }
}
