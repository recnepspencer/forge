use std::sync::Arc;

use worth_relational::facade::runtime::{
    RelationalExecutionBasisLease, RelationalExecutionBasisReleaseReceipt,
};
use worth_runtime_bridge::facade::{
    BridgeBoundExecutionBasis, BridgeExecutionBasisFinalizationReceipt,
    BridgeExecutionBasisTerminalDisposition,
};

use super::{
    provider_work::WorthQueryManagedProviderCleanupAuthority,
    WorthQueryManagedProviderWorkEvidence, WorthQueryManagedRunCleanupDisposition,
    WorthQueryManagedRunCleanupFailureKind, WorthQueryManagedRunCounters,
    WorthQueryManagedRunTerminalKind,
};
use crate::domain_computation::{
    WorthQueryDirectExecutionAttemptReleaseReceipt, WorthQueryDirectExecutionResourceAttempt,
};

pub struct WorthQueryDirectRunTerminal {
    pub(super) logical_run_identity: Arc<str>,
    pub(super) identity: Arc<str>,
    pub(super) kind: WorthQueryManagedRunTerminalKind,
    pub(super) resource_attempt: WorthQueryDirectExecutionResourceAttempt,
    pub(super) bridge_basis: BridgeBoundExecutionBasis,
    pub(super) relational_basis: RelationalExecutionBasisLease,
    pub(super) counters: WorthQueryManagedRunCounters,
    pub(super) provider_work: WorthQueryManagedProviderWorkEvidence,
    pub(super) provider_cleanup: WorthQueryManagedProviderCleanupAuthority,
}

impl WorthQueryDirectRunTerminal {
    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn kind(&self) -> WorthQueryManagedRunTerminalKind {
        self.kind
    }

    pub fn provider_work(&self) -> &WorthQueryManagedProviderWorkEvidence {
        &self.provider_work
    }

    pub fn counters(&self) -> &WorthQueryManagedRunCounters {
        &self.counters
    }

    pub fn cleanup(
        self,
    ) -> Result<WorthQueryDirectRunCleanupReceipt, WorthQueryDirectRunCleanupFailure> {
        let WorthQueryDirectRunTerminal {
            logical_run_identity,
            identity,
            kind,
            resource_attempt,
            mut bridge_basis,
            relational_basis,
            counters,
            mut provider_work,
            mut provider_cleanup,
        } = self;
        if let Err(failure) =
            provider_cleanup.release_queue_occupancies(&mut bridge_basis, &mut provider_work)
        {
            return Err(WorthQueryDirectRunCleanupFailure {
                failure_kind: WorthQueryManagedRunCleanupFailureKind::QueueRelease(failure.kind()),
                failure_detail: Arc::from(failure.detail()),
                terminal: WorthQueryDirectRunTerminal {
                    logical_run_identity,
                    identity,
                    kind,
                    resource_attempt,
                    bridge_basis,
                    relational_basis,
                    counters,
                    provider_work,
                    provider_cleanup,
                },
            });
        }
        let provider_retained_bytes = provider_cleanup.reconcile_provider_memory();
        provider_work.reconcile_provider_retained_bytes(provider_retained_bytes);
        if provider_retained_bytes != 0 {
            return Err(WorthQueryDirectRunCleanupFailure {
                failure_kind: WorthQueryManagedRunCleanupFailureKind::ProviderMemoryRetained,
                failure_detail: Arc::from(format!(
                    "managed provider retains {provider_retained_bytes} governed bytes"
                )),
                terminal: WorthQueryDirectRunTerminal {
                    logical_run_identity,
                    identity,
                    kind,
                    resource_attempt,
                    bridge_basis,
                    relational_basis,
                    counters,
                    provider_work,
                    provider_cleanup,
                },
            });
        }
        let bridge = match bridge_basis.finalize(bridge_terminal_disposition(kind)) {
            Ok(receipt) => receipt,
            Err(failure) => {
                let failure_kind = failure.kind();
                let failure_detail = Arc::from(failure.detail());
                return Err(WorthQueryDirectRunCleanupFailure {
                    failure_kind: WorthQueryManagedRunCleanupFailureKind::BridgeFinalization(
                        failure_kind,
                    ),
                    failure_detail,
                    terminal: WorthQueryDirectRunTerminal {
                        logical_run_identity,
                        identity,
                        kind,
                        resource_attempt,
                        bridge_basis: failure.into_basis(),
                        relational_basis,
                        counters,
                        provider_work,
                        provider_cleanup,
                    },
                });
            }
        };
        let relational = relational_basis.release();
        let attempt = resource_attempt.release();
        Ok(WorthQueryDirectRunCleanupReceipt {
            logical_run_identity,
            identity,
            terminal: kind,
            disposition: if provider_work.requires_cleanup_recovery() {
                WorthQueryManagedRunCleanupDisposition::RecoveryRequired
            } else {
                WorthQueryManagedRunCleanupDisposition::CleanupComplete
            },
            bridge,
            relational,
            attempt,
            counters,
            provider_work,
        })
    }
}

