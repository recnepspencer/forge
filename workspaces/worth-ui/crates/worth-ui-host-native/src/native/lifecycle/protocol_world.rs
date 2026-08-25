pub(super) mod presentation;
pub(super) mod resources;
pub(super) mod schema;

pub use schema::{
    UiNativeLifecycleProtocolReport, UiNativeLifecycleProtocolSchedule,
    UiNativeProtocolCloseDisposition, UiNativeProtocolClosePoint, UiNativeProtocolNextAction,
    UiNativeProtocolPredecessor, UiNativeProtocolReadback, UiNativeProtocolResourceCensus,
    UiNativeProtocolSurfaceTransition,
};

pub struct UiNativeLifecycleProtocolWorld;

impl UiNativeLifecycleProtocolWorld {
    pub fn run(schedule: UiNativeLifecycleProtocolSchedule) -> UiNativeLifecycleProtocolReport {
        super::orchestrator::run_protocol(schedule)
    }
}
