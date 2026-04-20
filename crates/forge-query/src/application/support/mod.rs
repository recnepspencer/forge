mod registry;
mod report;

pub use crate::query_context::QueryContextDeferredScopeMarker;
pub use registry::{
    ForgeQueryCapabilityDescriptor, ForgeQueryCapabilityFamily, ForgeQueryCapabilityRegistry,
    ForgeQueryCapabilityStatus, ForgeQueryCapabilitySupportStatus, ForgeQuerySupportMatrix,
};
pub use report::{
    ForgeQueryIdentityEvolutionSupportProfile, ForgeQueryQueryCompositionSupportProfile,
    ForgeQueryQueryContextSupportProfile, ForgeQuerySupportReport,
    ForgeQuerySupportReportCounters, ForgeQuerySupportSectionPosture,
};
