use super::super::stable_identity_digest::stable_digest;
use crate::graph_read_access_declarations::WorthGraphReadRequirementDerivationRecord;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAdmissionAttemptKind {
    BlockedByRequirementDerivationGap,
    MissingQueryReadFamilyArtifact,
    QueryAdmissionInspected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAdmissionAttempt {
    kind: WorthGraphReadAdmissionAttemptKind,
    source_requirement_record_digest: String,
    query_family_anchor_digest: String,
    query_api_required: &'static str,
    attempt_digest: String,
}

impl WorthGraphReadAdmissionAttempt {
    pub(crate) fn blocked_by_requirement_derivation_gap(
        record: &WorthGraphReadRequirementDerivationRecord,
    ) -> Self {
        Self::new(
            WorthGraphReadAdmissionAttemptKind::BlockedByRequirementDerivationGap,
            record,
            "admit_graph_read_access_for_family(...)",
        )
    }

    pub(crate) fn missing_query_read_family_artifact(
        record: &WorthGraphReadRequirementDerivationRecord,
    ) -> Self {
        Self::new(
            WorthGraphReadAdmissionAttemptKind::MissingQueryReadFamilyArtifact,
            record,
            "admit_graph_read_access_for_family(...)",
        )
    }

    pub(crate) fn query_admission_inspected(
        record: &WorthGraphReadRequirementDerivationRecord,
    ) -> Self {
        Self::new(
            WorthGraphReadAdmissionAttemptKind::QueryAdmissionInspected,
            record,
            "admit_graph_read_access_for_family(...)",
        )
    }

    fn new(
        kind: WorthGraphReadAdmissionAttemptKind,
        record: &WorthGraphReadRequirementDerivationRecord,
        query_api_required: &'static str,
    ) -> Self {
        let source_requirement_record_digest = record.record_digest().to_string();
        let query_family_anchor_digest = record.query_family_digest_seed().to_string();
        let attempt_digest = stable_digest(&[
            "worth_graph_read_admission_attempt_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("requirement_record:{source_requirement_record_digest}"),
            format!("query_family_anchor:{query_family_anchor_digest}"),
            format!("query_api_required:{query_api_required}"),
        ]);
        Self {
            kind,
            source_requirement_record_digest,
            query_family_anchor_digest,
            query_api_required,
            attempt_digest,
        }
    }

    pub const fn kind(&self) -> WorthGraphReadAdmissionAttemptKind {
        self.kind
    }

    pub fn source_requirement_record_digest(&self) -> &str {
        &self.source_requirement_record_digest
    }

    pub fn query_family_anchor_digest(&self) -> &str {
        &self.query_family_anchor_digest
    }

    pub const fn query_api_required(&self) -> &'static str {
        self.query_api_required
    }

    pub fn attempt_digest(&self) -> &str {
        &self.attempt_digest
    }
}

impl WorthGraphReadAdmissionAttemptKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockedByRequirementDerivationGap => "blocked_by_requirement_derivation_gap",
            Self::MissingQueryReadFamilyArtifact => "missing_query_read_family_artifact",
            Self::QueryAdmissionInspected => "query_admission_inspected",
        }
    }
}
