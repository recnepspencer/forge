use std::marker::PhantomData;

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfig, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationSignalCompatibilityContract,
    ForgeQueryDeclarationSignalCompatibilityInput, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryQueryConfig, ForgeQueryRelationalConfig,
    ForgeQuerySignalDeferredPosture, ForgeQuerySignalNotCompatiblePosture,
};
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
};

use super::support::{admitted_handle, orchestration_input, SignalDomain, SignalWorld};

#[test]
fn proof_stops_at_compatibility_when_no_bridge_request_is_supplied() {
    let handle = admitted_handle("main");
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
    let left = admitted_handle("left");
    let right = admitted_handle("right");
    let outcome = right.orchestrate_signal_compatibility(orchestration_input(&left, "face-a"));
    assert!(matches!(
        outcome,
        crate::signal_compatibility_orchestration::ForgeQuerySignalCompatibilityOrchestrationOutcome::WrongWorld(_)
    ));
}

#[test]
fn ordinary_outcome_keeps_signal_checked_topology_visible() {
    let left = admitted_handle("left");
    let right = admitted_handle("right");
    match right.orchestrate_signal_compatibility_outcome(orchestration_input(&left, "face-a")) {
        ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            assert_eq!(
                posture
                    .checked_topology()
                    .signal_compatibility_orchestration_kind(),
                Some(
                    ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::WrongWorld
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
    let handle = admitted_handle("main");

    match handle.orchestrate_signal_compatibility_outcome(local_input::<DeferredSignalInput>(
        &handle,
        "face-deferred",
    )) {
        ForgeQueryOrdinaryOutcome::Deferred(_) => {}
        _ => panic!("expected deferred ordinary outcome"),
    }
}

#[test]
fn ordinary_outcome_keeps_unsupported_distinct() {
    let handle = admitted_handle("main");

    match handle.orchestrate_signal_compatibility_outcome(local_input::<UnsupportedSignalInput>(
        &handle,
        "face-unsupported",
    )) {
        ForgeQueryOrdinaryOutcome::Unsupported(_) => {}
        _ => panic!("expected unsupported ordinary outcome"),
    }
}

#[test]
fn ordinary_outcome_keeps_basis_mismatch_distinct() {
    let handle = admitted_basis_mismatch_handle();

    match handle
        .orchestrate_signal_compatibility_outcome(basis_mismatch_input(&handle, "face-basis"))
    {
        ForgeQueryOrdinaryOutcome::BasisMismatch(_) => {}
        _ => panic!("expected basis-mismatch ordinary outcome"),
    }
}

#[test]
fn ordinary_outcome_keeps_wrong_handle_distinct() {
    let left = admitted_handle("shared");
    let right = admitted_same_world_different_handle();

    match right.orchestrate_signal_compatibility_outcome(orchestration_input(&left, "face-handle"))
    {
        ForgeQueryOrdinaryOutcome::WrongHandle(posture) => {
            assert_eq!(
                posture
                    .checked_topology()
                    .signal_compatibility_orchestration_kind(),
                Some(
                    ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::WrongHandle
                )
            );
        }
        _ => panic!("expected wrong-handle ordinary outcome"),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BasisMismatchWorld;

impl ForgeQueryDomainOperatingContext<SignalDomain> for BasisMismatchWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
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
        "signal-world-basis-mismatch".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SharedIdentityDifferentHandleWorld;

impl ForgeQueryDomainOperatingContext<SignalDomain> for SharedIdentityDifferentHandleWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryRead,
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
        SignalWorld("shared").context_identity_digest()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DeferredSignalFamily;

impl ForgeQueryDeclarationFamilyMarker<SignalDomain> for DeferredSignalFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalDeferredPosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "DeferredSignalFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        SignalFamilyShape::aspect_contract()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UnsupportedSignalFamily;

impl ForgeQueryDeclarationFamilyMarker<SignalDomain> for UnsupportedSignalFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "UnsupportedSignalFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        SignalFamilyShape::aspect_contract()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
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

impl ForgeQueryDeclarationFamilyMarker<SignalDomain> for HistoricalBasisSignalFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "HistoricalBasisSignalFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        SignalFamilyShape::aspect_contract()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(ForgeQueryDeclarationSignalCompatibilityContract::historical_derived_execution())
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

impl ForgeQueryDeclarationInput<SignalDomain> for DeferredSignalInput {
    type Family = DeferredSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

impl ForgeQueryDeclarationInput<SignalDomain> for UnsupportedSignalInput {
    type Family = UnsupportedSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

impl ForgeQueryDeclarationInput<SignalDomain> for HistoricalBasisSignalInput {
    type Family = HistoricalBasisSignalFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

fn admitted_basis_mismatch_handle(
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<SignalDomain, BasisMismatchWorld>
{
    ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default().with_relational(
            ForgeQueryRelationalConfig::enabled().with_historical_evaluation(false),
        ),
    )
    .unwrap()
    .domain(SignalDomain)
    .with_operating_context(BasisMismatchWorld)
    .validate()
    .unwrap()
    .admit()
    .unwrap()
}

fn admitted_same_world_different_handle(
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
    SignalDomain,
    SharedIdentityDifferentHandleWorld,
> {
    ForgeQueryApplicationFacade::new(
        ForgeQueryConfig::runtime_backed_default().with_query(ForgeQueryQueryConfig::enabled()),
    )
    .unwrap()
    .domain(SignalDomain)
    .with_operating_context(SharedIdentityDifferentHandleWorld)
    .validate()
    .unwrap()
    .admit()
    .unwrap()
}

fn local_input<I: ForgeQueryDeclarationInput<SignalDomain>>(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        SignalDomain,
        SignalWorld,
    >,
    id: &'static str,
) -> crate::signal_compatibility_orchestration::ForgeQuerySignalCompatibilityOrchestrationInput<
    SignalDomain,
    I,
>
where
    I: LocalSignalInput,
{
    let envelope = handle
        .declare_review_progress_describe_plan_receipt_and_envelope(I::new_local(id))
        .unwrap_or_else(|_| panic!("expected envelope-backed local signal input"));
    crate::signal_compatibility_orchestration::ForgeQuerySignalCompatibilityOrchestrationInput::new(
        ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope),
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
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        SignalDomain,
        BasisMismatchWorld,
    >,
    id: &'static str,
) -> crate::signal_compatibility_orchestration::ForgeQuerySignalCompatibilityOrchestrationInput<
    SignalDomain,
    HistoricalBasisSignalInput,
> {
    let envelope = handle
        .declare_review_progress_describe_plan_receipt_and_envelope(
            HistoricalBasisSignalInput::new(id),
        )
        .unwrap_or_else(|_| panic!("expected envelope-backed basis-mismatch input"));
    crate::signal_compatibility_orchestration::ForgeQuerySignalCompatibilityOrchestrationInput::new(
        ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope),
    )
}

struct SignalFamilyShape;

impl SignalFamilyShape {
    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face", "signal.dependency.runtime_inputs"],
            &["selection.neighborhood.local_topology"],
            &["signal.preview.surface"],
            &["signal.private_authority"],
            &["signal.conflicting_dependency"],
        )
    }
}
