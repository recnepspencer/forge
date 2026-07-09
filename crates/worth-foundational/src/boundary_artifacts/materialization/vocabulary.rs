#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryMaterializationSource {
    NativeAuthority,
    CompatibilityLowered,
    DerivedSupport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryMaterializationSeam {
    BoundaryExchange,
    SupportMaterialization,
    PersistenceExport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryDeliveryClass {
    MustBeHot,
    CanDefer,
    ReconstructableFromRetainedBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryAvailability {
    Present,
    Deferred,
    Reconstructable,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryDecisionSubject {
    CategoryRoleAdmission,
    DeliveryAvailabilityResolution,
    AttachmentInclusion,
    AttachmentElision,
    BundleMembership,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryDecisionCause {
    RequestedAsAdmitted,
    NarrowedByAuthority,
    ElidedByProfile,
    DeniedByBudget,
    UnavailableByRetention,
    ReconstructableFromRetainedBasis,
    DeferredBySupportPosture,
    DeniedByMilestoneBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBoundaryAttachmentPoint {
    ProfileMeaning,
    ProfileDecisions,
    CanonicalBasis,
    DiagnosticsAttachment,
    ProvenanceAttachment,
    PerformanceAccounting,
    SameFamilyResolutionAttachment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundarySurfaceDisposition {
    delivery_class: FoundationalBoundaryDeliveryClass,
    availability: FoundationalBoundaryAvailability,
}

impl FoundationalBoundarySurfaceDisposition {
    pub(crate) const fn new(
        delivery_class: FoundationalBoundaryDeliveryClass,
        availability: FoundationalBoundaryAvailability,
    ) -> Self {
        Self {
            delivery_class,
            availability,
        }
    }

    pub const fn delivery_class(&self) -> FoundationalBoundaryDeliveryClass {
        self.delivery_class
    }

    pub const fn availability(&self) -> FoundationalBoundaryAvailability {
        self.availability
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundarySurfaceDispositionDenial {
    MustBeHotCannotDefer,
    MustBeHotCannotReconstruct,
    MustBeHotCannotBeUnavailable,
    DeferredDeliveryCannotReconstruct,
    ReconstructableDeliveryCannotAppearPresent,
    ReconstructableDeliveryCannotDefer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundarySurfaceDispositionLegality {
    disposition: FoundationalBoundarySurfaceDisposition,
}

impl FoundationalBoundarySurfaceDispositionLegality {
    pub(crate) const fn new(disposition: FoundationalBoundarySurfaceDisposition) -> Self {
        Self { disposition }
    }

    pub const fn disposition(&self) -> FoundationalBoundarySurfaceDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundaryPlanningDenial {
    NativeAuthorityCannotUseSupportMaterialization,
    CompatibilityLoweredCannotUseSupportMaterialization,
    DerivedSupportCannotUseBoundaryExchange,
    DerivedSupportCannotUsePersistenceExport,
    IllegalSurfaceDisposition(FoundationalBoundarySurfaceDispositionDenial),
}
