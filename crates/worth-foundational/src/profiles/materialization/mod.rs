mod decisions;
mod inventory;
mod observation;
mod planning;
mod vocabulary;

pub use inventory::{
    boundary_artifact_surface_inventory, foundational_profile_applicability,
    proof_bearing_artifact_surface_inventory, support_artifact_surface_inventory,
    FoundationalProfileApplicability, FoundationalProfileDecisionKind, FoundationalProfileFamily,
    FoundationalTargetSurfaceInventory,
};
pub use observation::{FoundationalObservationActivationScope, FoundationalObservationDisposition};
pub use planning::{
    plan_foundational_profile_materialization,
    plan_foundational_profile_materialization_with_elision,
    plan_selected_foundational_profile_materialization,
    plan_selected_foundational_profile_materialization_with_disposition,
    FoundationalMaterializationCost, FoundationalMaterializationPlanningDenial,
    FoundationalProfileMaterializationPlan,
};
pub use vocabulary::{
    FoundationalDescriptiveElisionProfile, FoundationalDescriptiveSurface,
    FoundationalSurfaceAbsenceCause, FoundationalSurfaceAvailabilityDecision,
};
