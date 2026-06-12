mod artifact;
mod aspect_gate;
mod checked;
mod checked_input;
mod contract;
mod denial;
mod digest;
mod explain;
mod handle_gate;
mod lower;
mod lower_identity;
mod request;

pub use artifact::{
    ForgeQueryDeclarationBridgeBinding, ForgeQueryDeclarationBridgeRouting,
    ForgeQueryDeclarationBridgeRoutingClass,
};
pub use checked::{
    ForgeQueryDeclarationBridgeRoutingChecked, ForgeQueryDeclarationBridgeRoutingInput,
};
pub use contract::{
    ForgeQueryDeclarationBridgeContinuationContract, ForgeQueryDeclarationBridgeContinuationFamily,
    ForgeQueryDeclarationBridgeRoutingSupportReport, ForgeQueryDeclarationBridgeRoutingSupportRow,
    ForgeQueryDeclarationBridgeRoutingSupportStatus,
};
pub use denial::{
    ForgeQueryDeclarationBridgeRoutingDeferred, ForgeQueryDeclarationBridgeRoutingDenialCause,
    ForgeQueryDeclarationBridgeRoutingDenied, ForgeQueryDeclarationBridgeRoutingFailed,
    ForgeQueryDeclarationBridgeRoutingTerminalError, ForgeQueryDeclarationEntryBridgeRoutingError,
};
pub use explain::ForgeQueryDeclarationBridgeRoutingExplanation;
pub use request::{
    ForgeQueryDeclarationBridgeContinuationMode, ForgeQueryDeclarationBridgeContinuationRequest,
    ForgeQueryDeclarationBridgeTruthContext,
};

pub(crate) use checked::forge_query_checked_declaration_bridge_routing_on_handle;
pub(crate) use contract::derive_bridge_routing_support_report;
pub(crate) use lower_identity::{
    query_truth_branch_identity, query_truth_commit_identity, query_truth_snapshot_identity,
};

#[cfg(test)]
mod tests;
