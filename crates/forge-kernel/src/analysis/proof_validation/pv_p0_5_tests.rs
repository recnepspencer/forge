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

/// PV-14: Entity limit threshold gating works correctly.
#[test]
fn pv_14_entity_limit_respected() {
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

    let no_limit_config = ValidationConfig::default();
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
