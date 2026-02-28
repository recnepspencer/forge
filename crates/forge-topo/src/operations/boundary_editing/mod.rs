//! Boundary editing operators — loop wiring, face merging, ring/hole surgery.
//!
//! DOMAIN: Operators that modify face boundaries, merge/split loops,
//! and promote/demote holes. Includes MEKL, KEML, MFKRH, KFMRH,
//! JoinFaces, and compound face/loop builders.

pub mod join_faces;
pub mod join_faces_nmt;
pub mod kill_edge_make_loop;
pub mod kill_face_make_ring_hole;
pub mod make_edge_kill_loop;
pub mod make_face_from_vertices;
pub mod make_face_in_shell_from_vertices;
pub mod make_face_kill_ring_hole;
pub mod make_loop_in_face_from_vertices;
