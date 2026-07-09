use super::{
    BridgeSubscriptionCertificationComparisonPlan,
    BridgeSubscriptionCertificationComparisonRelationship,
    BridgeSubscriptionCertificationDivergenceAxis, BridgeSubscriptionCertificationFailureBoundary,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionCertificationComparisonOutcome {
    Equivalent,
    IntentionallyDivergent,
    RejectedAtExpectedBoundary,
    RejectedAtUnexpectedBoundary,
    DiagnosticsOnlyDifference,
    ResidueMismatch,
    ReplayMismatch,
    CounterContractViolation,
    BundleCompletenessViolation,
}

impl BridgeSubscriptionCertificationComparisonOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Equivalent => "equivalent",
            Self::IntentionallyDivergent => "intentionally_divergent",
            Self::RejectedAtExpectedBoundary => "rejected_at_expected_boundary",
            Self::RejectedAtUnexpectedBoundary => "rejected_at_unexpected_boundary",
            Self::DiagnosticsOnlyDifference => "diagnostics_only_difference",
            Self::ResidueMismatch => "residue_mismatch",
            Self::ReplayMismatch => "replay_mismatch",
            Self::CounterContractViolation => "counter_contract_violation",
            Self::BundleCompletenessViolation => "bundle_completeness_violation",
        }
    }
}

pub(crate) fn outcome_for(
    plan: &BridgeSubscriptionCertificationComparisonPlan,
    failures: &[BridgeSubscriptionCertificationFailureBoundary],
    primary_failure_boundary: Option<BridgeSubscriptionCertificationFailureBoundary>,
) -> BridgeSubscriptionCertificationComparisonOutcome {
    use BridgeSubscriptionCertificationComparisonOutcome as Outcome;
    use BridgeSubscriptionCertificationComparisonRelationship as Relationship;
    use BridgeSubscriptionCertificationFailureBoundary as Boundary;

    let mismatch_count = failures.len();

    match plan.relationship() {
        Relationship::SemanticEquivalence if mismatch_count == 0 => Outcome::Equivalent,
        Relationship::ReplayEquivalence if mismatch_count == 0 => Outcome::Equivalent,
        Relationship::DiagnosticsOnlyVariation if failures == [Boundary::DiagnosticsInfluence] => {
            Outcome::DiagnosticsOnlyDifference
        }
        Relationship::IntentionalDivergence
            if plan
                .divergence_axis()
                .and_then(expected_boundary_for_divergence_axis)
                == primary_failure_boundary =>
        {
            Outcome::IntentionallyDivergent
        }
        Relationship::ExpectedRejection
            if primary_failure_boundary == plan.expected_failure_boundary() =>
        {
            Outcome::RejectedAtExpectedBoundary
        }
        Relationship::ResidueAbsence if failures == [Boundary::PreviewResidueMismatch] => {
            Outcome::ResidueMismatch
        }
        Relationship::CounterContract if failures == [Boundary::CounterContractViolation] => {
            Outcome::CounterContractViolation
        }
        Relationship::BundleCompleteness
            if matches!(
                primary_failure_boundary,
                Some(Boundary::BundleInsufficiency | Boundary::TypedFieldStateMismatch)
            ) =>
        {
            Outcome::BundleCompletenessViolation
        }
        Relationship::ReplayEquivalence if failures == [Boundary::ReplayMismatch] => {
            Outcome::ReplayMismatch
        }
        _ => Outcome::RejectedAtUnexpectedBoundary,
    }
}

fn expected_boundary_for_divergence_axis(
    axis: BridgeSubscriptionCertificationDivergenceAxis,
) -> Option<BridgeSubscriptionCertificationFailureBoundary> {
    use BridgeSubscriptionCertificationDivergenceAxis as Axis;
    use BridgeSubscriptionCertificationFailureBoundary as Boundary;

    Some(match axis {
        Axis::DeclarationFamily => Boundary::DeclarationEquivalenceDrift,
        Axis::Basis => Boundary::BasisDrift,
        Axis::DeliveryFamily => Boundary::DeliveryDigestDrift,
        Axis::ContinuationDecision => Boundary::ContinuationDenialOrAmbiguity,
        Axis::BranchScope => Boundary::BranchLeakageAttempt,
        Axis::PreviewOutcome => Boundary::PreviewResidueMismatch,
        Axis::StrategyLowering => Boundary::StrategyLoweringProvenanceMismatch,
        Axis::DiagnosticsDetail => Boundary::DiagnosticsInfluence,
        Axis::CounterContract => Boundary::CounterContractViolation,
        Axis::BundleCompleteness => Boundary::BundleInsufficiency,
    })
}
