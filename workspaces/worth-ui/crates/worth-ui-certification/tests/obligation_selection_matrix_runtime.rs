use std::sync::Arc;

use worth_query::facade::certification::admit_runtime_current_snapshot_basis_for_certification;
use worth_query::facade::foundation::{
    snapshot_resolution_report, QueryExternalIdentityToken, QueryExternalSchemaBasisToken,
    WorthQuerySnapshotIdentity,
};
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::declaration::UiDeclarationSupportRowSchemaKind;
use worth_ui::facade::graph::{
    UiGraphAxisParticipation, UiGraphParticipationAxis, UiGraphParticipationStatus,
    UiGraphSessionLabel, UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchOriginClass,
    UiGraphTouchRuntimeLane, UiGraphTouchTargetClass, UiGraphTouchTiming, UiGraphWorldProfile,
};
use worth_ui::facade::obligations::{
    UiObligationCheckKind, UiObligationFamily, UiObligationSelectionReason,
    UiObligationSupportBasis, UiObligationSupportSelectionPosture, UiObligationWorldProfileClass,
    UiSelectedObligation,
};
use worth_ui_dsl::{
    UiDslPostureToken, UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, WorthUiDslPackage,
};

#[test]
fn structural_hot_reload_touch_selects_closed_structural_matrix_with_stable_identity() {
    let app = touch_app(UiGraphWorldProfile::hot_reload_candidate(
        UiGraphSessionLabel::new("worth-ui.phase4.hot-reload")
            .expect("hot-reload label should admit"),
    ));
    let graph = app.graph();
    let artifact = control_artifact(&app);
    let node = graph_node_identity(graph, artifact);
    let origin = graph
        .touches()
        .declaration_change_receipt(artifact)
        .expect("declaration change should admit");
    let touch = graph
        .touches()
        .from_slot_occupancy(
            origin,
            UiGraphTouchTiming::PostMutation,
            node,
            UiGraphTouchAspects::new()
                .structural(UiGraphTouchAspectPosture::Invalidated)
                .participation(UiGraphTouchAspectPosture::Written),
        )
        .expect("slot touch should admit");

    let left = app.admission().select_obligations(&touch);
    let right = app.admission().select_obligations(&touch);

    assert_eq!(left, right);
    assert_eq!(
        left.obligations()
            .iter()
            .map(|obligation| obligation.family())
            .collect::<Vec<_>>(),
        vec![
            UiObligationFamily::StructuralLegality,
            UiObligationFamily::SlotContract,
            UiObligationFamily::ParticipationLegality,
            UiObligationFamily::PortalHostRequirement,
        ]
    );
    assert!(left.obligations().iter().all(|obligation| {
        if obligation.family() == UiObligationFamily::PortalHostRequirement {
            obligation.check_kind() == UiObligationCheckKind::PrerequisiteRequirement
        } else {
            obligation.check_kind() == UiObligationCheckKind::BlockingInvariant
        }
    }));
    assert!(left.obligations().iter().all(|obligation| {
        if obligation.family() == UiObligationFamily::PortalHostRequirement {
            obligation.identity().support_basis() == UiObligationSupportBasis::ServiceUsage
        } else {
            obligation.identity().support_basis() == UiObligationSupportBasis::TouchMeaning
        }
    }));
    assert!(left.obligations().iter().all(|obligation| {
        obligation
            .selection_reasons()
            .contains(&UiObligationSelectionReason::WorldProfile(
                UiObligationWorldProfileClass::HotReloadCandidate,
            ))
    }));

    let portal = obligation_by_family(&left, UiObligationFamily::PortalHostRequirement);
    assert_eq!(
        portal.identity().support_basis(),
        UiObligationSupportBasis::ServiceUsage
    );
    assert_eq!(
        portal.identity().aspect_scope(),
        &[UiGraphTouchRuntimeLane::Structural]
    );
    assert_eq!(portal.identity().world(), touch.world());
    assert_eq!(
        portal.check_kind(),
        UiObligationCheckKind::PrerequisiteRequirement
    );
}

