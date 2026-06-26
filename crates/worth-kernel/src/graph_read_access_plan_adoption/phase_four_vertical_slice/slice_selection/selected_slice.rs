use crate::graph_read_access_plan_adoption::WorthGraphReadAccessResolvedPosture;

use super::super::stable_digest;
use super::selection_policy::WorthGraphReadAccessSliceSelectionReason;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSelectedVerticalSlice {
    requirement_identity: String,
    source_posture_row_digest: String,
    source_attempt_digest: Option<String>,
    source_carried_gap_digest: Option<String>,
    source_requirement_record_digest: String,
    read_family_identity_digest: Option<String>,
    requirement_row_digest: Option<String>,
    query_family_name: Option<String>,
    query_family_digest_seed: String,
    query_posture: String,
    denial_kind: Option<String>,
    selection_reason: WorthGraphReadAccessSliceSelectionReason,
    slice_digest: String,
}

impl WorthGraphReadAccessSelectedVerticalSlice {
    pub(crate) fn from_resolved_posture(
        posture: &WorthGraphReadAccessResolvedPosture,
        selection_reason: WorthGraphReadAccessSliceSelectionReason,
    ) -> Self {
        let slice_digest = stable_digest(&[
            "worth_graph_read_access_first_vertical_slice_v1".to_string(),
            format!("requirement:{}", posture.requirement_identity()),
            format!("posture_row:{}", posture.row_digest()),
            format!("query_posture:{}", posture.query_posture()),
            format!("selection_reason:{}", selection_reason.as_str()),
        ]);
        Self {
            requirement_identity: posture.requirement_identity().to_string(),
            source_posture_row_digest: posture.row_digest().to_string(),
            source_attempt_digest: posture.source_attempt_digest().map(str::to_string),
            source_carried_gap_digest: posture.source_carried_gap_digest().map(str::to_string),
            source_requirement_record_digest: posture
                .source_requirement_record_digest()
                .to_string(),
            read_family_identity_digest: posture.read_family_identity_digest().map(str::to_string),
            requirement_row_digest: posture.requirement_row_digest().map(str::to_string),
            query_family_name: posture.query_family_name().map(str::to_string),
            query_family_digest_seed: posture.query_family_digest_seed().to_string(),
            query_posture: posture.query_posture().to_string(),
            denial_kind: posture.denial_kind().map(str::to_string),
            selection_reason,
            slice_digest,
        }
    }

    pub fn requirement_identity(&self) -> &str {
        &self.requirement_identity
    }

    pub fn source_posture_row_digest(&self) -> &str {
        &self.source_posture_row_digest
    }

    pub fn source_attempt_digest(&self) -> Option<&str> {
        self.source_attempt_digest.as_deref()
    }

    pub fn source_carried_gap_digest(&self) -> Option<&str> {
        self.source_carried_gap_digest.as_deref()
    }

    pub fn source_requirement_record_digest(&self) -> &str {
        &self.source_requirement_record_digest
    }

    pub fn read_family_identity_digest(&self) -> Option<&str> {
        self.read_family_identity_digest.as_deref()
    }

    pub fn requirement_row_digest(&self) -> Option<&str> {
        self.requirement_row_digest.as_deref()
    }

    pub fn query_family_name(&self) -> Option<&str> {
        self.query_family_name.as_deref()
    }

    pub fn query_family_digest_seed(&self) -> &str {
        &self.query_family_digest_seed
    }

    pub fn query_posture(&self) -> &str {
        &self.query_posture
    }

    pub fn denial_kind(&self) -> Option<&str> {
        self.denial_kind.as_deref()
    }

    pub const fn selection_reason(&self) -> WorthGraphReadAccessSliceSelectionReason {
        self.selection_reason
    }

    pub fn slice_digest(&self) -> &str {
        &self.slice_digest
    }
}
