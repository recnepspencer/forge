use crate::declaration::UiDeclarationArtifact;
use crate::facade::inspection::{
    WorthUiAuthoredInspectionBoundary, WorthUiMeasurementInspectionBoundary,
    WorthUiPlanningInspectionBoundary,
};
use crate::facade::WorthUiApp;
use crate::graph::{
    UiGraphNodeIdentity, WorthUiAspectInspectionBoundary, WorthUiGraphInspectionBoundary,
};

pub(crate) fn authored_inspection_boundary<'a>(
    app: &'a WorthUiApp,
) -> WorthUiAuthoredInspectionBoundary<'a> {
    WorthUiAuthoredInspectionBoundary::new(
        app.declaration_artifacts(),
        app.authored_evidence_index(),
    )
}

pub(crate) fn graph_inspection_boundary<'a>(
    app: &'a WorthUiApp,
) -> WorthUiGraphInspectionBoundary<'a> {
    WorthUiGraphInspectionBoundary::new(
        app.declaration_artifacts(),
        app.graph_snapshot(),
        app.graph_node_evidence_index(),
    )
}

pub(crate) fn aspect_inspection_boundary<'a>(
    app: &'a WorthUiApp,
) -> WorthUiAspectInspectionBoundary<'a> {
    WorthUiAspectInspectionBoundary::new(
        app.declaration_artifacts(),
        app.graph_aspect_evidence_indexes(),
    )
}

pub(crate) fn measurement_inspection_boundary<'a>(
    app: &'a WorthUiApp,
) -> WorthUiMeasurementInspectionBoundary<'a> {
    WorthUiMeasurementInspectionBoundary::new(
        app.declaration_artifacts(),
        app.graph_snapshot(),
        app.authored_evidence_index(),
        app.graph_node_evidence_index(),
        app.measurement_inspection_evidence(),
    )
}

pub(crate) fn planning_inspection_boundary<'a>(
    app: &'a WorthUiApp,
) -> WorthUiPlanningInspectionBoundary<'a> {
    WorthUiPlanningInspectionBoundary::new(app.retained_allocation_planning_registry())
}

pub(crate) fn declaration_artifact_for_graph_node(
    app: &WorthUiApp,
    graph_node_identity: UiGraphNodeIdentity,
) -> Option<&UiDeclarationArtifact> {
    let artifact_index = app
        .graph_node_evidence_index()
        .lookup_graph_node_identity(graph_node_identity)?
        .neighborhood()
        .declaration_artifact_index();
    app.declaration_artifacts().get(artifact_index)
}
