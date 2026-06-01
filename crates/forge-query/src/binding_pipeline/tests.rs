use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
};
use crate::binding_pipeline::{
    ForgeQueryBindingOutcome, ForgeQueryBindingSourceKind, ForgeQueryBindingSpecificity,
    ForgeQueryContinuationBindingRequest, ForgeQueryDeclarationBindingRequest,
    ForgeQueryDeclarationContextCandidate, ForgeQueryEnvelopeContextCandidate,
    ForgeQueryEnvelopeResolverSubject, ForgeQueryProgressionContextCandidate,
    ForgeQueryReceiptResolverSubject, ForgeQueryResolveEnvelopeFromTargetRequest,
    ForgeQueryResolveReceiptFromTargetRequest, ForgeQueryResolveRouteFromTargetRequest,
    ForgeQueryRouteBindingRequest, ForgeQueryRouteResolverSubject,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BindingDomain;

impl ForgeQueryDomainEntryMarker for BindingDomain {
    fn domain_key(&self) -> &'static str {
        "test.binding.domain"
    }
    fn display_name(&self) -> &'static str {
        "BindingDomain"
    }
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BindingWorld(&'static str);

impl ForgeQueryDomainOperatingContext<BindingDomain> for BindingWorld {
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
        format!("binding-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RouteFamily;

impl ForgeQueryDeclarationFamilyMarker<BindingDomain> for RouteFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;
    fn semantic_family_key() -> &'static str {
        "RouteFamily"
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
struct BridgeFamily;

impl ForgeQueryDeclarationFamilyMarker<BindingDomain> for BridgeFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;
    fn semantic_family_key() -> &'static str {
        "BridgeFamily"
    }
    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(&["selection.face"], &[], &[], &[], &[])
    }
    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_and_bridge()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StrictBridgeFamily;

impl ForgeQueryDeclarationFamilyMarker<BindingDomain> for StrictBridgeFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;
    fn semantic_family_key() -> &'static str {
        "StrictBridgeFamily"
    }
    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(&["selection.face"], &[], &[], &[], &[])
    }
    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_and_bridge()
    }
    fn bridge_continuation_contract(
    ) -> Option<crate::application::ForgeQueryDeclarationBridgeContinuationContract> {
        Some(
            crate::application::ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current()
                .with_required_aspects(ForgeQueryDeclarationAspectContract::from_slices(
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

impl ForgeQueryDeclarationInput<BindingDomain> for Input<RouteFamily> {
    type Family = RouteFamily;
    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}
impl ForgeQueryDeclarationInput<BindingDomain> for Input<BridgeFamily> {
    type Family = BridgeFamily;
    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}
impl ForgeQueryDeclarationInput<BindingDomain> for Input<StrictBridgeFamily> {
    type Family = StrictBridgeFamily;
    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

fn admitted_handle(
    world: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<BindingDomain, BindingWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(BindingDomain)
        .with_operating_context(BindingWorld(world))
        .validate()
        .unwrap()
        .admit()
        .unwrap()
}

fn progressed_route(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<BindingDomain, BindingWorld>,
    id: &'static str,
) -> crate::application::ForgeQueryAdmittedDeclarationProgression<BindingDomain, Input<RouteFamily>>
{
    match handle.declare_review_and_progress(Input::<RouteFamily>::new(id)) {
        Ok(progressed) => progressed,
        Err(_) => panic!("expected progressed route declaration"),
    }
}

fn progressed_bridge(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<BindingDomain, BindingWorld>,
    id: &'static str,
) -> crate::application::ForgeQueryAdmittedDeclarationProgression<BindingDomain, Input<BridgeFamily>>
{
    match handle.declare_review_and_progress(Input::<BridgeFamily>::new(id)) {
        Ok(progressed) => progressed,
        Err(_) => panic!("expected progressed bridge declaration"),
    }
}

fn progressed_strict_bridge(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<BindingDomain, BindingWorld>,
    id: &'static str,
) -> crate::application::ForgeQueryAdmittedDeclarationProgression<
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
    let request = ForgeQueryDeclarationBindingRequest::new(
        vec![
            ForgeQueryDeclarationContextCandidate::new(
                "explicit-a",
                ForgeQueryBindingSourceKind::ExplicitSelection,
                ForgeQueryBindingSpecificity::ExactExplicit,
                Input::<RouteFamily>::new("edge-a"),
            ),
            ForgeQueryDeclarationContextCandidate::new(
                "explicit-b",
                ForgeQueryBindingSourceKind::ExplicitSelection,
                ForgeQueryBindingSpecificity::ExactExplicit,
                Input::<RouteFamily>::new("edge-b"),
            ),
        ],
        RouteFamily::aspect_contract(),
        vec![ForgeQueryBindingSourceKind::ExplicitSelection],
    );
    let outcome = handle.bind_declaration_from_context(request);
    assert!(matches!(outcome, ForgeQueryBindingOutcome::Ambiguous(_)));
}

#[test]
fn route_binding_from_context_matches_explicit_progressed_path() {
    let handle = admitted_handle("main");
    let progressed = progressed_route(&handle, "edge-a");
    let explicit = match handle.plan_routes_from_progressed(progressed.clone()) {
        Ok(plan) => plan,
        Err(_) => panic!("expected explicit route plan"),
    };
    let request = ForgeQueryRouteBindingRequest::new(
        vec![ForgeQueryProgressionContextCandidate::new(
            "current-progression",
            ForgeQueryBindingSourceKind::CurrentProgression,
            ForgeQueryBindingSpecificity::TypedCurrentArtifact,
            progressed,
        )],
        RouteFamily::aspect_contract(),
        vec![ForgeQueryBindingSourceKind::CurrentProgression],
    );
    let outcome = handle.bind_route_request_from_context(request);
    let bound_input = match outcome {
        ForgeQueryBindingOutcome::Bound(input) => input,
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
    let request = ForgeQueryResolveRouteFromTargetRequest::new(
        ForgeQueryRouteResolverSubject::Progression(progressed),
        RouteFamily::aspect_contract(),
    );
    let outcome = right.bind_route_from_target(request);
    assert!(matches!(outcome, ForgeQueryBindingOutcome::WrongWorld(_)));
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
        match handle.bind_receipt_from_target(ForgeQueryResolveReceiptFromTargetRequest::new(
            ForgeQueryReceiptResolverSubject::RoutePlan(route_plan),
            RouteFamily::aspect_contract(),
        )) {
            ForgeQueryBindingOutcome::Bound(input) => input,
            _ => panic!("expected receipt binding"),
        };
    let rebound_receipt = match handle.receipt_routes(receipt_input) {
        Ok(receipt) => receipt,
        Err(_) => panic!("expected rebound receipt"),
    };
    let envelope_input =
        match handle.bind_envelope_from_target(ForgeQueryResolveEnvelopeFromTargetRequest::new(
            ForgeQueryEnvelopeResolverSubject::Receipt(explicit_receipt),
            RouteFamily::aspect_contract(),
        )) {
            ForgeQueryBindingOutcome::Bound(input) => input,
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
    let request = ForgeQueryContinuationBindingRequest::new(
        vec![ForgeQueryEnvelopeContextCandidate::new(
            "current-envelope",
            ForgeQueryBindingSourceKind::CurrentEnvelope,
            ForgeQueryBindingSpecificity::TypedCurrentArtifact,
            envelope,
        )],
        BridgeFamily::aspect_contract(),
        vec![ForgeQueryBindingSourceKind::CurrentEnvelope],
    );
    let outcome = handle.bind_continuation_request_from_context(request);
    let bound = match outcome {
        ForgeQueryBindingOutcome::Bound(bound) => bound,
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
    let request = ForgeQueryContinuationBindingRequest::new(
        vec![ForgeQueryEnvelopeContextCandidate::new(
            "current-envelope",
            ForgeQueryBindingSourceKind::CurrentEnvelope,
            ForgeQueryBindingSpecificity::TypedCurrentArtifact,
            envelope,
        )],
        StrictBridgeFamily::aspect_contract(),
        vec![ForgeQueryBindingSourceKind::CurrentEnvelope],
    );
    let outcome = handle.bind_continuation_request_from_context(request);
    assert!(matches!(
        outcome,
        ForgeQueryBindingOutcome::AuthorityMismatch(_)
    ));
}

#[test]
fn binding_proof_exposes_linked_artifacts_and_witness_checks() {
    let handle = admitted_handle("main");
    let progressed = progressed_route(&handle, "edge-a");
    let proof = handle.bind_route_from_target_proof(ForgeQueryResolveRouteFromTargetRequest::new(
        ForgeQueryRouteResolverSubject::Progression(progressed.clone()),
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
    let exact = handle.bind_route_from_target_proof(ForgeQueryResolveRouteFromTargetRequest::new(
        ForgeQueryRouteResolverSubject::Progression(progressed.clone()),
        RouteFamily::aspect_contract(),
    ));
    let narrowed =
        handle.bind_route_from_target_proof(ForgeQueryResolveRouteFromTargetRequest::new(
            ForgeQueryRouteResolverSubject::Progression(progressed),
            ForgeQueryDeclarationAspectContract::from_slices(
                &["selection.edge", "selection.material"],
                &[],
                &[],
                &[],
                &[],
            ),
        ));
    assert_ne!(exact.binding_digest(), narrowed.binding_digest());
}
