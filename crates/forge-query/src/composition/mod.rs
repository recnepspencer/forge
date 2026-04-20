mod counters;
mod digests;
mod errors;
mod expand;
mod families;
mod report;
mod scopes;
mod templates;

pub use counters::CompositionCounters;
pub use digests::{CompositionDigest, ScopeLineageDigest, TemplateBindingDigest};
pub use errors::{QueryCompositionAdmissionFailureClass, QueryCompositionError};
pub use expand::{ComposedCanonicalQueryBundle, ExpandedComposedIntent, GuidedCompositionPath};
pub use families::{
    QueryCompositionComplexityStatus, QueryCompositionFamily, ScopeFamily, TemplateFamily,
};
pub use report::{
    runtime_backed_query_composition_support_profile, CompositionReport,
    QueryCompositionDeferredScopeMarker, QueryCompositionSupportProfile,
};
pub use scopes::{BasisScopeEvidence, ExpandedScopeArtifact, QueryScopeDescriptor};
pub use templates::{
    QueryTemplateDescriptor, TemplateBindingSet, TemplateInstantiationArtifact,
    TemplateParameterSlot, TemplateParameterSlotKind,
};

#[cfg(test)]
mod tests;
