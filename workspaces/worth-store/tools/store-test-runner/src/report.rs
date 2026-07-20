use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TestRunReport {
    pub(crate) product: String,
    pub(crate) revision: Option<String>,
    pub(crate) elapsed_ms: u128,
    pub(crate) success: bool,
    pub(crate) failure: Option<String>,
    pub(crate) units: Vec<UnitResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UnitResult {
    pub(crate) identity: String,
    pub(crate) command: String,
    pub(crate) elapsed_ms: u128,
    pub(crate) success: bool,
}

pub(crate) fn write(path: &Path, report: &TestRunReport) -> Result<(), String> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }
    let encoded = serde_json::to_vec_pretty(report)
        .map_err(|error| format!("failed to encode report: {error}"))?;
    fs::write(path, encoded).map_err(|error| format!("failed to write {}: {error}", path.display()))
}
