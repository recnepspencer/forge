use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::application::{
    ForgeQueryBridgeContinuationAuthority, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryGraphObligationOrchestrationBoundary,
    ForgeQueryGraphObligationOrchestrationDispatchError, ForgeQueryNeighborhoodCapableGrouping,
};
use crate::family_helpers::ForgeQueryGeometryNeighborhoodHelperFamily;
use crate::grouped_authoring::{
    forge_query_grouped_declaration_checked_on_handle, ForgeQueryGroupedDeclarationChecked,
};
use crate::runtime::{
    ForgeQueryAuthoritativeMutationObligationDispatch, ForgeQueryGraphObligationDispatchContext,
    ForgeQueryGraphObligationDispatchContextKind, ForgeQueryGraphObligationIndex,
    ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryGraphObligationRuleIdentity,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchReadVerb, ForgeQueryGraphTouchSelector,
};

use super::support::{admitted_handle, GeometryDomain};

static BLOCKED_CANONICALIZATION_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BlockingGroupedFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for BlockingGroupedFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "BlockingGroupedFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
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
        vec![phase_seven_registration(
            ForgeQueryGraphObligationSupportLane::DeclarationEntry,
        )]
    }
}

impl ForgeQueryGeometryNeighborhoodHelperFamily<GeometryDomain> for BlockingGroupedFamily {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BlockingGroupedInput {
    id: &'static str,
    _marker: PhantomData<BlockingGroupedFamily>,
}

impl BlockingGroupedInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for BlockingGroupedInput {
    type Family = BlockingGroupedFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CountingBlockingGroupedInput {
    id: &'static str,
    _marker: PhantomData<BlockingGroupedFamily>,
}

impl CountingBlockingGroupedInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for CountingBlockingGroupedInput {
    type Family = BlockingGroupedFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        BLOCKED_CANONICALIZATION_COUNT.fetch_add(1, Ordering::SeqCst);
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MissingTouchGroupedFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for MissingTouchGroupedFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = crate::application::ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MissingTouchGroupedFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        BlockingGroupedFamily::aspect_contract()
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn orchestration_graph_obligation_registrations() -> Vec<ForgeQueryGraphObligationRegistration>
    {
        vec![phase_seven_registration(
            ForgeQueryGraphObligationSupportLane::DeclarationEntry,
        )]
    }
}

impl ForgeQueryGeometryNeighborhoodHelperFamily<GeometryDomain> for MissingTouchGroupedFamily {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MissingTouchGroupedInput {
    id: &'static str,
    _marker: PhantomData<MissingTouchGroupedFamily>,
}

impl MissingTouchGroupedInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for MissingTouchGroupedInput {
    type Family = MissingTouchGroupedFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

#[test]
fn declaration_entry_dispatch_denies_group_before_group_artifact_materializes() {
    let handle = admitted_handle("phase-seven");
    BLOCKED_CANONICALIZATION_COUNT.store(0, Ordering::SeqCst);
    let result = forge_query_grouped_declaration_checked_on_handle(
        &handle,
        handle
            .geometry_helpers()
            .local_neighborhood_for_active_face_selection(CountingBlockingGroupedInput::new(
                "face-a",
            ))
            .with_member(CountingBlockingGroupedInput::new("face-b")),
    );

    let stop = match result {
        ForgeQueryGroupedDeclarationChecked::MemberStopped(stop) => stop,
        ForgeQueryGroupedDeclarationChecked::Bound(_) => {
            panic!("declaration-entry graph obligation should deny before group binding")
        }
    };
    assert_eq!(
        BLOCKED_CANONICALIZATION_COUNT.load(Ordering::SeqCst),
        0,
        "declaration-entry graph obligation denial must happen before member canonicalization",
    );
    assert_eq!(stop.declaration_family_key(), "BlockingGroupedFamily");
    let dispatch = stop
        .graph_obligation_dispatch()
        .expect("member stop should retain graph obligation dispatch evidence");
    assert_eq!(
        dispatch.boundary(),
        ForgeQueryGraphObligationOrchestrationBoundary::DeclarationEntry
    );
    assert_eq!(
        dispatch.operating_context_identity_digest(),
        handle.operating_context_identity_digest()
    );
    let projection = dispatch.evidence_projection();
    assert_eq!(
        projection.context_kind(),
        Some(ForgeQueryGraphObligationDispatchContextKind::DeclarationEntry)
    );
    assert!(dispatch.envelope_digest().is_some());
    assert_eq!(
        projection.rows()[0].support_lane(),
        ForgeQueryGraphObligationSupportLane::DeclarationEntry
    );
}

#[test]
fn declaration_entry_dispatch_failure_keeps_typed_error() {
    let handle = admitted_handle("phase-seven");
    let result = forge_query_grouped_declaration_checked_on_handle(
        &handle,
        handle
            .geometry_helpers()
            .local_neighborhood_for_active_face_selection(MissingTouchGroupedInput::new("face-a")),
    );

    let stop = match result {
        ForgeQueryGroupedDeclarationChecked::MemberStopped(stop) => stop,
        ForgeQueryGroupedDeclarationChecked::Bound(_) => {
            panic!("missing touch collection should fail grouped declaration dispatch")
        }
    };
    assert!(matches!(
        stop.graph_obligation_dispatch_error(),
        Some(
            ForgeQueryGraphObligationOrchestrationDispatchError::MissingTouchCollection {
                boundary: ForgeQueryGraphObligationOrchestrationBoundary::DeclarationEntry,
            }
        )
    ));
    assert!(stop.graph_obligation_dispatch().is_none());
}

#[test]
fn orchestration_and_execution_dispatch_share_rule_identity_for_same_rule() {
    let handle = admitted_handle("phase-seven");
    let result = forge_query_grouped_declaration_checked_on_handle(
        &handle,
        handle
            .geometry_helpers()
            .local_neighborhood_for_active_face_selection(BlockingGroupedInput::new("face-a")),
    );
    let orchestration = match result {
        ForgeQueryGroupedDeclarationChecked::MemberStopped(stop) => stop
            .graph_obligation_dispatch()
            .expect("orchestration stop should retain dispatch")
            .clone(),
        ForgeQueryGroupedDeclarationChecked::Bound(_) => {
            panic!("orchestration dispatch should deny before declaration binds")
        }
    };
    let execution_registration =
        phase_seven_registration(ForgeQueryGraphObligationSupportLane::GraphComposition);
    let touch_descriptor = ForgeQueryGraphTouchDescriptor::read_family(
        "faces",
        [ForgeQueryGraphTouchReadVerb::ExposesDerivedTopology],
    )
    .unwrap();
    let operating_world =
        ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle();
    let catalog = ForgeQueryGraphObligationRegistrationCatalog::from_registrations(vec![
        execution_registration,
    ])
    .unwrap();
    let selection = ForgeQueryGraphObligationIndex::from_catalog(&catalog)
        .select_for_touch(&touch_descriptor, &operating_world);
    let execution = ForgeQueryAuthoritativeMutationObligationDispatch::from_selection(
        ForgeQueryGraphObligationDispatchContext::graph_composition(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
        )
        .unwrap(),
        selection,
    )
    .unwrap();
    let orchestration_row = orchestration.evidence_projection().rows()[0].clone();
    let execution_row = execution.evidence_projection().rows()[0].clone();

    assert_eq!(
        orchestration_row.rule_identity_digest(),
        execution_row.rule_identity_digest(),
        "the same graph rule must stay one identity across orchestration and execution dispatch",
    );
    assert_eq!(orchestration_row.rule_name(), execution_row.rule_name());
}

fn phase_seven_registration(
    lane: ForgeQueryGraphObligationSupportLane,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new(
            "forge-query.phase-seven",
            "phase-seven-blocking",
            "v1",
        )
        .unwrap(),
        ForgeQueryGraphTouchSelector::any_graph_touch(),
        ForgeQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::unsupported(lane))
}
