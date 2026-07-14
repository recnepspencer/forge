use std::marker::PhantomData;

use crate::application::{
    WorthQueryApplicationFacade, WorthQueryBridgeContinuationAuthority,
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRouteContract, WorthQueryGraphObligationOrchestrationBoundary,
    WorthQueryGraphObligationOrchestrationDispatchError, WorthQueryNeighborhoodCapableGrouping,
};
use crate::contribution_composed_orchestration::{
    WorthQueryContributionComposedOrchestrationInput,
    WorthQueryContributionComposedOrchestrationOutcome, WorthQueryContributionIntent,
};
use crate::domain_capabilities::WorthQuerySupportContributionAuthoring;
use crate::runtime::{
    WorthQueryGraphObligationDispatchContextKind, WorthQueryGraphObligationOperatingWorldSelector,
    WorthQueryGraphObligationRegistration, WorthQueryGraphObligationRuleIdentity,
    WorthQueryGraphObligationSupportLane, WorthQueryGraphObligationSupportPosture,
    WorthQueryGraphTouchSelector,
};

use super::support::{ContributionDomain, ContributionWorld};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BlockingContributionFamily;

impl WorthQueryDeclarationFamilyMarker<ContributionDomain> for BlockingContributionFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::WorthQuerySignalNotCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "BlockingContributionFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(&["selection.face"], &[], &[], &[], &[])
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(&["selection.face"], &[], &[])
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn orchestration_graph_touch_collection() -> Option<&'static str> {
        Some("faces")
    }

    fn orchestration_graph_obligation_registrations() -> Vec<WorthQueryGraphObligationRegistration>
    {
        vec![blocking_registration(
            WorthQueryGraphObligationSupportLane::ContributionOrchestration,
        )]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BlockingContributionInput {
    id: &'static str,
    _marker: PhantomData<BlockingContributionFamily>,
}

impl BlockingContributionInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl WorthQueryDeclarationInput<ContributionDomain> for BlockingContributionInput {
    type Family = BlockingContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MissingTouchContributionFamily;

impl WorthQueryDeclarationFamilyMarker<ContributionDomain> for MissingTouchContributionFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::WorthQuerySignalNotCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MissingTouchContributionFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        BlockingContributionFamily::aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        BlockingContributionFamily::aspect_coverage()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn orchestration_graph_obligation_registrations() -> Vec<WorthQueryGraphObligationRegistration>
    {
        vec![blocking_registration(
            WorthQueryGraphObligationSupportLane::ContributionOrchestration,
        )]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MissingTouchContributionInput {
    id: &'static str,
    _marker: PhantomData<MissingTouchContributionFamily>,
}

impl MissingTouchContributionInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl WorthQueryDeclarationInput<ContributionDomain> for MissingTouchContributionInput {
    type Family = MissingTouchContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AdvisoryContributionFamily;

impl WorthQueryDeclarationFamilyMarker<ContributionDomain> for AdvisoryContributionFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::WorthQuerySignalNotCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AdvisoryContributionFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        BlockingContributionFamily::aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        BlockingContributionFamily::aspect_coverage()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn orchestration_graph_touch_collection() -> Option<&'static str> {
        Some("faces")
    }

    fn orchestration_graph_obligation_registrations() -> Vec<WorthQueryGraphObligationRegistration>
    {
        vec![advisory_registration(
            WorthQueryGraphObligationSupportLane::ContributionOrchestration,
        )]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AdvisoryContributionInput {
    id: &'static str,
    _marker: PhantomData<AdvisoryContributionFamily>,
}

impl AdvisoryContributionInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl WorthQueryDeclarationInput<ContributionDomain> for AdvisoryContributionInput {
    type Family = AdvisoryContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[test]
fn contribution_orchestration_dispatch_denies_before_contributions_materialize() {
    let handle = WorthQueryApplicationFacade::runtime_backed_default()
        .domain(ContributionDomain)
        .with_operating_context(ContributionWorld("main"))
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let proof = handle.orchestrate_declaration_with_contributions_proof(
        WorthQueryContributionComposedOrchestrationInput::new(BlockingContributionInput::new(
            "face-blocked",
        ))
        .with_contribution(WorthQueryContributionIntent::support(
            WorthQuerySupportContributionAuthoring::declaration_traceability(
                "domain.traceability.face",
                "this contribution must not materialize after graph obligation denial",
            ),
        )),
    );

    assert!(matches!(
        proof.outcome(),
        WorthQueryContributionComposedOrchestrationOutcome::DeclarationDenied(_)
    ));
    assert!(
        proof.intent_results().is_empty(),
        "contribution intents should not run after orchestration graph obligation denial",
    );
    let dispatch = proof
        .graph_obligation_dispatch()
        .expect("orchestration denial should carry graph obligation dispatch evidence");
    assert_eq!(
        dispatch.boundary(),
        WorthQueryGraphObligationOrchestrationBoundary::ContributionComposed
    );
    assert_eq!(
        dispatch.operating_context_identity_digest(),
        handle.operating_context_identity_digest()
    );
    let projection = dispatch.evidence_projection();
    assert_eq!(
        projection.context_kind(),
        Some(WorthQueryGraphObligationDispatchContextKind::ContributionComposed)
    );
    let row = projection
        .rows()
        .first()
        .expect("blocking orchestration dispatch should project one rule row");
    assert_eq!(row.rule_name(), "phase-seven-blocking");
    assert_eq!(
        row.support_lane(),
        WorthQueryGraphObligationSupportLane::ContributionOrchestration
    );
    assert_eq!(row.verdict(), "block");
}

#[test]
fn registration_without_orchestration_touch_collection_fails_typed() {
    let handle = WorthQueryApplicationFacade::runtime_backed_default()
        .domain(ContributionDomain)
        .with_operating_context(ContributionWorld("main"))
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let proof = handle.orchestrate_declaration_with_contributions_proof(
        WorthQueryContributionComposedOrchestrationInput::new(MissingTouchContributionInput::new(
            "face-missing-touch",
        ))
        .with_contribution(WorthQueryContributionIntent::support(
            WorthQuerySupportContributionAuthoring::declaration_traceability(
                "domain.traceability.face",
                "registration without touch collection must fail typed",
            ),
        )),
    );

    match proof.outcome() {
        WorthQueryContributionComposedOrchestrationOutcome::Failed(posture) => {
            assert!(matches!(
                posture.graph_obligation_dispatch_error(),
                Some(
                    WorthQueryGraphObligationOrchestrationDispatchError::MissingTouchCollection {
                        boundary:
                            WorthQueryGraphObligationOrchestrationBoundary::ContributionComposed,
                    }
                )
            ));
            assert!(posture.graph_obligation_dispatch().is_none());
        }
        _ => panic!("missing touch collection should fail orchestration dispatch"),
    }
}

#[test]
fn non_blocking_orchestration_dispatch_survives_bound_contribution_artifact() {
    let handle = WorthQueryApplicationFacade::runtime_backed_default()
        .domain(ContributionDomain)
        .with_operating_context(ContributionWorld("main"))
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let proof = handle.orchestrate_declaration_with_contributions_proof(
        WorthQueryContributionComposedOrchestrationInput::new(AdvisoryContributionInput::new(
            "face-advisory",
        ))
        .with_contribution(WorthQueryContributionIntent::support(
            WorthQuerySupportContributionAuthoring::declaration_traceability(
                "domain.traceability.face",
                "advisory graph obligation should not block materialization",
            ),
        )),
    );

    let artifact = match proof.outcome() {
        WorthQueryContributionComposedOrchestrationOutcome::Bound(artifact) => artifact,
        _ => panic!("advisory graph obligation should not block contribution composition"),
    };
    let dispatch = artifact
        .graph_obligation_dispatch()
        .expect("bound contribution artifact should retain advisory graph obligation evidence");
    let row = dispatch.evidence_projection().rows()[0].clone();
    assert_eq!(row.rule_name(), "phase-seven-advisory");
    assert_eq!(row.verdict(), "advise");
    assert_eq!(
        row.support_lane(),
        WorthQueryGraphObligationSupportLane::ContributionOrchestration
    );
}

fn blocking_registration(
    lane: WorthQueryGraphObligationSupportLane,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::blocking_invariant(
        WorthQueryGraphObligationRuleIdentity::new(
            "worth-query.phase-seven",
            "phase-seven-blocking",
            "v1",
        )
        .unwrap(),
        WorthQueryGraphTouchSelector::collection("faces").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
    )
    .with_support_posture(WorthQueryGraphObligationSupportPosture::unsupported(lane))
}

fn advisory_registration(
    lane: WorthQueryGraphObligationSupportLane,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::advisory_obligation(
        WorthQueryGraphObligationRuleIdentity::new(
            "worth-query.phase-seven",
            "phase-seven-advisory",
            "v1",
        )
        .unwrap(),
        WorthQueryGraphTouchSelector::collection("faces").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
    )
    .with_support_posture(WorthQueryGraphObligationSupportPosture::diagnostic_only(
        lane,
    ))
}
