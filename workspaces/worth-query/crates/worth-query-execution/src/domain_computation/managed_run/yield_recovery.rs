use worth_runtime_bridge::facade::{
    BridgeExecutionBasisFinalizationFailureKind, BridgeExecutionBasisSignalTerminal,
};

use super::WorthQueryProviderCheckpointSuspensionFailureKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryYieldRecoveryKind {
    BridgeTerminalization(BridgeExecutionBasisFinalizationFailureKind),
    SignalAttemptAlreadyTerminal(BridgeExecutionBasisSignalTerminal),
    ProviderCheckpointSuspension(WorthQueryProviderCheckpointSuspensionFailureKind),
    RetainedBytesExceeded,
}
