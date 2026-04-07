use crate::facade::RawCommittedPatchEnvelope;

use super::super::fixtures::{
    InMemoryRelationalBridgeSource, RecordingSignalBridgeSink, SnapshotFixture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeHarnessMutation {
    PublishCommittedPatch(RawCommittedPatchEnvelope),
    PublishSnapshot(SnapshotFixture),
}

#[derive(Debug, Clone)]
pub struct BridgeHarnessSession {
    pub(crate) runtime: Option<crate::facade::RuntimeBridge>,
    pub(crate) source: InMemoryRelationalBridgeSource,
    pub(crate) sink: RecordingSignalBridgeSink,
}

impl Default for BridgeHarnessSession {
    fn default() -> Self {
        Self {
            runtime: None,
            source: InMemoryRelationalBridgeSource::default(),
            sink: RecordingSignalBridgeSink::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeHarnessError(String);

impl BridgeHarnessError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl std::fmt::Display for BridgeHarnessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for BridgeHarnessError {}

#[derive(Debug, Clone, Copy, Default)]
pub struct BridgeHarnessAdapter;
