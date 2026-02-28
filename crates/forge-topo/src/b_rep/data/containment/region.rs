//! Data shape for the Region entity.
//!
//! DOMAIN: A 3D volume bounded by shells within a lump.

use serde::{Deserialize, Serialize};

use crate::handles::{LumpId, ShellId};

/// Data stored for each region — a 3D volume bounded by shells.
///
/// A region contains exactly one outer shell (material boundary) and
/// zero or more inner shells (cavities/voids). This is the topological
/// encoding of a 3-manifold with boundary.
///
/// The outer/inner distinction is enforced at the type level:
/// `set_outer_shell` must be called exactly once, and `add_inner_shell`
/// adds cavity walls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionData {
    outer_shell: Option<ShellId>,
    inner_shells: Vec<ShellId>,
    lump: LumpId,
}

impl RegionData {
    /// Construct a new region with its parent lump.
    ///
    /// The outer shell must be set via `set_outer_shell` before commit.
    pub fn new(lump: LumpId) -> Self {
        Self {
            outer_shell: None,
            inner_shells: Vec::new(),
            lump,
        }
    }

    /// The outer shell bounding this region's material (if set).
    pub fn outer_shell(&self) -> Option<ShellId> {
        self.outer_shell
    }

    /// Inner shells (cavity/void walls) within this region.
    pub fn inner_shells(&self) -> &[ShellId] {
        &self.inner_shells
    }

    /// All shells in this region: outer first (if present), then inner.
    ///
    /// Prefer `outer_shell()` and `inner_shells()` for type-safe access.
    pub fn shells(&self) -> Vec<ShellId> {
        let mut result = Vec::with_capacity(1 + self.inner_shells.len());
        if let Some(outer) = self.outer_shell {
            result.push(outer);
        }
        result.extend_from_slice(&self.inner_shells);
        result
    }

    /// Add a shell to this region. If no outer shell is set, the first
    /// shell added becomes the outer shell. Subsequent shells are inner.
    pub fn add_shell(&mut self, id: ShellId) {
        if self.outer_shell.is_none() {
            self.outer_shell = Some(id);
        } else {
            self.inner_shells.push(id);
        }
    }

    /// Set the outer shell explicitly.
    pub fn set_outer_shell(&mut self, id: ShellId) {
        self.outer_shell = Some(id);
    }

    /// Add an inner shell (cavity wall) to this region.
    pub fn add_inner_shell(&mut self, id: ShellId) {
        self.inner_shells.push(id);
    }

    /// Remove a shell from this region.
    ///
    /// If the shell is the outer shell, the outer is cleared (set to `None`).
    /// If the shell is an inner shell, it is removed from the inner list.
    /// Returns `true` if the shell was found and removed, `false` otherwise.
    pub fn remove_shell(&mut self, id: ShellId) -> bool {
        if self.outer_shell == Some(id) {
            self.outer_shell = None;
            return true;
        }
        if let Some(pos) = self.inner_shells.iter().position(|&s| s == id) {
            self.inner_shells.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// Total number of shells (outer + inner) in this region.
    pub fn shell_count(&self) -> usize {
        (if self.outer_shell.is_some() { 1 } else { 0 }) + self.inner_shells.len()
    }

    /// The lump this region belongs to.
    pub fn lump(&self) -> LumpId {
        self.lump
    }

    /// Set the parent lump.
    pub fn set_lump(&mut self, id: LumpId) {
        self.lump = id;
    }
}
