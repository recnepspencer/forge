//! Graph-touch authority for obligation-dispatch prerequisite scenarios.

use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::declaration::UiDeclarationArtifact;
use worth_ui::facade::graph::{
    UiGraphAxisParticipation, UiGraphMountedReceiptTransition, UiGraphNodeIdentity,
    UiGraphParticipationAxis, UiGraphParticipationStatus, UiGraphTouchAspectPosture,
    UiGraphTouchAspects, UiGraphTouchDescriptor, UiGraphTouchTiming,
};

pub fn structural_touch(app: &WorthUiApp) -> UiGraphTouchDescriptor {
    let artifact = artifact_from_module_path(app, "app/obligation_dispatch_prereq_runtime.wui");
    app.graph()
        .touches()
        .from_node(
            app.graph()
                .touches()
                .declaration_change_receipt(artifact)
                .expect("declaration change should admit"),
            UiGraphTouchTiming::PostMutation,
            graph_node_identity(app, artifact),
            UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("structural touch should admit")
}

pub fn service_touch(app: &WorthUiApp) -> UiGraphTouchDescriptor {
    let artifact = artifact_from_module_path(app, "app/obligation_dispatch_service_runtime.wui");
    app.graph()
        .touches()
        .from_node(
            app.graph()
                .touches()
                .declaration_change_receipt(artifact)
                .expect("declaration change should admit"),
            UiGraphTouchTiming::PostMutation,
            graph_node_identity(app, artifact),
            UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("service touch should admit")
}

pub fn query_touch(app: &WorthUiApp) -> UiGraphTouchDescriptor {
    let artifact = artifact_from_module_path(app, "app/obligation_dispatch_prereq_runtime.wui");
    app.graph()
        .touches()
        .from_mounted_receipt_transition(
            app.graph()
                .touches()
                .query_binding_change_receipt()
                .expect("query world should admit query receipt"),
            UiGraphTouchTiming::PostMutation,
            mounted_receipt_transition(app, artifact),
            UiGraphTouchAspects::new()
                .query_binding(UiGraphTouchAspectPosture::Invalidated)
                .participation(UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("query touch should admit")
}

pub fn focus_touch(app: &WorthUiApp) -> UiGraphTouchDescriptor {
    let artifact = artifact_from_module_path(app, "app/obligation_dispatch_focus_runtime.wui");
    app.graph()
        .touches()
        .from_node(
            app.graph()
                .touches()
                .declaration_change_receipt(artifact)
                .expect("declaration change should admit"),
            UiGraphTouchTiming::PostMutation,
            graph_node_identity(app, artifact),
            UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("focus touch should admit")
}

pub fn motion_touch(app: &WorthUiApp) -> UiGraphTouchDescriptor {
    let artifact = artifact_from_module_path(app, "app/obligation_dispatch_motion_runtime.wui");
    app.graph()
        .touches()
        .from_node(
            app.graph()
                .touches()
                .declaration_change_receipt(artifact)
                .expect("declaration change should admit"),
            UiGraphTouchTiming::PostMutation,
            graph_node_identity(app, artifact),
            UiGraphTouchAspects::new().structural(UiGraphTouchAspectPosture::Invalidated),
        )
        .expect("motion touch should admit")
}

pub fn artifact_from_module_path<'a>(
    app: &'a WorthUiApp,
    module_path: &str,
) -> &'a UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == module_path && provenance.declaration_index() == 0
        })
        .expect("control artifact should exist")
}

pub fn graph_node_identity(
    app: &WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> UiGraphNodeIdentity {
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should admit one graph node")
}

fn mounted_receipt_transition(
    app: &WorthUiApp,
    artifact: &UiDeclarationArtifact,
) -> UiGraphMountedReceiptTransition {
    let graph_node_identity = graph_node_identity(app, artifact);
    let control_node = app
        .graph()
        .lookup()
        .graph_node(graph_node_identity)
        .expect("graph should resolve node")
        .value();

    app.graph()
        .mounted_receipt_transition_for_node(
            graph_node_identity,
            control_node
                .participation_posture()
                .axis(UiGraphParticipationAxis::Mounted),
            UiGraphAxisParticipation::runtime_mutation(UiGraphParticipationStatus::Admitted),
        )
        .expect("mounted transition should admit")
}
