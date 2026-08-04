mod mutation;
mod posture;
mod protocol;

pub use mutation::{
    layout_mutation_admission, layout_mutation_admission_cases, LayoutMutationAdmission,
    LayoutMutationAdmissionCaseId, LayoutMutationAdmissionOutcome, LayoutMutationAdmissionView,
    LayoutMutationPlan,
};
pub use posture::{
    live_maintenance_posture, live_maintenance_posture_cases, AdvisoryMaintenanceCapability,
    DeferredMaintenanceWitness, IndexLagWitness, LazyMaintenanceCapability, LiveMaintenancePosture,
    LiveMaintenancePostureAdmission, LiveMaintenancePostureCaseId, LiveMaintenancePostureOutcome,
    LiveMaintenancePostureView, LiveMaintenanceRequest, MigrationMaintenanceCapability,
    RebuildOnlyMaintenanceCapability, VerifierMaintenanceCapability,
};
pub use protocol::{IndexMaintenanceFailureOutcome, IndexPublicationProtocol};

use super::{IndexMaintenanceMode, PhysicalMutationShape};
