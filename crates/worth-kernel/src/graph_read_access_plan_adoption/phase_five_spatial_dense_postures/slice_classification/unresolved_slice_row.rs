use crate::graph_read_access_plan_adoption::WorthGraphReadAccessResolvedPosture;

use super::super::stable_digest;
use super::WorthGraphReadAccessUnresolvedSliceKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessUnresolvedSliceRow {
    kind: WorthGraphReadAccessUnresolvedSliceKind,
    source_posture_row_digest: String,
    source_requirement_record_digest: String,
    read_family_identity_digest: Option<String>,
    requirement_row_digest: Option<String>,
    query_family_name: Option<String>,
    query_family_digest_seed: String,
    read_family_target: Option<String>,
    query_posture: String,
    denial_kind: Option<String>,
    blocker: Option<String>,
    removal_trigger: Option<String>,
    row_digest: String,
}

impl WorthGraphReadAccessUnresolvedSliceRow {
    pub(crate) fn from_posture(
        posture: &WorthGraphReadAccessResolvedPosture,
        kind: WorthGraphReadAccessUnresolvedSliceKind,
    ) -> Self {
        let row_digest = stable_digest(&[
            "worth_graph_read_access_unresolved_spatial_dense_slice_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("posture:{}", posture.row_digest()),
            format!("requirement:{}", posture.source_requirement_record_digest()),
            format!(
                "read_family:{}",
                posture.read_family_identity_digest().unwrap_or("none")
            ),
            format!(
                "requirement_row:{}",
                posture.requirement_row_digest().unwrap_or("none")
            ),
            format!("query_posture:{}", posture.query_posture()),
            format!(
                "read_family_target:{}",
                posture.read_family_target().unwrap_or("none")
            ),
            format!("denial:{}", posture.denial_kind().unwrap_or("none")),
        ]);
        Self {
            kind,
            source_posture_row_digest: posture.row_digest().to_string(),
            source_requirement_record_digest: posture
                .source_requirement_record_digest()
                .to_string(),
            read_family_identity_digest: posture.read_family_identity_digest().map(str::to_string),
            requirement_row_digest: posture.requirement_row_digest().map(str::to_string),
            query_family_name: posture.query_family_name().map(str::to_string),
            query_family_digest_seed: posture.query_family_digest_seed().to_string(),
            read_family_target: posture.read_family_target().map(str::to_string),
            query_posture: posture.query_posture().to_string(),
            denial_kind: posture.denial_kind().map(str::to_string),
            blocker: posture.blocker().map(str::to_string),
            removal_trigger: posture.removal_trigger().map(str::to_string),
            row_digest,
        }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessUnresolvedSliceKind {
        self.kind
    }

    pub fn source_posture_row_digest(&self) -> &str {
        &self.source_posture_row_digest
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

    pub fn read_family_target(&self) -> Option<&str> {
        self.read_family_target.as_deref()
    }

    pub fn query_posture(&self) -> &str {
        &self.query_posture
    }

    pub fn denial_kind(&self) -> Option<&str> {
        self.denial_kind.as_deref()
    }

    pub fn blocker(&self) -> Option<&str> {
        self.blocker.as_deref()
    }

    pub fn removal_trigger(&self) -> Option<&str> {
        self.removal_trigger.as_deref()
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
