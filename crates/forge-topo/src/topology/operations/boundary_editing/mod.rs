//! Operations that modify the boundary of faces without changing the number of entities.

pub mod join_faces;
pub mod join_faces_nmt;
pub mod kill_edge_make_loop;
pub mod make_edge_kill_loop;
pub mod make_face_from_vertices;
pub mod make_face_in_shell_from_vertices;
pub mod make_loop_in_face_from_vertices;

pub use join_faces::*;
pub use join_faces_nmt::*;
pub use kill_edge_make_loop::*;
pub use make_edge_kill_loop::*;
pub use make_face_from_vertices::*;
pub use make_face_in_shell_from_vertices::*;
pub use make_loop_in_face_from_vertices::*;
