mod bundle;
mod bundle_types;
mod derivation;
mod model;
mod surface;
mod vocabulary;

pub use bundle::{plan_artifact_boundary_bundle, FoundationalBoundaryMaterializationBundlePlan};
pub use bundle_types::{
    FoundationalBoundaryBundleMaterializationCost, FoundationalBoundaryBundleMaterializationDenial,
    FoundationalBoundaryBundlePlanningDenial, FoundationalBoundaryMaterializationBundle,
};
pub use derivation::evaluate_boundary_surface_disposition_legality;
pub use model::{
    FoundationalBoundaryMaterializationAttachment, FoundationalBoundaryMaterializationCost,
    FoundationalBoundaryMaterializationDecisionRow, FoundationalBoundaryMaterializationDenial,
    FoundationalBoundaryMaterializationInput, FoundationalBoundaryMaterializationPlan,
    FoundationalMaterializedBoundaryArtifact,
};
pub use surface::{
    materialize_authoritative_boundary_surface, materialize_descriptive_boundary_surface,
    plan_authoritative_boundary_materialization, plan_descriptive_boundary_materialization,
};
pub use vocabulary::{
    FoundationalBoundaryAttachmentPoint, FoundationalBoundaryAvailability,
    FoundationalBoundaryDecisionCause, FoundationalBoundaryDecisionSubject,
    FoundationalBoundaryDeliveryClass, FoundationalBoundaryMaterializationSeam,
    FoundationalBoundaryMaterializationSource, FoundationalBoundaryPlanningDenial,
    FoundationalBoundarySurfaceDisposition, FoundationalBoundarySurfaceDispositionDenial,
    FoundationalBoundarySurfaceDispositionLegality,
};
