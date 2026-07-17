#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAdmissionFamily {
    TouchMeaning,
    MeasurementRequirement,
    QueryBasis,
    HostCapability,
    Rebind,
    Freshness,
    Budget,
}
