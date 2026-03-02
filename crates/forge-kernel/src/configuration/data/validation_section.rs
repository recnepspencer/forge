//! Validation section of the unified configuration.
//!
//! DOMAIN: Invariant validation checkpoints and settings.

use forge_core::{KernelError, ValidationCheckpoint};
use serde::{Deserialize, Serialize};

use super::kernel_config::ConfigSection;

/// Invariant validation checkpoints and settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSection {
    pub checkpoints: Vec<ValidationCheckpoint>,
    pub include_geometric: bool,
    pub entity_limit: usize,
}

impl ConfigSection for ValidationSection {
    fn defaults() -> Self {
        if cfg!(debug_assertions) {
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
        } else {
            Self {
                checkpoints: vec![
                    ValidationCheckpoint::PostBoolean,
                    ValidationCheckpoint::PostImport,
                ],
                include_geometric: false,
                entity_limit: 50_000,
            }
        }
    }

    fn validate(&self) -> Result<(), KernelError> {
        Ok(())
    }
}

impl Default for ValidationSection {
    fn default() -> Self {
        Self::defaults()
    }
}
