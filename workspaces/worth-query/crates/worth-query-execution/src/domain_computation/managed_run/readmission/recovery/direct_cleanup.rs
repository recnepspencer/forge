use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisReleaseReceipt;
use worth_runtime_bridge::facade::{
    BridgeExecutionBasisFinalizationReceipt, BridgeExecutionBasisReadmissionCleanupOutcome,
    BridgeExecutionBasisReadmissionPending, BridgeExecutionBasisReadmissionRecoveryRequired,
};

use crate::domain_computation::managed_run::provider_restore::WorthQueryManagedGraphRestoreCleanupRequired;
use crate::domain_computation::managed_run::readmission::direct_state::{
    WorthQueryDirectBridgeCleanupRecoveryState, WorthQueryDirectYieldedState,
};
use crate::domain_computation::managed_run::readmission::evidence::{
    WorthQueryReadmissionEvidence, WorthQueryReadmissionProgress,
};
use crate::domain_computation::managed_run::{
    WorthQueryManagedProviderWorkEvidence, WorthQueryManagedRunCounters,
    WorthQueryYieldTransitionCounters,
};
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryProviderExecutionReleaseEvidence;
use crate::domain_computation::{
    WorthQueryDirectExecutionAttemptReleaseReceipt, WorthQueryDirectExecutionResourceAttempt,
    WorthQueryProviderCheckpointReleaseEvidence,
};

#[must_use = "direct readmission cleanup retains its receipt or unfinished cleanup authority"]
pub enum WorthQueryDirectReadmissionCleanupOutcome {
    Complete(WorthQueryDirectReadmissionCleanupReceipt),
    Pending(WorthQueryDirectReadmissionCleanupPending),
    RecoveryRequired(WorthQueryDirectReadmissionCleanupReceipt),
}

#[must_use = "direct readmission cleanup must be finished"]
pub struct WorthQueryDirectReadmissionCleanupRequired {
    state: WorthQueryDirectYieldedState,
    resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    bridge: WorthQueryDirectReadmissionBridgeCleanup,
    provider: WorthQueryManagedGraphRestoreCleanupRequired,
    progress: WorthQueryReadmissionProgress,
}

#[must_use = "pending direct readmission cleanup retains Bridge cleanup authority"]
pub struct WorthQueryDirectReadmissionCleanupPending {
    receipt: WorthQueryDirectReadmissionPartialCleanupReceipt,
    bridge: BridgeExecutionBasisReadmissionRecoveryRequired,
}

pub struct WorthQueryDirectReadmissionCleanupReceipt {
    partial: WorthQueryDirectReadmissionPartialCleanupReceipt,
    bridge: BridgeExecutionBasisFinalizationReceipt,
}

struct WorthQueryDirectReadmissionPartialCleanupReceipt {
    logical_run_identity: Arc<str>,
    yielded_attempt_identity: Arc<str>,
    checkpoint_release: WorthQueryProviderCheckpointReleaseEvidence,
    restored_execution_release: Option<WorthQueryProviderExecutionReleaseEvidence>,
    relational: RelationalExecutionBasisReleaseReceipt,
    attempt: WorthQueryDirectExecutionAttemptReleaseReceipt,
    run_counters: WorthQueryManagedRunCounters,
    provider_work: WorthQueryManagedProviderWorkEvidence,
    yield_counters: WorthQueryYieldTransitionCounters,
    readmission_progress: WorthQueryReadmissionProgress,
}

enum WorthQueryDirectReadmissionBridgeCleanup {
    Pending(BridgeExecutionBasisReadmissionPending),
    Recovery(BridgeExecutionBasisReadmissionRecoveryRequired),
}

impl WorthQueryDirectReadmissionCleanupRequired {
    pub(super) fn provider(
        state: WorthQueryDirectYieldedState,
        resource_attempt: WorthQueryDirectExecutionResourceAttempt,
        bridge: BridgeExecutionBasisReadmissionPending,
        provider: WorthQueryManagedGraphRestoreCleanupRequired,
        progress: WorthQueryReadmissionProgress,
    ) -> Self {
        Self {
            state,
            resource_attempt,
            bridge: WorthQueryDirectReadmissionBridgeCleanup::Pending(bridge),
            provider,
            progress,
        }
    }

    pub(super) fn bridge_recovery(
        recovery: WorthQueryDirectBridgeCleanupRecoveryState,
        progress: WorthQueryReadmissionProgress,
    ) -> Self {
        Self {
            state: recovery.state,
            resource_attempt: recovery.resource_attempt,
            bridge: WorthQueryDirectReadmissionBridgeCleanup::Recovery(recovery.bridge),
            provider: WorthQueryManagedGraphRestoreCleanupRequired::retained(
                recovery.execution,
                None,
            ),
            progress,
        }
    }

