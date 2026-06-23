use std::marker::PhantomData;

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryBridgeContinuationAuthority,
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryGraphObligationOrchestrationBoundary,
    ForgeQueryGraphObligationOrchestrationDispatchError, ForgeQueryNeighborhoodCapableGrouping,
};
use crate::contribution_composed_orchestration::{
    ForgeQueryContributionComposedOrchestrationInput,
    ForgeQueryContributionComposedOrchestrationOutcome, ForgeQueryContributionIntent,
};
use crate::domain_capabilities::ForgeQuerySupportContributionAuthoring;
use crate::runtime::{
    ForgeQueryGraphObligationDispatchContextKind, ForgeQueryGraphObligationOperatingWorldSelector,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationRuleIdentity,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphTouchSelector,
};

use super::support::{ContributionDomain, ContributionWorld};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BlockingContributionFamily;

impl ForgeQueryDeclarationFamilyMarker<ContributionDomain> for BlockingContributionFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "BlockingContributionFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(&["selection.face"], &[], &[], &[], &[])
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(&["selection.face"], &[], &[])
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn orchestration_graph_touch_collection() -> Option<&'static str> {
        Some("faces")
    }

    fn orchestration_graph_obligation_registrations() -> Vec<ForgeQueryGraphObligationRegistration>
    {
        vec![blocking_registration(
            ForgeQueryGraphObligationSupportLane::ContributionOrchestration,
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

impl ForgeQueryDeclarationInput<ContributionDomain> for BlockingContributionInput {
    type Family = BlockingContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MissingTouchContributionFamily;

impl ForgeQueryDeclarationFamilyMarker<ContributionDomain> for MissingTouchContributionFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MissingTouchContributionFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        BlockingContributionFamily::aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        BlockingContributionFamily::aspect_coverage()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn orchestration_graph_obligation_registrations() -> Vec<ForgeQueryGraphObligationRegistration>
    {
        vec![blocking_registration(
            ForgeQueryGraphObligationSupportLane::ContributionOrchestration,
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

impl ForgeQueryDeclarationInput<ContributionDomain> for MissingTouchContributionInput {
    type Family = MissingTouchContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AdvisoryContributionFamily;

impl ForgeQueryDeclarationFamilyMarker<ContributionDomain> for AdvisoryContributionFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AdvisoryContributionFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        BlockingContributionFamily::aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        BlockingContributionFamily::aspect_coverage()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn orchestration_graph_touch_collection() -> Option<&'static str> {
        Some("faces")
    }

    fn orchestration_graph_obligation_registrations() -> Vec<ForgeQueryGraphObligationRegistration>
    {
        vec![advisory_registration(
            ForgeQueryGraphObligationSupportLane::ContributionOrchestration,
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

impl ForgeQueryDeclarationInput<ContributionDomain> for AdvisoryContributionInput {
    type Family = AdvisoryContributionFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[test]
fn contribution_orchestration_dispatch_denies_before_contributions_materialize() {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ContributionDomain)
        .with_operating_context(ContributionWorld("main"))
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let proof = handle.orchestrate_declaration_with_contributions_proof(
        ForgeQueryContributionComposedOrchestrationInput::new(BlockingContributionInput::new(
            "face-blocked",
        ))
        .with_contribution(ForgeQueryContributionIntent::support(
            ForgeQuerySupportContributionAuthoring::declaration_traceability(
                "domain.traceability.face",
                "this contribution must not materialize after graph obligation denial",
            ),
        )),
    );

    assert!(matches!(
        proof.outcome(),
        ForgeQueryContributionComposedOrchestrationOutcome::DeclarationDenied(_)
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
        ForgeQueryGraphObligationOrchestrationBoundary::ContributionComposed
    );
    assert_eq!(
        dispatch.operating_context_identity_digest(),
        handle.operating_context_identity_digest()
    );
    let projection = dispatch.evidence_projection();
    assert_eq!(
        projection.context_kind(),
        Some(ForgeQueryGraphObligationDispatchContextKind::ContributionComposed)
    );
    let row = projection
        .rows()
        .first()
        .expect("blocking orchestration dispatch should project one rule row");
    assert_eq!(row.rule_name(), "phase-seven-blocking");
    assert_eq!(
        row.support_lane(),
        ForgeQueryGraphObligationSupportLane::ContributionOrchestration
    );
    assert_eq!(row.verdict(), "block");
}

#[test]
fn registration_without_orchestration_touch_collection_fails_typed() {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ContributionDomain)
        .with_operating_context(ContributionWorld("main"))
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let proof = handle.orchestrate_declaration_with_contributions_proof(
        ForgeQueryContributionComposedOrchestrationInput::new(MissingTouchContributionInput::new(
            "face-missing-touch",
        ))
        .with_contribution(ForgeQueryContributionIntent::support(
            ForgeQuerySupportContributionAuthoring::declaration_traceability(
                "domain.traceability.face",
                "registration without touch collection must fail typed",
            ),
        )),
    );

    match proof.outcome() {
        ForgeQueryContributionComposedOrchestrationOutcome::Failed(posture) => {
            assert!(matches!(
                posture.graph_obligation_dispatch_error(),
                Some(
                    ForgeQueryGraphObligationOrchestrationDispatchError::MissingTouchCollection {
                        boundary:
                            ForgeQueryGraphObligationOrchestrationBoundary::ContributionComposed,
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
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ContributionDomain)
        .with_operating_context(ContributionWorld("main"))
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let proof = handle.orchestrate_declaration_with_contributions_proof(
        ForgeQueryContributionComposedOrchestrationInput::new(AdvisoryContributionInput::new(
            "face-advisory",
        ))
        .with_contribution(ForgeQueryContributionIntent::support(
            ForgeQuerySupportContributionAuthoring::declaration_traceability(
                "domain.traceability.face",
                "advisory graph obligation should not block materialization",
            ),
        )),
    );

    let artifact = match proof.outcome() {
        ForgeQueryContributionComposedOrchestrationOutcome::Bound(artifact) => artifact,
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
        ForgeQueryGraphObligationSupportLane::ContributionOrchestration
    );
}

fn blocking_registration(
    lane: ForgeQueryGraphObligationSupportLane,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new(
            "forge-query.phase-seven",
            "phase-seven-blocking",
            "v1",
        )
        .unwrap(),
        ForgeQueryGraphTouchSelector::collection("faces").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::unsupported(lane))
}

fn advisory_registration(
    lane: ForgeQueryGraphObligationSupportLane,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::advisory_obligation(
        ForgeQueryGraphObligationRuleIdentity::new(
            "forge-query.phase-seven",
            "phase-seven-advisory",
            "v1",
        )
        .unwrap(),
        ForgeQueryGraphTouchSelector::collection("faces").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::diagnostic_only(
        lane,
    ))
}
