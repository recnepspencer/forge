#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationFamily {
    StructuralLegality,
    ParticipationLegality,
    SlotContract,
    MeasurementRequirement,
    QueryBindingRequirement,
    IntentOperabilityRequirement,
    PortalHostRequirement,
    FocusRouteRequirement,
    MotionSupportRequirement,
    ScrollRoutingRequirement,
    SelectionStateRequirement,
    CommandRouteRequirement,
    AccessibilityRequirement,
    HostCapabilityRequirement,
    DiagnosticSurfaceRequirement,
}
