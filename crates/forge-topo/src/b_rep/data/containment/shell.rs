//! Data shape for the Shell entity and its classification types.
//!
//! DOMAIN: A maximal connected subset of faces bounding a region.

use serde::{Deserialize, Serialize};

use crate::handles::{FaceId, RegionId};

/// Orientation of a shell within a solid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShellOrientation {
    /// Material-enclosing shell (outer boundary of a solid).
    Outer,
    /// Void-enclosing shell (inner boundary — a cavity).
    Inner,
}

/// Classification of a shell's topological character.
///
/// # Operator Contract
///
/// Operators that change a shell's topological character (e.g., sealing
/// an open sheet into a closed solid) **MUST** update this field via
/// [`ShellData::set_kind()`]. The validation policy system reads this at
/// draft creation to determine which invariant groups are applicable.
///
/// Debug builds verify consistency at commit time via
/// `verify_shell_kind_matches_structure()`. A mismatch will
/// `debug_assert!` with a descriptive message naming the shell and
/// the offending boundary edge.
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
