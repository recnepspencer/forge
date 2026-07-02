use super::UiObligationFamily;

const CLOSED_FAMILIES: [UiObligationFamily; 12] = [
    UiObligationFamily::StructuralLegality,
    UiObligationFamily::ParticipationLegality,
    UiObligationFamily::SlotContract,
    UiObligationFamily::MeasurementRequirement,
    UiObligationFamily::QueryBindingRequirement,
    UiObligationFamily::IntentOperabilityRequirement,
    UiObligationFamily::PortalHostRequirement,
    UiObligationFamily::FocusRouteRequirement,
    UiObligationFamily::MotionSupportRequirement,
    UiObligationFamily::AccessibilityRequirement,
    UiObligationFamily::HostCapabilityRequirement,
    UiObligationFamily::DiagnosticSurfaceRequirement,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiObligationFamilyCatalog {
    _sealed: (),
}

impl UiObligationFamilyCatalog {
    pub const fn closed() -> Self {
        Self { _sealed: () }
    }

    pub const fn families(self) -> &'static [UiObligationFamily] {
        &CLOSED_FAMILIES
    }

    pub fn contains(self, family: UiObligationFamily) -> bool {
        self.families().contains(&family)
    }
}
