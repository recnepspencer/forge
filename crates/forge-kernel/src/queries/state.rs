//! Kernel state and operation space query traits.
//!
//! DOMAIN: Named interfaces for accessing the operation coordinate frame
//! and transactional kernel state bundles.

use crate::context::facade::ModelingContext;
use crate::finalization::facade::OperationSpace;

/// Operation space (local coordinate frame) access.
///
/// Used by: pipeline executor (set up frame), features (transform inputs).
pub trait OperationSpaceQuery {
    /// Get the current operation space.
    fn operation_space(&self) -> &OperationSpace;
}

impl OperationSpaceQuery for ModelingContext {
    fn operation_space(&self) -> &OperationSpace {
        self.get_operation_space()
    }
}
