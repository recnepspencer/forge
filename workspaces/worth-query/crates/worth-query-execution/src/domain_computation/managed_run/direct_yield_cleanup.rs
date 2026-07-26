use std::sync::Arc;

use worth_relational::facade::runtime::RelationalExecutionBasisReleaseReceipt;
use worth_runtime_bridge::facade::BridgeExecutionBasisFinalizationReceipt;

use super::{
    WorthQueryManagedProviderWorkEvidence, WorthQueryManagedRunCounters,
    WorthQueryYieldTransitionCounters, WorthQueryYieldedDirectRun,
};
use crate::domain_computation::{
    WorthQueryDirectExecutionAttemptReleaseReceipt, WorthQueryProviderCheckpointEvidence,
    WorthQueryProviderCheckpointReleaseEvidence, WorthQueryYieldRecoveryResourceEvidence,
};

pub enum WorthQueryDirectYieldCleanupOutcome {
    Complete(WorthQueryDirectYieldCleanupReceipt),
    RecoveryRequired(WorthQueryDirectYieldCleanupReceipt),
}

pub struct WorthQueryDirectYieldCleanupReceipt {
    logical_run_identity: Arc<str>,
    attempt_identity: Arc<str>,
    checkpoint_release: Option<WorthQueryProviderCheckpointReleaseEvidence>,
    recovery_evidence: Option<WorthQueryYieldRecoveryResourceEvidence>,
    bridge: BridgeExecutionBasisFinalizationReceipt,
    relational: RelationalExecutionBasisReleaseReceipt,
    attempt: WorthQueryDirectExecutionAttemptReleaseReceipt,
    run_counters: WorthQueryManagedRunCounters,
    provider_work: WorthQueryManagedProviderWorkEvidence,
    yield_counters: WorthQueryYieldTransitionCounters,
}

pub(super) struct WorthQueryDirectYieldCleanupReceiptParts {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) attempt_identity: Arc<str>,
    pub(super) bridge: BridgeExecutionBasisFinalizationReceipt,
    pub(super) relational: RelationalExecutionBasisReleaseReceipt,
    pub(super) attempt: WorthQueryDirectExecutionAttemptReleaseReceipt,
    pub(super) run_counters: WorthQueryManagedRunCounters,
    pub(super) provider_work: WorthQueryManagedProviderWorkEvidence,
    pub(super) yield_counters: WorthQueryYieldTransitionCounters,
    pub(super) recovery_evidence: Option<WorthQueryYieldRecoveryResourceEvidence>,
}

impl WorthQueryDirectYieldCleanupReceipt {
    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn yielded_attempt_identity(&self) -> &str {
        &self.attempt_identity
    }

    pub fn checkpoint(&self) -> Option<&WorthQueryProviderCheckpointEvidence> {
        self.checkpoint_release
            .as_ref()
            .map(WorthQueryProviderCheckpointReleaseEvidence::checkpoint)
    }

    pub fn checkpoint_release(&self) -> Option<&WorthQueryProviderCheckpointReleaseEvidence> {
        self.checkpoint_release.as_ref()
    }

    pub fn recovery_evidence(&self) -> Option<&WorthQueryYieldRecoveryResourceEvidence> {
        self.recovery_evidence.as_ref()
    }

    pub fn bridge(&self) -> &BridgeExecutionBasisFinalizationReceipt {
        &self.bridge
    }

    pub fn relational(&self) -> &RelationalExecutionBasisReleaseReceipt {
        &self.relational
    }

    pub fn attempt(&self) -> &WorthQueryDirectExecutionAttemptReleaseReceipt {
        &self.attempt
    }

    pub fn provider_work(&self) -> &WorthQueryManagedProviderWorkEvidence {
        &self.provider_work
    }

    pub fn run_counters(&self) -> &WorthQueryManagedRunCounters {
        &self.run_counters
    }

    pub const fn yield_counters(&self) -> WorthQueryYieldTransitionCounters {
        self.yield_counters
    }
}

pub(super) fn cleanup_yielded_direct(
    yielded: WorthQueryYieldedDirectRun,
) -> WorthQueryDirectYieldCleanupOutcome {
    let WorthQueryYieldedDirectRun {
        logical_run_identity,
        attempt_identity,
        resource_attempt,
        relational_basis,
        bridge,
        execution,
        run_counters,
        provider_work,
        yield_counters,
    } = yielded;
    let checkpoint_release = execution.release();
    let recovery_required = checkpoint_release.disposition().recovery_required();
    let receipt = WorthQueryDirectYieldCleanupReceipt {
        logical_run_identity,
        attempt_identity,
        checkpoint_release: Some(checkpoint_release),
        recovery_evidence: None,
        bridge: bridge.release(),
        relational: relational_basis.release(),
        attempt: resource_attempt.release(),
        run_counters,
        provider_work: provider_work.into_evidence(),
        yield_counters,
    };
    if recovery_required {
        WorthQueryDirectYieldCleanupOutcome::RecoveryRequired(receipt)
    } else {
        WorthQueryDirectYieldCleanupOutcome::Complete(receipt)
    }
}

pub(super) fn terminalized_cleanup_receipt(
    parts: WorthQueryDirectYieldCleanupReceiptParts,
) -> WorthQueryDirectYieldCleanupReceipt {
    WorthQueryDirectYieldCleanupReceipt {
        logical_run_identity: parts.logical_run_identity,
        attempt_identity: parts.attempt_identity,
        checkpoint_release: None,
        recovery_evidence: parts.recovery_evidence,
        bridge: parts.bridge,
        relational: parts.relational,
        attempt: parts.attempt,
        run_counters: parts.run_counters,
        provider_work: parts.provider_work,
        yield_counters: parts.yield_counters,
    }
}
