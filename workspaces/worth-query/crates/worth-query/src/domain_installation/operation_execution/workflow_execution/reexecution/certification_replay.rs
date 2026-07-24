use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::WorthQueryBoundDomainOperation;
use crate::runtime::WorthQueryWorkspace;
use worth_proof::TransitionOutcome;

use super::{
    WorthQueryCompletedWorkflowTrace, WorthQueryExecutableDomainOperation,
    WorthQueryNormalizedWorkflowIntent, WorthQueryReplayComparison, WorthQueryReplayDivergence,
    WorthQueryWorkflowOperation, WorthQueryWorkflowReexecutionStop, WorthQueryWorkflowRunCounters,
    WorthQueryWorkflowTraceSemantics,
};

#[path = "certification_replay/execution.rs"]
mod execution;
pub(super) use execution::execute_admitted_replay;

/// Concrete, sealed audience witness. Its only constructor is re-exported by
/// Query's certification facade and therefore remains outside ordinary host and
/// declaration facades.
pub struct WorthQueryCertificationReplayCapability {
    _private: (),
}

pub fn issue_query_certification_replay_capability() -> WorthQueryCertificationReplayCapability {
    WorthQueryCertificationReplayCapability { _private: () }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryReplayBasisRelationship {
    ExactAdmittedBasis,
    AdmittedHistoricalBasis {
        correspondence: WorthQueryHistoricalBasisCorrespondence,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryHistoricalBasisCorrespondence {
    RetainedSnapshotIdentity,
}

pub struct WorthQueryHistoricalReplayAdmission {
    pub(super) original_operation_identity: String,
    pub(super) replay_operation_identity: String,
    pub(super) original_basis_identity: String,
    pub(super) execution_basis_identity: String,
    pub(super) historical_workspace_name: String,
    pub(super) historical_snapshot_identity: crate::memory_workspace::WorthQuerySnapshotIdentity,
    pub(super) correspondence: WorthQueryHistoricalBasisCorrespondence,
}

impl WorthQueryHistoricalReplayAdmission {
    pub fn original_basis_identity(&self) -> &str {
        &self.original_basis_identity
    }

    pub fn historical_snapshot_identity(
        &self,
    ) -> &crate::memory_workspace::WorthQuerySnapshotIdentity {
        &self.historical_snapshot_identity
    }

    pub const fn correspondence(&self) -> WorthQueryHistoricalBasisCorrespondence {
        self.correspondence
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryHistoricalReplayAdmissionDenial {
    ForeignOperation,
    ForeignRuntime,
    StaleInstallationGeneration,
    ReplayBasisCapabilityMismatch,
    HistoricalExecutionSubstrateUnavailable,
    HistoricalSnapshotDoesNotBindOriginalTrace,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryCertificationReplayCounters {
    pub authority_checks: usize,
    pub operation_checks: usize,
    pub basis_checks: usize,
    pub original_stage_index_entries: usize,
    pub intent_stage_checks: usize,
    pub semantic_stage_comparisons: usize,
    pub unrelated_trace_scans: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCertificationReplayAdmissionDenial {
    ReplayNotInstalled,
    ForeignOperation,
    ForeignRuntime,
    StaleInstallationGeneration,
    UnsupportedBasisRelationship,
    IntentDoesNotMatchOriginalTrace,
    HistoricalAdmissionMismatch,
    HistoricalExecutionBasisDrift,
    ReplayComparatorUnavailable,
}

#[derive(Debug)]
pub enum WorthQueryCertificationReplayStop {
    Admission(WorthQueryCertificationReplayAdmissionDenial),
    ResourceAdmission(super::WorthQueryExecutionResourceAdmissionDenial),
    Execution(WorthQueryWorkflowReexecutionStop),
    SemanticDivergence(WorthQueryReplayDivergence),
}

pub struct WorthQueryCertificationReplayResult<D, O, F, L: BasisOperationLane> {
    original_trace_identity: String,
    replay_trace_identity: String,
    intent: WorthQueryNormalizedWorkflowIntent,
    basis_relationship: WorthQueryReplayBasisRelationship,
    original_semantics: WorthQueryWorkflowTraceSemantics,
    replay_semantics: WorthQueryWorkflowTraceSemantics,
    comparison: WorthQueryReplayComparison,
    foundational_attachment: super::WorthQueryFoundationalReplayAttachment,
    original_execution_counters: WorthQueryWorkflowRunCounters,
    replay_execution_counters: WorthQueryWorkflowRunCounters,
    counters: WorthQueryCertificationReplayCounters,
    _operation: std::marker::PhantomData<fn() -> (D, O, F, L)>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryCertificationReplayResult<D, O, F, L> {
    pub fn original_trace_identity(&self) -> &str {
        &self.original_trace_identity
    }
    pub fn replay_trace_identity(&self) -> &str {
        &self.replay_trace_identity
    }
    pub fn intent(&self) -> &WorthQueryNormalizedWorkflowIntent {
        &self.intent
    }
    pub fn basis_relationship(&self) -> WorthQueryReplayBasisRelationship {
        self.basis_relationship
    }
    pub fn original_semantics(&self) -> &WorthQueryWorkflowTraceSemantics {
        &self.original_semantics
    }
    pub fn replay_semantics(&self) -> &WorthQueryWorkflowTraceSemantics {
        &self.replay_semantics
    }
    pub fn comparison(&self) -> &WorthQueryReplayComparison {
        &self.comparison
    }
    pub fn foundational_attachment(&self) -> &super::WorthQueryFoundationalReplayAttachment {
        &self.foundational_attachment
    }
    pub const fn replay_execution_counters(&self) -> WorthQueryWorkflowRunCounters {
        self.replay_execution_counters
    }
    pub const fn original_execution_counters(&self) -> WorthQueryWorkflowRunCounters {
        self.original_execution_counters
    }
    pub fn counters(&self) -> WorthQueryCertificationReplayCounters {
        self.counters
    }
}

pub type WorthQueryCertificationReplayOutcome<D, O, F, L> = TransitionOutcome<
    WorthQueryCertificationReplayResult<D, O, F, L>,
    WorthQueryCertificationReplayStop,
    WorthQueryCertificationReplayStop,
    WorthQueryCertificationReplayStop,
    WorthQueryCertificationReplayStop,
    WorthQueryCertificationReplayStop,
>;

pub fn replay_installed_workflow<
    D: 'static,
    O,
    F: 'static,
    LO: BasisOperationLane,
    LR: BasisOperationLane,
>(
    _capability: WorthQueryCertificationReplayCapability,
    original: &WorthQueryCompletedWorkflowTrace<D, O, F, LO>,
    bound: WorthQueryBoundDomainOperation<D, O, F, LR>,
    intent: WorthQueryNormalizedWorkflowIntent,
    resources: worth_query_declaration::facade::domain_computation::WorthQueryExecutionResourceRequest,
    workspace: &mut WorthQueryWorkspace,
) -> WorthQueryCertificationReplayOutcome<D, O, F, LR>
where
    O: WorthQueryExecutableDomainOperation<D, F, Execution = WorthQueryWorkflowOperation> + 'static,
{
    let counters = WorthQueryCertificationReplayCounters {
        authority_checks: 1,
        operation_checks: 1,
        basis_checks: 1,
        ..Default::default()
    };
    let original_bound = &original.run.bound;
    if original_bound.definition().canonical_identity() != bound.definition().canonical_identity() {
        return denied(WorthQueryCertificationReplayAdmissionDenial::ForeignOperation);
    }
    if original_bound
        .operation()
        .domain_authority()
        .runtime_authority()
        != bound.operation().domain_authority().runtime_authority()
    {
        return denied(WorthQueryCertificationReplayAdmissionDenial::ForeignRuntime);
    }
    if original_bound.operation().installation_generation()
        != bound.operation().installation_generation()
    {
        return denied(WorthQueryCertificationReplayAdmissionDenial::StaleInstallationGeneration);
    }
    if original_bound.basis().capability_digest() != bound.basis().capability_digest() {
        return denied(WorthQueryCertificationReplayAdmissionDenial::UnsupportedBasisRelationship);
    }
    execute_admitted_replay(
        original,
        bound,
        intent,
        resources,
        workspace,
        WorthQueryReplayBasisRelationship::ExactAdmittedBasis,
        counters,
    )
}

fn enforce_query_replay_comparison(
    mandatory: WorthQueryReplayComparison,
    domain_comparison: impl FnOnce() -> WorthQueryReplayComparison,
) -> WorthQueryReplayComparison {
    match mandatory {
        WorthQueryReplayComparison::Diverged(_) => mandatory,
        WorthQueryReplayComparison::Equivalent => domain_comparison(),
    }
}

fn denied<D, O, F, L: BasisOperationLane>(
    denial: WorthQueryCertificationReplayAdmissionDenial,
) -> WorthQueryCertificationReplayOutcome<D, O, F, L> {
    TransitionOutcome::Denied(WorthQueryCertificationReplayStop::Admission(denial))
}

fn execution(stop: WorthQueryWorkflowReexecutionStop) -> WorthQueryCertificationReplayStop {
    WorthQueryCertificationReplayStop::Execution(stop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_comparator_cannot_mask_query_detected_drift() {
        let contacted = std::cell::Cell::new(false);
        let mandatory = WorthQueryReplayComparison::Diverged(WorthQueryReplayDivergence::Effect {
            stage: "mutate".into(),
        });
        let resolved = enforce_query_replay_comparison(mandatory.clone(), || {
            contacted.set(true);
            WorthQueryReplayComparison::Equivalent
        });

        assert_eq!(resolved, mandatory);
        assert!(!contacted.get());
    }
}
