use worth_ui_host_contract::{UiHostProtocolContract, WorthUiHostMechanicsAdapter};

fn enter_headless(
    host: &worth_ui_host_headless::WorthUiHeadlessRecorder,
    raw: &UiHostProtocolContract,
) {
    let _ = host.perform_mounted_surface_presentation(raw);
}

fn main() {}
