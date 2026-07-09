use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkerDeploymentPosture {
    WorkerFirst,
    MainThreadCompatibility,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerDeploymentPostureSummary {
    pub label: &'static str,
    pub runtime_authority: &'static str,
    pub preferred_for_heavy_apps: bool,
}

pub(in crate::runtime::worker_bridge) fn worker_deployment_postures(
) -> Vec<WorkerDeploymentPostureSummary> {
    [
        WorkerDeploymentPosture::WorkerFirst,
        WorkerDeploymentPosture::MainThreadCompatibility,
    ]
    .into_iter()
    .map(WorkerDeploymentPosture::summary)
    .collect()
}

impl WorkerDeploymentPosture {
    fn summary(self) -> WorkerDeploymentPostureSummary {
        match self {
            Self::WorkerFirst => WorkerDeploymentPostureSummary {
                label: "workerFirst",
                runtime_authority: "workerOwnedRuntime",
                preferred_for_heavy_apps: true,
            },
            Self::MainThreadCompatibility => WorkerDeploymentPostureSummary {
                label: "mainThreadCompatibility",
                runtime_authority: "mainThreadRuntime",
                preferred_for_heavy_apps: false,
            },
        }
    }
}