    pub fn finish(mut self) -> WorthQueryDirectReadmissionCleanupOutcome {
        let provider = self.provider.finish();
        if let Some(release) = &provider.restored_execution_release {
            self.state
                .provider_work
                .record_provider_execution_release(release);
        }
        let partial = WorthQueryDirectReadmissionPartialCleanupReceipt {
            logical_run_identity: self.state.logical_run_identity,
            yielded_attempt_identity: self.state.yielded_attempt_identity,
            checkpoint_release: provider.checkpoint_release,
            restored_execution_release: provider.restored_execution_release,
            relational: self.state.relational_basis.release(),
            attempt: self.resource_attempt.release(),
            run_counters: self.state.run_counters,
            provider_work: self.state.provider_work.into_evidence(),
            yield_counters: self.state.yield_counters,
            readmission_progress: self.progress,
        };
        finish_bridge_cleanup(
            partial,
            match self.bridge {
                WorthQueryDirectReadmissionBridgeCleanup::Pending(bridge) => bridge.abort(),
                WorthQueryDirectReadmissionBridgeCleanup::Recovery(bridge) => {
                    bridge.retry_cleanup()
                }
            },
        )
    }
}

impl WorthQueryDirectReadmissionCleanupPending {
    pub fn retry(self) -> WorthQueryDirectReadmissionCleanupOutcome {
        finish_bridge_cleanup(self.receipt, self.bridge.retry_cleanup())
    }

    pub fn logical_run_identity(&self) -> &str {
        &self.receipt.logical_run_identity
    }

    pub fn yielded_attempt_identity(&self) -> &str {
        &self.receipt.yielded_attempt_identity
    }

    pub fn checkpoint_release(&self) -> &WorthQueryProviderCheckpointReleaseEvidence {
        &self.receipt.checkpoint_release
    }

    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.receipt.readmission_progress.evidence()
    }
}

impl WorthQueryDirectReadmissionCleanupReceipt {
    pub fn logical_run_identity(&self) -> &str {
        &self.partial.logical_run_identity
    }

    pub fn yielded_attempt_identity(&self) -> &str {
        &self.partial.yielded_attempt_identity
    }

    pub fn checkpoint_release(&self) -> &WorthQueryProviderCheckpointReleaseEvidence {
        &self.partial.checkpoint_release
    }

    pub fn restored_execution_release(
        &self,
    ) -> Option<&WorthQueryProviderExecutionReleaseEvidence> {
        self.partial.restored_execution_release.as_ref()
    }

    pub fn bridge(&self) -> &BridgeExecutionBasisFinalizationReceipt {
        &self.bridge
    }

    pub fn relational(&self) -> &RelationalExecutionBasisReleaseReceipt {
        &self.partial.relational
    }

    pub fn attempt(&self) -> &WorthQueryDirectExecutionAttemptReleaseReceipt {
        &self.partial.attempt
    }

    pub fn provider_work(&self) -> &WorthQueryManagedProviderWorkEvidence {
        &self.partial.provider_work
    }

    pub fn run_counters(&self) -> &WorthQueryManagedRunCounters {
        &self.partial.run_counters
    }

    pub const fn yield_counters(&self) -> WorthQueryYieldTransitionCounters {
        self.partial.yield_counters
    }

    pub const fn readmission_evidence(&self) -> WorthQueryReadmissionEvidence {
        self.partial.readmission_progress.evidence()
    }
}

fn finish_bridge_cleanup(
    mut partial: WorthQueryDirectReadmissionPartialCleanupReceipt,
    bridge: BridgeExecutionBasisReadmissionCleanupOutcome,
) -> WorthQueryDirectReadmissionCleanupOutcome {
    match bridge {
        BridgeExecutionBasisReadmissionCleanupOutcome::Complete(returned) => {
            let (bridge, bridge_counters) = returned.into_parts();
            partial.readmission_progress.observe_bridge(bridge_counters);
            let receipt = WorthQueryDirectReadmissionCleanupReceipt {
                partial,
                bridge: bridge.release(),
            };
            if direct_cleanup_recovery_required(&receipt) {
                WorthQueryDirectReadmissionCleanupOutcome::RecoveryRequired(receipt)
            } else {
                WorthQueryDirectReadmissionCleanupOutcome::Complete(receipt)
            }
        }
        BridgeExecutionBasisReadmissionCleanupOutcome::RecoveryRequired(bridge) => {
            partial
                .readmission_progress
                .observe_bridge(bridge.counters());
            WorthQueryDirectReadmissionCleanupOutcome::Pending(
                WorthQueryDirectReadmissionCleanupPending {
                    receipt: partial,
                    bridge,
                },
            )
        }
    }
}

fn direct_cleanup_recovery_required(receipt: &WorthQueryDirectReadmissionCleanupReceipt) -> bool {
    receipt
        .checkpoint_release()
        .disposition()
        .recovery_required()
        || receipt
            .restored_execution_release()
            .is_some_and(WorthQueryProviderExecutionReleaseEvidence::recovery_required)
}
