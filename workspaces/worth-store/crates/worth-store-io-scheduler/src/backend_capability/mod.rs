mod admission;
mod denial;
mod requirement;

pub use admission::{
    admit_backend_capability_for_scheduler_claim,
    admit_secure_frame_backend_capability_for_scheduler_claim,
    IoSchedulerBackendCapabilityAdmission,
};
pub use denial::IoSchedulerBackendCapabilityDenial;
pub use requirement::IoSchedulerBackendCapabilityRequirement;

#[cfg(test)]
mod tests;
