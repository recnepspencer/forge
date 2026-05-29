mod artifact;
mod checked;
mod contract;
mod denial;
mod digest;
mod explain;
mod input;
mod lower;

pub use artifact::{
    ForgeQueryDeclarationRelationalBinding, ForgeQueryDeclarationRelationalRouting,
    ForgeQueryDeclarationRelationalRoutingClass,
};
pub use checked::ForgeQueryDeclarationRelationalRoutingChecked;
pub use contract::{
    ForgeQueryDeclarationRelationalAuthorityFamily,
    ForgeQueryDeclarationRelationalRoutingSupportReport,
    ForgeQueryDeclarationRelationalRoutingSupportRow, ForgeQueryDeclarationRelationalTruthClaim,
    ForgeQueryDeclarationRelationalTruthContract,
    ForgeQueryDeclarationRelationalTruthRoutingSupportStatus,
};
pub use denial::{
    ForgeQueryDeclarationEntryRelationalRoutingError,
    ForgeQueryDeclarationRelationalRoutingDeferred,
    ForgeQueryDeclarationRelationalRoutingDenialCause,
    ForgeQueryDeclarationRelationalRoutingDenied, ForgeQueryDeclarationRelationalRoutingFailed,
    ForgeQueryDeclarationRelationalRoutingTerminalError,
};
pub use explain::ForgeQueryDeclarationRelationalRoutingExplanation;
pub use input::ForgeQueryDeclarationRelationalRoutingInput;

pub(crate) use checked::forge_query_checked_declaration_relational_routing_on_handle;
pub(crate) use contract::derive_relational_routing_support_report;

#[cfg(test)]
mod tests;
