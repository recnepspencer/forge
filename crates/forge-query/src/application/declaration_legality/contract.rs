use forge_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole,
    FoundationalBoundaryAvailability, FoundationalBoundaryDeliveryClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationLegalityClass {
    AuthoritativeHotArtifact,
    DescriptiveDeferredSupport,
    PlannedUnavailableSupport,
    ReceiptHotBoundary,
    DeferredBoundary,
    UnsupportedBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationLegalityContract {
    legality_class: ForgeQueryDeclarationLegalityClass,
    category: FoundationalBoundaryArtifactCategory,
    role: FoundationalBoundaryArtifactRole,
    delivery_class: FoundationalBoundaryDeliveryClass,
    availability: FoundationalBoundaryAvailability,
}

impl ForgeQueryDeclarationLegalityContract {
    pub(crate) fn new(
        legality_class: ForgeQueryDeclarationLegalityClass,
        category: FoundationalBoundaryArtifactCategory,
        role: FoundationalBoundaryArtifactRole,
        delivery_class: FoundationalBoundaryDeliveryClass,
        availability: FoundationalBoundaryAvailability,
    ) -> Self {
        Self {
            legality_class,
            category,
            role,
            delivery_class,
            availability,
        }
    }

    pub fn authoritative_hot_artifact() -> Self {
        Self::new(
            ForgeQueryDeclarationLegalityClass::AuthoritativeHotArtifact,
            FoundationalBoundaryArtifactCategory::Artifact,
            FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
            FoundationalBoundaryDeliveryClass::MustBeHot,
            FoundationalBoundaryAvailability::Present,
        )
    }

    pub fn descriptive_deferred_support() -> Self {
        Self::new(
            ForgeQueryDeclarationLegalityClass::DescriptiveDeferredSupport,
            FoundationalBoundaryArtifactCategory::Artifact,
            FoundationalBoundaryArtifactRole::SupportOnly,
            FoundationalBoundaryDeliveryClass::CanDefer,
            FoundationalBoundaryAvailability::Deferred,
        )
    }

    pub fn planned_unavailable_support() -> Self {
        Self::new(
            ForgeQueryDeclarationLegalityClass::PlannedUnavailableSupport,
            FoundationalBoundaryArtifactCategory::Report,
            FoundationalBoundaryArtifactRole::PlannedWork,
            FoundationalBoundaryDeliveryClass::CanDefer,
            FoundationalBoundaryAvailability::Unavailable,
        )
    }

    pub fn receipt_hot_boundary() -> Self {
        Self::new(
            ForgeQueryDeclarationLegalityClass::ReceiptHotBoundary,
            FoundationalBoundaryArtifactCategory::Receipt,
            FoundationalBoundaryArtifactRole::ReceiptEvidence,
            FoundationalBoundaryDeliveryClass::MustBeHot,
            FoundationalBoundaryAvailability::Present,
        )
    }

    pub fn deferred_boundary() -> Self {
        Self::new(
            ForgeQueryDeclarationLegalityClass::DeferredBoundary,
            FoundationalBoundaryArtifactCategory::Report,
            FoundationalBoundaryArtifactRole::SupportOnly,
            FoundationalBoundaryDeliveryClass::CanDefer,
            FoundationalBoundaryAvailability::Deferred,
        )
    }

    pub fn unsupported_boundary() -> Self {
        Self::new(
            ForgeQueryDeclarationLegalityClass::UnsupportedBoundary,
            FoundationalBoundaryArtifactCategory::Summary,
            FoundationalBoundaryArtifactRole::SupportOnly,
            FoundationalBoundaryDeliveryClass::CanDefer,
            FoundationalBoundaryAvailability::Unavailable,
        )
    }

    pub fn legality_class(&self) -> ForgeQueryDeclarationLegalityClass {
        self.legality_class
    }

    pub fn category(&self) -> FoundationalBoundaryArtifactCategory {
        self.category
    }

    pub fn role(&self) -> FoundationalBoundaryArtifactRole {
        self.role
    }

    pub fn delivery_class(&self) -> FoundationalBoundaryDeliveryClass {
        self.delivery_class
    }

    pub fn availability(&self) -> FoundationalBoundaryAvailability {
        self.availability
    }
}
