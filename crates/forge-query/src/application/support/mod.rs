mod registry;
mod report;

pub use crate::query_context::QueryContextDeferredScopeMarker;
pub use registry::{
    ForgeQueryCapabilityDescriptor, ForgeQueryCapabilityFamily, ForgeQueryCapabilityRegistry,
    ForgeQueryCapabilityStatus, ForgeQueryCapabilitySupportStatus, ForgeQuerySupportMatrix,
};
pub use report::{
    ForgeQueryIdentityEvolutionSupportProfile, ForgeQueryQueryContextSupportProfile,
    ForgeQuerySupportReport, ForgeQuerySupportReportCounters, ForgeQuerySupportSectionPosture,
};
