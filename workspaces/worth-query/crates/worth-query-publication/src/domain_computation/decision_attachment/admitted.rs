use worth_foundational::facade::RetentionDeliveryProfile;
use worth_query_installation::facade::{
    WorthQueryArtifactClassification, WorthQueryArtifactDeletionPosture,
    WorthQueryArtifactLegalHoldPosture, WorthQueryArtifactRedactionPosture,
    WorthQueryDecisionSchema, WorthQueryStructuralCounterSchema,
};

use super::{
    WorthQueryCandidateRecord, WorthQueryCandidateSearchSummary, WorthQueryDecisionRecord,
    WorthQueryDecisionSummaryCounts, WorthQueryTransformationRecord,
    WorthQueryTransformationSummary,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainEvidenceAuthorityPosture {
    DescriptiveOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEvidenceBinding {
    operation_identity: String,
    binding_identity: String,
    run_identity: Option<String>,
    stage_identity: Option<String>,
    basis_identity: String,
    execution_snapshot_identity: String,
    output_occurrence_identity: String,
}

impl WorthQueryDomainEvidenceBinding {
    pub(super) fn from_execution(
        binding: &worth_query_execution::facade::domain_computation::WorthQueryDomainEvidenceExecutionBinding,
    ) -> Self {
        Self {
            operation_identity: binding.operation_identity().to_owned(),
            binding_identity: binding.binding_identity().to_owned(),
            run_identity: binding.run_identity().map(str::to_owned),
            stage_identity: binding.stage_identity().map(str::to_owned),
            basis_identity: binding.basis_identity().to_owned(),
            execution_snapshot_identity: binding.execution_snapshot_identity().to_owned(),
            output_occurrence_identity: binding.output_occurrence_identity().to_owned(),
        }
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn run_identity(&self) -> Option<&str> {
        self.run_identity.as_deref()
    }

    pub fn stage_identity(&self) -> Option<&str> {
        self.stage_identity.as_deref()
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn execution_snapshot_identity(&self) -> &str {
        &self.execution_snapshot_identity
    }

    pub fn output_occurrence_identity(&self) -> &str {
        &self.output_occurrence_identity
    }
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

/// Immutable, installation-bound descriptive evidence. This value explains an
/// admitted execution receipt; it is not operation, artifact, repair, or
/// publication authority.
#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedDomainEvidence {
    contract_identity: String,
    binding: WorthQueryDomainEvidenceBinding,
    governance: WorthQueryDomainEvidenceGovernance,
    core: WorthQueryDomainEvidenceCore,
    counter_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryAdmittedStructuralCounter>,
    decision_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryDecisionRecord>,
    candidate_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryCandidateRecord>,
    transformation_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryTransformationRecord>,
    identity: String,
}

pub(super) struct WorthQueryAdmittedDomainEvidenceParts {
    pub(super) contract_identity: String,
    pub(super) binding: WorthQueryDomainEvidenceBinding,
    pub(super) governance: WorthQueryDomainEvidenceGovernance,
    pub(super) core: WorthQueryDomainEvidenceCore,
    pub(super) counter_sidecar:
        WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryAdmittedStructuralCounter>,
    pub(super) decision_sidecar: WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryDecisionRecord>,
    pub(super) candidate_sidecar:
        WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryCandidateRecord>,
    pub(super) transformation_sidecar:
        WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryTransformationRecord>,
    pub(super) identity: String,
}

impl WorthQueryAdmittedDomainEvidence {
    pub(super) fn from_parts(parts: WorthQueryAdmittedDomainEvidenceParts) -> Self {
        Self {
            contract_identity: parts.contract_identity,
            binding: parts.binding,
            governance: parts.governance,
            core: parts.core,
            counter_sidecar: parts.counter_sidecar,
            decision_sidecar: parts.decision_sidecar,
            candidate_sidecar: parts.candidate_sidecar,
            transformation_sidecar: parts.transformation_sidecar,
            identity: parts.identity,
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn contract_identity(&self) -> &str {
        &self.contract_identity
    }

    pub fn binding(&self) -> &WorthQueryDomainEvidenceBinding {
        &self.binding
    }

    pub fn governance(&self) -> &WorthQueryDomainEvidenceGovernance {
        &self.governance
    }

    pub fn core(&self) -> &WorthQueryDomainEvidenceCore {
        &self.core
    }

    pub fn counter_sidecar(
        &self,
    ) -> &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryAdmittedStructuralCounter> {
        &self.counter_sidecar
    }

    pub fn decision_sidecar(
        &self,
    ) -> &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryDecisionRecord> {
        &self.decision_sidecar
    }

    pub fn candidate_sidecar(
        &self,
    ) -> &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryCandidateRecord> {
        &self.candidate_sidecar
    }

    pub fn transformation_sidecar(
        &self,
    ) -> &WorthQueryAdmittedDomainEvidenceSidecar<WorthQueryTransformationRecord> {
        &self.transformation_sidecar
    }

    pub const fn authority_posture(&self) -> WorthQueryDomainEvidenceAuthorityPosture {
        WorthQueryDomainEvidenceAuthorityPosture::DescriptiveOnly
    }
}
