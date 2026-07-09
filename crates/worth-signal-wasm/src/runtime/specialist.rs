use serde::Serialize;

use crate::runtime::summaries::AspectVersionSummary;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionSummary {
    pub id: String,
    pub version: u64,
    pub aspect_versions: Vec<AspectVersionSummary>,
}
