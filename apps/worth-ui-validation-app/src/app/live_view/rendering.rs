use super::control_rendering::render_live_view_control;
use super::evidence_rendering::render_live_view_evidence;
use super::host_observation::collect_live_view_host_observations;
use super::interaction_rendering::render_live_view_actions_from_mounted;
use super::proof::ValidationLiveViewProjectionProof;
use super::receipt_color_translation::to_egui_color;
use super::viewport_adapter::node_visible;
use worth_ui::facade::{
    WorthUiEffectiveViewportParticipationReceipt, WorthUiLayoutAllocationReceipt,
    WorthUiLayoutAllocationRequest, WorthUiLiveViewStateEditIntent,
    WorthUiMountedCompositionChildReceipt, WorthUiMountedFlowAlign,
    WorthUiMountedInteractionNodeReceipt, WorthUiMountedNodeReceipt, WorthUiPrimitiveContentItem,
    WorthUiPrimitiveContentReceipt,
};

pub(crate) fn render_live_view_state_proof(
    ui: &mut egui::Ui,
    runtime: &worth_ui::facade::WorthUiRuntimeHost,
    proof: Result<&ValidationLiveViewProjectionProof, &str>,
    last_edit: Option<&worth_ui::facade::WorthUiLiveViewEditReceipt>,
    last_denial: Option<&worth_ui::facade::WorthUiLiveViewStateEditDenial>,
    last_submission: Option<&worth_ui::facade::WorthUiLiveViewInteractionSubmissionReceipt>,
    last_submission_denial: Option<&worth_ui::facade::WorthUiLiveViewInteractionActivationDenial>,
    last_source_denial: Option<&str>,
) -> ValidationLiveViewFrameIntents {
    let mut intents = ValidationLiveViewFrameIntents::default();
    let observation = collect_live_view_host_observations(ui, runtime, proof);
    let area = observation.area();
    ui.scope_builder(egui::UiBuilder::new().max_rect(area), |ui| {
        render_live_view_panel_contents(
            ui,
            runtime,
            proof,
            last_edit,
            last_denial,
            last_submission,
            last_submission_denial,
            last_source_denial,
            observation.measurement_proof(),
            &mut intents,
        );
    });
    intents
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ValidationLiveViewFrameIntents {
    pub state_edits: Vec<WorthUiLiveViewStateEditIntent>,
    pub submissions: Vec<WorthUiMountedInteractionNodeReceipt>,
}

fn render_live_view_panel_contents(
    ui: &mut egui::Ui,
    runtime: &worth_ui::facade::WorthUiRuntimeHost,
    proof: Result<&ValidationLiveViewProjectionProof, &str>,
    last_edit: Option<&worth_ui::facade::WorthUiLiveViewEditReceipt>,
    last_denial: Option<&worth_ui::facade::WorthUiLiveViewStateEditDenial>,
    last_submission: Option<&worth_ui::facade::WorthUiLiveViewInteractionSubmissionReceipt>,
    last_submission_denial: Option<&worth_ui::facade::WorthUiLiveViewInteractionActivationDenial>,
    last_source_denial: Option<&str>,
    measurement_proof: &super::host_observation::ValidationLiveViewFrameMeasurementProof,
    intents: &mut ValidationLiveViewFrameIntents,
) {
    let Ok(proof) = proof else {
        render_live_view_evidence(
            ui,
            runtime,
            proof,
            None,
            None,
            None,
            None,
            last_source_denial.or_else(|| proof.err()),
            Some(measurement_proof),
        );
        return;
    };
    render_mounted_nodes(
        ui,
        runtime,
        proof,
        last_edit,
        last_denial,
        last_submission,
        last_submission_denial,
        last_source_denial,
        measurement_proof,
        intents,
    );
}

fn render_mounted_nodes(
    ui: &mut egui::Ui,
    runtime: &worth_ui::facade::WorthUiRuntimeHost,
    proof: &ValidationLiveViewProjectionProof,
    last_edit: Option<&worth_ui::facade::WorthUiLiveViewEditReceipt>,
    last_denial: Option<&worth_ui::facade::WorthUiLiveViewStateEditDenial>,
    last_submission: Option<&worth_ui::facade::WorthUiLiveViewInteractionSubmissionReceipt>,
    last_submission_denial: Option<&worth_ui::facade::WorthUiLiveViewInteractionActivationDenial>,
    last_source_denial: Option<&str>,
    measurement_proof: &super::host_observation::ValidationLiveViewFrameMeasurementProof,
    intents: &mut ValidationLiveViewFrameIntents,
) {
    let tree = proof.mounted_product_view().composition_tree();
    let surface_child = tree
        .root_children()
        .iter()
        .find(|child| matches!(child.mounted_node(), WorthUiMountedNodeReceipt::Surface(_)))
        .expect("mounted composition tree must include a surface child");
    let WorthUiMountedNodeReceipt::Surface(surface) = surface_child.mounted_node() else {
        unreachable!("surface child was filtered by node kind");
    };
    let allocation = measurement_proof
        .measured_product_view()
        .and_then(|measured| {
            runtime
                .allocate_mounted_product_view(
                    measured,
                    WorthUiLayoutAllocationRequest::for_root_node(surface_child.node_id()),
                )
                .ok()
        });
    let Some(allocation) = allocation.as_ref() else {
        render_live_view_evidence(
            ui,
            runtime,
            Ok(proof),
            last_edit,
            last_denial,
            last_submission,
            last_submission_denial,
            last_source_denial,
            Some(measurement_proof),
        );
        return;
    };
    let viewport_boundary = measurement_proof
        .measured_product_view()
        .and_then(|measured| {
            runtime
                .resolve_viewport_boundaries(measured, allocation)
                .ok()
        });
    let effective_viewport = viewport_boundary.as_ref().map(|viewport| {
        runtime.resolve_effective_viewport_participation(proof.mounted_product_view(), viewport)
    });
    let surface_area = ui.max_rect();
    paint_mounted_surface(ui, surface_area, surface);
    for child in tree.ordered_children(surface_child.node_id()) {
        render_mounted_surface_child(
            ui,
            child,
            surface_area.min,
            proof,
            allocation,
            effective_viewport.as_ref(),
            intents,
        );
    }
    render_live_view_evidence(
        ui,
        runtime,
        Ok(proof),
        last_edit,
        last_denial,
        last_submission,
        last_submission_denial,
        last_source_denial,
        Some(measurement_proof),
    );
}

fn render_mounted_surface_child(
    ui: &mut egui::Ui,
    child: &WorthUiMountedCompositionChildReceipt,
    root_origin: egui::Pos2,
    proof: &ValidationLiveViewProjectionProof,
    allocation: &WorthUiLayoutAllocationReceipt,
    viewport: Option<&WorthUiEffectiveViewportParticipationReceipt>,
    intents: &mut ValidationLiveViewFrameIntents,
) {
    if !node_visible(viewport, child.node_id()) {
        return;
    }
    match child.mounted_node() {
        WorthUiMountedNodeReceipt::FlowContainer(_) => {
            render_mounted_flow_contents(
                ui,
                child,
                root_origin,
                proof,
                allocation,
                viewport,
                intents,
            );
        }
        WorthUiMountedNodeReceipt::Evidence(_)
        | WorthUiMountedNodeReceipt::Surface(_)
        | WorthUiMountedNodeReceipt::Text(_)
        | WorthUiMountedNodeReceipt::Icon(_)
        | WorthUiMountedNodeReceipt::DiagnosticPanel(_)
        | WorthUiMountedNodeReceipt::PortalHost(_)
        | WorthUiMountedNodeReceipt::MosaicRegion(_) => {}
        WorthUiMountedNodeReceipt::Content(_)
        | WorthUiMountedNodeReceipt::Control(_)
        | WorthUiMountedNodeReceipt::Interaction(_) => {
            render_mounted_child_at_receipt_frame(
                ui,
                child,
                WorthUiMountedFlowAlign::Start,
                root_origin,
                proof,
                allocation,
                viewport,
                intents,
            );
        }
    }
}

fn render_mounted_flow_contents(
    ui: &mut egui::Ui,
    flow_child: &WorthUiMountedCompositionChildReceipt,
    root_origin: egui::Pos2,
    proof: &ValidationLiveViewProjectionProof,
    allocation: &WorthUiLayoutAllocationReceipt,
    viewport: Option<&WorthUiEffectiveViewportParticipationReceipt>,
    intents: &mut ValidationLiveViewFrameIntents,
) {
    let align = match flow_child.mounted_node() {
        WorthUiMountedNodeReceipt::FlowContainer(flow) => flow.align(),
        _ => WorthUiMountedFlowAlign::Start,
    };
    for child in proof
        .mounted_product_view()
        .composition_tree()
        .ordered_children(flow_child.node_id())
    {
        if !node_visible(viewport, child.node_id()) {
            continue;
        }
        render_mounted_child_at_receipt_frame(
            ui,
            child,
            align,
            root_origin,
            proof,
            allocation,
            viewport,
            intents,
        );
    }
}

fn render_mounted_child_at_receipt_frame(
    ui: &mut egui::Ui,
    child: &WorthUiMountedCompositionChildReceipt,
    align: WorthUiMountedFlowAlign,
    root_origin: egui::Pos2,
    proof: &ValidationLiveViewProjectionProof,
    allocation: &WorthUiLayoutAllocationReceipt,
    viewport: Option<&WorthUiEffectiveViewportParticipationReceipt>,
    intents: &mut ValidationLiveViewFrameIntents,
) {
    let Some(frame) = allocation.child_frame(child.node_id()) else {
        return;
    };
    if frame.width() <= 0.0 || frame.height() <= 0.0 {
        return;
    }
    let rect = egui::Rect::from_min_size(
        egui::pos2(root_origin.x + frame.x(), root_origin.y + frame.y()),
        egui::vec2(frame.width(), frame.height()),
    );
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        render_mounted_child(
            ui,
            child,
            align,
            root_origin,
            proof,
            allocation,
            viewport,
            intents,
        );
    });
}

