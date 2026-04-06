//! Bridge-owned parent-runtime adapter contracts.

use std::sync::Arc;

use crate::delivery::BridgeDeliveryReceipt;
use crate::error::BridgeMessageError;
use crate::input::envelope::RawCommittedPatchEnvelope;
use crate::routing::BridgeSignalInvalidationDelivery;
use crate::snapshot::{TruthSnapshotIdentity, TruthSnapshotReader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalBridgeSourceErrorTag {}
pub type RelationalBridgeSourceError = BridgeMessageError<RelationalBridgeSourceErrorTag>;

pub trait CommittedPatchSource: Send + Sync + 'static {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError>;
}

pub trait SnapshotReadSource: Send + Sync + 'static {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError>;
}

pub trait SnapshotReaderPool: Send + Sync + 'static {
    fn acquire(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError>;

    fn release(&self, reader: Box<dyn TruthSnapshotReader>);
}

pub trait RelationalBridgeSource: CommittedPatchSource + SnapshotReadSource {}

impl<T> RelationalBridgeSource for T where T: CommittedPatchSource + SnapshotReadSource {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalCommittedPatchRequest {
    commit_identity: Arc<str>,
}

impl RelationalCommittedPatchRequest {
    pub fn new(commit_identity: impl Into<Arc<str>>) -> Self {
        Self {
            commit_identity: commit_identity.into(),
        }
    }

    pub fn commit_identity(&self) -> &str {
        self.commit_identity.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalBridgeSinkErrorTag {}
pub type SignalBridgeSinkError = BridgeMessageError<SignalBridgeSinkErrorTag>;

pub trait InvalidationSink: Send + Sync + 'static {
    fn deliver_invalidation(
        &self,
        delivery: BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError>;
}

pub trait SignalBridgeSink: InvalidationSink {}

impl<T> SignalBridgeSink for T where T: InvalidationSink {}
