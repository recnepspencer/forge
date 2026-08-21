#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDescriptiveSurface {
    History,
    Replay,
    Lineage,
    Provenance,
    ForensicDiagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDescriptiveElisionProfile {
    FullFidelity,
    OperationalSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalSurfaceAbsenceCause {
    ObservationNotActivated,
    OmittedByActiveRichness,
    DeniedByBudget,
    NotRetained,
    NotReconstructable,
    DeferredBySupportPosture,
    UncertifiedForRequestedPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalSurfaceAvailabilityDecision {
    surface: FoundationalDescriptiveSurface,
    availability: Option<FoundationalSurfaceAbsenceCause>,
}

impl FoundationalSurfaceAvailabilityDecision {
    pub(crate) const fn available(surface: FoundationalDescriptiveSurface) -> Self {
        Self {
            surface,
            availability: None,
        }
    }

    pub(crate) const fn unavailable(
        surface: FoundationalDescriptiveSurface,
        cause: FoundationalSurfaceAbsenceCause,
    ) -> Self {
        Self {
            surface,
            availability: Some(cause),
        }
    }

    pub const fn surface(&self) -> FoundationalDescriptiveSurface {
        self.surface
    }

    pub const fn is_available(&self) -> bool {
        self.availability.is_none()
    }

    pub const fn absence_cause(&self) -> Option<FoundationalSurfaceAbsenceCause> {
        self.availability
    }
}
