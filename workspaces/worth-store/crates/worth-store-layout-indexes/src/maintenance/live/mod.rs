mod copy_on_write_execution;
mod exact;
mod mutation;
mod posture;
mod protocol;

pub use exact::{
    live_exact_maintenance, live_exact_maintenance_cases, LiveExactMaintenance,
    LiveExactMaintenanceCaseId, LiveExactMaintenanceOutcome, LiveExactMaintenanceRequest,
    LiveExactMaintenanceView, LiveExactMaintenanceWitness,
};
pub use mutation::{
    layout_mutation_admission, layout_mutation_admission_cases, CopyOnWriteLayoutMutationPlan,
    CopyOnWriteLayoutMutationRequest, LayoutMutationAdmission, LayoutMutationAdmissionCaseId,
    LayoutMutationAdmissionOutcome, LayoutMutationAdmissionView, LayoutMutationPlan,
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
pub use copy_on_write_execution::{
    copy_on_write_layout_mutation_execution, copy_on_write_layout_mutation_execution_cases,
    CopyOnWriteLayoutMutationExecution, CopyOnWriteLayoutMutationExecutionCaseId,
    CopyOnWriteLayoutMutationExecutionOutcome, CopyOnWriteLayoutMutationExecutionView,
    CopyOnWriteLayoutMutationReceipt,
};
