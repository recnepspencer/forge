#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiScrollContractAdmissionDenial {
    PlanningPostureMismatch,
    NeighborhoodMismatch,
    SourceEvidenceMissing,
    SourceGenerationMissing,
    MeasurementBasisMismatch,
}
