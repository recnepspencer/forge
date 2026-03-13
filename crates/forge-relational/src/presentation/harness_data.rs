use std::fmt;

use serde::{Deserialize, Serialize};

use crate::facade::identity::KindId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalHarnessExpectations {
    pub execution_mode: forge_harness::facade::ExecutionMode,
    pub diagnostics_level: forge_harness::facade::DiagnosticsLevel,
    pub capture_depth: forge_harness::facade::CaptureDepth,
    pub serial_parallel_parity_required: bool,
}

impl Default for RelationalHarnessExpectations {
    fn default() -> Self {
        Self {
            execution_mode: forge_harness::facade::ExecutionMode::Serial,
            diagnostics_level: forge_harness::facade::DiagnosticsLevel::Forensic,
            capture_depth: forge_harness::facade::CaptureDepth::Rich,
            serial_parallel_parity_required: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalHarnessPlan {
    pub adapter_name: String,
    pub expectations: RelationalHarnessExpectations,
    pub required_seeders: Vec<String>,
}

impl RelationalHarnessPlan {
    pub fn relational() -> Self {
        Self {
            adapter_name: "forge-relational".to_string(),
            expectations: RelationalHarnessExpectations::default(),
            required_seeders: vec![
                "branch-history".to_string(),
                "replay-parity".to_string(),
                "diff-cdc".to_string(),
                "serialized-authority".to_string(),
                "cross-order-intents".to_string(),
            ],
        }
    }
}

pub fn default_harness_expectations() -> RelationalHarnessExpectations {
    RelationalHarnessExpectations::default()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalFixture {
    pub entities: Vec<FixtureEntity>,
    pub relations: Vec<FixtureRelation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureEntity {
    pub kind_id: KindId,
    pub client_key: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureRelation {
    pub kind_id: KindId,
    pub client_key: String,
    pub source_slot: u64,
    pub target_slot: u64,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalHarnessAdapter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalHarnessError(pub String);

impl fmt::Display for RelationalHarnessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for RelationalHarnessError {}
