use worth_query_installation::facade::{
    WorthQueryDecisionKind, WorthQueryTransformationDisposition,
    WorthQueryTransformationErrorPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryDecisionCausalParent {
    None,
    Single(String),
    Ordered(Vec<String>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDecisionRecord {
    kind: WorthQueryDecisionKind,
    reason_family: String,
    artifact_key_family: String,
    artifact_key: String,
    causal_parent: WorthQueryDecisionCausalParent,
    payload_version: u32,
    payload: String,
    recovery_relevant: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDecisionRecordParts {
    pub kind: WorthQueryDecisionKind,
    pub reason_family: String,
    pub artifact_key_family: String,
    pub artifact_key: String,
    pub causal_parent: WorthQueryDecisionCausalParent,
    pub payload_version: u32,
    pub payload: String,
    pub recovery_relevant: bool,
}

impl WorthQueryDecisionRecord {
    pub fn from_parts(parts: WorthQueryDecisionRecordParts) -> Self {
        Self {
            kind: parts.kind,
            reason_family: parts.reason_family,
            artifact_key_family: parts.artifact_key_family,
            artifact_key: parts.artifact_key,
            causal_parent: parts.causal_parent,
            payload_version: parts.payload_version,
            payload: parts.payload,
            recovery_relevant: parts.recovery_relevant,
        }
    }

    pub fn kind(&self) -> &WorthQueryDecisionKind {
        &self.kind
    }

    pub fn reason_family(&self) -> &str {
        &self.reason_family
    }

    pub fn artifact_key_family(&self) -> &str {
        &self.artifact_key_family
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn causal_parent(&self) -> &WorthQueryDecisionCausalParent {
        &self.causal_parent
    }

    pub const fn payload_version(&self) -> u32 {
        self.payload_version
    }

    pub fn payload(&self) -> &str {
        &self.payload
    }

    pub const fn recovery_relevant(&self) -> bool {
        self.recovery_relevant
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCandidateRecordDisposition {
    Considered,
    Rejected,
    Incumbent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCandidateRecord {
    identity: String,
    disposition: WorthQueryCandidateRecordDisposition,
}

impl WorthQueryCandidateRecord {
    pub fn new(
        identity: impl Into<String>,
        disposition: WorthQueryCandidateRecordDisposition,
    ) -> Self {
        Self {
            identity: identity.into(),
            disposition,
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub const fn disposition(&self) -> WorthQueryCandidateRecordDisposition {
        self.disposition
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryTransformationRecord {
    source_occurrence_identity: String,
    output_occurrence_identities: Vec<String>,
    disposition: WorthQueryTransformationDisposition,
    error: WorthQueryTransformationErrorPosture,
}

impl WorthQueryTransformationRecord {
    pub fn new(
        source_occurrence_identity: impl Into<String>,
        output_occurrence_identities: impl IntoIterator<Item = impl Into<String>>,
        disposition: WorthQueryTransformationDisposition,
        error: WorthQueryTransformationErrorPosture,
    ) -> Self {
        Self {
            source_occurrence_identity: source_occurrence_identity.into(),
            output_occurrence_identities: output_occurrence_identities
                .into_iter()
                .map(Into::into)
                .collect(),
            disposition,
            error,
        }
    }

    pub fn source_occurrence_identity(&self) -> &str {
        &self.source_occurrence_identity
    }

    pub fn output_occurrence_identities(&self) -> &[String] {
        &self.output_occurrence_identities
    }

    pub const fn disposition(&self) -> WorthQueryTransformationDisposition {
        self.disposition
    }

    pub const fn error(&self) -> WorthQueryTransformationErrorPosture {
        self.error
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryDomainEvidenceSidecar {
    decision_records: Option<Vec<WorthQueryDecisionRecord>>,
    candidate_records: Option<Vec<WorthQueryCandidateRecord>>,
    transformation_records: Option<Vec<WorthQueryTransformationRecord>>,
}

impl WorthQueryDomainEvidenceSidecar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn decision_records(
        mut self,
        records: impl IntoIterator<Item = WorthQueryDecisionRecord>,
    ) -> Self {
        self.decision_records = Some(records.into_iter().collect());
        self
    }

    pub fn candidate_records(
        mut self,
        records: impl IntoIterator<Item = WorthQueryCandidateRecord>,
    ) -> Self {
        self.candidate_records = Some(records.into_iter().collect());
        self
    }

    pub fn transformation_records(
        mut self,
        records: impl IntoIterator<Item = WorthQueryTransformationRecord>,
    ) -> Self {
        self.transformation_records = Some(records.into_iter().collect());
        self
    }

    pub fn decision_record_slice(&self) -> Option<&[WorthQueryDecisionRecord]> {
        self.decision_records.as_deref()
    }

    pub fn candidate_record_slice(&self) -> Option<&[WorthQueryCandidateRecord]> {
        self.candidate_records.as_deref()
    }

    pub fn transformation_record_slice(&self) -> Option<&[WorthQueryTransformationRecord]> {
        self.transformation_records.as_deref()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        Option<Vec<WorthQueryDecisionRecord>>,
        Option<Vec<WorthQueryCandidateRecord>>,
        Option<Vec<WorthQueryTransformationRecord>>,
    ) {
        (
            self.decision_records,
            self.candidate_records,
            self.transformation_records,
        )
    }
}
