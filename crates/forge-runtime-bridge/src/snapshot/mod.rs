//! Bridge snapshot contracts and packetized read surfaces.

mod context;
mod declaration;
mod history;
mod materialization;
mod packet;
mod policy;
mod selection;
pub(crate) mod token;

pub use context::{AdmittedSnapshotContext, BridgeSnapshotContext, TruthSnapshotReader};
pub use declaration::{
    BridgeDeliveryIntent, BridgeReplayMode, BridgeTruthViewKind, BridgeTruthViewSelector,
    BridgeTruthViewSelectorIdentity, HistoricalEvaluationDeclaration,
    HistoricalEvaluationDeclarationIdentity, ValidatedTruthViewSelectorSet,
};
pub use history::{
    LoweredHistoricalEvaluationArtifact, LoweredHistoricalEvaluationArtifactIdentity,
};
pub use materialization::{
    MaterializedTruthViewObservation, TruthViewObservationReader,
};
pub(crate) use packet::{canonical_subscription_slice_kind_label, validate_snapshot_read_result_contract};
pub use packet::{
    BridgeSnapshotReadError, SnapshotReadPacket, SnapshotReadPacketResult, SnapshotReadRecord,
    SnapshotReadRequest, ValidatedSnapshotReadPacketResult,
};
pub use policy::{
    BridgeTruthViewPolicyRejection, BridgeTruthViewPolicyResolution, ResolvedTruthViewPolicy,
    TruthViewPolicyRejectionKind, TruthViewReplayCompatibility,
    TruthViewRetentionAdmission, TruthViewSourceCapability,
};
pub use selection::{BridgeTruthViewAuthorityBasis, PlannedTruthViewPacket};
pub use token::{BridgeSnapshotToken, TruthSnapshotIdentity};
