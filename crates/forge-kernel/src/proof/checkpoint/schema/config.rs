use serde::{Deserialize, Serialize};

pub use forge_core::ValidationCheckpoint;

/// Configuration for the invariant checkpoint system.
///
/// Controls which checkpoints are enabled, whether geometric checks
/// are included (more expensive), and a performance safety valve
/// that skips validation on large models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Which checkpoints are enabled.
    pub(crate) checkpoints: Vec<ValidationCheckpoint>,
    /// Whether to include geometric invariants (zero-area, zero-length, signed volume).
    /// More expensive than structural-only validation.
    pub(crate) include_geometric: bool,
    /// Maximum entities before skipping (perf safety valve).
    /// A value of 0 means no limit (always validate).
    pub(crate) entity_limit: usize,
}

impl ValidationConfig {
    /// Debug-mode default: all checkpoints active, geometric checks on, no entity limit.
    pub fn debug_default() -> Self {
        Self {
            checkpoints: vec![
                ValidationCheckpoint::PostCommit,
                ValidationCheckpoint::PostBoolean,
                ValidationCheckpoint::PostFeature,
                ValidationCheckpoint::PostImport,
            ],
            include_geometric: true,
            entity_limit: 0,
        }
    }

    /// Release-mode default: PostBoolean + PostImport only, no geometric, 50k entity limit.
    pub fn release_default() -> Self {
        Self {
            checkpoints: vec![
                ValidationCheckpoint::PostBoolean,
                ValidationCheckpoint::PostImport,
            ],
            include_geometric: false,
            entity_limit: 50_000,
        }
    }

    /// All checkpoints active, geometric included, no entity limit.
    pub fn all_active() -> Self {
        Self {
            checkpoints: vec![
                ValidationCheckpoint::PostCommit,
                ValidationCheckpoint::PostBoolean,
                ValidationCheckpoint::PostFeature,
                ValidationCheckpoint::PostImport,
                ValidationCheckpoint::OnDemand,
            ],
            include_geometric: true,
            entity_limit: 0,
        }
    }

    /// No checkpoints active — validation fully disabled.
    pub fn disabled() -> Self {
        Self {
            checkpoints: Vec::new(),
            include_geometric: false,
            entity_limit: 0,
        }
    }

    /// Whether a given checkpoint is active.
    pub fn is_active(&self, checkpoint: ValidationCheckpoint) -> bool {
        self.checkpoints.contains(&checkpoint)
    }

    /// Whether geometric invariant checks are included.
    pub fn get_include_geometric(&self) -> bool {
        self.include_geometric
    }

    /// Set whether geometric invariant checks are included.
    pub fn set_include_geometric(&mut self, value: bool) {
        self.include_geometric = value;
    }

    /// The entity limit (0 = no limit).
    pub fn get_entity_limit(&self) -> usize {
        self.entity_limit
    }

    /// Set the entity limit.
    pub fn set_entity_limit(&mut self, limit: usize) {
        self.entity_limit = limit;
    }

    /// The active checkpoints.
    pub fn get_checkpoints(&self) -> &[ValidationCheckpoint] {
        &self.checkpoints
    }

    /// Set the active checkpoints.
    pub fn set_checkpoints(&mut self, checkpoints: Vec<ValidationCheckpoint>) {
        self.checkpoints = checkpoints;
    }

    /// Whether validation should be skipped due to entity count.
    ///
    /// Returns true when entity_limit > 0 AND total_entities >= entity_limit.
    pub fn should_skip_for_entity_count(&self, total_entities: usize) -> bool {
        self.entity_limit > 0 && total_entities >= self.entity_limit
    }
}

impl Default for ValidationConfig {
    /// Default: debug_default in debug builds, release_default in release.
    fn default() -> Self {
        if cfg!(debug_assertions) {
            Self::debug_default()
        } else {
            Self::release_default()
        }
    }
}
