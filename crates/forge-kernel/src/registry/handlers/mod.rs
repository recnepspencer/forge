//! Per-command handler functions for the registry dispatcher.
//!
//! DOMAIN: Each handler translates a `Command` variant into a `NativeFeature`.
//! Handlers are pure functions — no side effects, no tree access.
//! Entity reference resolution stays in `CommandDispatcher`.
//!
//! INVARIANTS:
//! - Handlers contain zero construction logic (no coordinate math)
//! - Handlers return `NativeFeature` or `KernelError`

pub mod add_block;
pub mod boolean;