fn render_mounted_child(
    ui: &mut egui::Ui,
    child: &WorthUiMountedCompositionChildReceipt,
    align: WorthUiMountedFlowAlign,
    root_origin: egui::Pos2,
    proof: &ValidationLiveViewProjectionProof,
    allocation: &WorthUiLayoutAllocationReceipt,
    viewport: Option<&WorthUiEffectiveViewportParticipationReceipt>,
    intents: &mut ValidationLiveViewFrameIntents,
) {
    match child.mounted_node() {
        WorthUiMountedNodeReceipt::Surface(_) => {}
        WorthUiMountedNodeReceipt::FlowContainer(_) => {
            render_mounted_flow_contents(
                ui,
                child,
                root_origin,
                proof,
                allocation,
                viewport,
                intents,
            );
        }
        WorthUiMountedNodeReceipt::Control(frame) => {
            render_live_view_control(ui, frame, &mut intents.state_edits);
        }
        WorthUiMountedNodeReceipt::Content(content) => {
            render_primitive_content(ui, content.content());
        }
        WorthUiMountedNodeReceipt::Interaction(row) => {
            render_live_view_actions_from_mounted(
                ui,
                align,
                std::slice::from_ref(row),
                &mut intents.submissions,
            );
        }
        WorthUiMountedNodeReceipt::Evidence(_)
        | WorthUiMountedNodeReceipt::Text(_)
        | WorthUiMountedNodeReceipt::Icon(_)
        | WorthUiMountedNodeReceipt::DiagnosticPanel(_)
        | WorthUiMountedNodeReceipt::PortalHost(_)
        | WorthUiMountedNodeReceipt::MosaicRegion(_) => {}
    }
}

