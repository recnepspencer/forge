#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionEvidenceSource {
    WorthLocal,
    QueryInspection,
    QueryProjectionConsumption,
    QueryCausalExplanation,
    HostCapability,
}
