use worth_store_io_scheduler::{
    IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityDenial,
    IoSchedulerBackendCapabilityRequirement,
};
use worth_store_physical_backend::QualifiedFilesystemMedia;

/// Store-owned admission route from qualified media evidence into scheduler
/// capability. It owns no media and cannot execute a physical effect.
pub(in crate::physical_runtime) struct PhysicalSchedulerAdmissionOwner;

impl PhysicalSchedulerAdmissionOwner {
    pub(super) const fn new() -> Self {
        Self
    }

    pub(in crate::physical_runtime) fn admit(
        &self,
        media: &QualifiedFilesystemMedia,
        requirement: IoSchedulerBackendCapabilityRequirement,
    ) -> Result<IoSchedulerBackendCapabilityAdmission, IoSchedulerBackendCapabilityDenial> {
        let claim = media
            .scheduler_capability_claim(
                requirement.capability_kind(),
                requirement.required_evidence(),
            )
            .map_err(IoSchedulerBackendCapabilityDenial::BackendCapabilityDenied)?;
        worth_store_io_scheduler::admit_backend_capability_for_scheduler_qualified_claim(
            claim,
            requirement,
        )
    }
}
