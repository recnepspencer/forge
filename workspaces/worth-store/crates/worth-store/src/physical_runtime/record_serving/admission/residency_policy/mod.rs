mod admitted;
mod builder;
mod defaults;
mod denial;

pub use admitted::AdmittedPhysicalRecordResidencyPolicy;
pub use builder::{PhysicalRecordResidencyPolicy, PhysicalRecordResidencyPolicyBuilder};
pub use denial::PhysicalRecordResidencyPolicyDenial;
pub use worth_store_buffer_pool::{
    PhysicalOperationAllocationScope, PhysicalResidencyDimension, PhysicalSpeculativeWorkKind,
};

pub type PhysicalRecordResidencyPolicyOutcome = worth_proof::DenialTransitionOutcome<
    AdmittedPhysicalRecordResidencyPolicy,
    PhysicalRecordResidencyPolicyDenial,
>;

pub(super) use defaults::canonical_residency_policy;
