use std::marker::PhantomData;

use crate::application::{
    WorthQueryApplicationFacade, WorthQueryBridgeContinuationAuthority, WorthQueryCapabilityFamily,
    WorthQueryConfig, WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRouteContract, WorthQueryDeclarationSignalCompatibilityContract,
    WorthQueryDeclarationSignalCompatibilityInput, WorthQueryDomainOperatingContext,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryQueryConfig, WorthQueryRelationalConfig,
    WorthQuerySignalDeferredPosture, WorthQuerySignalNotCompatiblePosture,
};
use crate::ordinary_outcome::{
    WorthQueryOrdinaryOutcome,
    WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
};

use super::support::{orchestration_input, SignalDomain, SignalFamily, SignalWorld};

fn proof_handle(
    world: &'static str,
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<SignalDomain, SignalWorld> {
    proof_signal_context(SignalWorld(world))
}

fn proof_signal_context<C>(
    context: C,
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<SignalDomain, C>
where
    C: WorthQueryDomainOperatingContext<SignalDomain>,
{
    crate::application::domain_test_support::installed_declaration_context(
        SignalDomain,
        context,
        [
            crate::application::domain_test_support::family::<SignalDomain, SignalFamily>(),
            crate::application::domain_test_support::family::<SignalDomain, DeferredSignalFamily>(),
            crate::application::domain_test_support::family::<
                SignalDomain,
                HistoricalBasisSignalFamily,
            >(),
            crate::application::domain_test_support::family::<SignalDomain, UnsupportedSignalFamily>(
            ),
        ],
    )
}

#[test]
fn proof_stops_at_compatibility_when_no_bridge_request_is_supplied() {
    let handle = proof_handle("main");
    let proof =
        handle.orchestrate_signal_compatibility_proof(orchestration_input(&handle, "face-a"));

    assert_eq!(
        proof.request().request_kind(),
        "signal_compatibility_orchestration"
    );
    assert_eq!(proof.witness_checks().len(), 1);
    assert!(proof.witness_checks()[0].did_pass());
    assert_eq!(proof.narrowing_decisions().len(), 1);
    assert!(proof.linked_artifacts().declaration_digest().is_some());
    assert!(proof.linked_artifacts().route_plan_digest().is_some());
    assert!(proof.linked_artifacts().receipt_digest().is_some());
    assert!(proof.linked_artifacts().envelope_digest().is_some());
}

#[test]
fn wrong_world_is_preserved_through_signal_orchestration() {
    let left = proof_handle("left");
    let right = proof_handle("right");
    let outcome = right.orchestrate_signal_compatibility(orchestration_input(&left, "face-a"));
    assert!(matches!(
        outcome,
        crate::signal_compatibility_orchestration::WorthQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(_)
    ));
}

#[test]
fn ordinary_outcome_keeps_signal_checked_topology_visible() {
    let left = proof_handle("left");
    let right = proof_handle("right");
    match right.orchestrate_signal_compatibility_outcome(orchestration_input(&left, "face-a")) {
        WorthQueryOrdinaryOutcome::WrongWorld(posture) => {
            assert_eq!(
                posture
                    .checked_topology()
                    .signal_compatibility_orchestration_kind(),
                Some(
                    WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::WrongWorld
                )
            );
            assert!(posture
                .checked_topology()
                .signal_compatibility_orchestration_linked_artifacts()
                .is_some());
        }
        _ => panic!("expected wrong-world ordinary outcome"),
    }
}

#[test]
fn ordinary_outcome_keeps_deferred_distinct() {
    let handle = proof_handle("main");

    match handle.orchestrate_signal_compatibility_outcome(local_input::<DeferredSignalInput>(
        &handle,
        "face-deferred",
    )) {
        WorthQueryOrdinaryOutcome::Deferred(_) => {}
        _ => panic!("expected deferred ordinary outcome"),
    }
}

#[test]
fn ordinary_outcome_keeps_unsupported_distinct() {
    let handle = proof_handle("main");

    match handle.orchestrate_signal_compatibility_outcome(local_input::<UnsupportedSignalInput>(
        &handle,
        "face-unsupported",
    )) {
        WorthQueryOrdinaryOutcome::Unsupported(_) => {}
        _ => panic!("expected unsupported ordinary outcome"),
    }
}

#[test]
fn ordinary_outcome_keeps_basis_mismatch_distinct() {
    let handle = admitted_basis_mismatch_handle();

    match handle
        .orchestrate_signal_compatibility_outcome(basis_mismatch_input(&handle, "face-basis"))
    {
        WorthQueryOrdinaryOutcome::BasisMismatch(_) => {}
        _ => panic!("expected basis-mismatch ordinary outcome"),
    }
}

#[test]
fn ordinary_outcome_keeps_wrong_handle_distinct() {
    let left = proof_handle("shared");
    let right = admitted_same_world_different_handle();

    match right.orchestrate_signal_compatibility_outcome(orchestration_input(&left, "face-handle"))
    {
        WorthQueryOrdinaryOutcome::WrongHandle(posture) => {
            assert_eq!(
                posture
                    .checked_topology()
                    .signal_compatibility_orchestration_kind(),
                Some(
                    WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::WrongHandle
                )
            );
        }
        _ => panic!("expected wrong-handle ordinary outcome"),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BasisMismatchWorld;

impl WorthQueryDomainOperatingContext<SignalDomain> for BasisMismatchWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
            WorthQueryConfigSectionFamily::RuntimeBridge,
            WorthQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "signal-world-basis-mismatch".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SharedIdentityDifferentHandleWorld;

impl WorthQueryDomainOperatingContext<SignalDomain> for SharedIdentityDifferentHandleWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::HistoricalEvaluation,
            WorthQueryCapabilityFamily::WorkflowOrchestration,
            WorthQueryCapabilityFamily::PreviewSession,
            WorthQueryCapabilityFamily::QueryComposition,
            WorthQueryCapabilityFamily::QueryRead,
        ]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
            WorthQueryConfigSectionFamily::RuntimeBridge,
            WorthQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        SignalWorld("shared").context_identity_digest()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DeferredSignalFamily;

impl WorthQueryDeclarationFamilyMarker<SignalDomain> for DeferredSignalFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalDeferredPosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "DeferredSignalFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        SignalFamilyShape::aspect_contract()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UnsupportedSignalFamily;

impl WorthQueryDeclarationFamilyMarker<SignalDomain> for UnsupportedSignalFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalNotCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "UnsupportedSignalFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        SignalFamilyShape::aspect_contract()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DeferredSignalInput {
    id: &'static str,
    _marker: PhantomData<DeferredSignalFamily>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct UnsupportedSignalInput {
    id: &'static str,
    _marker: PhantomData<UnsupportedSignalFamily>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HistoricalBasisSignalFamily;

impl WorthQueryDeclarationFamilyMarker<SignalDomain> for HistoricalBasisSignalFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "HistoricalBasisSignalFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        SignalFamilyShape::aspect_contract()
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        Some(WorthQueryDeclarationSignalCompatibilityContract::historical_derived_execution())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HistoricalBasisSignalInput {
    id: &'static str,
    _marker: PhantomData<HistoricalBasisSignalFamily>,
}

impl DeferredSignalInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl UnsupportedSignalInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl HistoricalBasisSignalInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl WorthQueryDeclarationInput<SignalDomain> for DeferredSignalInput {
    type Family = DeferredSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

impl WorthQueryDeclarationInput<SignalDomain> for UnsupportedSignalInput {
    type Family = UnsupportedSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

impl WorthQueryDeclarationInput<SignalDomain> for HistoricalBasisSignalInput {
    type Family = HistoricalBasisSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

fn admitted_basis_mismatch_handle(
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<SignalDomain, BasisMismatchWorld>
{
    proof_signal_context(BasisMismatchWorld)
}

fn admitted_same_world_different_handle(
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    SignalDomain,
    SharedIdentityDifferentHandleWorld,
> {
    proof_signal_context(SharedIdentityDifferentHandleWorld)
}

fn local_input<I: WorthQueryDeclarationInput<SignalDomain>>(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<
        SignalDomain,
        SignalWorld,
    >,
    id: &'static str,
) -> crate::signal_compatibility_orchestration::WorthQuerySignalCompatibilityOrchestrationInput<
    SignalDomain,
    I,
>
where
    I: LocalSignalInput,
{
    let envelope = handle
        .declare_review_progress_describe_plan_receipt_and_envelope(I::new_local(id))
        .unwrap_or_else(|_| panic!("expected envelope-backed local signal input"));
    crate::signal_compatibility_orchestration::WorthQuerySignalCompatibilityOrchestrationInput::new(
        WorthQueryDeclarationSignalCompatibilityInput::enveloped(envelope),
    )
}

trait LocalSignalInput: Sized {
    fn new_local(id: &'static str) -> Self;
}

impl LocalSignalInput for DeferredSignalInput {
    fn new_local(id: &'static str) -> Self {
        Self::new(id)
    }
}

impl LocalSignalInput for UnsupportedSignalInput {
    fn new_local(id: &'static str) -> Self {
        Self::new(id)
    }
}

fn basis_mismatch_input(
    handle: &crate::application::WorthQueryInstalledDomainDeclarationContext<
        SignalDomain,
        BasisMismatchWorld,
    >,
    id: &'static str,
) -> crate::signal_compatibility_orchestration::WorthQuerySignalCompatibilityOrchestrationInput<
    SignalDomain,
    HistoricalBasisSignalInput,
> {
    let envelope = handle
        .declare_review_progress_describe_plan_receipt_and_envelope(
            HistoricalBasisSignalInput::new(id),
        )
        .unwrap_or_else(|_| panic!("expected envelope-backed basis-mismatch input"));
    crate::signal_compatibility_orchestration::WorthQuerySignalCompatibilityOrchestrationInput::new(
        WorthQueryDeclarationSignalCompatibilityInput::enveloped(envelope),
    )
}

struct SignalFamilyShape;

impl SignalFamilyShape {
    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face", "signal.dependency.runtime_inputs"],
            &["selection.neighborhood.local_topology"],
            &["signal.preview.surface"],
            &["signal.private_authority"],
            &["signal.conflicting_dependency"],
        )
    }
}
