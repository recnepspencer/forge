//! Phase 1 observability & causality integration tests.
//!
//! DOMAIN: Tests that validate the tracing, lineage, pipeline, and
//! serialization infrastructure works end-to-end through real operations.
//!
//! These are acceptance gates — Phase 1 ships when every non-ignored test passes.
//!
//! INVARIANTS:
//! - Tests assert on structural properties (counts, kinds, presence) —
//!   never on specific DecisionId values or Vec indices.
//! - All tests use ModelingContext (production DecisionSink), not TestSink.

mod decision_sink;
mod lineage;
mod pipeline;
mod serialization;
