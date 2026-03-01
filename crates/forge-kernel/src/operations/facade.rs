//! Public API for modeling operations.
//!
//! External components should import operation entrypoints and DTOs from this
//! facade, not from deep operation internals.

pub use super::boolean::{
    execute_boolean, execute_boolean_direct, BooleanInput, BooleanIntrospection, BooleanOp,
    BooleanResult,
};
