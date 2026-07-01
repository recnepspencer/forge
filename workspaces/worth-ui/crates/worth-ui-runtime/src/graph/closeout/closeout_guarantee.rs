#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UiGraphCloseoutGuarantee {
    GraphTruthOwnedByCommittedSnapshot,
    DeclarationAuthorityLowersOnceIntoGraphCorrespondence,
    GraphAndIndexMutationCommitAsOneGenerationTransition,
    OrdinaryLookupRemainsReceiptBackedAndBounded,
    FormalInspectionCarriesThinTargetsEvidenceAndStopPoints,
    HandoffConsumesProofBearingGraphAuthorityRatherThanRawInternals,
}
