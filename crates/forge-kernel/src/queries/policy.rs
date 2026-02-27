//! Policy query traits.
//!
//! DOMAIN: Named interface for policy resolution. Features that need
//! runtime policy decisions query this trait instead of calling
//! `ModelingContext::resolve_policy_query` directly.

use forge_core::{KernelError, PolicyKind};
use crate::core::ModelingContext;

/// Policy pre-validation and resolution.
///
/// Used by: pipeline executor (policy pre-check), boolean NMT merge (coplanar policy).
pub trait PolicyQuery {
    /// Validate that a required policy is configured (fail-fast).
    fn validate_policy_configured(&self, kind: &PolicyKind) -> Result<(), KernelError>;
}

impl PolicyQuery for ModelingContext {
    fn validate_policy_configured(&self, kind: &PolicyKind) -> Result<(), KernelError> {
        self.validate_policy_configured(kind)
    }
}
