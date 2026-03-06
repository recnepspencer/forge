//! Bundled coedge metadata for the halfedge side-car.
//!
//! DOMAIN: Associates a halfedge with its parametric trim curve (coedge)
//! and direction sense relative to the parent Edge's 3D curve.

use crate::handles::CoedgeRef;
use serde::{Deserialize, Serialize};

/// Bundled coedge metadata (UV trim curve reference + direction sense).
///
/// Stored in a slot-parallel side-car vector on `TopologyArena`, indexed
/// by the halfedge's slot position. `None` entries indicate planar
/// halfedges where no coedge geometry exists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoedgeInfo {
    /// Opaque reference to this halfedge's UV trim curve in the `GeometryStore`.
    pub coedge_ref: CoedgeRef,
    /// Whether this coedge's parametric direction is aligned with the parent
    /// Edge's 3D curve direction. `true` = same direction, `false` = reversed.
    /// This is the "sense" in STEP terminology (`ORIENTED_EDGE.orientation`).
    pub direction: bool,
}
