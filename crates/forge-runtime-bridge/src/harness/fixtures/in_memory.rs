use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::adapter::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest, CommittedPatchSource,
    ContinuityLineageSource, InvalidationSink, RelationalBridgeSourceError, SignalBridgeSinkError,
    SnapshotReadSource, TruthBranchHeadSource,
};
use crate::delivery::BridgeDeliveryReceipt;
use crate::facade::{
    BridgeAspectRegistration, BridgeLineageContext, BridgeLineageSourceError,
    BridgeLineageSourceErrorKind, BridgeMappingRegistration, BridgeRuntimePolicy,
    RawCommittedPatchEnvelope, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
    TruthSnapshotIdentity, TruthSnapshotReader,
};

mod fixture_surface;
mod sink_surface;
mod source_surface;

pub use fixture_surface::{BridgeHarnessFixture, SnapshotFixture};
pub use sink_surface::RecordingSignalBridgeSink;
pub use source_surface::InMemoryRelationalBridgeSource;
