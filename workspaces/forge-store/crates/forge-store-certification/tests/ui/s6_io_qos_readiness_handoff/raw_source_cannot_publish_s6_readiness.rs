use forge_store_physical_isolation::{
    publish_scheduler_isolation_capability, SchedulerIsolationCapabilityRequest,
};

fn main() {
    let request: SchedulerIsolationCapabilityRequest = unimplemented!();
    let _ = publish_scheduler_isolation_capability(request);
}
