//! Mesh entity data shapes.
//!
//! DOMAIN: Face, HalfEdge, Vertex, Loop, and Edge schemas.

pub(crate) mod boundary_loop;
pub(crate) mod classification;
pub(crate) mod coedge_info;
pub(crate) mod edge;
pub(crate) mod face;
pub(crate) mod face_loops;
pub(crate) mod half_edge;
pub(crate) mod vertex;

pub(crate) use boundary_loop::LoopData;
pub(crate) use classification::{EdgeRadialClass, VertexDiskClass};
pub(crate) use coedge_info::CoedgeInfo;
pub(crate) use edge::EdgeData;
pub(crate) use face::FaceData;
pub(crate) use face_loops::FaceLoops;
pub(crate) use half_edge::HalfEdgeData;
pub(crate) use vertex::VertexData;
