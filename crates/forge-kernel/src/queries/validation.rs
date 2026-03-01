//! Validation query traits.
//!
//! DOMAIN: Named interface for validation checkpoint configuration.
//! Used by the pipeline executor and boolean assemble to check whether
//! post-feature or post-step validation is active.

use crate::proof::checkpoint::schema::ValidationConfig;
use crate::context::facade::ModelingContext;

/// Validation checkpoint configuration access.
///
/// Used by: pipeline executor (post-invariant checks), boolean merge (assembly validation).
pub trait ValidationQuery {
    /// Get the current validation checkpoint configuration.
    fn validation_config(&self) -> ValidationConfig;
}

impl ValidationQuery for ModelingContext {
    fn validation_config(&self) -> ValidationConfig {
        self.get_validation_config()
    }
}
