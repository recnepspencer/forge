//! BSP evaluation facade.
//!
//! Construction and clipping are separate algorithm phases. This module keeps
//! the existing BSP-facing path stable by declaring those children and
//! explicitly reexporting their three established capabilities.

mod clipping;
mod construction;

pub use clipping::clip_cell_by_plane;
pub use construction::{build_convex_polyhedron, BspConfig};
