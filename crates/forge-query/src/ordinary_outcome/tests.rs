use std::marker::PhantomData;

use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationEntryOrchestrationAutomationBoundary,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusal,
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationDenied, ForgeQueryDeclarationEntryOrchestrationRefusal,
    ForgeQueryDeclarationEntryOrchestrationStage, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
};
use crate::binding_pipeline::{ForgeQueryBindingChecked, ForgeQueryBindingLinkedArtifacts};
use crate::facade::{
    ForgeQueryApplicationFacade, ForgeQueryBindingOutcome, ForgeQueryBindingSourceKind,
    ForgeQueryBindingWrongWorld, ForgeQueryDeclarationBindingRequest,
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationTerminalError,
};
use crate::ordinary_outcome::{
    ordinary_outcome_from_binding_outcome, ordinary_outcome_from_orchestration_terminal,
    ForgeQueryOrdinaryBindingCheckedTopologyKind, ForgeQueryOrdinaryNextStep,
    ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPostureKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OrdinaryDomain;

impl ForgeQueryDomainEntryMarker for OrdinaryDomain {
    fn domain_key(&self) -> &'static str {
        "test.ordinary.domain"
    }

    fn display_name(&self) -> &'static str {
        "OrdinaryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OrdinaryWorld;

impl ForgeQueryDomainOperatingContext<OrdinaryDomain> for OrdinaryWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "ordinary-world".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OrdinaryFamily;

impl ForgeQueryDeclarationFamilyMarker<OrdinaryDomain> for OrdinaryFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "OrdinaryFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(&["selection.edge"], &[], &[], &[], &[])
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExpensiveFamily;

impl ForgeQueryDeclarationFamilyMarker<OrdinaryDomain> for ExpensiveFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ExpensiveFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::expensive_by_default_for_tests()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Input<F> {
    id: &'static str,
    _marker: PhantomData<F>,
}

impl<F> Input<F> {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<OrdinaryDomain> for Input<OrdinaryFamily> {
    type Family = OrdinaryFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

impl ForgeQueryDeclarationInput<OrdinaryDomain> for Input<ExpensiveFamily> {
    type Family = ExpensiveFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[test]
fn ordinary_binding_projection_keeps_wrong_world_distinct() {
    let checked = ForgeQueryBindingChecked::new(
        ForgeQueryBindingOutcome::<()>::WrongWorld(ForgeQueryBindingWrongWorld::new("wrong world")),
        "digest".to_string(),
        ForgeQueryBindingLinkedArtifacts::new(),
    );
    let ordinary = ordinary_outcome_from_binding_outcome(checked);
    match ordinary {
        ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            assert_eq!(posture.kind(), ForgeQueryOrdinaryPostureKind::WrongWorld);
            assert_eq!(
                posture.next_step(),
                ForgeQueryOrdinaryNextStep::CorrectWorld
            );
            assert_eq!(
                posture.checked_topology().binding_kind(),
                Some(ForgeQueryOrdinaryBindingCheckedTopologyKind::WrongWorld)
            );
            assert!(posture
                .checked_topology()
                .binding_linked_artifacts()
                .is_some());
        }
        other => panic!(
            "unexpected ordinary outcome: {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

#[test]
fn ordinary_orchestration_projection_preserves_route_stop_stage() {
    let terminal = ForgeQueryDeclarationEntryOrchestrationTerminalError::<
        OrdinaryDomain,
        Input<OrdinaryFamily>,
    >::Denied(ForgeQueryDeclarationEntryOrchestrationDenied::new(
        OrdinaryFamily::semantic_family_key(),
        ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
        "route denied",
        Some("route-denial".to_string()),
    ));
    let ordinary = ordinary_outcome_from_orchestration_terminal(terminal);
    match ordinary {
        ForgeQueryOrdinaryOutcome::Denied(posture) => {
            assert_eq!(posture.kind(), ForgeQueryOrdinaryPostureKind::Denied);
            assert_eq!(
                posture.checked_topology().orchestration_stop_stage(),
                Some(ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned)
            );
        }
        other => panic!(
            "unexpected ordinary outcome: {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

#[test]
fn ordinary_orchestration_projection_preserves_refusal_class() {
    let automation = ForgeQueryDeclarationEntryOrchestrationAutomationRefusal::new(
        ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::ExpensiveAutomationForbidden,
        ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
        "explicit handoff required",
        ExpensiveFamily::semantic_family_key(),
        Some("expensive-route".to_string()),
        "orchestration-digest",
        ForgeQueryDeclarationEntryOrchestrationAutomationBoundary::EnvelopeCeiling,
    );
    let terminal = ForgeQueryDeclarationEntryOrchestrationTerminalError::<
        OrdinaryDomain,
        Input<ExpensiveFamily>,
    >::Refused(
        ForgeQueryDeclarationEntryOrchestrationRefusal::from_automation(
            automation,
            ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
        ),
    );
    let ordinary = ordinary_outcome_from_orchestration_terminal(terminal);
    match ordinary {
        ForgeQueryOrdinaryOutcome::Refused(posture) => {
            assert_eq!(posture.kind(), ForgeQueryOrdinaryPostureKind::Refused);
            assert_eq!(
                posture.next_step(),
                ForgeQueryOrdinaryNextStep::UseExplicitHandoff
            );
            assert_eq!(
                posture.checked_topology().orchestration_refusal_class(),
                Some(
                    ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExpensiveWorkNotAdmittedByDefault
                )
            );
        }
        other => panic!(
            "unexpected ordinary outcome: {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

#[test]
fn admitted_handle_exposes_ordinary_binding_and_orchestration_entrypoints() {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(OrdinaryDomain)
        .with_operating_context(OrdinaryWorld)
        .validate()
        .expect("validated handle")
        .admit()
        .expect("admitted handle");

    let bind_outcome =
        handle.bind_declaration_from_context_outcome(ForgeQueryDeclarationBindingRequest::<
            Input<OrdinaryFamily>,
        >::new(
            vec![],
            OrdinaryFamily::aspect_contract(),
            vec![ForgeQueryBindingSourceKind::ExplicitSelection],
        ));
    assert!(matches!(
        bind_outcome,
        ForgeQueryOrdinaryOutcome::Unavailable(_)
    ));

    let orchestration_outcome =
        handle.orchestrate_declaration_entry_outcome(Input::<ExpensiveFamily>::new("edge-a"));
    assert!(matches!(
        orchestration_outcome,
        ForgeQueryOrdinaryOutcome::Refused(_)
    ));
}