pub(super) fn bridge_terminal_disposition(
    terminal: WorthQueryManagedRunTerminalKind,
) -> BridgeExecutionBasisTerminalDisposition {
    match terminal {
        WorthQueryManagedRunTerminalKind::Completed => {
            BridgeExecutionBasisTerminalDisposition::Completed
        }
        WorthQueryManagedRunTerminalKind::Cancelled => {
            BridgeExecutionBasisTerminalDisposition::Cancelled
        }
        WorthQueryManagedRunTerminalKind::TimedOut
        | WorthQueryManagedRunTerminalKind::Exhausted
        | WorthQueryManagedRunTerminalKind::Degraded
        | WorthQueryManagedRunTerminalKind::Failed => {
            BridgeExecutionBasisTerminalDisposition::Abandoned
        }
    }
}

pub struct WorthQueryDirectRunCleanupFailure {
    failure_kind: WorthQueryManagedRunCleanupFailureKind,
    failure_detail: Arc<str>,
    terminal: WorthQueryDirectRunTerminal,
}

impl WorthQueryDirectRunCleanupFailure {
    pub fn failure_kind(&self) -> WorthQueryManagedRunCleanupFailureKind {
        self.failure_kind
    }

    pub fn failure_detail(&self) -> &str {
        &self.failure_detail
    }

    pub fn provider_retained_bytes(&self) -> usize {
        self.terminal.provider_work.provider_retained_bytes()
    }

    pub fn disposition(&self) -> WorthQueryManagedRunCleanupDisposition {
        match self.failure_kind {
            WorthQueryManagedRunCleanupFailureKind::ProviderMemoryRetained => {
                WorthQueryManagedRunCleanupDisposition::CleanupPending
            }
            WorthQueryManagedRunCleanupFailureKind::QueueRelease(_)
            | WorthQueryManagedRunCleanupFailureKind::BridgeFinalization(_) => {
                WorthQueryManagedRunCleanupDisposition::RecoveryRequired
            }
        }
    }

    pub fn retry(
        self,
    ) -> Result<WorthQueryDirectRunCleanupReceipt, WorthQueryDirectRunCleanupFailure> {
        self.terminal.cleanup()
    }

    pub fn into_terminal(self) -> WorthQueryDirectRunTerminal {
        self.terminal
    }
}

impl std::fmt::Debug for WorthQueryDirectRunCleanupFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryDirectRunCleanupFailure")
            .field("failure_kind", &self.failure_kind)
            .field("failure_detail", &self.failure_detail)
            .field("run_identity", &self.terminal.identity())
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryDirectRunCleanupReceipt {
    logical_run_identity: Arc<str>,
    identity: Arc<str>,
    terminal: WorthQueryManagedRunTerminalKind,
    disposition: WorthQueryManagedRunCleanupDisposition,
    bridge: BridgeExecutionBasisFinalizationReceipt,
    relational: RelationalExecutionBasisReleaseReceipt,
    attempt: WorthQueryDirectExecutionAttemptReleaseReceipt,
    counters: WorthQueryManagedRunCounters,
    provider_work: WorthQueryManagedProviderWorkEvidence,
}

impl WorthQueryDirectRunCleanupReceipt {
    pub fn run_identity(&self) -> &str {
        &self.identity
    }

    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn terminal(&self) -> WorthQueryManagedRunTerminalKind {
        self.terminal
    }

    pub fn disposition(&self) -> WorthQueryManagedRunCleanupDisposition {
        self.disposition
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

    pub fn counters(&self) -> &WorthQueryManagedRunCounters {
        &self.counters
    }
}
