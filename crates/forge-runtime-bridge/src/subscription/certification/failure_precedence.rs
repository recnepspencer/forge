use super::BridgeSubscriptionCertificationFailureBoundary;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionCertificationFailurePrecedenceStage {
    BundleSchemaParity,
    RetainedArtifactCompleteness,
    DeclarationOrRegistry,
    BasisBinding,
    StrategyLowering,
    Lifecycle,
    ConsumerOrSharing,
    DeliveryTruth,
    ContinuationOrBranchScope,
    CheckpointResumeOrReplay,
    PreviewResidueOrPromotion,
    DiagnosticsInfluence,
    CounterContract,
}

impl BridgeSubscriptionCertificationFailurePrecedenceStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundleSchemaParity => "bundle_schema_parity",
            Self::RetainedArtifactCompleteness => "retained_artifact_completeness",
            Self::DeclarationOrRegistry => "declaration_or_registry",
            Self::BasisBinding => "basis_binding",
            Self::StrategyLowering => "strategy_lowering",
            Self::Lifecycle => "lifecycle",
            Self::ConsumerOrSharing => "consumer_or_sharing",
            Self::DeliveryTruth => "delivery_truth",
            Self::ContinuationOrBranchScope => "continuation_or_branch_scope",
            Self::CheckpointResumeOrReplay => "checkpoint_resume_or_replay",
            Self::PreviewResidueOrPromotion => "preview_residue_or_promotion",
            Self::DiagnosticsInfluence => "diagnostics_influence",
            Self::CounterContract => "counter_contract",
        }
    }

    pub const fn rank(self) -> u8 {
        match self {
            Self::BundleSchemaParity => 1,
            Self::RetainedArtifactCompleteness => 2,
            Self::DeclarationOrRegistry => 3,
            Self::BasisBinding => 4,
            Self::StrategyLowering => 5,
            Self::Lifecycle => 6,
            Self::ConsumerOrSharing => 7,
            Self::DeliveryTruth => 8,
            Self::ContinuationOrBranchScope => 9,
            Self::CheckpointResumeOrReplay => 10,
            Self::PreviewResidueOrPromotion => 11,
            Self::DiagnosticsInfluence => 12,
            Self::CounterContract => 13,
        }
    }
}

pub(crate) fn precedence_stage_for_boundary(
    boundary: BridgeSubscriptionCertificationFailureBoundary,
) -> BridgeSubscriptionCertificationFailurePrecedenceStage {
    use BridgeSubscriptionCertificationFailureBoundary as Boundary;
    use BridgeSubscriptionCertificationFailurePrecedenceStage as Stage;

    match boundary {
        Boundary::BundleSchemaOrDigestDivergence => Stage::BundleSchemaParity,
        Boundary::MissingRequiredRetainedArtifact | Boundary::TypedFieldStateMismatch => {
            Stage::RetainedArtifactCompleteness
        }
        Boundary::DeclarationEquivalenceDrift | Boundary::RegistryDrift => {
            Stage::DeclarationOrRegistry
        }
        Boundary::BasisDrift
        | Boundary::HistoricalBasisUnavailable
        | Boundary::BranchLeakageAttempt => Stage::BasisBinding,
        Boundary::StrategyLoweringProvenanceMismatch => Stage::StrategyLowering,
        Boundary::LifecycleTransitionMismatch => Stage::Lifecycle,
        Boundary::ConsumerContractMismatch | Boundary::IllegalSharingReuse => {
            Stage::ConsumerOrSharing
        }
        Boundary::DeliveryFamilyMismatch | Boundary::DeliveryDigestDrift => Stage::DeliveryTruth,
        Boundary::ContinuationDenialOrAmbiguity => Stage::ContinuationOrBranchScope,
        Boundary::CheckpointDivergence | Boundary::ReplayMismatch => {
            Stage::CheckpointResumeOrReplay
        }
        Boundary::PreviewResidueMismatch | Boundary::PromotionBoundaryMismatch => {
            Stage::PreviewResidueOrPromotion
        }
        Boundary::DiagnosticsInfluence => Stage::DiagnosticsInfluence,
        Boundary::BundleInsufficiency => Stage::RetainedArtifactCompleteness,
        Boundary::CounterContractViolation => Stage::CounterContract,
    }
}
