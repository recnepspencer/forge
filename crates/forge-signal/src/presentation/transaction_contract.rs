//! Public transaction contract for `forge-signal`.
//!
//! Normative semantics:
//! - Graph writes occur in-place during transaction execution.
//! - First touch captures the original `NodeEntry` in the sparse undo-log.
//! - `commit()` is a finalize attempt, not the visibility boundary for graph writes.
//! - If `commit()` fails, graph state is hard-rewound before the error returns.
//! - `rollback()` always hard-rewinds graph state and discards staged non-graph outputs.
//! - No overlay-read semantics are part of this crate's transaction model.
//! - Node-scale execution metadata is arena-aligned and generation-safe.
//! - Shared traversal scratch is non-reentrant; nested traversal attempts fail deterministically.

/// Contract marker for the in-place mutation plus hard-rewind transaction model.
#[derive(Debug, Clone, Copy, Default)]
pub struct TransactionRuntimeContract;
