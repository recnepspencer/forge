//! Data shape for the Face entity.
//!
//! DOMAIN: A face is a bounded planar or curved surface in the B-Rep.

use serde::{Deserialize, Serialize};

use crate::handles::{LoopId, ShellId, SurfaceRef};

/// Data stored for each face.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    outer_loop: LoopId,
    inner_loops: Vec<LoopId>,
    shell: ShellId,
    /// Opaque reference to this face's parametric surface in the `GeometryStore`.
    /// `None` for planar faces (the surface is an implicit plane defined by
    /// the face-plane association). `Some` for curved surfaces (Phase 4+).
    surface: Option<SurfaceRef>,
}

impl FaceData {
    /// Construct a new face with the given outer loop and shell.
    pub fn new(outer_loop: LoopId, shell: ShellId) -> Self {
        Self {
            outer_loop,
            inner_loops: Vec::new(),
            shell,
            surface: None,
        }
    }

    /// The outer boundary loop of this face.
    pub fn outer_loop(&self) -> LoopId {
        self.outer_loop
    }

    /// The shell this face belongs to.
    pub fn shell(&self) -> ShellId {
        self.shell
    }

    /// Set the outer boundary loop.
    pub fn set_outer_loop(&mut self, id: LoopId) {
        self.outer_loop = id;
    }

    /// Set the shell this face belongs to.
    pub fn set_shell(&mut self, id: ShellId) {
        self.shell = id;
    }

    /// Inner loops (holes) on this face.
    pub fn inner_loops(&self) -> &[LoopId] {
        &self.inner_loops
    }

    /// Add an inner loop (hole boundary) to this face.
    pub fn add_inner_loop(&mut self, id: LoopId) {
        self.inner_loops.push(id);
    }

    /// Remove an inner loop from this face.
    ///
    /// Returns `true` if the loop was found and removed, `false` otherwise.
    pub fn remove_inner_loop(&mut self, id: LoopId) -> bool {
        if let Some(pos) = self.inner_loops.iter().position(|&l| l == id) {
            self.inner_loops.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// Number of inner loops (rings) on this face.
    pub fn inner_loop_count(&self) -> usize {
        self.inner_loops.len()
    }

    /// Opaque reference to this face's parametric surface (None = planar).
    pub fn surface_ref(&self) -> Option<SurfaceRef> {
        self.surface
    }

    /// Set the surface reference (populated by the kernel for curved faces).
    pub fn set_surface_ref(&mut self, r: Option<SurfaceRef>) {
        self.surface = r;
    }
}
