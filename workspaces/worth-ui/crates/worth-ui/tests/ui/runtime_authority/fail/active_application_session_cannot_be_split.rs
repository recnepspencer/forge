use worth_ui::facade::app::{
    WorthUiActiveApplicationSession, WorthUiMountedApplicationReplacementInFlight,
};
use worth_ui_runtime::facade::WorthUiMountedPreviewInFlight;

fn split(session: WorthUiActiveApplicationSession) {
    let _runtime = session.runtime;
    let _inspection = session.app;
    let _application_aggregate = session.application;
    let _mounted_aggregate = session.mounted;
    let _host_exchange_aggregate = session.host_exchange;
}

fn bypass_mounted_frame_route(session: &mut WorthUiActiveApplicationSession) {
    let _whole_turn = session.execute_framework_turn(|_| {});
}

fn split_mounted_replacement(replacement: &WorthUiMountedApplicationReplacementInFlight<'_>) {
    let _ordinary_handle = replacement.handle();
}

fn split_mounted_preview(preview: &WorthUiMountedPreviewInFlight<'_>) {
    let _ordinary_handle = preview.handle();
}

fn main() {}
