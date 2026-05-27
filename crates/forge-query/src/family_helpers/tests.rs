use std::marker::PhantomData;

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationSignalCompatibilityContract,
    ForgeQueryDeclarationSignalExecutionFamily, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQuerySignalCompatiblePosture,
};
use crate::domain_capabilities::{
    ForgeQuerySupportContributionAuthoring, ForgeQueryWorkflowContributionAuthoring,
};
use crate::family_helpers::{
    ForgeQueryGeometryActiveFaceSelectionHelperFamily,
    ForgeQueryGeometryMaterialAttachmentHelperFamily, ForgeQueryGeometryMaterialAttachmentInput,
};
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryContributionComposedCheckedTopologyKind, ForgeQueryOrdinaryOutcome,
};
use crate::signal_compatibility_orchestration::{
    ForgeQuerySignalCompatibilityOrchestration, ForgeQuerySignalCompatibilityOrchestrationInput,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.family_helpers.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryFamilyHelpersDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryWorld(&'static str);

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
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
        format!("family-helpers-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for GeometryFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "GeometryFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &["selection.material_preview"],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_and_bridge()
    }

    fn bridge_continuation_contract(
    ) -> Option<crate::application::ForgeQueryDeclarationBridgeContinuationContract> {
        Some(crate::application::ForgeQueryDeclarationBridgeContinuationContract::preview_session())
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution())
    }
}

impl ForgeQueryGeometryActiveFaceSelectionHelperFamily<GeometryDomain> for GeometryFamily {}

impl ForgeQueryGeometryMaterialAttachmentHelperFamily<GeometryDomain> for GeometryFamily {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct GeometryInput {
    id: &'static str,
    _marker: PhantomData<GeometryFamily>,
}

impl GeometryInput {
    fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for GeometryInput {
    type Family = GeometryFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
    }
}

fn admitted_handle(
    world: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld(world))
        .validate()
        .unwrap()
        .admit()
        .unwrap()
}

fn preview_request() -> crate::application::ForgeQueryDeclarationBridgeContinuationRequest {
    crate::application::ForgeQueryDeclarationBridgeContinuationRequest::new(
        crate::application::ForgeQueryDeclarationBridgeContinuationMode::PreviewSession,
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Preview,
    )
}

fn runtime_route_request() -> crate::application::ForgeQueryDeclarationBridgeContinuationRequest {
    crate::application::ForgeQueryDeclarationBridgeContinuationRequest::new(
        crate::application::ForgeQueryDeclarationBridgeContinuationMode::RuntimeRoute,
        crate::application::ForgeQueryDeclarationBridgeTruthContext::Current,
    )
}

fn progressed(
    handle: &crate::application::ForgeQueryAdmittedConfiguredDomainHandle<
        GeometryDomain,
        GeometryWorld,
    >,
    id: &'static str,
) -> crate::application::ForgeQueryAdmittedDeclarationProgression<GeometryDomain, GeometryInput> {
    match handle.declare_review_and_progress(GeometryInput::new(id)) {
        Ok(value) => value,
        Err(_) => panic!("expected progressed geometry selection"),
    }
}

#[test]
fn preview_helper_matches_generic_signal_orchestration_path() {
    let handle = admitted_handle("main");
    let helper_progressed = match handle
        .geometry_helpers()
        .progress_active_face_selection(GeometryInput::new("face-a"))
    {
        Ok(value) => value,
        Err(_) => panic!("expected helper progression to succeed"),
    };
    let generic_progressed = progressed(&handle, "face-a");

    let helper = handle
        .geometry_helpers()
        .prepare_preview_for_active_face_selection_checked(helper_progressed);
    let generic = handle.orchestrate_signal_compatibility_checked(
        ForgeQuerySignalCompatibilityOrchestrationInput::from_progressed(generic_progressed)
            .with_bridge_request(preview_request()),
    );

    assert_eq!(
        helper.orchestration_digest(),
        generic.orchestration_digest()
    );
    assert_eq!(helper.linked_artifacts(), generic.linked_artifacts());
}

