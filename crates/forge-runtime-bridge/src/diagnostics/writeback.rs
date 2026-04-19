use crate::writeback::{
    BridgeMappedWritebackFamilyInput, BridgeValidatedWritebackCandidate,
    BridgeWritebackAuthorityOutcome, BridgeWritebackExecutionRecord,
    BridgeWritebackFamilyAdmissionRecord, BridgeWritebackLoopPreventionReport,
    BridgeWritebackMapperEnvelope, BridgeWritebackMapperRecord, BridgeWritebackReplayBundle,
    BridgeWritebackReplayRecord, BridgeWritebackStrategyCompatibilityReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackAdmissionExplanation {
    record_identity: String,
    declaration_identity: String,
    contract_digest: String,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: crate::writeback::BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: String,
    family_basis_digest: String,
    strategy_basis_digest: String,
    lowered_policy_digest: String,
    diagnostics_tier: crate::policy::BridgeDiagnosticsTier,
    replay_artifacts_permitted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackCandidateExplanation {
    candidate_digest: String,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: String,
    retry_disposition: crate::writeback::BridgeWritebackRetryDisposition,
    loop_prevention_digest: String,
    strategy_compatibility_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackLoopPreventionExplanation {
    loop_prevention_digest: String,
    disposition: crate::writeback::BridgeWritebackLoopDisposition,
    current_feedback_provenance_digest: String,
    current_causality_digest: String,
    incoming_feedback_provenance_digest: Option<String>,
    incoming_feedback_causality_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackStrategyCompatibilityExplanation {
    compatibility_digest: String,
    disposition: crate::writeback::BridgeWritebackStrategyCompatibilityDisposition,
    contract_digest: String,
    strategy_basis_digest: String,
    effect_digest: String,
    idempotence_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackOutcomeExplanation {
    outcome_digest: String,
    outcome_class: crate::writeback::BridgeWritebackOutcomeClass,
    idempotence_digest: String,
    authoritative_artifact_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackReplayExplanation {
    replay_bundle_digest: String,
    semantic_digest: String,
    contract_digest: String,
    effect_digest: String,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: String,
    causality_digest: String,
    idempotence_digest: String,
    lowered_policy_digest: String,
    retry_disposition: crate::writeback::BridgeWritebackRetryDisposition,
    outcome_digest: String,
    outcome_class: crate::writeback::BridgeWritebackOutcomeClass,
    authoritative_artifact_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackExecutionExplanation {
    record_identity: String,
    contract_digest: String,
    derived_effect_digest: String,
    proposed_effect_digest: String,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    causality_digest: String,
    idempotence_digest: String,
    loop_prevention_digest: String,
    strategy_compatibility_digest: String,
    mapper_record_digest: Option<String>,
    candidate_digest: Option<String>,
    outcome_digest: Option<String>,
    outcome_class: Option<crate::writeback::BridgeWritebackOutcomeClass>,
    replay_bundle_digest: Option<String>,
    request_digest: Option<String>,
    receipt_digest: Option<String>,
    failure_class: Option<crate::writeback::BridgeWritebackFailureClass>,
    failure_digest: Option<String>,
    counter_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackMapperExplanation {
    envelope_digest: String,
    mapped_input_digest: String,
    record_identity: String,
    witness_digest: String,
    candidate_digest: String,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: crate::writeback::BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: String,
    causality_digest: String,
    proposed_effect_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackMapperEnvelopeExplanation {
    envelope_identity: String,
    contract_digest: String,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: crate::writeback::BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: String,
    causality_digest: String,
    domain_payload_digest: String,
    domain_evidence_digest: String,
    envelope_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMappedWritebackFamilyInputExplanation {
    mapped_input_identity: String,
    mapper_envelope_digest: String,
    contract_digest: String,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: crate::writeback::BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: String,
    causality_digest: String,
    domain_payload_digest: String,
    domain_evidence_digest: String,
    mapped_input_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackReplayRecordExplanation {
    record_identity: String,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    expected_replay_digest: String,
    replayed_replay_digest: String,
    expected_semantic_digest: String,
    replayed_semantic_digest: String,
    expected_causality_digest: String,
    replayed_causality_digest: String,
    failure_class: Option<crate::writeback::BridgeWritebackFailureClass>,
    counter_digest: String,
}

impl BridgeWritebackCandidateExplanation {
    pub fn from_candidate(candidate: &BridgeValidatedWritebackCandidate) -> Self {
        Self {
            candidate_digest: candidate.digest().to_owned(),
            family_kind: candidate.family_kind(),
            strategy_class: candidate.strategy_class(),
            strategy_descriptor_digest: candidate.strategy_descriptor_digest().to_owned(),
            retry_disposition: candidate.retry_disposition(),
            loop_prevention_digest: candidate.loop_prevention_digest().to_owned(),
            strategy_compatibility_digest: candidate.strategy_compatibility_digest().to_owned(),
        }
    }

    pub fn candidate_digest(&self) -> &str {
        &self.candidate_digest
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        &self.strategy_descriptor_digest
    }

    pub fn retry_disposition(&self) -> crate::writeback::BridgeWritebackRetryDisposition {
        self.retry_disposition
    }

    pub fn loop_prevention_digest(&self) -> &str {
        &self.loop_prevention_digest
    }

    pub fn strategy_compatibility_digest(&self) -> &str {
        &self.strategy_compatibility_digest
    }
}

impl BridgeWritebackAdmissionExplanation {
    pub fn from_record(record: &BridgeWritebackFamilyAdmissionRecord) -> Self {
        Self {
            record_identity: record.record_identity().as_str().to_owned(),
            declaration_identity: record.declaration_identity().to_owned(),
            contract_digest: record.contract_digest().to_owned(),
            family_kind: record.family_kind(),
            effect_class: record.effect_class(),
            strategy_class: record.strategy_class(),
            strategy_descriptor_digest: record.strategy_descriptor_digest().to_owned(),
            family_basis_digest: record.family_basis_digest().to_owned(),
            strategy_basis_digest: record.strategy_basis_digest().to_owned(),
            lowered_policy_digest: record.lowered_policy_digest().to_owned(),
            diagnostics_tier: record.diagnostics_tier(),
            replay_artifacts_permitted: record.replay_artifacts_permitted(),
        }
    }

    pub fn record_identity(&self) -> &str {
        &self.record_identity
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }
}

impl BridgeWritebackLoopPreventionExplanation {
    pub fn from_report(report: &BridgeWritebackLoopPreventionReport) -> Self {
        Self {
            loop_prevention_digest: report.digest().to_owned(),
            disposition: report.disposition(),
            current_feedback_provenance_digest: report
                .current_feedback_provenance_digest()
                .to_owned(),
            current_causality_digest: report.current_causality_digest().to_owned(),
            incoming_feedback_provenance_digest: report
                .incoming_feedback_provenance_digest()
                .map(str::to_owned),
            incoming_feedback_causality_digest: report
                .incoming_feedback_causality_digest()
                .map(str::to_owned),
        }
    }

    pub fn loop_prevention_digest(&self) -> &str {
        &self.loop_prevention_digest
    }

    pub fn disposition(&self) -> crate::writeback::BridgeWritebackLoopDisposition {
        self.disposition
    }
}

impl BridgeWritebackStrategyCompatibilityExplanation {
    pub fn from_report(report: &BridgeWritebackStrategyCompatibilityReport) -> Self {
        Self {
            compatibility_digest: report.digest().to_owned(),
            disposition: report.disposition(),
            contract_digest: report.contract_digest().to_owned(),
            strategy_basis_digest: report.strategy_basis_digest().to_owned(),
            effect_digest: report.effect_digest().to_owned(),
            idempotence_digest: report.idempotence_digest().to_owned(),
        }
    }

    pub fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub fn disposition(&self) -> crate::writeback::BridgeWritebackStrategyCompatibilityDisposition {
        self.disposition
    }
}

impl BridgeWritebackOutcomeExplanation {
    pub fn from_outcome(outcome: &BridgeWritebackAuthorityOutcome) -> Self {
        Self {
            outcome_digest: outcome.digest().to_owned(),
            outcome_class: outcome.outcome_class(),
            idempotence_digest: outcome.idempotence_digest().to_owned(),
            authoritative_artifact_digest: outcome.authoritative_artifact_digest().to_owned(),
        }
    }

    pub fn outcome_digest(&self) -> &str {
        &self.outcome_digest
    }

    pub fn outcome_class(&self) -> crate::writeback::BridgeWritebackOutcomeClass {
        self.outcome_class
    }
}

impl BridgeWritebackReplayExplanation {
    pub fn from_bundle(bundle: &BridgeWritebackReplayBundle) -> Self {
        Self {
            replay_bundle_digest: bundle.digest().to_owned(),
            semantic_digest: bundle.semantic_digest().to_owned(),
            contract_digest: bundle.contract_digest().to_owned(),
            effect_digest: bundle.effect_digest().to_owned(),
            family_kind: bundle.family_kind(),
            strategy_class: bundle.strategy_class(),
            strategy_descriptor_digest: bundle.strategy_descriptor_digest().to_owned(),
            causality_digest: bundle.causality_digest().to_owned(),
            idempotence_digest: bundle.idempotence_digest().to_owned(),
            lowered_policy_digest: bundle.lowered_policy_digest().to_owned(),
            retry_disposition: bundle.retry_disposition(),
            outcome_digest: bundle.outcome_digest().to_owned(),
            outcome_class: bundle.outcome_class(),
            authoritative_artifact_digest: bundle.authoritative_artifact_digest().to_owned(),
        }
    }

    pub fn replay_bundle_digest(&self) -> &str {
        &self.replay_bundle_digest
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn semantic_digest(&self) -> &str {
        &self.semantic_digest
    }

    pub fn causality_digest(&self) -> &str {
        &self.causality_digest
    }

    pub fn retry_disposition(&self) -> crate::writeback::BridgeWritebackRetryDisposition {
        self.retry_disposition
    }

    pub fn outcome_class(&self) -> crate::writeback::BridgeWritebackOutcomeClass {
        self.outcome_class
    }
}

impl BridgeWritebackExecutionExplanation {
    pub fn from_record(record: &BridgeWritebackExecutionRecord) -> Self {
        Self {
            record_identity: record.record_identity().as_str().to_owned(),
            contract_digest: record.contract_digest().to_owned(),
            derived_effect_digest: record.derived_effect_digest().to_owned(),
            proposed_effect_digest: record.proposed_effect_digest().to_owned(),
            family_kind: record.family_kind(),
            strategy_class: record.strategy_class(),
            causality_digest: record.causality_digest().to_owned(),
            idempotence_digest: record.idempotence_digest().to_owned(),
            loop_prevention_digest: record.loop_prevention_digest().to_owned(),
            strategy_compatibility_digest: record.strategy_compatibility_digest().to_owned(),
            mapper_record_digest: record.mapper_record_digest().map(str::to_owned),
            candidate_digest: record.candidate_digest().map(str::to_owned),
            outcome_digest: record.outcome_digest().map(str::to_owned),
            outcome_class: record.outcome_class(),
            replay_bundle_digest: record.replay_bundle_digest().map(str::to_owned),
            request_digest: record.request_digest().map(str::to_owned),
            receipt_digest: record.receipt_digest().map(str::to_owned),
            failure_class: record.failure_class(),
            failure_digest: record.failure_digest().map(str::to_owned),
            counter_digest: record.counters().digest().to_owned(),
        }
    }

    pub fn record_identity(&self) -> &str {
        &self.record_identity
    }

    pub fn failure_class(&self) -> Option<crate::writeback::BridgeWritebackFailureClass> {
        self.failure_class
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn idempotence_digest(&self) -> &str {
        &self.idempotence_digest
    }

    pub fn loop_prevention_digest(&self) -> &str {
        &self.loop_prevention_digest
    }

    pub fn strategy_compatibility_digest(&self) -> &str {
        &self.strategy_compatibility_digest
    }

    pub fn mapper_record_digest(&self) -> Option<&str> {
        self.mapper_record_digest.as_deref()
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }
}

impl BridgeWritebackMapperExplanation {
    pub fn from_record(record: &BridgeWritebackMapperRecord) -> Self {
        Self {
            envelope_digest: record.mapper_envelope_digest().to_owned(),
            mapped_input_digest: record.mapped_input_digest().to_owned(),
            record_identity: record.record_identity().as_str().to_owned(),
            witness_digest: record.witness_digest().to_owned(),
            candidate_digest: record.candidate_digest().to_owned(),
            family_kind: record.family_kind(),
            effect_class: record.effect_class(),
            strategy_class: record.strategy_class(),
            strategy_descriptor_digest: record.strategy_descriptor_digest().to_owned(),
            causality_digest: record.causality_digest().to_owned(),
            proposed_effect_digest: record.proposed_effect_digest().to_owned(),
        }
    }

    pub fn record_identity(&self) -> &str {
        &self.record_identity
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn mapped_input_digest(&self) -> &str {
        &self.mapped_input_digest
    }

    pub fn witness_digest(&self) -> &str {
        &self.witness_digest
    }

    pub fn candidate_digest(&self) -> &str {
        &self.candidate_digest
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        &self.strategy_descriptor_digest
    }

    pub fn causality_digest(&self) -> &str {
        &self.causality_digest
    }

    pub fn proposed_effect_digest(&self) -> &str {
        &self.proposed_effect_digest
    }
}

impl BridgeWritebackMapperEnvelopeExplanation {
    pub fn from_envelope(envelope: &BridgeWritebackMapperEnvelope) -> Self {
        Self {
            envelope_identity: envelope.envelope_identity().as_str().to_owned(),
            contract_digest: envelope.contract_digest().to_owned(),
            family_kind: envelope.family_kind(),
            effect_class: envelope.effect_class(),
            strategy_class: envelope.strategy_class(),
            strategy_descriptor_digest: envelope.strategy_descriptor_digest().to_owned(),
            causality_digest: envelope.causality_digest().to_owned(),
            domain_payload_digest: envelope.domain_payload_digest().to_owned(),
            domain_evidence_digest: envelope.domain_evidence_digest().to_owned(),
            envelope_digest: envelope.digest().to_owned(),
        }
    }

    pub fn envelope_identity(&self) -> &str {
        &self.envelope_identity
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        &self.strategy_descriptor_digest
    }

    pub fn causality_digest(&self) -> &str {
        &self.causality_digest
    }

    pub fn domain_payload_digest(&self) -> &str {
        &self.domain_payload_digest
    }

    pub fn domain_evidence_digest(&self) -> &str {
        &self.domain_evidence_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }
}

impl BridgeMappedWritebackFamilyInputExplanation {
    pub fn from_mapped_input(mapped_input: &BridgeMappedWritebackFamilyInput) -> Self {
        Self {
            mapped_input_identity: mapped_input.mapped_input_identity().as_str().to_owned(),
            mapper_envelope_digest: mapped_input.mapper_envelope_digest().to_owned(),
            contract_digest: mapped_input.contract_digest().to_owned(),
            family_kind: mapped_input.family_kind(),
            effect_class: mapped_input.effect_class(),
            strategy_class: mapped_input.strategy_class(),
            strategy_descriptor_digest: mapped_input.strategy_descriptor_digest().to_owned(),
            causality_digest: mapped_input.causality_digest().to_owned(),
            domain_payload_digest: mapped_input.domain_payload_digest().to_owned(),
            domain_evidence_digest: mapped_input.domain_evidence_digest().to_owned(),
            mapped_input_digest: mapped_input.digest().to_owned(),
        }
    }

    pub fn mapped_input_identity(&self) -> &str {
        &self.mapped_input_identity
    }

    pub fn mapper_envelope_digest(&self) -> &str {
        &self.mapper_envelope_digest
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        &self.strategy_descriptor_digest
    }

    pub fn causality_digest(&self) -> &str {
        &self.causality_digest
    }

    pub fn domain_payload_digest(&self) -> &str {
        &self.domain_payload_digest
    }

    pub fn domain_evidence_digest(&self) -> &str {
        &self.domain_evidence_digest
    }

    pub fn mapped_input_digest(&self) -> &str {
        &self.mapped_input_digest
    }
}

impl BridgeWritebackReplayRecordExplanation {
    pub fn from_record(record: &BridgeWritebackReplayRecord) -> Self {
        Self {
            record_identity: record.record_identity().as_str().to_owned(),
            family_kind: record.family_kind(),
            expected_replay_digest: record.expected_replay_digest().to_owned(),
            replayed_replay_digest: record.replayed_replay_digest().to_owned(),
            expected_semantic_digest: record.expected_semantic_digest().to_owned(),
            replayed_semantic_digest: record.replayed_semantic_digest().to_owned(),
            expected_causality_digest: record.expected_causality_digest().to_owned(),
            replayed_causality_digest: record.replayed_causality_digest().to_owned(),
            failure_class: record.failure_class(),
            counter_digest: record.counters().digest().to_owned(),
        }
    }

    pub fn record_identity(&self) -> &str {
        &self.record_identity
    }

    pub fn failure_class(&self) -> Option<crate::writeback::BridgeWritebackFailureClass> {
        self.failure_class
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn expected_causality_digest(&self) -> &str {
        &self.expected_causality_digest
    }

    pub fn replayed_causality_digest(&self) -> &str {
        &self.replayed_causality_digest
    }
}
