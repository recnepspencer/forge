//! Euler Operators — low-level atomic topology mutations.
//!
//! # Purpose
//! Euler operators are the fundamental transactional building blocks of boundary
//! representation (B-Rep). They are the *only* way the topology arena is modified.
//!
//! # Structural Invariants (Euler-Poincaré Formula)
//! Any valid B-Rep model must satisfy the Euler-Poincaré formula:
//!
//! `V - E + F = 2(S - H) + R`
//!
//! Where:
//! - **V**: Vertices
//! - **E**: Edges (geometric curves)
//! - **F**: Faces
//! - **S**: Shells (connected closed surfaces)
//! - **H**: Holes through the solid (genus)
//! - **R**: Ring-like inner loops within faces
//!
//! # Transactional Execution
//! Each operator executes against a `MutableDraft`, which allows it to mutate
//! the arena and append to the transaction history. If an operator fails,
//! the draft is discarded; if it succeeds, the draft records a structurally
//! sound mutation that can be analyzed or committed to the `TopologyState`.
//!
//! # Lineage
//! All Euler operators accept an `OpSignature`, which tags the operation
//! in the geometric log.

// Note: Operators have been refactored into domain-specific subdirectories under `operations/`.
// This module currently serves as a redirection or legacy compatibility layer
// during the refactoring process.
