use crate::runtime::WorthQueryRuntimeFacadeFamily;

use super::super::support_snapshot::WorthQuerySupportSnapshotRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryObservedSupportPin {
    family: WorthQueryRuntimeFacadeFamily,
    surface: String,
    observed_status: String,
    observed_teaching_posture: String,
    observed_live_row_digest: Option<String>,
}

impl WorthQueryObservedSupportPin {
    pub(crate) fn from_validated_parts(
        family: WorthQueryRuntimeFacadeFamily,
        surface: String,
        observed_status: String,
        observed_teaching_posture: String,
        observed_live_row_digest: Option<String>,
    ) -> Self {
        Self {
            family,
            surface,
            observed_status,
            observed_teaching_posture,
            observed_live_row_digest,
        }
    }

    pub(crate) fn present(
        family: WorthQueryRuntimeFacadeFamily,
        row: &WorthQuerySupportSnapshotRow,
    ) -> Self {
        Self {
            family,
            surface: row.surface().to_string(),
            observed_status: row.status().to_string(),
            observed_teaching_posture: row.teaching_posture().to_string(),
            observed_live_row_digest: Some(row.live_row_digest().to_string()),
        }
    }

    pub(crate) fn missing(family: WorthQueryRuntimeFacadeFamily) -> Self {
        Self {
            family,
            surface: family.as_str().to_string(),
            observed_status: "missing".to_string(),
            observed_teaching_posture: "missing".to_string(),
            observed_live_row_digest: None,
        }
    }

    pub fn family(&self) -> WorthQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn surface(&self) -> &str {
        &self.surface
    }

    pub fn observed_status(&self) -> &str {
        &self.observed_status
    }

    pub fn observed_teaching_posture(&self) -> &str {
        &self.observed_teaching_posture
    }

    pub fn observed_live_row_digest(&self) -> Option<&str> {
        self.observed_live_row_digest.as_deref()
    }
}
