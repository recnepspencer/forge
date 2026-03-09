use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RelationalRuntimeProfile {
    CertificationCore,
    GeometryKernel,
    ChipSimulation,
    AiWorkflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConfigValueSource {
    ProfileDefault,
    BuilderOverride,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigProvenanceEntry {
    pub source: ConfigValueSource,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigProvenance {
    pub profile: RelationalRuntimeProfile,
    pub entries: BTreeMap<String, ConfigProvenanceEntry>,
}

impl ConfigProvenance {
    pub fn source_for(&self, key: &str) -> Option<&ConfigProvenanceEntry> {
        self.entries.get(key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotReleasePolicy {
    ExplicitRelease,
    ReleaseOnRetentionPass,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MvccConfig {
    pub track_visibility_metadata: bool,
    pub snapshot_release_policy: SnapshotReleasePolicy,
    pub auto_reclaim_deleted_records: bool,
    pub reclaim_batch_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageLayoutConfig {
    pub entity_chunk_size: usize,
    pub relation_chunk_size: usize,
    pub scan_packet_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationConfig {
    pub coherent_publication_required: bool,
    pub max_patch_records_per_commit: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalConfigOverride {
    pub runtime_name: Option<String>,
    pub initial_entity_capacity: Option<usize>,
    pub initial_relation_capacity: Option<usize>,
    pub mvcc: Option<MvccConfig>,
    pub storage_layout: Option<StorageLayoutConfig>,
    pub publication: Option<PublicationConfig>,
}

impl RelationalConfigOverride {
    pub fn is_empty(&self) -> bool {
        self.runtime_name.is_none()
            && self.initial_entity_capacity.is_none()
            && self.initial_relation_capacity.is_none()
            && self.mvcc.is_none()
            && self.storage_layout.is_none()
            && self.publication.is_none()
    }
}

impl Default for RelationalConfigOverride {
    fn default() -> Self {
        Self {
            runtime_name: None,
            initial_entity_capacity: None,
            initial_relation_capacity: None,
            mvcc: None,
            storage_layout: None,
            publication: None,
        }
    }
}
