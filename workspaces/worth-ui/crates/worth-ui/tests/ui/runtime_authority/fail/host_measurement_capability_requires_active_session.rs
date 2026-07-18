use std::rc::Rc;
use worth_ui::facade::host::{
    WorthUiHostCapabilityReport, WorthUiHostMeasurementCapability, WorthUiHostSessionIdentity,
    WorthUiOperationalHostAdapter,
};

fn forge_capability(
    session_identity: WorthUiHostSessionIdentity,
    capability_report: WorthUiHostCapabilityReport,
    adapter: Rc<dyn WorthUiOperationalHostAdapter>,
) -> WorthUiHostMeasurementCapability {
    WorthUiHostMeasurementCapability {
        session_identity,
        capability_report,
        adapter,
    }
}

fn main() {}