#[test]
fn query_and_diagnostic_touches_retain_exact_identity_and_reason_topology() {
    let app = touch_app(query_snapshot_world_profile(
        "snapshot:phase4-selection",
        ["worth-ui.phase4", "selection", "query"],
    ));
    let graph = app.graph();
    let artifact = control_artifact(&app);
    let touch = graph
        .touches()
        .from_mounted_receipt_transition(
            graph
                .touches()
                .query_fact_change_receipt()
                .expect("query world should admit query receipt"),
            UiGraphTouchTiming::PostMutation,
            mounted_receipt_transition(&app, artifact),
            UiGraphTouchAspects::new()
                .query_binding(UiGraphTouchAspectPosture::Invalidated)
                .participation(UiGraphTouchAspectPosture::Invalidated)
                .diagnostic(UiGraphTouchAspectPosture::Written),
        )
        .expect("query-backed touch should admit");

    let left = app.admission().select_obligations(&touch);
    let right = app.admission().select_obligations(&touch);

    assert_eq!(left, right);
    assert_eq!(
        left.obligations()
            .iter()
            .map(|obligation| obligation.family())
            .collect::<Vec<_>>(),
        vec![
            UiObligationFamily::ParticipationLegality,
            UiObligationFamily::QueryBindingRequirement,
            UiObligationFamily::DiagnosticSurfaceRequirement,
        ]
    );

    let participation = obligation_by_family(&left, UiObligationFamily::ParticipationLegality);
    assert_eq!(
        participation.identity().support_basis(),
        UiObligationSupportBasis::TouchMeaning
    );
    assert_eq!(
        participation.identity().aspect_scope(),
        &[UiGraphTouchRuntimeLane::Participation]
    );
    assert_eq!(participation.identity().world(), touch.world());
    assert_eq!(
        participation.check_kind(),
        UiObligationCheckKind::BlockingInvariant
    );
    assert_eq!(
        participation.selection_reasons(),
        [
            UiObligationSelectionReason::TouchTargetClass(UiGraphTouchTargetClass::AttachmentLane),
            UiObligationSelectionReason::TouchOriginClass(UiGraphTouchOriginClass::QueryFactChange),
            UiObligationSelectionReason::WorldProfile(
                UiObligationWorldProfileClass::QuerySnapshotBasis,
            ),
            UiObligationSelectionReason::SupportPosture(
                UiObligationSupportSelectionPosture::Supported,
            ),
            UiObligationSelectionReason::SupportRow(
                UiDeclarationSupportRowSchemaKind::TouchMeaning
            ),
            UiObligationSelectionReason::TouchRuntimeLane(UiGraphTouchRuntimeLane::Participation),
            UiObligationSelectionReason::TouchAspectPosture(UiGraphTouchAspectPosture::Invalidated,),
        ]
    );

    let query_binding = obligation_by_family(&left, UiObligationFamily::QueryBindingRequirement);
    assert_eq!(
        query_binding.identity().support_basis(),
        UiObligationSupportBasis::QueryBinding
    );
    assert_eq!(
        query_binding.identity().aspect_scope(),
        &[UiGraphTouchRuntimeLane::QueryBinding]
    );
    assert_eq!(query_binding.identity().world(), touch.world());
    assert_eq!(
        query_binding.check_kind(),
        UiObligationCheckKind::PrerequisiteRequirement
    );
    assert_eq!(
        query_binding.selection_reasons(),
        [
            UiObligationSelectionReason::TouchTargetClass(UiGraphTouchTargetClass::AttachmentLane),
            UiObligationSelectionReason::TouchOriginClass(UiGraphTouchOriginClass::QueryFactChange),
            UiObligationSelectionReason::WorldProfile(
                UiObligationWorldProfileClass::QuerySnapshotBasis,
            ),
            UiObligationSelectionReason::SupportPosture(
                UiObligationSupportSelectionPosture::Supported,
            ),
            UiObligationSelectionReason::SupportRow(
                UiDeclarationSupportRowSchemaKind::QueryBinding,
            ),
            UiObligationSelectionReason::TouchRuntimeLane(UiGraphTouchRuntimeLane::QueryBinding),
            UiObligationSelectionReason::TouchAspectPosture(UiGraphTouchAspectPosture::Invalidated,),
            UiObligationSelectionReason::GraphQueryBindingAttachment,
        ]
    );

    let diagnostic = obligation_by_family(&left, UiObligationFamily::DiagnosticSurfaceRequirement);
    assert_eq!(
        diagnostic.identity().support_basis(),
        UiObligationSupportBasis::ServiceUsage
    );
    assert_eq!(
        diagnostic.identity().aspect_scope(),
        &[UiGraphTouchRuntimeLane::Diagnostic]
    );
    assert_eq!(diagnostic.identity().world(), touch.world());
    assert_eq!(
        diagnostic.check_kind(),
        UiObligationCheckKind::DiagnosticOnlyCheck
    );
    assert_eq!(
        diagnostic.selection_reasons(),
        [
            UiObligationSelectionReason::TouchTargetClass(UiGraphTouchTargetClass::AttachmentLane),
            UiObligationSelectionReason::TouchOriginClass(UiGraphTouchOriginClass::QueryFactChange),
            UiObligationSelectionReason::WorldProfile(
                UiObligationWorldProfileClass::QuerySnapshotBasis,
            ),
            UiObligationSelectionReason::SupportPosture(
                UiObligationSupportSelectionPosture::Supported,
            ),
            UiObligationSelectionReason::SupportRow(
                UiDeclarationSupportRowSchemaKind::ServiceUsage,
            ),
            UiObligationSelectionReason::TouchRuntimeLane(UiGraphTouchRuntimeLane::Diagnostic),
            UiObligationSelectionReason::TouchAspectPosture(UiGraphTouchAspectPosture::Written),
        ]
    );
}

