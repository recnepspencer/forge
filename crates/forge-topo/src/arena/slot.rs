//! Generational slot and handle validation helpers.
//!
//! DOMAIN: Low-level arena infrastructure — the Slot wrapper and
//! error-path functions for stale/deleted/out-of-bounds handles.

use serde::{Deserialize, Serialize};
use forge_core::{KernelError, TopologyError, ErrorContext, ErrorScope};

/// A slot in the arena that may be occupied or vacant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Slot<T: Clone> {
    /// The current generation of this slot.
    pub(crate) generation: u32,
    /// The current version of the data in this slot (increments on mutation).
    pub(crate) version: u32,
    /// The data, if the slot is occupied.
    pub(crate) data: Option<T>,
    /// Next vacant slot in the arena free-list.
    #[serde(default)]
    pub(crate) next_free: Option<u32>,
}

impl<T: Clone> Slot<T> {
    /// Create a new empty slot at generation 0.
    pub(crate) fn empty() -> Self {
        Self {
            generation: 0,
            version: 0,
            data: None,
            next_free: None,
        }
    }

    /// Occupy this slot with data, returning the current generation.
    /// Resets version to 0.
    pub(crate) fn occupy(&mut self, data: T) -> u32 {
        self.data = Some(data);
        self.version = 0;
        self.next_free = None;
        self.generation
    }
}

/// Validate that a handle's generation matches the slot's generation.
#[inline]
pub(crate) fn validate_generation(
    slot_gen: u32,
    handle_gen: u32,
    entity_type: &str,
    index: u32,
) -> Result<(), KernelError> {
    if slot_gen != handle_gen {
        return Err(cold_err_stale(entity_type, index, handle_gen, slot_gen));
    }
    Ok(())
}

#[inline(never)]
pub(crate) fn cold_err_bounds(kind: &str, idx: u32, gen: u32) -> KernelError {
    KernelError::TopologyViolation {
        err: TopologyError::StaleHandle {
            entity_kind: kind.to_string(),
            index: idx,
            expected_generation: gen,
            actual_generation: 0,
        },
        context: Some(ErrorContext {
            scope: ErrorScope::Entity { entity_kind: kind.to_string(), index: idx },
            suggested_fixes: Vec::new(),
            detail: format!("{} index {} out of bounds", kind, idx),
        }),
    }
}

#[cold]
#[inline(never)]
pub(crate) fn cold_err_stale(kind: &str, idx: u32, expected: u32, actual: u32) -> KernelError {
    KernelError::TopologyViolation {
        err: TopologyError::StaleHandle {
            entity_kind: kind.to_string(),
            index: idx,
            expected_generation: expected,
            actual_generation: actual,
        },
        context: Some(ErrorContext {
            scope: ErrorScope::Entity { entity_kind: kind.to_string(), index: idx },
            suggested_fixes: Vec::new(),
            detail: format!("Stale {} handle at index {} (expected gen {}, got gen {})", kind, idx, expected, actual),
        }),
    }
}

#[cold]
#[inline(never)]
pub(crate) fn cold_err_deleted(kind: &str, idx: u32, expected_gen: u32, actual_gen: u32) -> KernelError {
    KernelError::TopologyViolation {
        err: TopologyError::StaleHandle {
            entity_kind: kind.to_string(),
            index: idx,
            expected_generation: expected_gen,
            actual_generation: actual_gen,
        },
        context: Some(ErrorContext {
            scope: ErrorScope::Entity { entity_kind: kind.to_string(), index: idx },
            suggested_fixes: Vec::new(),
            detail: format!("{} {} has been deleted", kind, idx),
        }),
    }
}
