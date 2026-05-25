mod artifact;
mod checked;
mod contract;
mod denial;
mod digest;
mod explain;
mod lower;
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

#[cfg(test)]
mod tests;
