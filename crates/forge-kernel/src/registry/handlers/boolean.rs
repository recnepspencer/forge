//! Handler for Boolean commands (`BooleanUnion`, `BooleanSubtract`).
//!
//! DOMAIN: Constructs a `NativeFeature::boolean` from resolved entity
//! references and operation type. Entity resolution is done by the
//! dispatcher before calling this handler.

use forge_signal::facade::NodeId;

use super::super::native_feature::NativeFeature;
use crate::operations::boolean::BooleanOp;

/// Create a boolean feature from resolved target/tool node IDs and operation.
pub fn boolean(name: &str, op: BooleanOp, target: NodeId, tool: NodeId) -> NativeFeature {
    NativeFeature::boolean(name, op, target, tool)
}
