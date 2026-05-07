use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkerFallbackPolicy {
    DenyByDefault,
    ProductDeclaredFallbackOnly,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerFallbackPolicySummary {
    pub label: &'static str,
    pub hidden_fallback_allowed: bool,
    pub denial_artifact_required: bool,
}

pub(in crate::runtime::worker_bridge) fn worker_fallback_policies(
) -> Vec<WorkerFallbackPolicySummary> {
    [
        WorkerFallbackPolicy::DenyByDefault,
        WorkerFallbackPolicy::ProductDeclaredFallbackOnly,
    ]
    .into_iter()
    .map(WorkerFallbackPolicy::summary)
    .collect()
}

impl WorkerFallbackPolicy {
    fn summary(self) -> WorkerFallbackPolicySummary {
        match self {
            Self::DenyByDefault => WorkerFallbackPolicySummary {
                label: "denyByDefault",
                hidden_fallback_allowed: false,
                denial_artifact_required: true,
            },
            Self::ProductDeclaredFallbackOnly => WorkerFallbackPolicySummary {
                label: "productDeclaredFallbackOnly",
                hidden_fallback_allowed: false,
                denial_artifact_required: true,
            },
        }
    }
}
