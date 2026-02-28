//! Euler operator test suite.
//!
//! STRUCTURE: Each file tests exactly one operator or one concern.
//! No test logic lives here — this mod.rs is strictly a table of contents.
//!
//! FILE MAP:
//! - helpers        : shared test utilities (logged_op)
//! - mvf_tests      : MakeVertexFace — seed creation + lineage
//! - mef_tests      : MakeEdgeFace — face splitting
//! - mev_tests      : MakeEdgeVertex — vertex extension (wire edges)
//! - mekl_keml_tests: MakeEdgeKillLoop + KillEdgeMakeLoop — loop merge/split
//! - split_edge_tests : SplitEdge — degenerate and normal cases
//! - join_faces_tests : JoinFaces — face merge + inner loop preservation
//! - kill_edge_vertex_tests : KillEdgeVertex — vertex collapse
//! - lineage_tests  : D1 determinism + ancestry derivation
//! - integration_tests : multi-operator sequences, traversal, validation guard
//! - shell_edge_tests : Shell/Edge entity lifecycle and referential integrity
//! - brutality_tests : domain-specific stress tests (high-valence, sliver churn, DAG)

mod helpers;

mod brutality_tests;
mod integration_tests;
pub(crate) mod invariant_checker;
mod join_faces_nmt_tests;
mod join_faces_tests;
mod kill_edge_vertex_tests;
mod lineage_tests;
mod mef_tests;
mod mekl_keml_tests;
mod mev_tests;
mod mffv_tests;
mod mfis_tests;
mod mlifv_tests;
mod mvf_tests;
mod sew_edge_tests;
mod shell_edge_tests;
mod split_edge_tests;
