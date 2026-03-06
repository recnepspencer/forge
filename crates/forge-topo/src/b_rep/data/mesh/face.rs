//! Data shape for the Face entity.
//!
//! DOMAIN: A face is a bounded planar or curved surface in the B-Rep.

use serde::{Deserialize, Serialize};

use crate::b_rep::data::mesh::FaceLoops;
use crate::handles::{LoopId, ShellId, SurfaceRef};

/// Data stored for each face.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    pub loops: FaceLoops,
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
            loops: FaceLoops::new(outer_loop),
            shell,
            surface: None,
        }
    }

    /// The shell this face belongs to.
    pub fn shell(&self) -> ShellId {
        self.shell
    }

    /// Set the shell this face belongs to.
    pub fn set_shell(&mut self, id: ShellId) {
        self.shell = id;
    }

    /// Number of inner loops (rings) on this face.
    pub fn inner_loop_count(&self) -> usize {
        self.loops.inners().len()
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
