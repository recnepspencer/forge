#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupReuseDecisionPosture {
    ReuseAdmitted,
    FreshRebuildRequired,
    AdvisoryMatchRequiresRebuild,
    Denied,
}
