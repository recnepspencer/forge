use serde::{Deserialize, Serialize};

use forge_signal::facade::RuntimePolicy as NativeRuntimePolicy;

use crate::boundary::errors::ForgeSignalJsError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimePolicySpec {
    pub preset: RuntimePolicyPreset,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RuntimePolicyPreset {
    Development,
    Operational,
    Forensic,
    WebDevelopment,
    Fintech,
    Kernel,
    GameEngine,
}

impl Default for RuntimePolicySpec {
    fn default() -> Self {
        Self {
            preset: RuntimePolicyPreset::WebDevelopment,
        }
    }
}

impl RuntimePolicySpec {
    pub fn into_native(self) -> Result<NativeRuntimePolicy, ForgeSignalJsError> {
        let policy = match self.preset {
            RuntimePolicyPreset::Development => NativeRuntimePolicy::development(),
            RuntimePolicyPreset::Operational => NativeRuntimePolicy::operational(),
            RuntimePolicyPreset::Forensic => NativeRuntimePolicy::forensic(),
            RuntimePolicyPreset::WebDevelopment => NativeRuntimePolicy::web_development(),
            RuntimePolicyPreset::Fintech => NativeRuntimePolicy::fintech(),
            RuntimePolicyPreset::Kernel => NativeRuntimePolicy::kernel(),
            RuntimePolicyPreset::GameEngine => NativeRuntimePolicy::game_engine(),
        };
        Ok(policy)
    }
}
