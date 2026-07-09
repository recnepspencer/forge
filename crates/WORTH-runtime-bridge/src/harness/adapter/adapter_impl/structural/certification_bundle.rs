use super::counter_snapshot::StructuralHarnessCounterSnapshot;
use super::*;
use crate::identity::{
    BridgeIdentity, StructuralBranchComparisonRecordIdentityTag, StructuralContractIdentityTag,
};
use crate::structural::StructuralIdentityDeclarationIdentity;
use crate::structural::StructuralMatchOutcomeClass;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StructuralHarnessSummary {
    pub(super) structural_declaration_identity: StructuralIdentityDeclarationIdentity,
    pub(super) structural_contract_identity: BridgeIdentity<StructuralContractIdentityTag>,
    pub(super) structural_match_digest: Option<String>,
    pub(super) structural_reuse_digest: Option<String>,
    pub(super) branch_compare_digest: Option<String>,
    pub(super) replay_digest: Option<String>,
    pub(super) diagnostics_digest: String,
    pub(super) failure_digest: Option<String>,
    pub(super) outcome_class: StructuralMatchOutcomeClass,
    pub(super) counter_snapshot: StructuralHarnessCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StructuralHarnessCertificationBundle {
    pub(super) structural_match_digest: Option<String>,
    pub(super) ambiguity_report: Option<StructuralAmbiguityReport>,
    pub(super) remap_artifact_digest: Option<String>,
    pub(super) failure_digest: Option<String>,
    pub(super) structural_reuse_digest: Option<String>,
    pub(super) identity_separation_report: Option<StructuralIdentitySeparationReport>,
    pub(super) replay_digest: Option<String>,
    pub(super) diagnostics_digest: String,
    pub(super) branch_compare_digest: Option<String>,
    pub(super) structural_diff_report: Option<StructuralDiffReport>,
    pub(super) counter_snapshot: StructuralHarnessCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StructuralAmbiguityReport {
    pub(super) outcome_class: StructuralMatchOutcomeClass,
    pub(super) retained_candidates: StructuralRetainedCandidateSet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StructuralIdentitySeparationReport {
    pub(super) declaration_identity: StructuralIdentityDeclarationIdentity,
    pub(super) outcome_class: StructuralMatchOutcomeClass,
    pub(super) retained_candidates: StructuralRetainedCandidateSet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StructuralDiffReport {
    pub(super) record_identity: BridgeIdentity<StructuralBranchComparisonRecordIdentityTag>,
    pub(super) branch_diff_count: usize,
    pub(super) retained_candidates: StructuralRetainedCandidateSet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StructuralRetainedCandidate {
    identity: String,
}

impl StructuralRetainedCandidate {
    fn from_candidate(candidate: &impl ToString) -> Self {
        Self {
            identity: candidate.to_string(),
        }
    }

    pub(super) fn identity(&self) -> &str {
        &self.identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct StructuralRetainedCandidateSet {
    candidates: Vec<StructuralRetainedCandidate>,
}

impl StructuralRetainedCandidateSet {
    fn from_reduced_match_set(reduced: &ReducedStructuralMatchSet) -> Self {
        Self {
            candidates: reduced
                .retained_candidates()
                .iter()
                .map(StructuralRetainedCandidate::from_candidate)
                .collect(),
        }
    }

    pub(super) fn len(&self) -> usize {
        self.candidates.len()
    }

    pub(super) fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }

    pub(super) fn candidates(&self) -> &[StructuralRetainedCandidate] {
        &self.candidates
    }
}

impl StructuralHarnessSummary {
    pub(super) fn from_execution(execution: &StructuralHarnessExecution) -> Self {
        match execution {
            StructuralHarnessExecution::Remap {
                contract,
                planned,
                reduced,
                artifact,
                record,
            } => Self {
                structural_declaration_identity: declaration_identity(contract).clone(),
                structural_contract_identity: contract.contract_identity().clone(),
                structural_match_digest: Some(planned.digest().to_string()),
                structural_reuse_digest: Some(artifact.digest().to_string()),
                branch_compare_digest: None,
                replay_digest: None,
                diagnostics_digest: diagnostics_digest::remap_diagnostics_digest(record),
                failure_digest: None,
                outcome_class: reduced.outcome_class(),
                counter_snapshot: StructuralHarnessCounterSnapshot::from_counters(
                    record.counters(),
                    false,
                ),
            },
            StructuralHarnessExecution::RemapReplay {
                contract,
                planned,
                reduced,
                artifact,
                record,
                replayed,
            } => Self {
                structural_declaration_identity: declaration_identity(contract).clone(),
                structural_contract_identity: contract.contract_identity().clone(),
                structural_match_digest: Some(planned.digest().to_string()),
                structural_reuse_digest: Some(artifact.digest().to_string()),
                branch_compare_digest: None,
                replay_digest: Some(replayed.digest().to_string()),
                diagnostics_digest: diagnostics_digest::remap_diagnostics_digest(record),
                failure_digest: None,
                outcome_class: reduced.outcome_class(),
                counter_snapshot: StructuralHarnessCounterSnapshot::from_counters(
                    record.counters(),
                    true,
                ),
            },
            StructuralHarnessExecution::Branch {
                contract,
                reduced,
                artifact,
                record,
                ..
            } => Self {
                structural_declaration_identity: declaration_identity(contract).clone(),
                structural_contract_identity: contract.contract_identity().clone(),
                structural_match_digest: None,
                structural_reuse_digest: None,
                branch_compare_digest: Some(artifact.digest().to_string()),
                replay_digest: None,
                diagnostics_digest: diagnostics_digest::branch_diagnostics_digest(record),
                failure_digest: None,
                outcome_class: reduced.outcome_class(),
                counter_snapshot: StructuralHarnessCounterSnapshot::from_counters(
                    record.counters(),
                    false,
                ),
            },
            StructuralHarnessExecution::BranchReplay {
                contract,
                reduced,
                artifact,
                record,
                replayed,
                ..
            } => Self {
                structural_declaration_identity: declaration_identity(contract).clone(),
                structural_contract_identity: contract.contract_identity().clone(),
                structural_match_digest: None,
                structural_reuse_digest: None,
                branch_compare_digest: Some(artifact.digest().to_string()),
                replay_digest: Some(replayed.digest().to_string()),
                diagnostics_digest: diagnostics_digest::branch_diagnostics_digest(record),
                failure_digest: None,
                outcome_class: reduced.outcome_class(),
                counter_snapshot: StructuralHarnessCounterSnapshot::from_counters(
                    record.counters(),
                    true,
                ),
            },
            StructuralHarnessExecution::Rejected {
                contract,
                planned,
                reduced,
            } => Self {
                structural_declaration_identity: declaration_identity(contract).clone(),
                structural_contract_identity: contract.contract_identity().clone(),
                structural_match_digest: Some(planned.digest().to_string()),
                structural_reuse_digest: None,
                branch_compare_digest: None,
                replay_digest: None,
                diagnostics_digest: diagnostics_digest::rejection_diagnostics_digest(
                    contract, planned, reduced,
                ),
                failure_digest: Some(reduced.digest().to_string()),
                outcome_class: reduced.outcome_class(),
                counter_snapshot: StructuralHarnessCounterSnapshot::from_rejection(
                    contract, planned, reduced,
                ),
            },
        }
    }
}

impl StructuralHarnessCertificationBundle {
    pub(super) fn from_execution(execution: &StructuralHarnessExecution) -> Self {
        match execution {
            StructuralHarnessExecution::Remap {
                planned,
                artifact,
                record,
                ..
            } => Self::remap_success(planned, artifact, record, None),
            StructuralHarnessExecution::RemapReplay {
                planned,
                artifact,
                record,
                replayed,
                ..
            } => Self::remap_success(planned, artifact, record, Some(replayed.digest())),
            StructuralHarnessExecution::Branch {
                reduced,
                artifact,
                record,
                ..
            } => Self::branch_success(reduced, artifact, record, None),
            StructuralHarnessExecution::BranchReplay {
                reduced,
                artifact,
                record,
                replayed,
                ..
            } => Self::branch_success(reduced, artifact, record, Some(replayed.digest())),
            StructuralHarnessExecution::Rejected {
                contract,
                planned,
                reduced,
            } => Self {
                structural_match_digest: Some(planned.digest().to_string()),
                ambiguity_report: ambiguity_report(reduced),
                remap_artifact_digest: None,
                failure_digest: Some(reduced.digest().to_string()),
                structural_reuse_digest: None,
                identity_separation_report: identity_separation_report(contract, reduced),
                replay_digest: None,
                diagnostics_digest: diagnostics_digest::rejection_diagnostics_digest(
                    contract, planned, reduced,
                ),
                branch_compare_digest: None,
                structural_diff_report: None,
                counter_snapshot: StructuralHarnessCounterSnapshot::from_rejection(
                    contract, planned, reduced,
                ),
            },
        }
    }

    fn remap_success(
        planned: &PlannedStructuralMatchPacketSet,
        artifact: &PublishedStructuralRemapArtifact,
        record: &BridgeCanonicalStructuralRemapRecord,
        replay_digest: Option<&str>,
    ) -> Self {
        Self {
            structural_match_digest: Some(planned.digest().to_string()),
            ambiguity_report: None,
            remap_artifact_digest: Some(artifact.digest().to_string()),
            failure_digest: None,
            structural_reuse_digest: Some(artifact.digest().to_string()),
            identity_separation_report: None,
            replay_digest: replay_digest.map(str::to_string),
            diagnostics_digest: diagnostics_digest::remap_diagnostics_digest(record),
            branch_compare_digest: None,
            structural_diff_report: None,
            counter_snapshot: StructuralHarnessCounterSnapshot::from_counters(
                record.counters(),
                replay_digest.is_some(),
            ),
        }
    }

    fn branch_success(
        reduced: &ReducedStructuralMatchSet,
        artifact: &PublishedBranchComparisonArtifact,
        record: &BridgeCanonicalStructuralBranchComparisonRecord,
        replay_digest: Option<&str>,
    ) -> Self {
        Self {
            structural_match_digest: None,
            ambiguity_report: None,
            remap_artifact_digest: None,
            failure_digest: None,
            structural_reuse_digest: None,
            identity_separation_report: None,
            replay_digest: replay_digest.map(str::to_string),
            diagnostics_digest: diagnostics_digest::branch_diagnostics_digest(record),
            branch_compare_digest: Some(artifact.digest().to_string()),
            structural_diff_report: Some(StructuralDiffReport {
                record_identity: record.record_identity().clone(),
                branch_diff_count: reduced.branch_diff_count(),
                retained_candidates: StructuralRetainedCandidateSet::from_reduced_match_set(
                    reduced,
                ),
            }),
            counter_snapshot: StructuralHarnessCounterSnapshot::from_counters(
                record.counters(),
                replay_digest.is_some(),
            ),
        }
    }
}

fn ambiguity_report(reduced: &ReducedStructuralMatchSet) -> Option<StructuralAmbiguityReport> {
    (reduced.outcome_class() == StructuralMatchOutcomeClass::RejectedAmbiguousStructuralMatch).then(
        || StructuralAmbiguityReport {
            outcome_class: reduced.outcome_class(),
            retained_candidates: StructuralRetainedCandidateSet::from_reduced_match_set(reduced),
        },
    )
}

fn identity_separation_report(
    contract: &AdmittedStructuralComparisonContract,
    reduced: &ReducedStructuralMatchSet,
) -> Option<StructuralIdentitySeparationReport> {
    matches!(
        reduced.outcome_class(),
        StructuralMatchOutcomeClass::RejectedIdentityAuthorityConflict
            | StructuralMatchOutcomeClass::RejectedLineageStructuralDivergence
    )
    .then(|| StructuralIdentitySeparationReport {
        declaration_identity: declaration_identity(contract).clone(),
        outcome_class: reduced.outcome_class(),
        retained_candidates: StructuralRetainedCandidateSet::from_reduced_match_set(reduced),
    })
}