#[test]
fn truth_view_helpers_keep_current_and_historical_meaning_distinct() {
    let handle = admitted_handle("main");
    let current_helper = handle
        .geometry_helpers()
        .prepare_current_truth_view_for_active_face_selection_checked(progressed(
            &handle, "face-b",
        ));
    let historical_helper = handle
        .geometry_helpers()
        .prepare_historical_truth_view_for_active_face_selection_checked(progressed(
            &handle, "face-c",
        ));
    let current_generic = handle.orchestrate_signal_compatibility_checked(
        ForgeQuerySignalCompatibilityOrchestrationInput::from_progressed(progressed(
            &handle, "face-b",
        ))
        .with_bridge_request(
            crate::application::ForgeQueryDeclarationBridgeContinuationRequest::new(
                crate::application::ForgeQueryDeclarationBridgeContinuationMode::TruthView,
                crate::application::ForgeQueryDeclarationBridgeTruthContext::Current,
            ),
        ),
    );
    let historical_generic = handle.orchestrate_signal_compatibility_checked(
        ForgeQuerySignalCompatibilityOrchestrationInput::from_progressed(progressed(
            &handle, "face-c",
        ))
        .with_bridge_request(
            crate::application::ForgeQueryDeclarationBridgeContinuationRequest::new(
                crate::application::ForgeQueryDeclarationBridgeContinuationMode::TruthView,
                crate::application::ForgeQueryDeclarationBridgeTruthContext::Historical,
            ),
        ),
    );

    assert_eq!(
        current_helper.orchestration_digest(),
        current_generic.orchestration_digest()
    );
    assert_eq!(
        historical_helper.orchestration_digest(),
        historical_generic.orchestration_digest()
    );
    assert_ne!(
        current_helper.orchestration_digest(),
        historical_helper.orchestration_digest()
    );
    if let ForgeQuerySignalCompatibilityOrchestrationOutcome::Bound(
        ForgeQuerySignalCompatibilityOrchestration::Prepared(prepared),
    ) = current_helper.outcome()
    {
        assert_eq!(
            prepared.signal_execution_family(),
            Some(ForgeQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution)
        );
    }
}

#[test]
fn material_attachment_helper_matches_generic_composed_path() {
    let handle = admitted_handle("main");
    let helper_input = ForgeQueryGeometryMaterialAttachmentInput::new(GeometryInput::new("face-d"))
        .with_support_contribution(
            ForgeQuerySupportContributionAuthoring::declaration_traceability(
                "geometry.trace",
                "track selection to material attachment",
            ),
        )
        .with_workflow_contribution(ForgeQueryWorkflowContributionAuthoring::preview_only(
            "geometry.preview",
            "preview material attachment before promotion",
        ));
    let generic_input =
        crate::contribution_composed_orchestration::ForgeQueryContributionComposedOrchestrationInput::new(
            GeometryInput::new("face-d"),
        )
        .with_contribution(crate::contribution_composed_orchestration::ForgeQueryContributionIntent::support(
            ForgeQuerySupportContributionAuthoring::declaration_traceability(
                "geometry.trace",
                "track selection to material attachment",
            ),
        ))
        .with_contribution(crate::contribution_composed_orchestration::ForgeQueryContributionIntent::workflow(
            ForgeQueryWorkflowContributionAuthoring::preview_only(
                "geometry.preview",
                "preview material attachment before promotion",
            ),
        ));

    let helper = handle
        .geometry_helpers()
        .orchestrate_material_attachment_for_active_face_selection_proof(helper_input);
    let generic = handle.orchestrate_declaration_with_contributions_proof(generic_input);

    assert_eq!(helper.request_digest(), generic.request_digest());
    assert_eq!(helper.linked_artifacts(), generic.linked_artifacts());
    assert_eq!(helper.contribution_digest(), generic.contribution_digest());
}

#[test]
fn helper_wrong_world_matches_generic_path() {
    let left = admitted_handle("left");
    let right = admitted_handle("right");
    let helper = right
        .geometry_helpers()
        .prepare_runtime_route_for_active_face_selection_outcome(progressed(&left, "face-e"));
    let generic = right.orchestrate_signal_compatibility_outcome(
        ForgeQuerySignalCompatibilityOrchestrationInput::from_progressed(progressed(
            &left, "face-e",
        ))
        .with_bridge_request(runtime_route_request()),
    );

    match (helper, generic) {
        (
            ForgeQueryOrdinaryOutcome::WrongWorld(left_posture),
            ForgeQueryOrdinaryOutcome::WrongWorld(right_posture),
        ) => {
            assert_eq!(left_posture.reason(), right_posture.reason());
        }
        _ => panic!("expected helper and generic paths to preserve wrong-world posture"),
    }
}

#[test]
fn material_attachment_helper_preserves_contribution_checked_topology() {
    let handle = admitted_handle("main");
    let outcome = handle
        .geometry_helpers()
        .orchestrate_material_attachment_for_active_face_selection_outcome(
            ForgeQueryGeometryMaterialAttachmentInput::new(GeometryInput::new("face-f"))
                .with_support_contribution(
                    ForgeQuerySupportContributionAuthoring::declaration_traceability(
                        "geometry.traceability",
                        "prove material attachment request lineage",
                    ),
                ),
        );

    match outcome {
        ForgeQueryOrdinaryOutcome::Bound(_) => {}
        ForgeQueryOrdinaryOutcome::Denied(posture) => {
            panic!(
                "expected bound material-attachment helper, got topology {:?}",
                posture.checked_topology().contribution_composed_kind()
            );
        }
        ForgeQueryOrdinaryOutcome::Unsupported(posture) => {
            assert_ne!(
                posture.checked_topology().contribution_composed_kind(),
                Some(ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::ContributionDenied)
            );
        }
        _ => {}
    }
}
