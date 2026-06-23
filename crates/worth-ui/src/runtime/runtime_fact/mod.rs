mod changed_facts;
mod fact_family;
mod fact_id;
mod fact_set;
mod fact_set_digest;
mod lowering;
mod projection_dependencies;
mod typed_fact_identity;

pub use fact_family::WorthUiRuntimeFactFamily;
pub use fact_id::WorthUiRuntimeFactId;
pub use fact_set::WorthUiRuntimeFactSet;
pub use fact_set_digest::WorthUiRuntimeFactSetDigest;
pub(crate) use lowering::WorthUiAuthoredStructuralRuntimeFactLowering;
pub use lowering::{
    WorthUiAuthoredStructuralChangedFactRow, WorthUiCapabilityDeltaRuntimeFactLowering,
    WorthUiQueryBindingRuntimeFactLowering, WorthUiValidationReloadRuntimeFactLowering,
};
pub use projection_dependencies::WorthUiProjectionDependencySet;
pub use typed_fact_identity::{
    WorthUiActionPostureId, WorthUiAppearanceRecipeId, WorthUiContentSlotId, WorthUiDensityTokenId,
    WorthUiInspectorSurfaceId, WorthUiOverlaySurfaceId, WorthUiPageInstanceId,
    WorthUiPageTemplateId, WorthUiRuntimeFactIdentityError, WorthUiShellSurfaceId,
    WorthUiToastSurfaceId,
};

#[cfg(test)]
mod runtime_fact_taxonomy_tests;
pub use changed_facts::{
    WorthUiCapabilityChangedFacts, WorthUiChangedRuntimeFacts, WorthUiChangedRuntimeFactsProof,
    WorthUiQueryBindingChangedFacts, WorthUiValidationChangedFacts,
};
