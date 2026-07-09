use serde::Serialize;

use super::WorkerRuntimeShellLock;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerRuntimeBootstrapRecord {
    pub shell_lock: WorkerRuntimeShellLock,
    pub boundary_surface: &'static str,
    pub transport_posture: &'static str,
    pub host_capability_ingress: &'static str,
    pub host_effect_egress: &'static str,
}

impl WorkerRuntimeBootstrapRecord {
    pub(in crate::runtime::worker_host) fn worker_first_portable_runtime() -> Self {
        Self {
            shell_lock: WorkerRuntimeShellLock::dedicated_worker_runtime_shell(),
            boundary_surface: "workerFirstConstruction",
            transport_posture: "inProcessBootstrapBeforeTransportBridge",
            host_capability_ingress: "deferredToHostBridge",
            host_effect_egress: "deferredToHostBridge",
        }
    }
}
