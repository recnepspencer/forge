use forge_query::facade::{
    discover_basis_lifecycle_support, BasisFamily, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, QuerySubscriptionFamily, QuerySubscriptionSupportPosture,
    ResultShapeFamily, ViewShapeDescriptor,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};
use worth_ui_host_contract::{
    UiHostObservation, UiHostObservationValue, UiMeasurementEvidenceFamily, UiMeasurementRequest,
    UiMeasurementRequestIdentity, UiPortalAnchorRectObservation, UiPortalAnchorRectRequest,
    UiScrollContainerViewportObservation, UiScrollContainerViewportRequest,
    UiTextIntrinsicSizeObservation, UiTextIntrinsicSizeRequest, WorthUiHostCapability,
    WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::capability::{
    QueryBasisPostureReference, QueryDenialPresentation, QueryLiveCompatibility,
    QueryResultShapeReference, QueryViewCapabilityReference, ViewBindingDescriptor,
    ViewBindingFamily, ViewBindingId,
};
use crate::declaration::{
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementConstraintModifier,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementPolicyPosture,
};
use crate::evidence::{
    admit_measurement_basis, MeasurementEvidenceInput, UiMeasurementCoordinateSpace,
    UiMeasurementResult, UiMeasurementRoundingPosture, UiMeasurementUnitPosture,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::graph::{
    UiGraphAxisParticipation, UiGraphGeneration, UiGraphNode, UiGraphNodeIdentity,
    UiGraphParticipationAxis, UiGraphParticipationStatus, UiGraphSnapshot, UiGraphWorldProfile,
};
use crate::host::UiHostMeasurementAssumptionProfile;
use crate::source::{
    WorthUiArtifact, WorthUiArtifactInputResolver, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIdentitySeedLowerer,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiRustAuthoredToArtifactInputLowerer, WorthUiStructuralLegalityLowerer,
};

pub(super) fn measurement_policy(
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    bounded: bool,
) -> UiDeclaredMeasurementPolicyPosture {
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        bounded.then_some(UiDeclaredMeasurementConstraintModifier::Bounded),
        basis_source,
        None,
        vec![],
    )
    .expect("suite measurement policy should admit")
}

pub(super) fn container_basis(
    app: &WorthUiApp,
    root: UiGraphNodeIdentity,
    generation: UiEvidenceAuthorityGeneration,
    bounded: bool,
) -> crate::evidence::UiMeasurementBasis {
    let report = capability_report(77);
    let mut inputs = vec![MeasurementEvidenceInput::host_capability_report(&report)];
    if !bounded {
        inputs.clear();
    }
    admit_measurement_basis(
        declaration_identity_for(app, 0),
        root,
        app.graph_snapshot().world_profile().clone(),
        generation,
        &measurement_policy(None, bounded),
        &inputs,
    )
}

pub(super) fn intrinsic_basis(
    app: &WorthUiApp,
    root: UiGraphNodeIdentity,
    nodes: usize,
    generation: UiEvidenceAuthorityGeneration,
    bounded: bool,
) -> crate::evidence::UiMeasurementBasis {
    let report = capability_report(81);
    let mut inputs = vec![MeasurementEvidenceInput::host_capability_report(&report)];
    for index in 1..nodes {
        inputs.push(MeasurementEvidenceInput::child_host_measurement_result(
            graph_node_identity_for_provenance(app, index),
            &host_text_intrinsic_result(810 + index as u64, &report, generation),
        ));
    }
    admit_measurement_basis(
        declaration_identity_for(app, 0),
        root,
        app.graph_snapshot().world_profile().clone(),
        generation,
        &measurement_policy(None, bounded),
        &inputs,
    )
}

pub(super) fn query_app() -> WorthUiApp {
    let support_report = ForgeQueryApplicationFacade::runtime_backed_default().support_report();
    let query_capability = support_report
        .support_matrix()
        .descriptor(ForgeQueryCapabilityFamily::QueryComposition)
        .expect("suite query capability posture");
    let query_composition = support_report
        .query_composition_support_profile()
        .expect("suite query composition profile");
    let basis_support = discover_basis_lifecycle_support(BasisFamily::CurrentHead, "observation");
    WorthUi::app()
        .register_view_binding(
            ViewBindingDescriptor::query_owned(
                ViewBindingId::new("workspace.view_binding.selection").expect("valid binding id"),
                ViewBindingFamily::collection(),
            )
            .with_query_capability_posture(
                QueryViewCapabilityReference::from_query_capability_descriptor(query_capability),
            )
            .with_query_composition_support(query_composition)
            .with_view_shape(ViewShapeDescriptor::table())
            .with_result_shape(QueryResultShapeReference::from_result_shape_family(
                ResultShapeFamily::Collection,
            ))
            .with_basis_posture(QueryBasisPostureReference::from_basis_support_discovery(
                &basis_support,
            ))
            .with_live_compatibility(QueryLiveCompatibility::from_subscription_posture(
                QuerySubscriptionFamily::CollectionMembership,
                QuerySubscriptionSupportPosture::RuntimeBackedCertified,
            ))
            .with_denial_presentation(QueryDenialPresentation::structured_status()),
        )
        .freeze()
}

pub(super) fn artifact_from_modules<const N: usize>(
    app: &WorthUiApp,
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> WorthUiArtifact {
    let input = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    let snapshot = app.capabilities();
    let resolved =
        WorthUiArtifactInputResolver::resolve(&input, snapshot).expect("suite input resolves");
    let structured =
        WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).expect("suite structure lowers");
    let bound =
        WorthUiBindingSemanticsLowerer::lower(&structured, snapshot).expect("suite semantics lower");
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("suite identity seeds lower")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded).expect("suite artifact assembles")
}

