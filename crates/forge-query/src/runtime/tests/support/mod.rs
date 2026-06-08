pub(in crate::runtime::tests) use super::super::*;
pub(in crate::runtime::tests) use crate::declarative_live::{
    DeclarativeLiveViewShape, DeclarativeProjectionField,
};
pub(in crate::runtime::tests) use crate::program::{
    ForgeQueryAspectValueTemplate, ForgeQueryOperation, ForgeQueryPortType,
    ForgeQueryProgramSource, ForgeQuerySchemaAdapter, ForgeQueryTypedPort, ForgeQueryValueExpr,
    ForgeQueryWriteCommandTemplate,
};
pub(in crate::runtime::tests) use crate::runtime::async_result_state::ForgeQueryRuntimeAsyncResultProjection;
pub(in crate::runtime::tests) use crate::runtime::remask_posture::ForgeQueryRuntimeRemaskProjection;
pub(in crate::runtime::tests) use crate::schema_view::{SchemaFieldKind, SchemaFieldView};
pub(in crate::runtime::tests) use crate::subscription::QueryPatchGroupKind;
pub(in crate::runtime::tests) use forge_foundational::facade::{
    AspectKey, AspectLocator, CanonicalFieldPath, FieldKey, LocatorAuthority, ScalarAspectType,
};
pub(in crate::runtime::tests) use forge_runtime_bridge::facade::{
    BridgeAsyncCompletionClass, BridgeAsyncCompletionDenialClass, BridgeAsyncCompletionState,
    BridgeAsyncForwardCausalityClass, BridgeCommittedPatchEnvelope,
    BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem, BridgeCommittedPatchTarget,
    BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode,
    InvalidationSink, MappingSelector, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SnapshotReadContract, SnapshotReadPacket, SnapshotReadPacketResult,
    SnapshotReadRecord, SnapshotReadSource, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity, TruthSnapshotReader,
};
pub(in crate::runtime::tests) use serde_json::{json, Value};

mod adapters;
mod bridge;
mod commands;
mod domains;
mod graph_composition;
mod mixed_cause;
mod program;
mod schema;
mod stateful_bridge_runtime;

pub(in crate::runtime::tests) use adapters::*;
pub(in crate::runtime::tests) use bridge::*;
pub(in crate::runtime::tests) use commands::*;
pub(in crate::runtime::tests) use domains::*;
pub(in crate::runtime::tests) use graph_composition::*;
pub(in crate::runtime::tests) use mixed_cause::*;
pub(in crate::runtime::tests) use program::*;
pub(in crate::runtime::tests) use schema::*;
pub(crate) use stateful_bridge_runtime::*;

pub(in crate::runtime::tests) fn live_subscription_async_identity(
    runtime: &ForgeQueryRuntime,
    view_name: &str,
) -> (String, String) {
    let state = runtime
        .live_subscriptions
        .get(view_name)
        .expect("live subscription state should exist");
    (
        state.installation.basis_binding_digest().to_string(),
        state
            .active_lane_handle
            .checkpoint_identity_digest()
            .to_string(),
    )
}
