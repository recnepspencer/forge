use crate::capability::{
    ComponentCanvasSpatialContract, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership,
};
use crate::facade::WorthUi;
use crate::runtime::{
    WorthUiCanvasSpatialFrameReceipt, WorthUiCanvasSpatialFrameTarget, WorthUiSourceProvider,
    WorthUiWatcherEvent,
};

use super::source_ingress_boundary_test_support::lower_file_submission;

pub(super) fn canvas_spatial_frame_receipt() -> WorthUiCanvasSpatialFrameReceipt {
    let component_id = "workspace.component.certified_canvas";
    let descriptor = || {
        ComponentDescriptor::new(
            ComponentId::new(component_id).expect("fixture component id is valid"),
            ComponentPropSchema::named("certified.canvas.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        )
        .with_canvas_spatial_contract(
            ComponentCanvasSpatialContract::new(512, 4, 2)
                .expect("fixture canvas contract has a positive scale limit"),
        )
    };
    let capability_app = WorthUi::app()
        .register_component(descriptor())
        .freeze()
        .expect("canvas fixture capabilities freeze");
    let submission = lower_file_submission(
        WorthUiSourceProvider::in_memory("lane-frame-cost.canvas")
            .with_file("app/main.wui", format!("component {component_id} {{}}\n")),
        [WorthUiWatcherEvent::provider_revision(
            "lane-frame-cost.canvas",
        )],
        capability_app.capabilities(),
    );
    let mut session = WorthUi::app()
        .register_component(descriptor())
        .with_candidate_submission(submission)
        .freeze()
        .expect("canvas fixture application freezes")
        .launch()
        .expect("canvas fixture application launches");
    let handle = session
        .first_canvas_spatial_handle()
        .expect("canvas fixture publishes an active target");
    let execution = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("empty collection completes the fixture turn"));
    execution
        .execute_canvas_spatial_frame(WorthUiCanvasSpatialFrameTarget::draw(handle))
        .expect("active canvas fixture produces certified lane evidence")
        .receipt()
        .clone()
}
