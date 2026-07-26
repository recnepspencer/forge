use worth_runtime_bridge::facade::{
    BridgeExecutionBasisFinalizationFailureKind, BridgeExecutionBasisSignalTerminal,
};

use super::WorthQueryProviderCheckpointSuspensionFailureKind;
use crate::domain_computation::artifact_owner::WorthQueryArtifactDenialKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryYieldRecoveryKind {
    ArtifactProductionFreeze(WorthQueryArtifactDenialKind),
    BridgeTerminalization(BridgeExecutionBasisFinalizationFailureKind),
    SignalAttemptAlreadyTerminal(BridgeExecutionBasisSignalTerminal),
    ProviderCheckpointSuspension(WorthQueryProviderCheckpointSuspensionFailureKind),
    RetainedBytesExceeded,
}
