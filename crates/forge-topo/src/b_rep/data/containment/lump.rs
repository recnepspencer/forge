//! Data shape for the Lump entity.
//!
//! DOMAIN: A connected component of material within a body.

use serde::{Deserialize, Serialize};

use crate::handles::{BodyId, RegionId};

/// Data stored for each lump — a connected component of material.
///
/// A lump contains one or more regions. Disconnected boolean results
/// produce multiple lumps within a single body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LumpData {
    regions: Vec<RegionId>,
    body: BodyId,
}

impl LumpData {
    /// Construct a new lump with its parent body.
    pub fn new(body: BodyId) -> Self {
        Self {
            regions: Vec::new(),
            body,
        }
    }

    /// The regions belonging to this lump.
    pub fn regions(&self) -> &[RegionId] {
        &self.regions
    }

    /// Add a region to this lump.
    pub fn add_region(&mut self, id: RegionId) {
        self.regions.push(id);
    }

    /// Remove a region from this lump.
    pub fn remove_region(&mut self, id: RegionId) -> bool {
        if let Some(pos) = self.regions.iter().position(|&r| r == id) {
            self.regions.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// Number of regions in this lump.
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// The body this lump belongs to.
    pub fn body(&self) -> BodyId {
        self.body
    }

    /// Set the parent body.
    pub fn set_body(&mut self, id: BodyId) {
        self.body = id;
    }
}
