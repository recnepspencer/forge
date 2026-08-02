mod drain;
mod managed_work;
#[cfg(feature = "certification-test-authority")]
mod yieldpoint;

pub use drain::PhysicalMutationShutdown;
pub(in crate::physical_runtime) use managed_work::{
    PhysicalMutationRuntimeOwner, PhysicalMutationStartPort,
};
#[cfg(feature = "certification-test-authority")]
pub use yieldpoint::{
    CertificationPhysicalMutationCheckpoint, CertificationPhysicalMutationPauseGate,
};
