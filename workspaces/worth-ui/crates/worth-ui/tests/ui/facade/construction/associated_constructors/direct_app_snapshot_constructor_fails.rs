use worth_ui::facade::{
    app::{WorthUi, WorthUiApp},
    diagnostics::CapabilityRegistrationReport,
};

fn main() {
    let _ = WorthUiApp::from_registration_report(registration_report());
}

fn registration_report() -> CapabilityRegistrationReport {
    WorthUi::app().freeze_with_registration_report()
}