fn render_primitive_content(ui: &mut egui::Ui, content: &WorthUiPrimitiveContentReceipt) {
    ui.horizontal(|ui| {
        for item in content.items() {
            match item {
                WorthUiPrimitiveContentItem::Text(text) => {
                    ui.label(egui::RichText::new(text.text()).size(text.size_points()));
                }
                WorthUiPrimitiveContentItem::Badge(badge) => {
                    ui.label(egui::RichText::new(badge.text()).size(badge.size_points()));
                }
                WorthUiPrimitiveContentItem::Icon(icon) => {
                    ui.label(egui::RichText::new(icon.icon_id()).size(icon.size_points()));
                }
                WorthUiPrimitiveContentItem::Image(image) => {
                    ui.label(egui::RichText::new(image.asset_id()).size(image.height_points()));
                }
                WorthUiPrimitiveContentItem::Spacer(spacer) => {
                    ui.add_space(spacer.size_points());
                }
                WorthUiPrimitiveContentItem::Divider(divider) => {
                    ui.separator();
                    ui.add_space(divider.thickness_points());
                }
            }
        }
    });
}

fn paint_mounted_surface(
    ui: &mut egui::Ui,
    area: egui::Rect,
    surface: &worth_ui::facade::WorthUiMountedSurfaceNodeReceipt,
) {
    ui.painter().rect(
        area,
        egui::CornerRadius::same(surface.radius_points() as u8),
        to_egui_color(surface.background_color()),
        egui::Stroke::new(
            surface.border_width_points(),
            to_egui_color(surface.border_color()),
        ),
        egui::StrokeKind::Inside,
    );
}
