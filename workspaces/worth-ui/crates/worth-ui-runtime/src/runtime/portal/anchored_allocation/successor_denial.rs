#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPortalAnchorSuccessorDenial {
    EvidenceCategoryMismatch,
    StaleEvidenceGeneration,
    NormalizationAuthorityMismatch,
    ObservationInvalid,
}
