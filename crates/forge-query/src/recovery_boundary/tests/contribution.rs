use std::marker::PhantomData;

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping,
};
use crate::contribution_composed_orchestration::{
    ForgeQueryContributionComposedOrchestrationInput, ForgeQueryContributionIntent,
};
use crate::domain_capabilities::ForgeQuerySupportContributionAuthoring;
use crate::recovery_boundary::{
    ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryConflictPosture,
    ForgeQueryRecoverySourceFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ContributionRecoveryDomain;

impl ForgeQueryDomainEntryMarker for ContributionRecoveryDomain {
    fn domain_key(&self) -> &'static str {
        "test.recovery.contribution.domain"
    }
    fn display_name(&self) -> &'static str {
        "RecoveryContributionDomain"
    }
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ContributionRecoveryWorld;

impl ForgeQueryDomainOperatingContext<ContributionRecoveryDomain> for ContributionRecoveryWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::QueryComposition,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "recovery-contribution-world".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ContributionRecoveryFamily;

impl ForgeQueryDeclarationFamilyMarker<ContributionRecoveryDomain> for ContributionRecoveryFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "RecoveryContributionFamily"
    }
    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }
    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.face", "selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }
    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
            &["selection.face", "selection.active_face"],
            &[],
            &[],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContributionRecoveryInput {
    id: &'static str,
    _marker: PhantomData<ContributionRecoveryFamily>,
}

impl ContributionRecoveryInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<ContributionRecoveryDomain> for ContributionRecoveryInput {
    type Family = ContributionRecoveryFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

fn contribution_handle() -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    ContributionRecoveryDomain,
    ContributionRecoveryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ContributionRecoveryDomain)
        .with_operating_context(ContributionRecoveryWorld)
        .validate()
        .expect("contribution recovery world should validate")
        .admit()
        .expect("contribution recovery world should admit")
}

fn denied_traceability_input() -> ForgeQueryContributionComposedOrchestrationInput<
    ContributionRecoveryDomain,
    ContributionRecoveryInput,
> {
    ForgeQueryContributionComposedOrchestrationInput::new(ContributionRecoveryInput::new(
        "face-proof",
    ))
    .with_contribution(ForgeQueryContributionIntent::support(
        ForgeQuerySupportContributionAuthoring::declaration_traceability(
            "domain.traceability.face",
            "",
        ),
    ))
}

#[test]
fn contribution_proof_preserves_intent_level_aspect_context_for_contribution_denial() {
    let handle = contribution_handle();
    let proof =
        handle.orchestrate_declaration_with_contributions_proof(denied_traceability_input());

    let brief = handle
        .recover_from_contribution_composed_proof(proof)
        .expect("contribution denial proof should recover");

    assert_eq!(
        brief.source_family(),
        ForgeQueryRecoverySourceFamily::ContributionComposed
    );
    assert_eq!(
        brief.aspect_posture(),
        ForgeQueryRecoveryAspectPosture::RetainedContractAndCoverage
    );
    assert_eq!(
        brief.conflict_posture(),
        ForgeQueryRecoveryConflictPosture::None
    );
    assert!(brief
        .explanation()
        .has_retained_intent_level_aspect_context());
    assert_eq!(
        brief
            .explanation()
            .contribution_intent_descriptor()
            .map(|value| value.semantic_code()),
        Some("domain.traceability.face")
    );
}

#[test]
fn contribution_checked_and_proof_recovery_stay_aligned_for_same_denial() {
    let handle = contribution_handle();
    let checked =
        handle.orchestrate_declaration_with_contributions_checked(denied_traceability_input());
    let proof =
        handle.orchestrate_declaration_with_contributions_proof(denied_traceability_input());

    let checked_brief = handle
        .recover_from_contribution_composed_checked(checked)
        .expect("checked contribution denial should recover");
    let proof_brief = handle
        .recover_from_contribution_composed_proof(proof)
        .expect("proof contribution denial should recover");

    assert_eq!(checked_brief.stop_family(), proof_brief.stop_family());
    assert_eq!(checked_brief.stop_kind(), proof_brief.stop_kind());
    assert_eq!(checked_brief.source_family(), proof_brief.source_family());
    assert_eq!(checked_brief.aspect_posture(), proof_brief.aspect_posture());
    assert_eq!(
        checked_brief.recommended_action(),
        proof_brief.recommended_action()
    );
    assert_eq!(
        checked_brief
            .explanation()
            .contribution_intent_descriptor()
            .map(|value| value.semantic_code()),
        proof_brief
            .explanation()
            .contribution_intent_descriptor()
            .map(|value| value.semantic_code())
    );
}
