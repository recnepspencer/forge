/// Evidence labels categories without granting conversion authority between them.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationTruthCategory {
    EphemeralStreamEvent,
    LocalProjectedInteractionState,
    Candidate,
    PreviewCandidate,
    CommittedReceipt,
    DurableSemanticState,
}
