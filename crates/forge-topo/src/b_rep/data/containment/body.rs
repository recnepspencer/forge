//! Data shape for the Body entity.
//!
//! DOMAIN: The top-level solid container in the containment hierarchy.

use serde::{Deserialize, Serialize};

use crate::handles::LumpId;

/// Data stored for each solid — the top-level topology container.
///
/// A solid owns one or more lumps. Each lump is a connected
/// component of material within this body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyData {
    lumps: Vec<LumpId>,
}

impl BodyData {
    /// Construct a new empty solid.
    pub fn new() -> Self {
        Self { lumps: Vec::new() }
    }

    /// The lumps belonging to this solid.
    pub fn lumps(&self) -> &[LumpId] {
        &self.lumps
    }

    /// Add a lump to this solid.
    pub fn add_lump(&mut self, id: LumpId) {
        self.lumps.push(id);
    }

    /// Remove a lump from this solid.
    ///
    /// Returns `true` if the lump was found and removed, `false` otherwise.
    pub fn remove_lump(&mut self, id: LumpId) -> bool {
        if let Some(pos) = self.lumps.iter().position(|&l| l == id) {
            self.lumps.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// Number of lumps in this solid.
    pub fn lump_count(&self) -> usize {
        self.lumps.len()
    }
}
