use std::marker::PhantomData;

use crate::application::{
    WorthQueryApplicationFacade, WorthQueryBridgeContinuationAuthority, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRouteContract, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryInstalledDomainDeclarationContext,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority,
    WorthQuerySignalCompatiblePosture,
};
use crate::binding_pipeline::{
    WorthQueryBindingOutcome, WorthQueryBindingSourceKind, WorthQueryBindingSpecificity,
    WorthQueryContinuationBindingRequest, WorthQueryDeclarationBindingRequest,
    WorthQueryDeclarationContextCandidate, WorthQueryEnvelopeContextCandidate,
    WorthQueryEnvelopeResolverSubject, WorthQueryProgressionContextCandidate,
    WorthQueryReceiptResolverSubject, WorthQueryResolveEnvelopeFromTargetRequest,
    WorthQueryResolveReceiptFromTargetRequest, WorthQueryResolveRouteFromTargetRequest,
    WorthQueryRouteBindingRequest, WorthQueryRouteResolverSubject,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BindingDomain;

impl WorthQueryDomainEntryMarker for BindingDomain {
    fn domain_key(&self) -> &'static str {
        "test.binding.domain"
    }
    fn display_name(&self) -> &'static str {
        "BindingDomain"
    }
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BindingWorld(&'static str);

impl WorthQueryDomainOperatingContext<BindingDomain> for BindingWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }
    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
            WorthQueryConfigSectionFamily::RuntimeBridge,
        ]
    }
    fn context_identity_digest(&self) -> String {
        format!("binding-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RouteFamily;

impl WorthQueryDeclarationFamilyMarker<BindingDomain> for RouteFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;
    fn semantic_family_key() -> &'static str {
        "RouteFamily"
    }
    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(&["selection.edge"], &[], &[], &[], &[])
    }
    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BridgeFamily;

impl WorthQueryDeclarationFamilyMarker<BindingDomain> for BridgeFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;
    fn semantic_family_key() -> &'static str {
        "BridgeFamily"
    }
    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(&["selection.face"], &[], &[], &[], &[])
    }
    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_and_bridge()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StrictBridgeFamily;

