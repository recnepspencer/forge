#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StoreBackendCapabilityTier {
    Bootstrap,
    SemanticCertification,
    Compatibility,
    PhysicalFoundation,
    PlatformGrade,
}

pub trait BackendTierMarker: private::Sealed {
    const TIER: StoreBackendCapabilityTier;
}

mod private {
    #[allow(dead_code)]
    pub trait Sealed {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapBackend;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticCertificationBackend;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompatibilityBackend;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalFoundationBackend;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformGradeBackend;

impl private::Sealed for BootstrapBackend {}

impl BackendTierMarker for BootstrapBackend {
    const TIER: StoreBackendCapabilityTier = StoreBackendCapabilityTier::Bootstrap;
}

impl private::Sealed for SemanticCertificationBackend {}

impl BackendTierMarker for SemanticCertificationBackend {
    const TIER: StoreBackendCapabilityTier = StoreBackendCapabilityTier::SemanticCertification;
}

impl private::Sealed for CompatibilityBackend {}

impl BackendTierMarker for CompatibilityBackend {
    const TIER: StoreBackendCapabilityTier = StoreBackendCapabilityTier::Compatibility;
}

impl private::Sealed for PhysicalFoundationBackend {}

impl BackendTierMarker for PhysicalFoundationBackend {
    const TIER: StoreBackendCapabilityTier = StoreBackendCapabilityTier::PhysicalFoundation;
}

impl private::Sealed for PlatformGradeBackend {}

impl BackendTierMarker for PlatformGradeBackend {
    const TIER: StoreBackendCapabilityTier = StoreBackendCapabilityTier::PlatformGrade;
}
