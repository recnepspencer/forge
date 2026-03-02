//! PV Suite P0.5b — Mid-Pipeline Diagnostic Tests
//!
//! Tests the pipeline diagnostic infrastructure:
//! - diagnose_arena produces correct diagnostics for valid and broken arenas
//! - PipelineDiagnostic reports unpaired twins accurately
//! - run_checkpoint with position_fn enables geometric checks

use super::checkpoint::{run_checkpoint, ValidationCheckpoint, ValidationConfig};
use super::diagnose_pipeline::{diagnose_arena, PipelineStage};
use crate::operations::primitives::make_cube;
use forge_core::FlatToleranceProvider;

/// Valid cube arena produces a healthy diagnostic.
#[test]
fn diagnose_valid_cube_all_pass() {
    let result = make_cube([0.0, 0.0, 0.0], 1.0).unwrap();
    let (topo, _geom) = result.into_parts();

    let diag = diagnose_arena(topo.arena(), PipelineStage::PostCopy);

    assert!(diag.structural_ok());
    assert!(diag.structural_errors().is_empty());
    assert_eq!(diag.unpaired_twins(), 0);
    assert!(diag.is_healthy());
    assert_eq!(diag.face_count(), 6);
    assert!(diag.half_edge_count() > 0);
    assert!(diag.vertex_count() > 0);
}

/// Arena with manually broken twin pointers is detected.
#[test]
fn diagnose_broken_twins_detected() {
    let result = make_cube([0.0, 0.0, 0.0], 1.0).unwrap();
    let (topo, _geom) = result.into_parts();

    let mut draft = topo.into_mutation();

    let first_he = draft
        .arena()
        .iter_half_edges()
        .next()
        .map(|(id, _)| id)
        .expect("cube should have halfedges");

    draft
        .arena_mut()
        .get_half_edge_mut(first_he)
        .unwrap()
        .set_radial_next(first_he);

    let diag = diagnose_arena(draft.arena(), PipelineStage::PostStitch);

    assert!(!diag.is_healthy());
    assert!(diag.unpaired_twins() >= 1);
}

/// run_checkpoint with a real position_fn reports included_geometric = true.
#[test]
fn checkpoint_with_position_fn() {
    let result = make_cube([0.0, 0.0, 0.0], 1.0).unwrap();
    let (topo, geom) = result.into_parts();

    let config = ValidationConfig::debug_default();
    let pos_fn = |vid| geom.get_vertex_position(vid).copied();
    let flat = FlatToleranceProvider::new(1e-10);
    let vr = run_checkpoint(
        topo.arena(),
        &config,
        ValidationCheckpoint::PostBoolean,
        Some(&pos_fn),
        &flat,
    )
    .unwrap();

    assert!(vr.is_passed());
    assert!(vr.included_geometric());
}

/// PipelineDiagnostic Display output contains stage and status.
#[test]
fn diagnostic_display_format() {
    let result = make_cube([0.0, 0.0, 0.0], 1.0).unwrap();
    let (topo, _geom) = result.into_parts();

    let diag = diagnose_arena(topo.arena(), PipelineStage::PostCopy);
    let display = format!("{diag}");

    assert!(display.contains("PostCopy"));
    assert!(display.contains("HEALTHY"));
    assert!(display.contains("unpaired=0"));
}