pub(super) fn control_app(
    world_profile: UiGraphWorldProfile,
    operator_token: &str,
    nodes: usize,
    bounded: bool,
) -> WorthUiApp {
    multi_control_app(
        world_profile,
        operator_token,
        nodes,
        bounded,
        "allocation_suite_control.wui",
    )
}

pub(super) fn peer_app(
    world_profile: UiGraphWorldProfile,
    operator_token: &str,
    nodes: usize,
    bounded: bool,
) -> WorthUiApp {
    multi_control_app(
        world_profile,
        operator_token,
        nodes,
        bounded,
        "allocation_suite_peer.wui",
    )
}

fn multi_control_app(
    world_profile: UiGraphWorldProfile,
    operator_token: &str,
    nodes: usize,
    bounded: bool,
    module_path: &str,
) -> WorthUiApp {
    let mut package = WorthUiDslPackage::named("worth-ui.runtime.allocation-planning-suite");
    for index in 0..nodes {
        let mut spec = UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new(format!("planning.suite.node.{index}")),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored(format!("app/{module_path}"), index),
        )
        .with_structural_token(UiDslStructuralToken::new("control:primary"))
        .with_structural_token(UiDslStructuralToken::new("slot:footer"))
        .with_structural_token(UiDslStructuralToken::new(operator_token))
        .with_posture_token(UiDslPostureToken::new("touch:press"));
        if bounded {
            spec = spec.with_posture_token(UiDslPostureToken::new("measurement:constraint:bounded"));
        }
        package = package.with_semantic_artifact_spec(spec);
    }
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(package)
        .freeze()
}

pub(super) fn declaration_identity_for(
    app: &WorthUiApp,
    index: usize,
) -> crate::declaration::UiDeclarationIdentity {
    app.declaration_artifacts()[index].identity().clone()
}

pub(super) fn graph_node_identity_for_provenance(
    app: &WorthUiApp,
    declaration_index: usize,
) -> UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| artifact.provenance().source_provenance().declaration_index() == declaration_index)
        .expect("suite declaration artifact should exist");
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("suite declaration should project one node")
}

