//! Phase 2 retention-registry hook.
//!
//! Runtime World Phase 1 deliberately does not implement a retention owner.
//! The later retention lane owns the operational map, component-owner lease
//! contact, capacity accounting, dependency counts, and reclamation.  This
//! opaque receipt remains in shared consumer signatures so that lane can add
//! those semantics without changing the publication/recovery contract.

/// Opaque, move-only evidence issued by the future retention owner.
#[derive(Debug)]
pub(crate) struct RetentionTransferReceipt {
    _sealed: (),
}
