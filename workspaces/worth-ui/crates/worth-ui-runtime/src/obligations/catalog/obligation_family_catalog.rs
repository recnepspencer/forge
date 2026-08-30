use super::UiObligationFamily;

const CLOSED_FAMILIES: [UiObligationFamily; 15] = [
    UiObligationFamily::StructuralLegality,
    UiObligationFamily::ParticipationLegality,
    UiObligationFamily::SlotContract,
    UiObligationFamily::MeasurementRequirement,
    UiObligationFamily::QueryBindingRequirement,
    UiObligationFamily::IntentOperabilityRequirement,
    UiObligationFamily::PortalHostRequirement,
    UiObligationFamily::FocusRouteRequirement,
    UiObligationFamily::MotionSupportRequirement,
    UiObligationFamily::ScrollRoutingRequirement,
    UiObligationFamily::SelectionStateRequirement,
    UiObligationFamily::CommandRouteRequirement,
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

#[cfg(test)]
mod tests {
    use super::{UiObligationFamily, UiObligationFamilyCatalog};

    #[test]
    fn closed_catalog_names_each_runtime_service_obligation_axis() {
        let catalog = UiObligationFamilyCatalog::closed();
        assert_eq!(catalog.families().len(), 15);
        for family in [
            UiObligationFamily::PortalHostRequirement,
            UiObligationFamily::FocusRouteRequirement,
            UiObligationFamily::MotionSupportRequirement,
            UiObligationFamily::ScrollRoutingRequirement,
            UiObligationFamily::SelectionStateRequirement,
            UiObligationFamily::CommandRouteRequirement,
        ] {
            assert!(catalog.contains(family), "missing {family:?}");
        }
    }
}