pub(super) fn snapshot_with_admitted_layout(
    app: &WorthUiApp,
    admitted_nodes: &[UiGraphNodeIdentity],
) -> UiGraphSnapshot {
    let snapshot = app.graph_snapshot();
    let nodes = snapshot
        .nodes()
        .iter()
        .map(|node| {
            let posture = if admitted_nodes.contains(&node.graph_node_identity()) {
                node.participation_posture().with_axis(
                    UiGraphParticipationAxis::Layout,
                    UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Admitted),
                )
            } else {
                node.participation_posture()
            };
            UiGraphNode::new(
                node.graph_node_identity(),
                node.declaration_identity().clone(),
                node.structural_digest(),
                node.structural_role(),
                node.operator_kind(),
                node.repetition_posture(),
                node.measurement_constraint_modifier(),
                node.authored_provenance_digest(),
                node.repeated_instance_basis().clone(),
                node.attachment_posture(),
                posture,
            )
        })
        .collect::<Vec<_>>();
    UiGraphSnapshot::new(
        UiGraphGeneration::successor_of(snapshot.generation()),
        snapshot.world_profile().clone(),
        nodes,
        snapshot.topology().clone(),
        snapshot.mounted_receipts().clone(),
        snapshot.core_indexes().clone(),
    )
}

pub(super) fn capability_report(generation: u64) -> WorthUiHostCapabilityReport {
    WorthUiHostCapabilityReport::available(vec![
        WorthUiHostCapability::TextIntrinsicMeasurement,
        WorthUiHostCapability::ScrollContainerObservation,
        WorthUiHostCapability::PortalAnchorObservation,
    ])
    .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(generation))
}

pub(super) fn host_text_intrinsic_result(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    let request = UiMeasurementRequest::text_intrinsic_size(
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::TextIntrinsicSize,
        UiTextIntrinsicSizeRequest::single_line(
            "Inbox",
            worth_ui_host_contract::UiFontMeasurementKey::new("body-md"),
        ),
        report,
    )
    .expect("suite text request should admit");
    measurement_result_from_request(
        &request,
        UiHostObservationValue::TextIntrinsicSize(UiTextIntrinsicSizeObservation {
            width: 240.0,
            height: 48.0,
        }),
        generation,
        report,
    )
}

pub(super) fn host_scroll_viewport_result(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    let request = UiMeasurementRequest::scroll_container_viewport(
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::ScrollContainerViewport,
        UiScrollContainerViewportRequest::new(55),
        report,
    )
    .expect("suite scroll request should admit");
    measurement_result_from_request(
        &request,
        UiHostObservationValue::ScrollContainerViewport(UiScrollContainerViewportObservation {
            width: 120.0,
            height: 60.0,
        }),
        generation,
        report,
    )
}

pub(super) fn host_portal_anchor_result(
    request_seed: u64,
    report: &WorthUiHostCapabilityReport,
    generation: UiEvidenceAuthorityGeneration,
) -> UiMeasurementResult {
    let request = UiMeasurementRequest::portal_anchor_rect(
        UiMeasurementRequestIdentity::new(request_seed),
        UiMeasurementEvidenceFamily::PortalAnchorRect,
        UiPortalAnchorRectRequest::new(66),
        report,
    )
    .expect("suite portal request should admit");
    measurement_result_from_request(
        &request,
        UiHostObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
            x: 12.0,
            y: 24.0,
            width: 120.0,
            height: 32.0,
        }),
        generation,
        report,
    )
}

fn measurement_result_from_request(
    request: &UiMeasurementRequest,
    value: UiHostObservationValue,
    generation: UiEvidenceAuthorityGeneration,
    report: &WorthUiHostCapabilityReport,
) -> UiMeasurementResult {
    UiMeasurementResult::from_host_observation(
        UiHostObservation::from_request(request, value)
            .expect("suite host observation should align with request"),
        generation,
        UiMeasurementUnitPosture::LogicalPx,
        UiMeasurementCoordinateSpace::GraphNodeLocal,
        UiMeasurementRoundingPosture::ExactFloat,
        UiHostMeasurementAssumptionProfile::from_capability_report(report, 11, 22, 33, 44),
    )
}
