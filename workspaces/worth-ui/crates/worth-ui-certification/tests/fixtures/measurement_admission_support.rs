use worth_ui::facade::admission::{UiAdmissionTarget, UiAdmissionWorld};
use worth_ui::facade::graph::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchDescriptor, UiGraphTouchTiming,
};
use worth_ui_host_contract::{WorthUiHostCapability, WorthUiHostCapabilityReport};
use worth_ui_test_support::WorthUiTouchOriginCertificationFixture;

pub fn host_measurement_touch(
    fixture: &WorthUiTouchOriginCertificationFixture,
) -> UiGraphTouchDescriptor {
    fixture
        .app
        .graph()
        .touches()
        .from_node(
            fixture
                .app
                .graph()
                .touches()
                .host_observation_receipt(fixture.runtime.inspect_active(), &fixture.inspection)
                .expect("host observation should admit"),
            UiGraphTouchTiming::ReactiveObservation,
            fixture.control_graph_node_identity(),
            UiGraphTouchAspects::new().measurement(UiGraphTouchAspectPosture::Read),
        )
        .expect("host measurement touch should admit")
}

pub fn available_measurement_target(touch: &UiGraphTouchDescriptor) -> UiAdmissionTarget {
    UiAdmissionTarget::graph_node(
        touch.target().graph_node_identity(),
        UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
    )
    .with_host_capability_report(WorthUiHostCapabilityReport::available(vec![
        WorthUiHostCapability::DpiObservation,
        WorthUiHostCapability::FontMetrics,
        WorthUiHostCapability::NativeControlIntrinsicMeasurement,
        WorthUiHostCapability::PortalAnchorObservation,
        WorthUiHostCapability::ScrollContainerObservation,
        WorthUiHostCapability::TextBaselineMeasurement,
        WorthUiHostCapability::TextIntrinsicMeasurement,
        WorthUiHostCapability::ViewportObservation,
    ]))
}
