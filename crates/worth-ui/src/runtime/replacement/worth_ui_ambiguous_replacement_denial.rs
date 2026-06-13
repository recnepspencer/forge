use crate::runtime::WorthUiNodeReplacementCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiAmbiguousReplacementDenial {
    ImpactClassificationDigestMismatch {
        impact_active_artifact_digest: u64,
        identity_active_artifact_digest: u64,
        impact_candidate_artifact_digest: u64,
        identity_candidate_artifact_digest: u64,
        counters: WorthUiNodeReplacementCounters,
    },
    NarrowingDigestMismatch {
        narrowing_active_artifact_digest: u64,
        identity_active_artifact_digest: u64,
        narrowing_candidate_artifact_digest: u64,
        identity_candidate_artifact_digest: u64,
        counters: WorthUiNodeReplacementCounters,
    },
    AmbiguousIdentityGraph {
        counters: WorthUiNodeReplacementCounters,
    },
    DuplicateReplacementClassification {
        identity_basis: String,
        counters: WorthUiNodeReplacementCounters,
    },
    LaneAffectingImpactWithoutLaneNarrowing {
        counters: WorthUiNodeReplacementCounters,
    },
    LaneAffectingImpactWithoutAffectedLaneScope {
        counters: WorthUiNodeReplacementCounters,
    },
}
