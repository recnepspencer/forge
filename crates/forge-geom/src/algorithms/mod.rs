//! Algorithms module — composed geometry algorithms.

pub mod chord;

pub use chord::{compute_intersection_line, clip_line_to_face_polygon};