impl WorthQueryDeclarationFamilyMarker<BindingDomain> for StrictBridgeFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;
    fn semantic_family_key() -> &'static str {
        "StrictBridgeFamily"
    }
    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(&["selection.face"], &[], &[], &[], &[])
    }
    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_and_bridge()
    }
    fn bridge_continuation_contract(
    ) -> Option<crate::application::WorthQueryDeclarationBridgeContinuationContract> {
        Some(
            crate::application::WorthQueryDeclarationBridgeContinuationContract::runtime_route_current()
                .with_required_aspects(WorthQueryDeclarationAspectContract::from_slices(
                    &["selection.face", "bridge.runtime.token"],
                    &[],
                    &[],
                    &[],
                    &[],
                )),
        )
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

impl WorthQueryDeclarationInput<BindingDomain> for Input<RouteFamily> {
    type Family = RouteFamily;
    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}
impl WorthQueryDeclarationInput<BindingDomain> for Input<BridgeFamily> {
    type Family = BridgeFamily;
    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}
impl WorthQueryDeclarationInput<BindingDomain> for Input<StrictBridgeFamily> {
    type Family = StrictBridgeFamily;
    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

fn admitted_handle(
    world: &'static str,
) -> WorthQueryInstalledDomainDeclarationContext<BindingDomain, BindingWorld> {
    crate::application::domain_test_support::installed_declaration_context(
        BindingDomain,
        BindingWorld(world),
        [
            crate::application::domain_test_support::family::<BindingDomain, RouteFamily>(),
            crate::application::domain_test_support::family::<BindingDomain, BridgeFamily>(),
            crate::application::domain_test_support::family::<BindingDomain, StrictBridgeFamily>(),
        ],
    )
}

fn progressed_route(
    handle: &WorthQueryInstalledDomainDeclarationContext<BindingDomain, BindingWorld>,
    id: &'static str,
) -> crate::application::WorthQueryAdmittedDeclarationProgression<BindingDomain, Input<RouteFamily>>
{
    match handle.declare_review_and_progress(Input::<RouteFamily>::new(id)) {
        Ok(progressed) => progressed,
        Err(_) => panic!("expected progressed route declaration"),
    }
}

fn progressed_bridge(
    handle: &WorthQueryInstalledDomainDeclarationContext<BindingDomain, BindingWorld>,
    id: &'static str,
) -> crate::application::WorthQueryAdmittedDeclarationProgression<BindingDomain, Input<BridgeFamily>>
{
    match handle.declare_review_and_progress(Input::<BridgeFamily>::new(id)) {
        Ok(progressed) => progressed,
        Err(_) => panic!("expected progressed bridge declaration"),
    }
}

fn progressed_strict_bridge(
    handle: &WorthQueryInstalledDomainDeclarationContext<BindingDomain, BindingWorld>,
    id: &'static str,
) -> crate::application::WorthQueryAdmittedDeclarationProgression<
    BindingDomain,
    Input<StrictBridgeFamily>,
> {
    match handle.declare_review_and_progress(Input::<StrictBridgeFamily>::new(id)) {
        Ok(progressed) => progressed,
        Err(_) => panic!("expected progressed strict bridge declaration"),
    }
}

#[test]
fn declaration_context_binding_denies_when_candidates_tie() {
    let handle = admitted_handle("main");
    let request = WorthQueryDeclarationBindingRequest::new(
        vec![
            WorthQueryDeclarationContextCandidate::new(
                "explicit-a",
                WorthQueryBindingSourceKind::ExplicitSelection,
                WorthQueryBindingSpecificity::ExactExplicit,
                Input::<RouteFamily>::new("edge-a"),
            ),
            WorthQueryDeclarationContextCandidate::new(
                "explicit-b",
                WorthQueryBindingSourceKind::ExplicitSelection,
                WorthQueryBindingSpecificity::ExactExplicit,
                Input::<RouteFamily>::new("edge-b"),
            ),
        ],
        RouteFamily::aspect_contract(),
        vec![WorthQueryBindingSourceKind::ExplicitSelection],
    );
    let outcome = handle.bind_declaration_from_context(request);
    assert!(matches!(outcome, WorthQueryBindingOutcome::Ambiguous(_)));
}

#[test]
fn route_binding_from_context_matches_explicit_progressed_path() {
    let handle = admitted_handle("main");
    let progressed = progressed_route(&handle, "edge-a");
    let explicit = match handle.plan_routes_from_progressed(progressed.clone()) {
        Ok(plan) => plan,
        Err(_) => panic!("expected explicit route plan"),
    };
    let request = WorthQueryRouteBindingRequest::new(
        vec![WorthQueryProgressionContextCandidate::new(
            "current-progression",
            WorthQueryBindingSourceKind::CurrentProgression,
            WorthQueryBindingSpecificity::TypedCurrentArtifact,
            progressed,
        )],
        RouteFamily::aspect_contract(),
        vec![WorthQueryBindingSourceKind::CurrentProgression],
    );
    let outcome = handle.bind_route_request_from_context(request);
    let bound_input = match outcome {
        WorthQueryBindingOutcome::Bound(input) => input,
        _ => panic!("expected bound route input"),
    };
    let rebound = match handle.plan_routes(bound_input) {
        Ok(plan) => plan,
        Err(_) => panic!("expected rebound route plan"),
    };
    assert_eq!(explicit.route_plan_digest(), rebound.route_plan_digest());
}

#[test]
fn route_target_binding_preserves_wrong_world() {
    let left = admitted_handle("left");
    let right = admitted_handle("right");
    let progressed = progressed_route(&left, "edge-a");
    let request = WorthQueryResolveRouteFromTargetRequest::new(
        WorthQueryRouteResolverSubject::Progression(progressed),
        RouteFamily::aspect_contract(),
    );
    let outcome = right.bind_route_from_target(request);
    assert!(matches!(outcome, WorthQueryBindingOutcome::WrongWorld(_)));
}

#[test]
fn retained_target_binding_matches_explicit_receipt_and_envelope_paths() {
    let handle = admitted_handle("main");
    let progressed = progressed_route(&handle, "edge-a");
    let route_plan = match handle.plan_routes_from_progressed(progressed.clone()) {
        Ok(plan) => plan,
        Err(_) => panic!("expected explicit route plan"),
    };
    let explicit_receipt = match handle.receipt_routes_from_progressed(progressed.clone()) {
        Ok(receipt) => receipt,
        Err(_) => panic!("expected explicit receipt"),
    };
    let explicit_receipt_digest = format!("{:?}", explicit_receipt.receipt_digest());
    let receipt_input =
        match handle.bind_receipt_from_target(WorthQueryResolveReceiptFromTargetRequest::new(
            WorthQueryReceiptResolverSubject::RoutePlan(route_plan),
            RouteFamily::aspect_contract(),
        )) {
            WorthQueryBindingOutcome::Bound(input) => input,
            _ => panic!("expected receipt binding"),
        };
    let rebound_receipt = match handle.receipt_routes(receipt_input) {
        Ok(receipt) => receipt,
        Err(_) => panic!("expected rebound receipt"),
    };
    let envelope_input =
        match handle.bind_envelope_from_target(WorthQueryResolveEnvelopeFromTargetRequest::new(
            WorthQueryEnvelopeResolverSubject::Receipt(explicit_receipt),
            RouteFamily::aspect_contract(),
        )) {
            WorthQueryBindingOutcome::Bound(input) => input,
            _ => panic!("expected envelope binding"),
        };
    let rebound_envelope = match handle.envelope_routes(envelope_input) {
        Ok(envelope) => envelope,
        Err(_) => panic!("expected rebound envelope"),
    };
    let explicit_envelope = match handle.envelope_routes_from_progressed(progressed) {
        Ok(envelope) => envelope,
        Err(_) => panic!("expected explicit envelope"),
    };
    assert_eq!(
        explicit_receipt_digest,
        format!("{:?}", rebound_receipt.receipt_digest())
    );
    assert_eq!(
        explicit_envelope.envelope_digest(),
        rebound_envelope.envelope_digest()
    );
}

#[test]
fn continuation_binding_from_envelope_produces_bridge_request() {
    let handle = admitted_handle("main");
    let progressed = progressed_bridge(&handle, "face-a");
    let envelope = match handle.envelope_routes_from_progressed(progressed.clone()) {
        Ok(envelope) => envelope,
        Err(_) => panic!("expected envelope from progressed bridge declaration"),
    };
    let request = WorthQueryContinuationBindingRequest::new(
        vec![WorthQueryEnvelopeContextCandidate::new(
            "current-envelope",
            WorthQueryBindingSourceKind::CurrentEnvelope,
            WorthQueryBindingSpecificity::TypedCurrentArtifact,
            envelope,
        )],
        BridgeFamily::aspect_contract(),
        vec![WorthQueryBindingSourceKind::CurrentEnvelope],
    );
    let outcome = handle.bind_continuation_request_from_context(request);
    let bound = match outcome {
        WorthQueryBindingOutcome::Bound(bound) => bound,
        _ => panic!("expected continuation binding"),
    };
    assert_eq!(bound.bridge_request().mode().as_str(), "runtime_route");
}

#[test]
fn continuation_binding_reports_authority_mismatch_when_contract_requires_hidden_bridge_slice() {
    let handle = admitted_handle("main");
    let progressed = progressed_strict_bridge(&handle, "face-a");
    let envelope = match handle.envelope_routes_from_progressed(progressed) {
        Ok(envelope) => envelope,
        Err(_) => panic!("expected strict bridge envelope"),
    };
    let request = WorthQueryContinuationBindingRequest::new(
        vec![WorthQueryEnvelopeContextCandidate::new(
            "current-envelope",
            WorthQueryBindingSourceKind::CurrentEnvelope,
            WorthQueryBindingSpecificity::TypedCurrentArtifact,
            envelope,
        )],
        StrictBridgeFamily::aspect_contract(),
        vec![WorthQueryBindingSourceKind::CurrentEnvelope],
    );
    let outcome = handle.bind_continuation_request_from_context(request);
    assert!(matches!(
        outcome,
        WorthQueryBindingOutcome::AuthorityMismatch(_)
    ));
}

#[test]
fn binding_proof_exposes_linked_artifacts_and_witness_checks() {
    let handle = admitted_handle("main");
    let progressed = progressed_route(&handle, "edge-a");
    let proof = handle.bind_route_from_target_proof(WorthQueryResolveRouteFromTargetRequest::new(
        WorthQueryRouteResolverSubject::Progression(progressed.clone()),
        RouteFamily::aspect_contract(),
    ));
    assert_eq!(proof.request().request_kind(), "resolve_route_from_target");
    assert_eq!(proof.witness_checks().len(), 1);
    assert!(proof.witness_checks()[0].did_pass());
    assert!(proof.resolved_target().is_some());
    assert_eq!(
        proof.linked_artifacts().progression_digest(),
        Some(progressed.progression_digest())
    );
}

#[test]
fn binding_digest_changes_when_required_aspect_contract_changes() {
    let handle = admitted_handle("main");
    let progressed = progressed_route(&handle, "edge-a");
    let exact = handle.bind_route_from_target_proof(WorthQueryResolveRouteFromTargetRequest::new(
        WorthQueryRouteResolverSubject::Progression(progressed.clone()),
        RouteFamily::aspect_contract(),
    ));
    let narrowed =
        handle.bind_route_from_target_proof(WorthQueryResolveRouteFromTargetRequest::new(
            WorthQueryRouteResolverSubject::Progression(progressed),
            WorthQueryDeclarationAspectContract::from_slices(
                &["selection.edge", "selection.material"],
                &[],
                &[],
                &[],
                &[],
            ),
        ));
    assert_ne!(exact.binding_digest(), narrowed.binding_digest());
}
