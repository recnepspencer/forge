use worth_foundational::facade::RetentionDeliveryProfile;
use worth_query_installation::facade::{
    WorthQueryArtifactClassification, WorthQueryArtifactDeletionPosture,
    WorthQueryArtifactLegalHoldPosture, WorthQueryArtifactRedactionPosture,
    WorthQueryDecisionSchema, WorthQueryStructuralCounterSchema,
};

use super::{
    WorthQueryCandidateSearchSummary, WorthQueryDecisionSummaryCounts,
    WorthQueryTransformationSummary,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainEvidenceAuthorityPosture {
    DescriptiveOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEvidenceGovernance {
    audiences: Vec<String>,
    classification: WorthQueryArtifactClassification,
    redaction: WorthQueryArtifactRedactionPosture,
    retention: RetentionDeliveryProfile,
    deletion: WorthQueryArtifactDeletionPosture,
    legal_hold: WorthQueryArtifactLegalHoldPosture,
}

impl WorthQueryDomainEvidenceGovernance {
    pub(super) fn from_contract(
        contract: &worth_query_installation::facade::WorthQueryArtifactGovernanceContract,
    ) -> Self {
        Self {
            audiences: contract.audiences().to_vec(),
            classification: contract.classification(),
            redaction: contract.redaction(),
            retention: contract.retention(),
            deletion: contract.deletion(),
            legal_hold: contract.legal_hold(),
        }
    }

    pub fn audiences(&self) -> &[String] {
        &self.audiences
    }

    pub const fn classification(&self) -> WorthQueryArtifactClassification {
        self.classification
    }

    pub const fn redaction(&self) -> WorthQueryArtifactRedactionPosture {
        self.redaction
    }

    pub const fn retention(&self) -> RetentionDeliveryProfile {
        self.retention
    }

    pub const fn deletion(&self) -> WorthQueryArtifactDeletionPosture {
        self.deletion
    }

    pub const fn legal_hold(&self) -> WorthQueryArtifactLegalHoldPosture {
        self.legal_hold
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedStructuralCounter {
    schema: WorthQueryStructuralCounterSchema,
    initial: u64,
    observed: u64,
    provider_certification: Option<String>,
}

impl WorthQueryAdmittedStructuralCounter {
    pub(super) fn new(
        schema: WorthQueryStructuralCounterSchema,
        initial: u64,
        observed: u64,
        provider_certification: Option<String>,
    ) -> Self {
        Self {
            schema,
            initial,
            observed,
            provider_certification,
        }
    }

    pub fn schema(&self) -> &WorthQueryStructuralCounterSchema {
        &self.schema
    }

    pub const fn initial(&self) -> u64 {
        self.initial
    }

    pub const fn observed(&self) -> u64 {
        self.observed
    }

    pub fn provider_certification(&self) -> Option<&str> {
        self.provider_certification.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedDecisionSummary {
    schema: WorthQueryDecisionSchema,
    counts: WorthQueryDecisionSummaryCounts,
}

impl WorthQueryAdmittedDecisionSummary {
    pub(super) const fn new(
        schema: WorthQueryDecisionSchema,
        counts: WorthQueryDecisionSummaryCounts,
    ) -> Self {
        Self { schema, counts }
    }

    pub fn schema(&self) -> &WorthQueryDecisionSchema {
        &self.schema
    }

    pub const fn counts(&self) -> WorthQueryDecisionSummaryCounts {
        self.counts
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEvidenceCore {
    pub(super) counters: Vec<WorthQueryAdmittedStructuralCounter>,
    pub(super) decisions: Vec<WorthQueryAdmittedDecisionSummary>,
    pub(super) candidate_search: Option<WorthQueryCandidateSearchSummary>,
    pub(super) transformation: Option<WorthQueryTransformationSummary>,
}

impl WorthQueryDomainEvidenceCore {
    pub fn counters(&self) -> &[WorthQueryAdmittedStructuralCounter] {
        &self.counters
    }

    pub fn decisions(&self) -> &[WorthQueryAdmittedDecisionSummary] {
        &self.decisions
    }

    pub fn candidate_search(&self) -> Option<&WorthQueryCandidateSearchSummary> {
        self.candidate_search.as_ref()
    }

    pub fn transformation(&self) -> Option<&WorthQueryTransformationSummary> {
        self.transformation.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryAdmittedDomainEvidenceSidecar<T> {
    NotApplicable,
    Omitted,
    DigestOnly { digest: String },
    PartiallyMaterialized { digest: String, records: Vec<T> },
    Materialized { digest: String, records: Vec<T> },
}

impl<T> WorthQueryAdmittedDomainEvidenceSidecar<T> {
    pub fn digest(&self) -> Option<&str> {
        match self {
            Self::NotApplicable | Self::Omitted => None,
            Self::DigestOnly { digest }
            | Self::PartiallyMaterialized { digest, .. }
            | Self::Materialized { digest, .. } => Some(digest),
        }
    }

    pub fn records(&self) -> Option<&[T]> {
        match self {
            Self::PartiallyMaterialized { records, .. } | Self::Materialized { records, .. } => {
                Some(records)
            }
            _ => None,
        }
    }
}
