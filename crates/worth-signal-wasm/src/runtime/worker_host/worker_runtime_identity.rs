use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerRuntimeIdentity {
    pub deployment_posture: &'static str,
    pub runtime_authority: &'static str,
    pub topology: &'static str,
}

impl WorkerRuntimeIdentity {
    pub(in crate::runtime::worker_host) fn dedicated_worker_owned_runtime() -> Self {
        Self {
            deployment_posture: "workerFirst",
            runtime_authority: "workerOwnedRuntime",
            topology: "oneDedicatedWorkerRuntimePerRuntimeInstance",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerRuntimeShellLock {
    pub identity: WorkerRuntimeIdentity,
    pub graph_publication_admission: &'static str,
    pub committed_envelope_family: &'static str,
    pub callback_publication_before_lowering: &'static str,
}

impl WorkerRuntimeShellLock {
    pub(in crate::runtime::worker_host) fn dedicated_worker_runtime_shell() -> Self {
        Self {
            identity: WorkerRuntimeIdentity::dedicated_worker_owned_runtime(),
            graph_publication_admission: "portableDefinitionsOnly",
            committed_envelope_family: "transactionResult",
            callback_publication_before_lowering: "denied",
        }
    }
}
