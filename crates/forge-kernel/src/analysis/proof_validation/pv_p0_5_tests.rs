//! PV Suite P0.5 — Invariant Checkpoint System Tests
//!
//! Tests that the checkpoint system:
//! - PV-13: Correct checkpoint activation, debug/release defaults, OnDemand
//! - PV-14: Entity limit gating, geometric flag, ValidationResult semantics
//! - PostBoolean automatic validation on valid cube Boolean
//! - run_checkpoint against a valid arena

use super::checkpoint::{
    ValidationCheckpoint, ValidationConfig, ValidationResult, run_checkpoint,
};
use crate::mesh_builder::make_cube;
use forge_topo::validate::ValidationLevel;

/// PV-13: ValidationConfig correctly enables/disables checkpoints.
#[test]
fn pv_13_checkpoint_activation() {
    let debug_config = ValidationConfig::debug_default();
    assert!(debug_config.is_active(ValidationCheckpoint::PostCommit));
    assert!(debug_config.is_active(ValidationCheckpoint::PostBoolean));
    assert!(debug_config.is_active(ValidationCheckpoint::PostFeature));
    assert!(debug_config.is_active(ValidationCheckpoint::PostImport));
    assert!(!debug_config.is_active(ValidationCheckpoint::OnDemand));
    assert!(debug_config.get_include_geometric());

    let release_config = ValidationConfig::release_default();
    assert!(!release_config.is_active(ValidationCheckpoint::PostCommit));
    assert!(release_config.is_active(ValidationCheckpoint::PostBoolean));
    assert!(release_config.is_active(ValidationCheckpoint::PostImport));
    assert!(!release_config.get_include_geometric());
    assert_eq!(release_config.get_entity_limit(), 50_000);

    let all_config = ValidationConfig::all_active();
    assert!(all_config.is_active(ValidationCheckpoint::OnDemand));

    let disabled_config = ValidationConfig::disabled();
    assert!(!disabled_config.is_active(ValidationCheckpoint::PostCommit));
    assert!(!disabled_config.is_active(ValidationCheckpoint::PostBoolean));
}

/// PV-14: Validation of 50,000-entity arena completes in < 100ms.
///
/// Builds one canonical cube via `make_cube` (BSP pipeline — guaranteed
/// correct winding), then replicates that exact topology structure ~1,400
/// times into a single arena via entity copying with handle remapping.
/// Each cube is disjoint, so copied cubes share no handles.
///
/// Entity budget: one `make_cube` produces 8V + 24HE + 6F = 38 countable entities.
/// 1,320 cubes → 50,160 entities (≥ 50K).
#[test]
fn pv_14_50k_entities_under_100ms() {
    use forge_topo::state::{TopologyState, DraftConfig};
    use forge_topo::arena::{FaceData, HalfEdgeData, VertexData, LoopData};
    use forge_topo::handles::{FaceId, HalfEdgeId, VertexId, LoopId};
    use std::collections::BTreeMap;

    let template = make_cube([0.0, 0.0, 0.0], 2.0).unwrap();
    let (template_topo, _template_geom) = template.into_parts();
    let src = template_topo.arena();

    let mut config = DraftConfig::default();
    config.validation_level = ValidationLevel::None;

    let base = TopologyState::empty();
    let mut draft = base.into_mutation_with(config);
    let dst = draft.arena_mut();

    let placeholder_he = HalfEdgeId::from_raw_parts(0, 0);
    let placeholder_face = FaceId::from_raw_parts(0, 0);

    let cube_count = 1_320;

    for _ in 0..cube_count {
        let mut vert_map: BTreeMap<u32, VertexId> = BTreeMap::new();
        let mut he_map: BTreeMap<u32, HalfEdgeId> = BTreeMap::new();
        let mut face_map: BTreeMap<u32, FaceId> = BTreeMap::new();
        let mut loop_map: BTreeMap<u32, LoopId> = BTreeMap::new();

        for (vid, _vdata) in src.iter_vertices() {
            let new_vid = dst.insert_vertex(VertexData::new(placeholder_he));
            vert_map.insert(vid.index(), new_vid);
        }

        for (lid, _ldata) in src.iter_loops() {
            let new_lid = dst.insert_loop(LoopData::new(placeholder_he, placeholder_face));
            loop_map.insert(lid.index(), new_lid);
        }

        for (fid, fdata) in src.iter_faces() {
            let new_loop = loop_map[&fdata.outer_loop().index()];
            let new_fid = dst.insert_face(FaceData::new(new_loop));
            face_map.insert(fid.index(), new_fid);
        }

        for (heid, _hedata) in src.iter_half_edges() {
            let new_heid = dst.insert_half_edge(HalfEdgeData::new(
                placeholder_he, placeholder_he, placeholder_he,
                placeholder_face, VertexId::from_raw_parts(0, 0),
            ));
            he_map.insert(heid.index(), new_heid);
        }

        for (heid, hedata) in src.iter_half_edges() {
            let new_heid = he_map[&heid.index()];
            let he_mut = dst.get_half_edge_mut(new_heid).unwrap();
            he_mut.set_next(he_map[&hedata.next().index()]);
            he_mut.set_prev(he_map[&hedata.prev().index()]);
            he_mut.set_twin(he_map[&hedata.twin().index()]);
            he_mut.set_face(face_map[&hedata.face().index()]);
            he_mut.set_origin(vert_map[&hedata.origin().index()]);
        }

        for (vid, vdata) in src.iter_vertices() {
            let new_vid = vert_map[&vid.index()];
            dst.get_vertex_mut(new_vid).unwrap().set_outgoing(he_map[&vdata.outgoing().index()]);
        }

        for (lid, ldata) in src.iter_loops() {
            let new_lid = loop_map[&lid.index()];
            let l_mut = dst.get_loop_mut(new_lid).unwrap();
            l_mut.set_half_edge(he_map[&ldata.half_edge().index()]);
            l_mut.set_face(face_map[&ldata.face().index()]);
        }

        for (fid, fdata) in src.iter_faces() {
            let new_fid = face_map[&fid.index()];
            dst.get_face_mut(new_fid).unwrap().set_outer_loop(loop_map[&fdata.outer_loop().index()]);
        }
    }

    let total_entities = dst.vertex_count() + dst.half_edge_count() + dst.face_count();
    assert!(
        total_entities >= 50_000,
        "Expected >= 50,000 entities, got {}",
        total_entities
    );

    let start = std::time::Instant::now();
    let result = forge_topo::validate::validate_topology(dst, ValidationLevel::Full);
    let elapsed = start.elapsed();

    assert!(
        result.is_ok(),
        "50K entity arena should pass validation: {:?}",
        result.err()
    );

    let budget_ms: u128 = if cfg!(debug_assertions) { 1_000 } else { 100 };

    assert!(
        elapsed.as_millis() < budget_ms,
        "Structural validation of {} entities took {}ms (budget: {}ms, {})",
        total_entities, elapsed.as_millis(), budget_ms,
        if cfg!(debug_assertions) { "debug" } else { "release" }
    );

    eprintln!(
        "PV-14: validated {} entities in {:.2}ms (budget: {}ms)",
        total_entities,
        elapsed.as_secs_f64() * 1000.0,
        budget_ms
    );
}


