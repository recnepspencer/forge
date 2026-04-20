mod admission;
mod compatibility;
mod delivery;
mod descriptor;
mod digest;
mod error;
mod family;
mod grouped_binding;
mod grouped_maintenance;
mod grouped_planning;
mod grouped_policy;
mod performance;
mod plan_artifact;
mod planning;
mod support;

pub use admission::{admit_view_shape, AdmittedViewShape};
pub use compatibility::ViewShapeCompatibilityMatrixArtifact;
pub use delivery::{
    ViewShapeDeliveryMetadata, ViewShapeInvalidationPosture, ViewShapePatchPosture,
};
pub use descriptor::ViewShapeDescriptor;
pub use digest::ViewShapeDigest;
pub use error::{ViewShapeError, ViewShapeFailureClass};
pub use family::ViewShapeFamily;
pub use grouped_binding::QueryResultBindingProof;
pub use grouped_maintenance::ViewShapeMaintenanceContract;
pub use grouped_planning::{
    GroupedBaselineMaterializationContract, GroupedViewPlanningArtifact,
};
pub use grouped_policy::{
    GroupedDeltaAdmissionPolicy, GroupedReplayDeliveryPosture, KanbanGroupedLiveContract,
};
pub use performance::{
    ViewShapeComplexityReport, ViewShapeComplexityStatus, ViewShapeCostClass,
};
pub use plan_artifact::{ViewShapePlanArtifact, ViewShapePlanDigest, ViewShapeValidatedBundle};
pub use planning::{plan_admitted_view_shape, validate_canonical_bundle_for_admitted_view_shape};
pub use support::runtime_backed_view_shape_support_profile;

#[cfg(test)]
mod tests;
