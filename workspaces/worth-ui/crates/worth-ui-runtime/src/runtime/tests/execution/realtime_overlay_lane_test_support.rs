use crate::capability::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentRealtimeOverlayContract, ComponentRealtimeOverlayPriority, ComponentStateOwnership,
};
use crate::facade::WorthUi;
use crate::runtime::{
    WorthUiRealtimeFrameDenial, WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameTarget,
    WorthUiRendererSurfaceHandle, WorthUiRuntimeLaunchDenial, WorthUiSourceProvider,
    WorthUiWatcherEvent,
};

use super::source_ingress_boundary_test_support::lower_file_submission;

pub(super) fn realtime_overlay_fixture() -> RealtimeOverlayFixture {
    RealtimeOverlayFixture::new(8, 4, 16)
}

pub(super) struct RealtimeOverlayFixture {
    pub(super) session: crate::facade::WorthUiActiveApplicationSession,
}

impl RealtimeOverlayFixture {
    pub(super) fn new(row_limit: u16, declared_cost: u16, budget: u32) -> Self {
        Self {
            session: realtime_app(row_limit, declared_cost, budget)
                .launch()
                .expect("fixture application launches"),
        }
    }

    pub(super) fn handle(&self) -> WorthUiRendererSurfaceHandle {
        self.session
            .first_realtime_renderer_surface()
            .expect("fixture publishes a realtime renderer surface")
    }

    pub(super) fn execute(
        &mut self,
        target: WorthUiRealtimeFrameTarget,
    ) -> Result<WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameDenial> {
        let execution = self
            .session
            .execute_framework_turn(|_| {})
            .expect("no mounted presentation lease is active")
            .into_execution()
            .unwrap_or_else(|_| panic!("empty collection completes the fixture turn"));
        execution
            .execute_realtime_frame(target)
            .map(|completion| completion.receipt().clone())
    }
}

pub(super) fn realtime_launch_denial(
    row_limit: u16,
    declared_cost: u16,
    budget: u32,
) -> WorthUiRuntimeLaunchDenial {
    match realtime_app(row_limit, declared_cost, budget).launch() {
        Ok(_) => panic!("over-budget realtime application must not publish"),
        Err(denial) => denial,
    }
}

fn realtime_app(row_limit: u16, declared_cost: u16, budget: u32) -> crate::facade::WorthUiApp {
    let descriptor = || realtime_descriptor(row_limit, declared_cost, budget);
    let capabilities = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .register_component(descriptor())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("fixture capability application freezes");
    let submission = lower_file_submission(
        WorthUiSourceProvider::in_memory("realtime-overlay.fixture")
            .with_file("app/main.wui", "component workspace.component.hud {}\n"),
        [WorthUiWatcherEvent::provider_revision(
            "realtime-overlay.fixture",
        )],
        capabilities.capabilities(),
    );
    WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .register_component(descriptor())
        .with_candidate_submission(submission)
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("fixture source application freezes")
}

fn realtime_descriptor(row_limit: u16, declared_cost: u16, budget: u32) -> ComponentDescriptor {
    let contract = ComponentRealtimeOverlayContract::new(
        row_limit,
        declared_cost,
        budget,
        ComponentRealtimeOverlayPriority::HudOverlay,
    )
    .expect("fixture realtime contract is structurally valid");
    ComponentDescriptor::new(
        ComponentId::new("workspace.component.hud").expect("fixture id is valid"),
        ComponentPropSchema::named("hud.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
    .with_realtime_overlay_contract(contract)
}