/// PV-14b: Entity limit configuration tests (originally PV-14).
#[test]
fn pv_14b_entity_limit_config() {
    let mut config = ValidationConfig::all_active();
    config.set_entity_limit(1000);

    assert!(!config.should_skip_for_entity_count(999));
    assert!(config.should_skip_for_entity_count(1000));
    assert!(config.should_skip_for_entity_count(5000));

    let result_skipped = ValidationResult::skipped(ValidationCheckpoint::PostCommit, 5000);
    assert!(result_skipped.is_skipped());
    assert!(result_skipped.is_passed());

    let result_passed = ValidationResult::passed(
        ValidationCheckpoint::PostCommit, 500, true, 42,
    );
    assert!(!result_passed.is_skipped());
    assert!(result_passed.is_passed());
    assert!(result_passed.included_geometric());
    assert_eq!(result_passed.duration_micros(), 42);

    let result_failed = ValidationResult::failed(
        ValidationCheckpoint::PostCommit, 500, "Euler violation".to_string(), false, 100,
    );
    assert!(!result_failed.is_passed());
    assert_eq!(result_failed.error_detail(), Some("Euler violation"));

    let no_limit_config = ValidationConfig::debug_default();
    assert!(!no_limit_config.should_skip_for_entity_count(999999));
}

/// run_checkpoint skips when checkpoint is inactive.
#[test]
fn run_checkpoint_skips_inactive() {
    let result = make_cube([0.0, 0.0, 0.0], 1.0).unwrap();
    let (topo, _geom) = result.into_parts();

    let config = ValidationConfig::disabled();
    let vr = run_checkpoint(
        topo.arena(), &config, ValidationCheckpoint::PostBoolean,
        None, 1e-10, 1e-12,
    ).unwrap();

    assert!(vr.is_skipped());
}

/// run_checkpoint passes on valid cube with all checks active.
#[test]
fn run_checkpoint_passes_valid_cube() {
    let result = make_cube([0.0, 0.0, 0.0], 1.0).unwrap();
    let (topo, geom) = result.into_parts();

    let config = ValidationConfig::debug_default();
    let pos_fn = |vid| geom.get_vertex_position(vid).copied();
    let vr = run_checkpoint(
        topo.arena(), &config, ValidationCheckpoint::PostBoolean,
        Some(&pos_fn), 1e-10, 1e-12,
    ).unwrap();

    assert!(vr.is_passed());
    assert!(!vr.is_skipped());
    assert!(vr.included_geometric());
    assert!(vr.duration_micros() < 1_000_000);
}

/// run_checkpoint skips when entity limit exceeded.
#[test]
fn run_checkpoint_skips_entity_limit() {
    let result = make_cube([0.0, 0.0, 0.0], 1.0).unwrap();
    let (topo, _geom) = result.into_parts();

    let mut config = ValidationConfig::all_active();
    config.set_entity_limit(1);
    let vr = run_checkpoint(
        topo.arena(), &config, ValidationCheckpoint::PostCommit,
        None, 1e-10, 1e-12,
    ).unwrap();

    assert!(vr.is_skipped());
}

/// run_checkpoint runs structural-only when include_geometric is false.
#[test]
fn run_checkpoint_structural_only() {
    let result = make_cube([0.0, 0.0, 0.0], 1.0).unwrap();
    let (topo, _geom) = result.into_parts();

    let mut config = ValidationConfig::all_active();
    config.set_include_geometric(false);
    let vr = run_checkpoint(
        topo.arena(), &config, ValidationCheckpoint::PostBoolean,
        None, 1e-10, 1e-12,
    ).unwrap();

    assert!(vr.is_passed());
    assert!(!vr.included_geometric());
}
