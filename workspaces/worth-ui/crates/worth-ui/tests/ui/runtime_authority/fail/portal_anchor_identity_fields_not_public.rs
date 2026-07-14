use worth_ui_runtime::facade::host_observation::UiPortalAnchorTargetIdentity;
use worth_ui_runtime::facade::runtime_handoff::UiPortalAnchorIdentity;

fn main() {
    let _forged = UiPortalAnchorIdentity {
        target: UiPortalAnchorTargetIdentity::new(7),
        coordinate_space: worth_ui_runtime::facade::evidence::UiMeasurementCoordinateSpace::PortalLayer,
    };
}
