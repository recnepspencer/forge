use std::marker::PhantomData;

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationSignalCompatibilityContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQuerySignalCompatiblePosture,
};
use crate::binding_pipeline::ForgeQueryResolveContinuationFromTargetRequest;
use crate::binding_pipeline::{
    ForgeQueryBindingSourceKind, ForgeQueryBindingSpecificity,
    ForgeQueryContinuationBindingRequest, ForgeQueryEnvelopeContextCandidate,
};
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryContinuationCheckedTopologyKind, ForgeQueryOrdinaryOutcome,
};

use super::{
    ForgeQueryContinuationExecutionOutcome, ForgeQueryContinuationRuntimeContract,
    ForgeQueryContinuationTruthContext, ForgeQueryContinuationWorkspaceContract,
    ForgeQueryPreparedContinuationFamily, ForgeQueryPreparedContinuationOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ContinuationDomain;

impl ForgeQueryDomainEntryMarker for ContinuationDomain {
    fn domain_key(&self) -> &'static str {
        "test.continuation.domain"
    }

    fn display_name(&self) -> &'static str {
        "ContinuationDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ContinuationWorld(&'static str);

impl ForgeQueryDomainOperatingContext<ContinuationDomain> for ContinuationWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::PreviewSession]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("continuation-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ContinuationFamily;

impl ForgeQueryDeclarationFamilyMarker<ContinuationDomain> for ContinuationFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ContinuationFamily"
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
        Some(crate::application::ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContinuationInput {
    id: &'static str,
    _marker: PhantomData<ContinuationFamily>,
}

impl ContinuationInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<ContinuationDomain> for ContinuationInput {
    type Family = ContinuationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

fn admitted_handle(
    world: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    ContinuationDomain,
    ContinuationWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ContinuationDomain)
        .with_operating_context(ContinuationWorld(world))
        .validate()
        .unwrap()
        .admit()
        .unwrap()
}

fn runtime_route_request() -> crate::application::ForgeQueryDeclarationBridgeContinuationRequest {
    crate::application::ForgeQueryDeclarationBridgeContinuationRequest::new(
        crate::application::ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute,
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Current,
    )
}

fn envelope(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        ContinuationDomain,
        ContinuationWorld,
    >,
    id: &'static str,
) -> crate::application::ForgeQueryDeclarationEnvelope<ContinuationDomain, ContinuationInput> {
    let progressed = handle
        .declare_review_and_progress(ContinuationInput::new(id))
        .unwrap_or_else(|_| panic!("expected progressed continuation declaration"));
    handle
        .envelope_routes_from_progressed(progressed)
        .unwrap_or_else(|_| panic!("expected envelope"))
}

fn target_request(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        ContinuationDomain,
        ContinuationWorld,
    >,
    id: &'static str,
) -> ForgeQueryResolveContinuationFromTargetRequest<ContinuationDomain, ContinuationInput> {
    ForgeQueryResolveContinuationFromTargetRequest::new(
        envelope(handle, id),
        ContinuationFamily::aspect_contract(),
    )
    .with_bridge_request(runtime_route_request())
}

fn context_request(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        ContinuationDomain,
        ContinuationWorld,
    >,
    id: &'static str,
) -> ForgeQueryContinuationBindingRequest<ContinuationDomain, ContinuationInput> {
    ForgeQueryContinuationBindingRequest::new(
        vec![ForgeQueryEnvelopeContextCandidate::new(
            "current envelope",
            ForgeQueryBindingSourceKind::CurrentEnvelope,
            ForgeQueryBindingSpecificity::TypedCurrentArtifact,
            envelope(handle, id),
        )],
        ContinuationFamily::aspect_contract(),
        vec![ForgeQueryBindingSourceKind::CurrentEnvelope],
    )
    .with_bridge_request(runtime_route_request())
}

#[test]
fn prepare_and_execute_continuation_from_target() {
    let handle = admitted_handle("main");
    let prepared = match handle.prepare_continuation_from_target(target_request(&handle, "face-a"))
    {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("expected prepared continuation"),
    };
    assert_eq!(
        prepared.family(),
        ForgeQueryPreparedContinuationFamily::BridgeRuntimeRoute
    );
    assert_eq!(
        prepared.truth_context(),
        ForgeQueryContinuationTruthContext::Current
    );
    assert_eq!(
        prepared.runtime_contract(),
        ForgeQueryContinuationRuntimeContract::RuntimeRoute
    );
    assert_eq!(
        prepared.workspace_contract(),
        ForgeQueryContinuationWorkspaceContract::RuntimeWorkspace
    );
    assert!(!prepared.prepared_digest().is_empty());

    let executed = match handle.execute_prepared_continuation(prepared) {
        ForgeQueryContinuationExecutionOutcome::Executed(executed) => executed,
        _ => panic!("expected executed continuation"),
    };
    assert_eq!(
        executed.family(),
        ForgeQueryPreparedContinuationFamily::BridgeRuntimeRoute
    );
    assert!(!executed.execution_digest().is_empty());
}

#[test]
fn prepare_continuation_preserves_wrong_world() {
    let left = admitted_handle("left");
    let right = admitted_handle("right");
    let outcome = right.prepare_continuation_from_target(target_request(&left, "face-a"));
    assert!(matches!(
        outcome,
        ForgeQueryPreparedContinuationOutcome::WrongWorld(_)
    ));
}

#[test]
fn context_and_target_preparation_converge_for_equivalent_meaning() {
    let handle = admitted_handle("main");
    let from_target =
        handle.prepare_continuation_from_target_checked(target_request(&handle, "face-a"));
    let from_context =
        handle.prepare_continuation_from_context_checked(context_request(&handle, "face-a"));

    let target_prepared = match from_target.outcome() {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("expected target preparation"),
    };
    let context_prepared = match from_context.outcome() {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("expected context preparation"),
    };

    assert_eq!(
        target_prepared.prepared_digest(),
        context_prepared.prepared_digest()
    );
    assert_eq!(
        from_target.linked_artifacts(),
        from_context.linked_artifacts()
    );
}

#[test]
fn continuation_prepare_proof_exposes_target_specific_witnesses() {
    let handle = admitted_handle("main");
    let proof = handle.prepare_continuation_from_target_proof(target_request(&handle, "face-a"));

    assert_eq!(proof.request().request_kind(), "prepared_continuation");
    assert!(matches!(
        proof.outcome(),
        ForgeQueryPreparedContinuationOutcome::Prepared(_)
    ));
    assert_eq!(proof.witness_checks().len(), 3);
    assert_eq!(proof.witness_checks()[0].name(), "continuation_binding");
    assert!(proof.witness_checks()[0].did_pass());
    assert_eq!(proof.witness_checks()[1].name(), "signal_compatibility");
    assert_eq!(proof.witness_checks()[2].name(), "bridge_routing");
    assert!(proof.witness_checks()[2].did_pass());
    let prepared = match proof.outcome() {
        ForgeQueryPreparedContinuationOutcome::Prepared(prepared) => prepared,
        _ => panic!("expected prepared continuation proof outcome"),
    };
    if proof.witness_checks()[1].did_pass() {
        assert_eq!(
            prepared.signal_posture(),
            crate::continuation_pipeline::ForgeQueryPreparedContinuationSignalPosture::Compatible
        );
    } else {
        assert_ne!(
            prepared.signal_posture(),
            crate::continuation_pipeline::ForgeQueryPreparedContinuationSignalPosture::Compatible
        );
        assert!(!proof.narrowing_decisions().is_empty());
    }
    assert!(proof.linked_artifacts().declaration_digest().is_some());
    assert!(proof.linked_artifacts().route_plan_digest().is_some());
    assert!(proof.linked_artifacts().receipt_digest().is_some());
    assert!(proof.linked_artifacts().envelope_digest().is_some());
}

#[test]
fn continuation_ordinary_outcome_keeps_checked_topology_visible() {
    let handle = admitted_handle("main");
    let outcome =
        handle.prepare_continuation_from_target_outcome(target_request(&handle, "face-a"));

    let prepared = match outcome {
        ForgeQueryOrdinaryOutcome::Bound(prepared) => prepared,
        _ => panic!("expected bound continuation outcome"),
    };
    let execution_outcome = handle.execute_prepared_continuation_outcome(prepared);
    match execution_outcome {
        ForgeQueryOrdinaryOutcome::Bound(_) => {}
        ForgeQueryOrdinaryOutcome::WrongWorld(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture) => {
            panic!(
                "expected successful execution outcome, got topology kind {:?}",
                posture.checked_topology().continuation_kind()
            );
        }
        _ => panic!("unexpected continuation ordinary outcome"),
    }

    let wrong_world_outcome = admitted_handle("right")
        .prepare_continuation_from_target_outcome(target_request(&handle, "face-a"));
    match wrong_world_outcome {
        ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            assert_eq!(
                posture.checked_topology().continuation_kind(),
                Some(ForgeQueryOrdinaryContinuationCheckedTopologyKind::WrongWorld)
            );
        }
        _ => panic!("expected wrong-world ordinary outcome"),
    }
}
