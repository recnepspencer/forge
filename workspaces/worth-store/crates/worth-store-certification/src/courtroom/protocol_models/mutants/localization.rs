use worth_store_formal_models::protocol_bindings::OwnerOperationFamily;
use worth_store_formal_models::runner::{
    AbstractionFunctionIdentity, CanonicalProtocolAction, CanonicalProtocolTrace,
    CertificationLaneIdentity, CounterexampleLocalization, CrossProtocolLocalization,
    ProtocolCounterexample, ProtocolFrontierIdentity, SharedFrontierIdentity,
};
use worth_store_formal_models::{
    current_protocol_binding_manifest, CompactionVisibilityAction, DurabilityRecoveryAction,
    ImportPublicationAction, LeaseReclaimAction, ModelActionFamily, ProtocolFamily,
    QuarantineReadmissionState, ReplicationAdmissionAction, SharedFrontierAction,
    SourcePrecedenceAction,
};

use super::ControlledProtocolMutant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlledMutantLocalization {
    Owner(CounterexampleLocalization),
    Shared(SharedControlledMutantLocalization),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedControlledMutantLocalization {
    counterexample: ProtocolCounterexample,
    diagnostic: CrossProtocolLocalization,
    failing_lane: CertificationLaneIdentity,
    trace_excerpt: CanonicalProtocolTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlledMutantLocalizationDenial {
    MissingOwnerBinding,
    InvalidLane,
    InvalidCanonicalTrace,
    InvalidOwnerLocalization,
    InvalidCrossProtocolLocalization,
    CounterexampleEdgeMismatch,
}

pub(super) fn localize_controlled_mutant(
    mutant: ControlledProtocolMutant,
    counterexample: ProtocolCounterexample,
) -> Result<ControlledMutantLocalization, ControlledMutantLocalizationDenial> {
    if !counterexample_records_expected_mutant_edge(&counterexample, mutant) {
        return Err(ControlledMutantLocalizationDenial::CounterexampleEdgeMismatch);
    }
    if mutant == ControlledProtocolMutant::SharedReachableAuthorityReclaimed {
        return localize_shared_reclaim(counterexample);
    }
    let (operation, family, abstraction, frontier, action) = owner_localization_spec(mutant);
    let binding = current_protocol_binding_manifest()
        .bindings()
        .find(|binding| {
            binding.protocol() == mutant.protocol()
                && binding.model_action_family() == family
                && binding.operation() == operation
        })
        .ok_or(ControlledMutantLocalizationDenial::MissingOwnerBinding)?;
    let lane = CertificationLaneIdentity::admit(mutant.certification_lane())
        .map_err(|_| ControlledMutantLocalizationDenial::InvalidLane)?;
    let trace = CanonicalProtocolTrace::admit(mutant.protocol(), frontier, [action])
        .map_err(|_| ControlledMutantLocalizationDenial::InvalidCanonicalTrace)?;
    CounterexampleLocalization::localize(counterexample, binding, abstraction, lane, trace)
        .map(ControlledMutantLocalization::Owner)
        .map_err(|_| ControlledMutantLocalizationDenial::InvalidOwnerLocalization)
}

fn counterexample_records_expected_mutant_edge(
    counterexample: &ProtocolCounterexample,
    mutant: ControlledProtocolMutant,
) -> bool {
    let expected = expected_checker_edge(mutant);
    counterexample.states().iter().any(|state| {
        state
            .valuation("mutantEdge")
            .is_some_and(|value| value.trim_matches('"') == expected)
    })
}

pub(super) const fn expected_checker_edge(mutant: ControlledProtocolMutant) -> &'static str {
    match mutant {
        ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence => "AcknowledgeBeforeFence",
        ControlledProtocolMutant::RecoveryQuarantinedSourceSelected => "SelectQuarantinedSource",
        ControlledProtocolMutant::CompactionPublicationBeforeCutover => "PublishBeforeCutover",
        ControlledProtocolMutant::LeaseIdentityReuseWithLiveLease => "ReuseWithLiveLease",
        ControlledProtocolMutant::QuarantineReleaseWithoutVerification => {
            "ReleaseWithoutVerification"
        }
        ControlledProtocolMutant::ImportPublicationWithoutDurability => "PublishWithoutDurability",
        ControlledProtocolMutant::ReplicationDivergenceAcceptedAsResume => {
            "AcceptDivergenceAsResume"
        }
        ControlledProtocolMutant::SharedReachableAuthorityReclaimed => "ReclaimReachableAuthority",
    }
}

impl ControlledMutantLocalization {
    pub const fn counterexample(&self) -> &ProtocolCounterexample {
        match self {
            Self::Owner(localization) => localization.counterexample(),
            Self::Shared(localization) => localization.counterexample(),
        }
    }

    pub fn failing_lane(&self) -> &CertificationLaneIdentity {
        match self {
            Self::Owner(localization) => localization.failing_lane(),
            Self::Shared(localization) => localization.failing_lane(),
        }
    }
}

impl SharedControlledMutantLocalization {
    pub const fn counterexample(&self) -> &ProtocolCounterexample {
        &self.counterexample
    }

    pub const fn diagnostic(&self) -> &CrossProtocolLocalization {
        &self.diagnostic
    }

    pub const fn failing_lane(&self) -> &CertificationLaneIdentity {
        &self.failing_lane
    }

    pub const fn trace_excerpt(&self) -> &CanonicalProtocolTrace {
        &self.trace_excerpt
    }
}

fn localize_shared_reclaim(
    counterexample: ProtocolCounterexample,
) -> Result<ControlledMutantLocalization, ControlledMutantLocalizationDenial> {
    let diagnostic = CrossProtocolLocalization::diagnostic(
        ProtocolFamily::CompactionVisibility,
        ProtocolFamily::LeaseReclaim,
        SharedFrontierIdentity::Reachability,
        "reachable old authority -> identity reuse",
    )
    .map_err(|_| ControlledMutantLocalizationDenial::InvalidCrossProtocolLocalization)?;
    let failing_lane = CertificationLaneIdentity::admit(
        ControlledProtocolMutant::SharedReachableAuthorityReclaimed.certification_lane(),
    )
    .map_err(|_| ControlledMutantLocalizationDenial::InvalidLane)?;
    let trace_excerpt = CanonicalProtocolTrace::admit(
        ProtocolFamily::SharedFrontiers,
        ProtocolFrontierIdentity::Reachability,
        [CanonicalProtocolAction::SharedFrontier(
            SharedFrontierAction::GenerationReused,
        )],
    )
    .map_err(|_| ControlledMutantLocalizationDenial::InvalidCanonicalTrace)?;
    Ok(ControlledMutantLocalization::Shared(
        SharedControlledMutantLocalization {
            counterexample,
            diagnostic,
            failing_lane,
            trace_excerpt,
        },
    ))
}

#[allow(clippy::type_complexity)]
fn owner_localization_spec(
    mutant: ControlledProtocolMutant,
) -> (
    OwnerOperationFamily,
    ModelActionFamily,
    AbstractionFunctionIdentity,
    ProtocolFrontierIdentity,
    CanonicalProtocolAction,
) {
    match mutant {
        ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence => (
            OwnerOperationFamily::DurableAcknowledgement,
            ModelActionFamily::DurabilityFrontier,
            AbstractionFunctionIdentity::DurabilityOwnerMapping,
            ProtocolFrontierIdentity::Durability,
            CanonicalProtocolAction::DurabilityRecovery(
                DurabilityRecoveryAction::PhysicalMutationAcknowledged,
            ),
        ),
        ControlledProtocolMutant::RecoveryQuarantinedSourceSelected => (
            OwnerOperationFamily::RecoverySourceSelection,
            ModelActionFamily::RecoverySourcePrecedence,
            AbstractionFunctionIdentity::RecoverySourceTraceMapping,
            ProtocolFrontierIdentity::RecoveryPrecedence,
            CanonicalProtocolAction::RecoverySourcePrecedence(
                SourcePrecedenceAction::SourceSelected,
            ),
        ),
        ControlledProtocolMutant::CompactionPublicationBeforeCutover => (
            OwnerOperationFamily::PhysicalCompactionCutover,
            ModelActionFamily::PhysicalCompaction,
            AbstractionFunctionIdentity::CompactionVisibilityOwnerMapping,
            ProtocolFrontierIdentity::Visibility,
            CanonicalProtocolAction::CompactionVisibility(
                CompactionVisibilityAction::PublishRewrite,
            ),
        ),
        ControlledProtocolMutant::LeaseIdentityReuseWithLiveLease => (
            OwnerOperationFamily::ReclaimReuseFence,
            ModelActionFamily::GenerationReuse,
            AbstractionFunctionIdentity::LeaseReclaimOwnerMapping,
            ProtocolFrontierIdentity::Reachability,
            CanonicalProtocolAction::LeaseReclaim(LeaseReclaimAction::IdentityReuseAdmitted {
                old_generation: 1,
                new_generation: 2,
            }),
        ),
        ControlledProtocolMutant::QuarantineReleaseWithoutVerification => (
            OwnerOperationFamily::LayoutReadmission,
            ModelActionFamily::QuarantineReadmission,
            AbstractionFunctionIdentity::QuarantineReadmissionOwnerMapping,
            ProtocolFrontierIdentity::Quarantine,
            CanonicalProtocolAction::QuarantineReadmission(QuarantineReadmissionState::Readmitted),
        ),
        ControlledProtocolMutant::ImportPublicationWithoutDurability => (
            OwnerOperationFamily::ImportPublicationCompletion,
            ModelActionFamily::ImportPublication,
            AbstractionFunctionIdentity::ImportPublicationOwnerMapping,
            ProtocolFrontierIdentity::Admission,
            CanonicalProtocolAction::ImportPublication(ImportPublicationAction::PublicationDurable),
        ),
        ControlledProtocolMutant::ReplicationDivergenceAcceptedAsResume => (
            OwnerOperationFamily::ReplicationProgressObservation,
            ModelActionFamily::ReplicationAdmission,
            AbstractionFunctionIdentity::ReplicationAdmissionOwnerMapping,
            ProtocolFrontierIdentity::Admission,
            CanonicalProtocolAction::ReplicationAdmission(
                ReplicationAdmissionAction::SourceEpochDivergenceDetected,
            ),
        ),
        ControlledProtocolMutant::SharedReachableAuthorityReclaimed => {
            unreachable!("shared mutants use cross-protocol localization")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_text_without_structured_mutant_edge_cannot_localize() {
        let mutant = ControlledProtocolMutant::DurabilityAcknowledgmentBeforeFence;
        let counterexample = ProtocolCounterexample::diagnostic(
            mutant.protocol(),
            vec![expected_checker_edge(mutant).to_owned()],
        );

        assert_eq!(
            localize_controlled_mutant(mutant, counterexample),
            Err(ControlledMutantLocalizationDenial::CounterexampleEdgeMismatch)
        );
    }
}
