#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionCertificationFailureBoundary {
    BundleSchemaOrDigestIncompatibility,
    MissingRequiredRetainedArtifact,
    TypedFieldStateMismatch,
    DeclarationEquivalenceDrift,
    RegistryDrift,
    BasisDrift,
    StrategyLoweringProvenanceMismatch,
    LifecycleTransitionMismatch,
    ConsumerContractMismatch,
    IllegalSharingReuse,
    DeliveryFamilyMismatch,
    DeliveryDigestDrift,
    ContinuationDenialOrAmbiguity,
    CheckpointIncompatibility,
    ReplayMismatch,
    PreviewResidueMismatch,
    PromotionBoundaryMismatch,
    HistoricalBasisUnavailable,
    BranchLeakageAttempt,
    DiagnosticsInfluence,
    BundleInsufficiency,
    CounterContractViolation,
}

impl BridgeSubscriptionCertificationFailureBoundary {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundleSchemaOrDigestIncompatibility => "bundle_schema_or_digest_incompatibility",
            Self::MissingRequiredRetainedArtifact => "missing_required_retained_artifact",
            Self::TypedFieldStateMismatch => "typed_field_state_mismatch",
            Self::DeclarationEquivalenceDrift => "declaration_equivalence_drift",
            Self::RegistryDrift => "registry_drift",
            Self::BasisDrift => "basis_drift",
            Self::StrategyLoweringProvenanceMismatch => "strategy_lowering_provenance_mismatch",
            Self::LifecycleTransitionMismatch => "lifecycle_transition_mismatch",
            Self::ConsumerContractMismatch => "consumer_contract_mismatch",
            Self::IllegalSharingReuse => "illegal_sharing_reuse",
            Self::DeliveryFamilyMismatch => "delivery_family_mismatch",
            Self::DeliveryDigestDrift => "delivery_digest_drift",
            Self::ContinuationDenialOrAmbiguity => "continuation_denial_or_ambiguity",
            Self::CheckpointIncompatibility => "checkpoint_incompatibility",
            Self::ReplayMismatch => "replay_mismatch",
            Self::PreviewResidueMismatch => "preview_residue_mismatch",
            Self::PromotionBoundaryMismatch => "promotion_boundary_mismatch",
            Self::HistoricalBasisUnavailable => "historical_basis_unavailable",
            Self::BranchLeakageAttempt => "branch_leakage_attempt",
            Self::DiagnosticsInfluence => "diagnostics_influence",
            Self::BundleInsufficiency => "bundle_insufficiency",
            Self::CounterContractViolation => "counter_contract_violation",
        }
    }
}