fn touch_app(world_profile: UiGraphWorldProfile) -> worth_ui::facade::app::WorthUiApp {
    WorthUi::app()
        .with_graph_world_profile(world_profile)
        .with_dsl_package(
            WorthUiDslPackage::named("worth-ui.certification.obligation-selection")
                .with_semantic_artifact_spec(control_spec()),
        )
        .freeze()
        .expect("application preparation should succeed")
}

fn control_spec() -> UiDslSemanticArtifactSpec {
    UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("workflow_editor.inspector.save"),
        UiDslSemanticFamily::Control,
        UiDslSourceProvenance::file_authored("app/obligation_selection_runtime.wui", 0),
    )
    .with_structural_token(UiDslStructuralToken::new("control:save"))
    .with_structural_token(UiDslStructuralToken::new("slot:footer"))
    .with_posture_token(UiDslPostureToken::new("query-binding:attached:view"))
    .with_posture_token(UiDslPostureToken::new("service:portal"))
}

fn control_artifact(app: &worth_ui::facade::app::WorthUiApp) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/obligation_selection_runtime.wui"
                && provenance.declaration_index() == 0
        })
        .expect("control artifact should exist")
}

fn graph_node_identity(
    graph: worth_ui::facade::graph::UiGraphAuthority<'_>,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphNodeIdentity {
    graph
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}

fn mounted_receipt_transition(
    app: &worth_ui::facade::app::WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> worth_ui::facade::graph::UiGraphMountedReceiptTransition {
    let graph = app.graph();
    let graph_node_identity = graph_node_identity(graph, artifact);
    let control_node = graph
        .lookup()
        .graph_node(graph_node_identity)
        .expect("graph should resolve node")
        .value();

    graph
        .mounted_receipt_transition_for_node(
            graph_node_identity,
            control_node
                .participation_posture()
                .axis(UiGraphParticipationAxis::Mounted),
            UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Admitted),
        )
        .expect("mounted transition should admit")
}

fn query_snapshot_world_profile(
    snapshot_label: &str,
    schema_basis_parts: [&str; 3],
) -> UiGraphWorldProfile {
    let snapshot_identity = WorthQuerySnapshotIdentity::admit_external_token(
        QueryExternalIdentityToken::new(Arc::<str>::from(snapshot_label)),
    );
    let basis = admit_runtime_current_snapshot_basis_for_certification(
        snapshot_identity.evidence_identity(),
        QueryExternalSchemaBasisToken::from_domain_parts(
            schema_basis_parts
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
        ),
    )
    .expect("runtime current snapshot basis should resolve");

    let prerequisites = worth_ui_query_binding::WorthUiQueryPrerequisiteBoundary::new()
        .graph_aligned(basis.clone(), snapshot_resolution_report(&basis))
        .expect("query prerequisites should admit");
    UiGraphWorldProfile::query_snapshot_basis(prerequisites)
}

fn obligation_by_family(
    selection: &worth_ui_runtime::facade::obligations::UiSelectedObligationSet,
    family: UiObligationFamily,
) -> &UiSelectedObligation {
    selection
        .obligations()
        .iter()
        .find(|obligation| obligation.family() == family)
        .unwrap_or_else(|| panic!("expected obligation for family {family:?}"))
}
