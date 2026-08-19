use crate::logic::planner::StageExecutor;
use crate::runtime_policy::SignalRuntimePolicy;

/// Recommended deployment presets for common workload shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalDeploymentPreset {
    /// Request-driven or UI-driven workloads where cheap operational mode matters most.
    WebDevelopment,
    /// Frame-style or editor-style recomputation with lightweight observability.
    GameEngine,
    /// Audit/replay-heavy workloads where richer retained artifacts are worth the cost.
    Fintech,
    /// Heavy investigative or kernel-style workloads with maximal retained detail.
    Kernel,
}

/// Recommended runtime policy and executor pairing for a deployment preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalDeploymentPlan {
    pub preset: SignalDeploymentPreset,
    pub runtime_policy: SignalRuntimePolicy,
    pub executor: StageExecutor,
    pub summary: &'static str,
    pub certification_command: &'static str,
}

impl SignalDeploymentPreset {
    /// Return the recommended runtime policy, executor, summary, and local
    /// certification command for this deployment shape.
    pub fn recommended(self) -> SignalDeploymentPlan {
        match self {
            Self::WebDevelopment => SignalDeploymentPlan {
                preset: self,
                runtime_policy: SignalRuntimePolicy::web_development(),
                executor: recommended_parallel_executor(Self::WebDevelopment),
                summary: "Low-overhead operational policy with conservative parallelism for request/interaction-driven workloads.",
                certification_command:
                    "bash scripts/ci/run_signal_local_certification.sh web",
            },
            Self::GameEngine => SignalDeploymentPlan {
                preset: self,
                runtime_policy: SignalRuntimePolicy::game_engine(),
                executor: recommended_parallel_executor(Self::GameEngine),
                summary:
                    "Operational-first policy with earlier staged parallelism for frame-style recomputation.",
                certification_command:
                    "bash scripts/ci/run_signal_local_certification.sh game-engine",
            },
            Self::Fintech => SignalDeploymentPlan {
                preset: self,
                runtime_policy: SignalRuntimePolicy::fintech(),
                executor: recommended_parallel_executor(Self::Fintech),
                summary:
                    "Development-rich policy with stronger replay detail and conservative deterministic parallel admission.",
                certification_command:
                    "bash scripts/ci/run_signal_local_certification.sh fintech",
            },
            Self::Kernel => SignalDeploymentPlan {
                preset: self,
                runtime_policy: SignalRuntimePolicy::kernel(),
                executor: recommended_parallel_executor(Self::Kernel),
                summary:
                    "Forensic-rich policy with extended observability and conservative full-parallel admission for heavy compute kernels.",
                certification_command:
                    "bash scripts/ci/run_signal_local_certification.sh kernel",
            },
        }
    }
}

#[cfg(feature = "parallel")]
fn recommended_parallel_executor(preset: SignalDeploymentPreset) -> StageExecutor {
    match preset {
        SignalDeploymentPreset::WebDevelopment => StageExecutor::conservative_parallel(),
        SignalDeploymentPreset::GameEngine => StageExecutor::aggressive_parallel(),
        SignalDeploymentPreset::Fintech => StageExecutor::balanced_parallel(),
        SignalDeploymentPreset::Kernel => StageExecutor::conservative_parallel(),
    }
}

#[cfg(not(feature = "parallel"))]
fn recommended_parallel_executor(_preset: SignalDeploymentPreset) -> StageExecutor {
    StageExecutor::Serial
}
