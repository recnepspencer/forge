use worth_runtime_bridge::facade::{
    BridgeConditionalDecisionEvidence, BridgeManagedConditionalExecutionRequest,
    BridgeManagedDueWake, BridgeOwnedSignalRuntime, TruthBranchIdentity, TruthSnapshotIdentity,
};

use super::predicate_admission::QueryConditionalComputeContext;
use crate::domain_computation::primary_graph::{
    primary_relational_branch_id, primary_truth_branch_identity, WorthQueryApplicationSnapshotLease,
};

pub(in crate::domain_computation::primary_graph) struct WorthQueryConditionalTruthBasis {
    _lease: WorthQueryApplicationSnapshotLease,
    observation: std::sync::Arc<worth_relational::facade::bridge::RelationalBridgeObservationLease>,
    _branch: TruthBranchIdentity,
    snapshot: TruthSnapshotIdentity,
    branch_projection: worth_runtime_bridge::facade::BridgeIdentityEvidence,
    snapshot_projection: worth_runtime_bridge::facade::BridgeIdentityEvidence,
}

pub(super) enum WorthQueryConditionalTruthBasisDenial {
    ActiveSnapshotCapacityExhausted { maximum_active_snapshots: usize },
    RetentionCapacityExhausted,
    RetentionIdentityExhausted,
    SnapshotIdentityExhausted,
    RuntimeRejected(&'static str),
}

impl WorthQueryConditionalTruthBasis {
    pub(super) fn acquire(
        runtime: &crate::domain_computation::execution_runtime::WorthQueryExecutionRuntime,
    ) -> Result<Self, WorthQueryConditionalTruthBasisDenial> {
        let graph = runtime.primary_graph().ok_or(
            WorthQueryConditionalTruthBasisDenial::RuntimeRejected(
                "conditional execution lost the installed primary graph",
            ),
        )?;
        let integration = graph.integration_handle();
        let lease = WorthQueryApplicationSnapshotLease::acquire(
            integration.clone(),
            graph.retain_layout(),
            &primary_relational_branch_id(),
        )
        .map_err(|denial| match denial {
            crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLeaseDenial::ActiveSnapshotCapacityExhausted {
                maximum_active_snapshots,
            } => WorthQueryConditionalTruthBasisDenial::ActiveSnapshotCapacityExhausted {
                maximum_active_snapshots,
            },
            crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLeaseDenial::RetentionCapacityExhausted => {
                WorthQueryConditionalTruthBasisDenial::RetentionCapacityExhausted
            }
            crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLeaseDenial::RetentionIdentityExhausted => {
                WorthQueryConditionalTruthBasisDenial::RetentionIdentityExhausted
            }
            crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLeaseDenial::SnapshotIdentityExhausted => {
                WorthQueryConditionalTruthBasisDenial::SnapshotIdentityExhausted
            }
            _ => WorthQueryConditionalTruthBasisDenial::RuntimeRejected(
                "conditional execution could not pin the primary branch head",
            ),
        })?;
        let source = integration.relational_bridge_source();
        let basis = source
            .readmit_branch_basis(lease.basis_descriptor())
            .map_err(|denial| match denial {
                worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionCapacityExhausted => {
                    WorthQueryConditionalTruthBasisDenial::RetentionCapacityExhausted
                }
                worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionIdentityExhausted => {
                    WorthQueryConditionalTruthBasisDenial::RetentionIdentityExhausted
                }
                worth_relational::facade::branch::RelationalBranchBasisDenial::SnapshotIdentityExhausted => {
                    WorthQueryConditionalTruthBasisDenial::SnapshotIdentityExhausted
                }
                _ => WorthQueryConditionalTruthBasisDenial::RuntimeRejected(
                    "conditional execution could not readmit its exact primary basis",
                ),
            })?;
        let observation = source.retain_branch_basis_for_bridge(&basis).map_err(|denial| {
            match denial {
                worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionCapacityExhausted => {
                    WorthQueryConditionalTruthBasisDenial::RetentionCapacityExhausted
                }
                worth_relational::facade::branch::RelationalBranchBasisDenial::RetentionIdentityExhausted => {
                    WorthQueryConditionalTruthBasisDenial::RetentionIdentityExhausted
                }
                worth_relational::facade::branch::RelationalBranchBasisDenial::SnapshotIdentityExhausted => {
                    WorthQueryConditionalTruthBasisDenial::SnapshotIdentityExhausted
                }
                _ => WorthQueryConditionalTruthBasisDenial::RuntimeRejected(
                    "conditional execution could not bind its exact Bridge observation",
                ),
            }
        })?;
        let snapshot = observation.snapshot_identity().clone();
        let branch = primary_truth_branch_identity();
        Ok(Self {
            _lease: lease,
            observation: std::sync::Arc::new(observation),
            branch_projection: branch.bridge_admission_evidence(),
            snapshot_projection: snapshot.bridge_admission_evidence(),
            _branch: branch,
            snapshot,
        })
    }

