use crate::evidence::{
    WorthQueryCertificationCounter, WorthQueryCertificationCounters,
    WorthQueryCertificationDenialBoundary, WorthQueryCertificationDenialEvidence,
};

/// Generic authority attacks owned once by Query certification.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryCertificationHostileAttack {
    ForeignInstallation,
    StaleGeneration,
    SecondOperatingWorldRoot,
    HiddenGraphAdapter,
    ForgedGraphAdapter,
    IndependentOperationFamily,
    ReconstructedInstalledOperation,
    CopiedSemanticAspectKey,
    DriftedContractRevision,
    DriftedFieldMask,
    ForgedTruthDeltaTarget,
    CopiedSignalAspect,
    CopiedSignalMask,
    ForgedAspectCorrespondence,
    StableNameScopeSubstitution,
    CopiedSignalCondition,
    CustomConditionString,
    ForeignConditionProvider,
    StaleComparatorProvider,
    ForeignTriggerProvider,
    ForgedBridgeLowering,
    DetachedSignalDecision,
    RestampedSignalDecision,
    DetachedCompletion,
    CopiedConsumedFacts,
    WrongDeclarationKey,
    ForgedDependencyClosure,
    ReportingDigestCollision,
    ForgedSupportProjection,
    FalseSharingEquivalence,
    WrongCompatibilityWitness,
    ForeignLease,
    ForeignInvalidationDelta,
    CopiedCursor,
    StaleCollectionPatch,
    DisposedLifecycle,
    CrossRunStageReceipt,
    ForgedReplayScope,
    ForgedReversalScope,
    RepresentationDerivedLineage,
    CrossProviderArtifact,
}

impl WorthQueryCertificationHostileAttack {
    pub const ALL: [Self; 41] = [
        Self::ForeignInstallation,
        Self::StaleGeneration,
        Self::SecondOperatingWorldRoot,
        Self::HiddenGraphAdapter,
        Self::ForgedGraphAdapter,
        Self::IndependentOperationFamily,
        Self::ReconstructedInstalledOperation,
        Self::CopiedSemanticAspectKey,
        Self::DriftedContractRevision,
        Self::DriftedFieldMask,
        Self::ForgedTruthDeltaTarget,
        Self::CopiedSignalAspect,
        Self::CopiedSignalMask,
        Self::ForgedAspectCorrespondence,
        Self::StableNameScopeSubstitution,
        Self::CopiedSignalCondition,
        Self::CustomConditionString,
        Self::ForeignConditionProvider,
        Self::StaleComparatorProvider,
        Self::ForeignTriggerProvider,
        Self::ForgedBridgeLowering,
        Self::DetachedSignalDecision,
        Self::RestampedSignalDecision,
        Self::DetachedCompletion,
        Self::CopiedConsumedFacts,
        Self::WrongDeclarationKey,
        Self::ForgedDependencyClosure,
        Self::ReportingDigestCollision,
        Self::ForgedSupportProjection,
        Self::FalseSharingEquivalence,
        Self::WrongCompatibilityWitness,
        Self::ForeignLease,
        Self::ForeignInvalidationDelta,
        Self::CopiedCursor,
        Self::StaleCollectionPatch,
        Self::DisposedLifecycle,
        Self::CrossRunStageReceipt,
        Self::ForgedReplayScope,
        Self::ForgedReversalScope,
        Self::RepresentationDerivedLineage,
        Self::CrossProviderArtifact,
    ];

    fn earliest_boundary(self) -> WorthQueryCertificationDenialBoundary {
        use WorthQueryCertificationDenialBoundary as Boundary;
        use WorthQueryCertificationHostileAttack as Attack;
        match self {
            Attack::ForeignInstallation
            | Attack::StaleGeneration
            | Attack::SecondOperatingWorldRoot => Boundary::OperatingWorldEntry,
            Attack::IndependentOperationFamily => Boundary::FamilyLookup,
            Attack::ReconstructedInstalledOperation
            | Attack::CopiedSemanticAspectKey
            | Attack::DriftedContractRevision
            | Attack::DriftedFieldMask
            | Attack::CrossProviderArtifact => Boundary::OperationBinding,
            Attack::HiddenGraphAdapter
            | Attack::ForgedGraphAdapter
            | Attack::ForgedTruthDeltaTarget => Boundary::GraphParticipation,
            Attack::CopiedSignalAspect
            | Attack::CopiedSignalMask
            | Attack::ForgedAspectCorrespondence
            | Attack::StableNameScopeSubstitution
            | Attack::CopiedSignalCondition
            | Attack::CustomConditionString
            | Attack::ForeignConditionProvider
            | Attack::StaleComparatorProvider
            | Attack::ForeignTriggerProvider
            | Attack::ForgedBridgeLowering => Boundary::ConditionalInstallation,
            Attack::DetachedSignalDecision
            | Attack::RestampedSignalDecision
            | Attack::CrossRunStageReceipt => Boundary::ExecutionAdmission,
            Attack::DetachedCompletion | Attack::RepresentationDerivedLineage => {
                Boundary::PublicationAdmission
            }
            Attack::CopiedConsumedFacts | Attack::WrongDeclarationKey => {
                Boundary::ConsumptionAdmission
            }
            Attack::ForgedDependencyClosure
            | Attack::ReportingDigestCollision
            | Attack::ForgedSupportProjection
            | Attack::WrongCompatibilityWitness => Boundary::CompatibilityAdmission,
            Attack::FalseSharingEquivalence | Attack::ForeignLease => Boundary::SharingAdmission,
            Attack::ForeignInvalidationDelta => Boundary::InvalidationAdmission,
            Attack::CopiedCursor | Attack::StaleCollectionPatch => Boundary::CollectionAdmission,
            Attack::DisposedLifecycle => Boundary::LifecycleAdmission,
            Attack::ForgedReplayScope => Boundary::ReplayAdmission,
            Attack::ForgedReversalScope => Boundary::ReversalAdmission,
        }
    }
}

/// Canonical attack and its required earliest denial evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCertificationHostileCase {
    attack: WorthQueryCertificationHostileAttack,
    expected: WorthQueryCertificationDenialEvidence,
}

impl WorthQueryCertificationHostileCase {
    pub fn attack(&self) -> WorthQueryCertificationHostileAttack {
        self.attack
    }

    pub fn expected(&self) -> &WorthQueryCertificationDenialEvidence {
        &self.expected
    }
}

/// Complete generic hostile matrix. Domain packages execute this registry;
/// they do not reproduce its compile-time or taxonomy cross-product.
pub fn canonical_hostile_matrix() -> Vec<WorthQueryCertificationHostileCase> {
    WorthQueryCertificationHostileAttack::ALL
        .into_iter()
        .map(|attack| WorthQueryCertificationHostileCase {
            attack,
            expected: WorthQueryCertificationDenialEvidence::observed(
                attack.earliest_boundary(),
                WorthQueryCertificationCounters::exact([(
                    WorthQueryCertificationCounter::BoundaryChecks,
                    1,
                )])
                .expect("canonical hostile expectations contain unique counters"),
            ),
        })
        .collect()
}
