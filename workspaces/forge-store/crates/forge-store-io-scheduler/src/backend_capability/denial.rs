use forge_store_physical_backend::BackendCapabilityAdmissionDenial;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoSchedulerBackendCapabilityDenial {
    BackendCapabilityDenied(BackendCapabilityAdmissionDenial),
    SecureFrameRequiresSecurityScope,
}
