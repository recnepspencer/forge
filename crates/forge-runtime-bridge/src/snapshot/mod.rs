//! Bridge snapshot contracts and packetized read surfaces.

mod context;
mod packet;
pub(crate) mod token;

pub use context::{AdmittedSnapshotContext, BridgeSnapshotContext, TruthSnapshotReader};
pub(crate) use packet::{canonical_subscription_slice_kind_label, validate_snapshot_read_result_contract};
pub use packet::{
    BridgeSnapshotReadError, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
    SnapshotReadRequest, ValidatedSnapshotReadPacketResult,
};
pub use token::{BridgeSnapshotToken, TruthSnapshotIdentity};
