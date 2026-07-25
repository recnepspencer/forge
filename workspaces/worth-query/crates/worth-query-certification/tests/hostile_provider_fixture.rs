use worth_query_certification::facade::{
    WorthQueryCertificationCounter, WorthQueryCertificationCounters,
    WorthQueryCertificationDenialBoundary as Boundary, WorthQueryCertificationDenialEvidence,
    WorthQueryCertificationHostileAttack as Attack, WorthQueryHostileCertificationProvider,
};

/// Contract substitute for the hostile-runner harness.
///
/// It deliberately owns an expectation-independent attack-to-boundary mapping
/// and records dispatch. Production Query denial behavior is proved by the
/// compile and runtime boundary suites, not by this substitute.
pub(super) struct HostileHarnessProvider {
    identity: &'static str,
    counter_drift: Option<Attack>,
    observed_attacks: Vec<Attack>,
}

impl HostileHarnessProvider {
    pub(super) fn conforming(identity: &'static str) -> Self {
        Self {
            identity,
            counter_drift: None,
            observed_attacks: Vec::new(),
        }
    }

    pub(super) fn with_counter_drift(identity: &'static str, attack: Attack) -> Self {
        Self {
            identity,
            counter_drift: Some(attack),
            observed_attacks: Vec::new(),
        }
    }

    pub(super) fn observed_attacks(&self) -> &[Attack] {
        &self.observed_attacks
    }
}

impl WorthQueryHostileCertificationProvider for HostileHarnessProvider {
    fn provider_identity(&self) -> &str {
        self.identity
    }

    fn attack(&mut self, attack: Attack) -> Result<WorthQueryCertificationDenialEvidence, String> {
        self.observed_attacks.push(attack);
        let counters = if self.counter_drift == Some(attack) {
            WorthQueryCertificationCounters::default()
        } else {
            WorthQueryCertificationCounters::exact([(
                WorthQueryCertificationCounter::BoundaryChecks,
                1,
            )])
            .expect("the harness emits one unique boundary counter")
        };
        Ok(WorthQueryCertificationDenialEvidence::observed(
            fixture_boundary(attack),
            counters,
        ))
    }
}

fn fixture_boundary(attack: Attack) -> Boundary {
    match attack {
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
        Attack::CopiedConsumedFacts | Attack::WrongDeclarationKey => Boundary::ConsumptionAdmission,
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
