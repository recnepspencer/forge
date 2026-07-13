mod admission;
#[cfg(test)]
mod admission_case_tests;
mod basis;
mod corruption;
mod entrypoint;
mod exact_certification;
mod failure;
mod identity;
mod lag;
mod lag_observation;
#[cfg(test)]
pub(crate) mod live_tests;
mod lowering;
mod lsm;
mod maintenance_mode;
mod mutation_plan;
mod outcome;
mod parity;
mod plan;
mod publication_protocol;
mod rebuild;
mod scope;
mod source;
#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
pub(crate) mod tests;
mod transition;

pub use basis::{DerivedIndexParityBasis, DerivedIndexParityRow};
pub use corruption::LayoutCorruptionClassification;
pub use entrypoint::{layout_maintenance, LayoutMaintenanceFacade};
pub use failure::{
    IndexMaintenanceFailureOutcome, MutationProofRequirement, PublicationProofRequirement,
};
pub use identity::{
    DerivedIndexCostEnvelopeParity, DerivedIndexCounterShapeParity, DerivedIndexCoverageParity,
    DerivedIndexIdentityParity, DerivedIndexOrderingParity,
};
pub use lag::{IndexLagOutcome, IndexLagWitness, LagReason};
pub use lsm::{
    layout_lsm_maintenance, LayoutLsmMaintenance, LsmCompactionAdmissionRequest,
    LsmMaintenanceAdmissionDenied, LsmReplayAdmissionRequest, LsmRunPublicationAdmissionRequest,
};
pub use maintenance_mode::IndexMaintenanceMode;
pub use mutation_plan::{
    ExactPublicationAuthoritySource, LayoutMutationPlan, LiveExactMaintenanceWitness,
    LiveMaintenanceRequest, LsmManifestPublicationBinding, PhysicalMutationShape,
};
pub use outcome::{
    DerivedIndexParityOutcome, DerivedIndexRebuildDenied, DerivedIndexRebuildOutcome,
};
#[cfg(test)]
pub(crate) use outcome::{DerivedIndexParityView, DerivedIndexRebuildView};
pub use parity::DerivedIndexParityWitness;
pub use plan::{DerivedIndexRebuildPlan, DerivedIndexRebuildRequest, DerivedIndexResultIdentity};
pub use publication_protocol::IndexPublicationProtocol;
pub use rebuild::{layout_rebuild, DerivedIndexRebuildReceipt, LayoutRebuildFacade};
pub use scope::{DerivedIndexPartialKeySpace, DerivedIndexRebuildScope};
pub use source::DerivedIndexRebuildSourceInput;
pub use transition::{
    maintenance_admission_cases, AdvisoryMaintenancePlan, ExactMaintenancePlan,
    ExactMaintenanceProtocol, LaggedMaintenancePlan, LaggedMaintenanceProtocol,
    LayoutMutationAdmissionOutcome, LayoutMutationAdmissionView, LazyMaintenancePlan,
    MaintenanceAdmissionCaseId, MigrationMaintenancePlan, RebuildMaintenancePlan,
    VerifierMaintenancePlan, VerifierMaintenanceProtocol,
};
