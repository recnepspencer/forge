use super::*;

#[derive(Clone, Copy)]
pub(super) struct ExpectedLaneEvidence {
    pub(super) outcome: Option<BridgeSubscriptionCertificationComparisonOutcome>,
    pub(super) primary_failure_boundary: Option<BridgeSubscriptionCertificationFailureBoundary>,
}

pub(super) fn expected_evidence_for_lane(
    lane_kind: BridgeSubscriptionReferenceWorkloadLaneKind,
) -> Option<ExpectedLaneEvidence> {
    use BridgeSubscriptionCertificationComparisonOutcome as Outcome;
    use BridgeSubscriptionCertificationFailureBoundary as Boundary;
    use BridgeSubscriptionReferenceWorkloadLaneKind as Lane;

    Some(match lane_kind {
        Lane::AuthoritativeLive => return None,
        Lane::CanonicalOrderingHostility => ExpectedLaneEvidence {
            outcome: Some(Outcome::Equivalent),
            primary_failure_boundary: None,
        },
        Lane::TimeOnlyRouting => ExpectedLaneEvidence {
            outcome: Some(Outcome::RejectedAtExpectedBoundary),
            primary_failure_boundary: Some(Boundary::DeliveryFamilyMismatch),
        },
        Lane::HistoricalBasisReplay | Lane::SharedFanout => ExpectedLaneEvidence {
            outcome: Some(Outcome::Equivalent),
            primary_failure_boundary: None,
        },
        Lane::DiagnosticsTierVariation => ExpectedLaneEvidence {
            outcome: Some(Outcome::DiagnosticsOnlyDifference),
            primary_failure_boundary: Some(Boundary::DiagnosticsInfluence),
        },
        Lane::HostileAdapterVariation => ExpectedLaneEvidence {
            outcome: Some(Outcome::RejectedAtExpectedBoundary),
            primary_failure_boundary: Some(Boundary::MissingRequiredRetainedArtifact),
        },
        Lane::HistoricalReplay | Lane::RestartResume => ExpectedLaneEvidence {
            outcome: Some(Outcome::RejectedAtExpectedBoundary),
            primary_failure_boundary: Some(Boundary::ReplayMismatch),
        },
        Lane::BranchLocal => ExpectedLaneEvidence {
            outcome: Some(Outcome::IntentionallyDivergent),
            primary_failure_boundary: Some(Boundary::DeclarationEquivalenceDrift),
        },
        Lane::DivergentSharingRejection => ExpectedLaneEvidence {
            outcome: Some(Outcome::RejectedAtExpectedBoundary),
            primary_failure_boundary: Some(Boundary::IllegalSharingReuse),
        },
        Lane::StaleCheckpointRejection => ExpectedLaneEvidence {
            outcome: Some(Outcome::RejectedAtExpectedBoundary),
            primary_failure_boundary: Some(Boundary::CheckpointDivergence),
        },
        Lane::Continuation => ExpectedLaneEvidence {
            outcome: Some(Outcome::IntentionallyDivergent),
            primary_failure_boundary: Some(Boundary::ContinuationDenialOrAmbiguity),
        },
        Lane::DeniedContinuation => ExpectedLaneEvidence {
            outcome: Some(Outcome::RejectedAtExpectedBoundary),
            primary_failure_boundary: Some(Boundary::ContinuationDenialOrAmbiguity),
        },
        Lane::StrategyLoweringProvenance => ExpectedLaneEvidence {
            outcome: Some(Outcome::IntentionallyDivergent),
            primary_failure_boundary: Some(Boundary::StrategyLoweringProvenanceMismatch),
        },
        Lane::BundleInsufficiency => ExpectedLaneEvidence {
            outcome: Some(Outcome::BundleCompletenessViolation),
            primary_failure_boundary: Some(Boundary::BundleInsufficiency),
        },
        Lane::PreviewDiscard | Lane::PreviewPromotion => ExpectedLaneEvidence {
            outcome: Some(Outcome::IntentionallyDivergent),
            primary_failure_boundary: Some(Boundary::PreviewResidueMismatch),
        },
    })
}
use super::super::{
    BridgeSubscriptionCertificationComparisonOutcome,
    BridgeSubscriptionCertificationFailureBoundary,
};
