//! Data shapes for containment entities: Body, Lump, Region, Shell.
//!
//! DOMAIN: Defines the per-entity data structs for the ownership
//! hierarchy (Body → Lump → Region → Shell).
//!
//! DEPENDENCIES: `handles` (typed IDs)

use serde::{Deserialize, Serialize};

use crate::handles::{
    BodyId, FaceId, LumpId, RegionId, ShellId,
};


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
        Self {
            lumps: Vec::new(),
        }
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

/// Orientation of a shell within a solid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShellOrientation {
    /// Material-enclosing shell (outer boundary of a solid).
    Outer,
    /// Void-enclosing shell (inner boundary — a cavity).
    Inner,
}

/// Classification of a shell's topological character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShellKind {
    /// Closed watertight shell (every edge has exactly 2 incident faces).
    Solid(ShellOrientation),
    /// Open shell with boundary edges (car body panels, sheet metal).
    Sheet,
    /// Wire body: edges and vertices only, no faces.
    Wire,
}

/// Data stored for each shell — a maximal connected subset of faces.
///
/// Solid shells bound material or voids (cavities). Sheet shells are
/// open surfaces with boundary edges. Wire shells have only edges/vertices.
/// Shell membership is tracked via `FaceData::shell`. The representative
/// face provides a traversal entry point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellData {
    representative_face: FaceId,
    kind: ShellKind,
    region: RegionId,
}

impl ShellData {
    /// Construct a new shell with the given representative face and parent region.
    pub fn new(representative_face: FaceId, kind: ShellKind, region: RegionId) -> Self {
        Self {
            representative_face,
            kind,
            region,
        }
    }

    /// One representative face (entry point for shell traversal).
    pub fn representative_face(&self) -> FaceId {
        self.representative_face
    }

    /// Shell kind (solid, sheet, or wire).
    pub fn kind(&self) -> ShellKind {
        self.kind
    }

    /// Shell orientation for solid shells, `None` for sheet/wire.
    pub fn orientation(&self) -> Option<ShellOrientation> {
        match self.kind {
            ShellKind::Solid(o) => Some(o),
            _ => None,
        }
    }

    /// The region this shell belongs to.
    pub fn region(&self) -> RegionId {
        self.region
    }

    /// Set the representative face.
    pub fn set_representative_face(&mut self, id: FaceId) {
        self.representative_face = id;
    }

    /// Set the shell kind.
    pub fn set_kind(&mut self, kind: ShellKind) {
        self.kind = kind;
    }

    /// Set the region this shell belongs to.
    pub fn set_region(&mut self, id: RegionId) {
        self.region = id;
    }
}
