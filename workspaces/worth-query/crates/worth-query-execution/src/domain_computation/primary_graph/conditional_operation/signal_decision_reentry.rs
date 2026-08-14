use worth_relational::facade::bridge::bridge_snapshot_identity_for_handle;
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
    _branch: TruthBranchIdentity,
    snapshot: TruthSnapshotIdentity,
    branch_projection: worth_runtime_bridge::facade::BridgeIdentityEvidence,
    snapshot_projection: worth_runtime_bridge::facade::BridgeIdentityEvidence,
}

impl WorthQueryConditionalTruthBasis {
    pub(super) fn acquire(
        runtime: &crate::domain_computation::execution_runtime::WorthQueryExecutionRuntime,
    ) -> Result<Self, &'static str> {
        let graph = runtime
            .primary_graph()
            .ok_or("conditional execution lost the installed primary graph")?;
        let lease = WorthQueryApplicationSnapshotLease::acquire(
            graph.integration_handle(),
            graph.retain_layout(),
            &primary_relational_branch_id(),
        )
        .ok_or("conditional execution could not pin the primary branch head")?;
        let snapshot = bridge_snapshot_identity_for_handle(lease.snapshot());
        let branch = primary_truth_branch_identity();
        Ok(Self {
            _lease: lease,
            branch_projection: branch.bridge_admission_evidence(),
            snapshot_projection: snapshot.bridge_admission_evidence(),
            _branch: branch,
            snapshot,
        })
    }

    pub(super) fn snapshot(&self) -> &TruthSnapshotIdentity {
        &self.snapshot
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
    OperationIndeterminate(BridgeConditionalDecisionEvidence, String),
    OperationCommitted,
    OperationAlreadyCommitted,
    Failed(String),
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
            WorthQueryRetainedConditionalDecision::OperationRetryable(evidence, detail)
            | WorthQueryRetainedConditionalDecision::OperationIndeterminate(evidence, detail) => {
                let _decision = evidence.signal().class();
                let _failure_detail = detail.as_str();
                counts.failed += 1;
            }
            WorthQueryRetainedConditionalDecision::OperationCommitted
            | WorthQueryRetainedConditionalDecision::OperationAlreadyCommitted => {}
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