    pub(super) fn snapshot(&self) -> &TruthSnapshotIdentity {
        &self.snapshot
    }

    pub(super) fn granular_source_read_basis(
        &self,
    ) -> crate::domain_computation::primary_graph::WorthQueryGranularSourceReadBasis {
        crate::domain_computation::primary_graph::WorthQueryGranularSourceReadBasis::new(
            self.snapshot.clone(),
            self._branch.clone(),
            std::sync::Arc::clone(&self.observation),
        )
    }

    fn snapshot_projection(&self) -> &str {
        self.snapshot_projection.terminal_projection_for_reporting()
    }

    fn branch_projection(&self) -> &str {
        self.branch_projection.terminal_projection_for_reporting()
    }
}

pub(super) enum WorthQueryRetainedConditionalDecision {
    Eligible(BridgeConditionalDecisionEvidence),
    Suppressed(BridgeConditionalDecisionEvidence),
    Deferred(BridgeConditionalDecisionEvidence),
    OperationRetryable(BridgeConditionalDecisionEvidence, String),
    OperationBackpressured(
        BridgeConditionalDecisionEvidence,
        WorthQueryOperationBackpressureCause,
    ),
    OperationControlStopped(
        BridgeConditionalDecisionEvidence,
        super::application_operation_reentry::WorthQueryTemporalControlStop,
    ),
    OperationTerminalFailure(
        BridgeConditionalDecisionEvidence,
        super::application_operation_reentry::WorthQueryTemporalTerminalFailure,
    ),
    OperationSettlementDeferred(
        BridgeConditionalDecisionEvidence,
        crate::domain_computation::primary_graph::WorthQueryApplicationSettlementDeferred,
    ),
    OperationIndeterminate(BridgeConditionalDecisionEvidence, String),
    OperationCommitted(BridgeConditionalDecisionEvidence),
    OperationAlreadyCommitted(BridgeConditionalDecisionEvidence),
    Failed(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WorthQueryOperationBackpressureCause {
    ActiveSnapshotCapacityExhausted {
        maximum_active_snapshots: usize,
    },
    RetentionCapacityExhausted,
    ProviderCommit(
        crate::domain_computation::primary_graph::WorthQueryApplicationCommitDeferredKind,
    ),
}

pub(super) struct WorthQueryRetainedConditionalWake {
    pub(super) lifecycle_token: std::sync::Arc<()>,
    pub(super) due: BridgeManagedDueWake,
    pub(super) decision: WorthQueryRetainedConditionalDecision,
    pub(super) attempt: u64,
    pub(super) last_signal_decision: Option<super::WorthQueryConditionalSignalDecision>,
    pub(super) application_attempted: bool,
    pub(super) application_admission_canonical_work:
        worth_query_installation::facade::WorthQueryCanonicalWorkEvidence,
}

#[derive(Clone, Copy, Default)]
pub(super) struct WorthQueryConditionalDecisionCounts {
    pub(super) eligible: usize,
    pub(super) suppressed: usize,
    pub(super) deferred: usize,
    pub(super) failed: usize,
}

pub(super) fn retained_decision_counts(
    wakes: &[WorthQueryRetainedConditionalWake],
) -> WorthQueryConditionalDecisionCounts {
    let mut counts = WorthQueryConditionalDecisionCounts::default();
    for wake in wakes {
        let _intent_revision = wake.due.revision();
        match &wake.decision {
            WorthQueryRetainedConditionalDecision::Eligible(evidence) => {
                let _decision = evidence.signal().class();
                counts.eligible += 1;
            }
            WorthQueryRetainedConditionalDecision::Suppressed(evidence) => {
                let _decision = evidence.signal().class();
                counts.suppressed += 1;
            }
            WorthQueryRetainedConditionalDecision::Deferred(evidence) => {
                let _decision = evidence.signal().class();
                counts.deferred += 1;
            }
            WorthQueryRetainedConditionalDecision::OperationBackpressured(evidence, cause) => {
                let _decision = evidence.signal().class();
                let _typed_backpressure_cause = cause;
                counts.deferred += 1;
            }
            WorthQueryRetainedConditionalDecision::OperationControlStopped(evidence, cause) => {
                let _decision = evidence.signal().class();
                let _typed_control_stop = cause;
                counts.failed += 1;
            }
            WorthQueryRetainedConditionalDecision::OperationTerminalFailure(evidence, cause) => {
                let _decision = evidence.signal().class();
                let _typed_terminal_failure = cause;
                counts.failed += 1;
            }
            WorthQueryRetainedConditionalDecision::OperationRetryable(evidence, detail)
            | WorthQueryRetainedConditionalDecision::OperationIndeterminate(evidence, detail) => {
                let _decision = evidence.signal().class();
                let _failure_detail = detail.as_str();
                counts.failed += 1;
            }
            WorthQueryRetainedConditionalDecision::OperationSettlementDeferred(
                evidence,
                deferred,
            ) => {
                let _decision = evidence.signal().class();
                let _settlement = deferred.settlement();
                counts.deferred += 1;
            }
            WorthQueryRetainedConditionalDecision::OperationCommitted(_)
            | WorthQueryRetainedConditionalDecision::OperationAlreadyCommitted(_) => {}
            WorthQueryRetainedConditionalDecision::Failed(detail) => {
                let _failure_detail = detail.as_str();
                counts.failed += 1;
            }
        }
    }
    counts
}

pub(super) fn evaluate_due_wake(
    bridge: &mut BridgeOwnedSignalRuntime,
    due: BridgeManagedDueWake,
    lowering: &std::sync::Arc<worth_runtime_bridge::facade::BridgeInstalledConditionalLowering>,
    query_binding_identity: &str,
    query_capability_identity: u64,
    truth: &WorthQueryConditionalTruthBasis,
    triggering_correspondence: Option<
        &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    >,
) -> WorthQueryRetainedConditionalWake {
    let attempt = due.signal_ready_ordinal();
    let mut compute = QueryConditionalComputeContext {
        output_version: attempt,
    };
    let result = bridge.execute_managed_due_wake(
        BridgeManagedConditionalExecutionRequest {
            due_wake: &due,
            lowering,
            query_binding_identity,
            query_capability_identity,
            snapshot_identity: truth.snapshot_projection(),
            truth_branch_identity: Some(truth.branch_projection()),
            bridge_snapshot_identity: Some(truth.snapshot()),
            triggering_correspondence,
            attempt,
        },
        &mut compute,
    );
    let last_signal_decision = result
        .as_ref()
        .ok()
        .map(|evidence| super::execution_provenance::signal_decision(evidence.signal().class()));
    let decision = match result {
        Ok(evidence) => classify(evidence),
        Err(denial) => WorthQueryRetainedConditionalDecision::Failed(format!(
            "{:?}: {}",
            denial.kind(),
            denial.detail()
        )),
    };
    WorthQueryRetainedConditionalWake {
        lifecycle_token: Default::default(),
        due,
        decision,
        attempt,
        last_signal_decision,
        application_attempted: false,
        application_admission_canonical_work:
            worth_query_installation::facade::WorthQueryCanonicalWorkEvidence::zero(),
    }
}

pub(super) fn reconsider_retained_wake(
    bridge: &mut BridgeOwnedSignalRuntime,
    wake: &mut WorthQueryRetainedConditionalWake,
    lowering: &std::sync::Arc<worth_runtime_bridge::facade::BridgeInstalledConditionalLowering>,
    query_binding_identity: &str,
    query_capability_identity: u64,
    truth: &WorthQueryConditionalTruthBasis,
    triggering_correspondence: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
) {
    if !matches!(
        wake.decision,
        WorthQueryRetainedConditionalDecision::Suppressed(_)
            | WorthQueryRetainedConditionalDecision::Deferred(_)
    ) {
        return;
    }
    wake.attempt = wake.attempt.saturating_add(1);
    let mut compute = QueryConditionalComputeContext {
        output_version: wake.attempt,
    };
    let result = bridge.execute_managed_due_wake(
        BridgeManagedConditionalExecutionRequest {
            due_wake: &wake.due,
            lowering,
            query_binding_identity,
            query_capability_identity,
            snapshot_identity: truth.snapshot_projection(),
            truth_branch_identity: Some(truth.branch_projection()),
            bridge_snapshot_identity: Some(truth.snapshot()),
            triggering_correspondence: Some(triggering_correspondence),
            attempt: wake.attempt,
        },
        &mut compute,
    );
    wake.last_signal_decision = result
        .as_ref()
        .ok()
        .map(|evidence| super::execution_provenance::signal_decision(evidence.signal().class()));
    wake.application_attempted = false;
    wake.application_admission_canonical_work =
        worth_query_installation::facade::WorthQueryCanonicalWorkEvidence::zero();
    wake.decision = match result {
        Ok(evidence) => classify(evidence),
        Err(denial) => WorthQueryRetainedConditionalDecision::Failed(format!(
            "{:?}: {}",
            denial.kind(),
            denial.detail()
        )),
    };
}

fn classify(evidence: BridgeConditionalDecisionEvidence) -> WorthQueryRetainedConditionalDecision {
    match classify_bridge_signal(&evidence) {
        super::WorthQueryConditionalSignalDecision::Eligible => {
            WorthQueryRetainedConditionalDecision::Eligible(evidence)
        }
        super::WorthQueryConditionalSignalDecision::DependencyUnchanged
        | super::WorthQueryConditionalSignalDecision::RevertedClean
        | super::WorthQueryConditionalSignalDecision::Suppressed => {
            WorthQueryRetainedConditionalDecision::Suppressed(evidence)
        }
        super::WorthQueryConditionalSignalDecision::Deferred => {
            WorthQueryRetainedConditionalDecision::Deferred(evidence)
        }
    }
}

pub(crate) fn classify_bridge_signal(
    evidence: &BridgeConditionalDecisionEvidence,
) -> super::WorthQueryConditionalSignalDecision {
    super::execution_provenance::signal_decision(evidence.signal().class())
}
