use crate::diagnostics::{BridgeCanonicalMergeRecord, BridgeMergeExplanation};
use crate::identity::{BridgeIdentity, MergeContractIdentityTag};
use crate::merge::MergeHistoryDeclarationIdentity;

use super::counter_snapshot::MergeHarnessCounterSnapshot;
use super::declaration_identity;
use super::diagnostics_digest::merge_diagnostics_digest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::adapter::adapter_impl) struct MergeHarnessCertificationBundle {
    merge_history_digest: String,
    merge_contract_identity: BridgeIdentity<MergeContractIdentityTag>,
    merge_declaration_identity: MergeHistoryDeclarationIdentity,
    ontology_mapping_report: MergeOntologyMappingReport,
    support_matrix: MergeSupportMatrix,
    denial_stage_report: MergeDenialStageReport,
    result_bundle_digest: String,
    replay_digest: Option<String>,
    failure_digest: Option<String>,
    diagnostics_digest: String,
    record_identity: crate::diagnostics::BridgeMergeRecordIdentity,
    record_evidence: MergeRecordEvidence,
    counter_snapshot: MergeHarnessCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MergeOntologyMappingReport {
    bridge_class: crate::facade::BridgeMergeConsumptionClass,
    ontology_mapping_digest: String,
    ontology_version: String,
    schema_policy_descriptor_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MergeSupportMatrix {
    outcome_class: crate::facade::BridgeMergeRoutingOutcomeClass,
    continuity_published: bool,
    remap_published: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MergeDenialStageReport {
    blocked_stage: Option<crate::facade::BridgeMergePrecedenceStage>,
    denial_class: Option<crate::facade::BridgeMergeDenialClass>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MergeRecordEvidence {
    record_identity: crate::diagnostics::BridgeMergeRecordIdentity,
    merge_contract_identity: BridgeIdentity<MergeContractIdentityTag>,
    merge_declaration_identity: MergeHistoryDeclarationIdentity,
    bundle_digest: String,
    lowered_digest: String,
    reduced_digest: String,
    continuity_digest: Option<String>,
    remap_digest: Option<String>,
    explanation_digest: String,
    outcome_class: crate::facade::BridgeMergeRoutingOutcomeClass,
    blocked_stage: Option<crate::facade::BridgeMergePrecedenceStage>,
    denial_class: Option<crate::facade::BridgeMergeDenialClass>,
}

impl MergeHarnessCertificationBundle {
    pub(super) fn from_execution(
        contract: crate::facade::AdmittedMergeHistoryContract,
        bundle: crate::facade::MergeReplayCertificationBundle,
        record: BridgeCanonicalMergeRecord,
        explanation: BridgeMergeExplanation,
        replayed: Option<crate::facade::BridgeMergeReplaySummary>,
    ) -> Self {
        let counter_snapshot = replayed
            .as_ref()
            .map(|replayed| replayed.reduced_routing_artifact().counters())
            .unwrap_or_else(|| bundle.reduced_routing_artifact().counters());
        let counter_snapshot = MergeHarnessCounterSnapshot::from_counters(counter_snapshot);
        let failure_digest =
            if bundle.continuity_artifact().is_none() && bundle.remap_artifact().is_none() {
                Some(bundle.explanation_artifact().digest().to_string())
            } else {
                None
            };

        Self {
            merge_history_digest: contract.digest().to_string(),
            merge_contract_identity: contract.contract_identity().clone(),
            merge_declaration_identity: declaration_identity(&contract).clone(),
            ontology_mapping_report: MergeOntologyMappingReport::from_contract(&contract),
            support_matrix: MergeSupportMatrix::from_bundle(&bundle),
            denial_stage_report: MergeDenialStageReport::from_explanation(&explanation),
            result_bundle_digest: bundle.digest().to_string(),
            replay_digest: replayed.map(|replayed| replayed.digest().to_string()),
            failure_digest,
            diagnostics_digest: merge_diagnostics_digest(&explanation),
            record_identity: record.record_identity().clone(),
            record_evidence: MergeRecordEvidence::from_record_parts(
                &contract,
                &bundle,
                &record,
                &explanation,
            ),
            counter_snapshot,
        }
    }

    pub(super) fn merge_history_digest(&self) -> &str {
        &self.merge_history_digest
    }

    pub(super) fn merge_contract_identity(&self) -> &BridgeIdentity<MergeContractIdentityTag> {
        &self.merge_contract_identity
    }

    pub(super) fn merge_declaration_identity(&self) -> &MergeHistoryDeclarationIdentity {
        &self.merge_declaration_identity
    }

    pub(super) fn ontology_mapping_report(&self) -> &MergeOntologyMappingReport {
        &self.ontology_mapping_report
    }

    pub(super) fn support_matrix(&self) -> &MergeSupportMatrix {
        &self.support_matrix
    }

    pub(super) fn denial_stage_report(&self) -> &MergeDenialStageReport {
        &self.denial_stage_report
    }

    pub(super) fn result_bundle_digest(&self) -> &str {
        &self.result_bundle_digest
    }

    pub(super) fn replay_digest(&self) -> Option<&str> {
        self.replay_digest.as_deref()
    }

    pub(super) fn failure_digest(&self) -> Option<&str> {
        self.failure_digest.as_deref()
    }

    pub(super) fn diagnostics_digest(&self) -> &str {
        &self.diagnostics_digest
    }

    pub(super) fn record_identity(&self) -> &crate::diagnostics::BridgeMergeRecordIdentity {
        &self.record_identity
    }

    pub(super) fn record_evidence(&self) -> &MergeRecordEvidence {
        &self.record_evidence
    }

    pub(super) fn counter_snapshot(&self) -> MergeHarnessCounterSnapshot {
        self.counter_snapshot
    }
}

impl MergeOntologyMappingReport {
    fn from_contract(contract: &crate::facade::AdmittedMergeHistoryContract) -> Self {
        let declaration = contract.validated_declaration().declaration();
        Self {
            bridge_class: declaration.bridge_class(),
            ontology_mapping_digest: declaration.ontology_mapping().digest().to_string(),
            ontology_version: declaration.authority_basis().ontology_version().to_string(),
            schema_policy_descriptor_version: declaration
                .authority_basis()
                .schema_policy_descriptor_version()
                .to_string(),
        }
    }

    pub(super) fn bridge_class(&self) -> crate::facade::BridgeMergeConsumptionClass {
        self.bridge_class
    }

    pub(super) fn ontology_mapping_digest(&self) -> &str {
        &self.ontology_mapping_digest
    }

    pub(super) fn ontology_version(&self) -> &str {
        &self.ontology_version
    }

    pub(super) fn schema_policy_descriptor_version(&self) -> &str {
        &self.schema_policy_descriptor_version
    }
}

impl MergeSupportMatrix {
    fn from_bundle(bundle: &crate::facade::MergeReplayCertificationBundle) -> Self {
        Self {
            outcome_class: bundle.reduced_routing_artifact().outcome_class(),
            continuity_published: bundle.continuity_artifact().is_some(),
            remap_published: bundle.remap_artifact().is_some(),
        }
    }

    pub(super) fn outcome_class(&self) -> crate::facade::BridgeMergeRoutingOutcomeClass {
        self.outcome_class
    }

    pub(super) fn continuity_published(&self) -> bool {
        self.continuity_published
    }

    pub(super) fn remap_published(&self) -> bool {
        self.remap_published
    }
}

impl MergeDenialStageReport {
    fn from_explanation(explanation: &BridgeMergeExplanation) -> Self {
        Self {
            blocked_stage: explanation.blocked_stage(),
            denial_class: explanation.denial_class(),
        }
    }

    pub(super) fn blocked_stage(&self) -> Option<crate::facade::BridgeMergePrecedenceStage> {
        self.blocked_stage
    }

    pub(super) fn denial_class(&self) -> Option<crate::facade::BridgeMergeDenialClass> {
        self.denial_class
    }
}

impl MergeRecordEvidence {
    fn from_record_parts(
        contract: &crate::facade::AdmittedMergeHistoryContract,
        bundle: &crate::facade::MergeReplayCertificationBundle,
        record: &BridgeCanonicalMergeRecord,
        explanation: &BridgeMergeExplanation,
    ) -> Self {
        Self {
            record_identity: record.record_identity().clone(),
            merge_contract_identity: contract.contract_identity().clone(),
            merge_declaration_identity: declaration_identity(contract).clone(),
            bundle_digest: bundle.digest().to_string(),
            lowered_digest: explanation.lowered_digest().to_string(),
            reduced_digest: explanation.reduced_digest().to_string(),
            continuity_digest: explanation.continuity_digest().map(str::to_string),
            remap_digest: explanation.remap_digest().map(str::to_string),
            explanation_digest: explanation.explanation_digest().to_string(),
            outcome_class: explanation.outcome_class(),
            blocked_stage: explanation.blocked_stage(),
            denial_class: explanation.denial_class(),
        }
    }

    pub(super) fn record_identity(&self) -> &crate::diagnostics::BridgeMergeRecordIdentity {
        &self.record_identity
    }

    pub(super) fn merge_contract_identity(&self) -> &BridgeIdentity<MergeContractIdentityTag> {
        &self.merge_contract_identity
    }

    pub(super) fn merge_declaration_identity(&self) -> &MergeHistoryDeclarationIdentity {
        &self.merge_declaration_identity
    }

    pub(super) fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub(super) fn lowered_digest(&self) -> &str {
        &self.lowered_digest
    }

    pub(super) fn reduced_digest(&self) -> &str {
        &self.reduced_digest
    }

    pub(super) fn continuity_digest(&self) -> Option<&str> {
        self.continuity_digest.as_deref()
    }

    pub(super) fn remap_digest(&self) -> Option<&str> {
        self.remap_digest.as_deref()
    }

    pub(super) fn explanation_digest(&self) -> &str {
        &self.explanation_digest
    }

    pub(super) fn outcome_class(&self) -> crate::facade::BridgeMergeRoutingOutcomeClass {
        self.outcome_class
    }

    pub(super) fn blocked_stage(&self) -> Option<crate::facade::BridgeMergePrecedenceStage> {
        self.blocked_stage
    }

    pub(super) fn denial_class(&self) -> Option<crate::facade::BridgeMergeDenialClass> {
        self.denial_class
    }
}
