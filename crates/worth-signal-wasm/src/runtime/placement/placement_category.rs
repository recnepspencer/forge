use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WorkerPlacementCategory {
    WorkerExecutable,
    MainThreadHosted,
    Unavailable,
}
