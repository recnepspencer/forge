use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::BridgeYieldedExecutionBasis;

use super::{
    direct_yield_cleanup::{self, WorthQueryDirectYieldCleanupReceiptParts},
    WorthQueryDirectYieldCleanupReceipt, WorthQueryDirectYieldOutcome,
    WorthQueryManagedProviderWorkEvidence, WorthQueryManagedRunCounters,
    WorthQueryPausedDirectGraphExecution, WorthQueryYieldRecoveryKind,
    WorthQueryYieldRecoveryResourceEvidence, WorthQueryYieldTransitionCounters,
};
use crate::domain_computation::WorthQueryDirectExecutionResourceAttempt;

pub(super) enum WorthQueryDirectYieldRecoveryState {
    Running(WorthQueryPausedDirectGraphExecution),
    Terminalized {
        logical_run_identity: Arc<str>,
        attempt_identity: Arc<str>,
        resource_attempt: WorthQueryDirectExecutionResourceAttempt,
        relational_basis: RelationalExecutionBasisLease,
        bridge: BridgeYieldedExecutionBasis,
        run_counters: WorthQueryManagedRunCounters,
        provider_work: WorthQueryManagedProviderWorkEvidence,
    },
}

pub struct WorthQueryDirectYieldRecoveryRequired {
    kind: WorthQueryYieldRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryYieldTransitionCounters,
    resource_evidence: WorthQueryYieldRecoveryResourceEvidence,
    state: WorthQueryDirectYieldRecoveryState,
}

impl WorthQueryDirectYieldRecoveryRequired {
    pub const fn kind(&self) -> WorthQueryYieldRecoveryKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> WorthQueryYieldTransitionCounters {
        self.counters
    }

    pub fn resource_evidence(&self) -> &WorthQueryYieldRecoveryResourceEvidence {
        &self.resource_evidence
    }

    pub fn run_counters(&self) -> &WorthQueryManagedRunCounters {
        match &self.state {
            WorthQueryDirectYieldRecoveryState::Running(paused) => &paused.active.running.counters,
            WorthQueryDirectYieldRecoveryState::Terminalized { run_counters, .. } => run_counters,
        }
    }

    pub fn running_attempt_recoverable(&self) -> bool {
        matches!(self.state, WorthQueryDirectYieldRecoveryState::Running(_))
    }

    pub fn into_paused(self) -> Result<WorthQueryPausedDirectGraphExecution, Self> {
        match self.state {
            WorthQueryDirectYieldRecoveryState::Running(paused) => Ok(paused),
            state => Err(Self {
                kind: self.kind,
                detail: self.detail,
                counters: self.counters,
                resource_evidence: self.resource_evidence,
                state,
            }),
        }
    }

    pub fn cleanup_terminalized(self) -> Result<WorthQueryDirectYieldCleanupReceipt, Self> {
        let WorthQueryDirectYieldRecoveryState::Terminalized {
            logical_run_identity,
            attempt_identity,
            resource_attempt,
            relational_basis,
            bridge,
            run_counters,
            provider_work,
        } = self.state
        else {
            return Err(self);
        };
        Ok(direct_yield_cleanup::terminalized_cleanup_receipt(
            WorthQueryDirectYieldCleanupReceiptParts {
                logical_run_identity,
                attempt_identity,
                bridge: bridge.release(),
                relational: relational_basis.release(),
                attempt: resource_attempt.release(),
                run_counters,
                provider_work,
                yield_counters: self.counters,
                recovery_evidence: Some(self.resource_evidence),
            },
        ))
    }
}

pub(super) fn running_recovery(
    kind: WorthQueryYieldRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryYieldTransitionCounters,
    paused: WorthQueryPausedDirectGraphExecution,
) -> WorthQueryDirectYieldOutcome {
    WorthQueryDirectYieldOutcome::RecoveryRequired(WorthQueryDirectYieldRecoveryRequired {
        kind,
        detail,
        counters,
        resource_evidence: WorthQueryYieldRecoveryResourceEvidence::default(),
        state: WorthQueryDirectYieldRecoveryState::Running(paused),
    })
}

pub(super) struct WorthQueryTerminalizedDirectYieldRecovery {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) attempt_identity: Arc<str>,
    pub(super) resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    pub(super) bridge: BridgeYieldedExecutionBasis,
    pub(super) run_counters: WorthQueryManagedRunCounters,
    pub(super) provider_work: WorthQueryManagedProviderWorkEvidence,
}

pub(super) fn terminalized_recovery(
    kind: WorthQueryYieldRecoveryKind,
    detail: Arc<str>,
    counters: WorthQueryYieldTransitionCounters,
    state: WorthQueryTerminalizedDirectYieldRecovery,
    resource_evidence: WorthQueryYieldRecoveryResourceEvidence,
) -> WorthQueryDirectYieldOutcome {
    WorthQueryDirectYieldOutcome::RecoveryRequired(WorthQueryDirectYieldRecoveryRequired {
        kind,
        detail,
        counters,
        resource_evidence,
        state: WorthQueryDirectYieldRecoveryState::Terminalized {
            logical_run_identity: state.logical_run_identity,
            attempt_identity: state.attempt_identity,
            resource_attempt: state.resource_attempt,
            relational_basis: state.relational_basis,
            bridge: state.bridge,
            run_counters: state.run_counters,
            provider_work: state.provider_work,
        },
    })
}
