#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhysicalDurabilityPolicyDenial {
    UnsupportedBackendProfile {
        profile: worth_store_physical_backend::BackendTargetProfile,
    },
    InvalidCapabilityBinding {
        capability: worth_store_physical_backend::BackendCapabilityKind,
    },
}

pub enum PhysicalDurabilityPolicyDeferred {}
pub enum PhysicalDurabilityPolicyStale {}
pub enum PhysicalDurabilityPolicyRebindRequired {}
pub enum PhysicalDurabilityPolicyFailure {}
