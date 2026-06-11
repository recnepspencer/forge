mod closure;
mod registry;
mod report;
#[cfg(test)]
mod tests;

pub use crate::query_context::QueryContextDeferredScopeMarker;
pub use closure::{
    ForgeQueryEvidenceIdentityBoundaryClosure, ForgeQueryFolkloreResidueStatus,
    ForgeQueryIdentityBoundaryClosure, ForgeQueryMilestoneClosureStatus,
    ForgeQuerySessionLabelBoundaryClosure, ForgeQueryStopClassBoundaryClosure,
};
pub use registry::{
    ForgeQueryCapabilityDescriptor, ForgeQueryCapabilityFamily, ForgeQueryCapabilityRegistry,
    ForgeQueryCapabilityStatus, ForgeQueryCapabilitySupportStatus, ForgeQuerySupportMatrix,
};
pub use report::{
    ForgeQueryIdentityEvolutionSupportProfile, ForgeQueryQueryCompositionSupportProfile,
    ForgeQueryQueryContextSupportProfile, ForgeQuerySupportReport, ForgeQuerySupportReportCounters,
    ForgeQuerySupportSectionPosture,
};
