#[cfg(feature = "parallel")]
use std::collections::BTreeMap;

#[cfg(feature = "parallel")]
use super::audit_surface::PrimaryAuditSurface;
#[cfg(feature = "parallel")]
use super::market_seed::MarketSeed;
#[cfg(feature = "parallel")]
use super::scenarios::setup_seeded_world;
#[cfg(feature = "parallel")]
use crate::facade::*;

#[cfg(feature = "parallel")]
struct BranchDivergenceOutcome {
    main_audit: PrimaryAuditSurface,
    analysis_audit: PrimaryAuditSurface,
    correction_audit: PrimaryAuditSurface,
    analysis_replay: ReplaySlice,
    correction_replay: ReplaySlice,
    correction_lineage: Vec<LineageRecord>,
    branch_heads: BTreeMap<&'static str, Option<SignalSnapshotId>>,
}

#[cfg(feature = "parallel")]
fn run_parallel_drift_workflow(executor: StageExecutor) -> BranchDivergenceOutcome {
    let mut fixture = setup_seeded_world();
    let main_checkpoint = fixture.capture_active_checkpoint(executor).unwrap();

    let analysis = fixture.open_branch("analysis-drift").unwrap();
    fixture.seed_market(MarketSeed::high_vol(17)).unwrap();
    let analysis_audit = fixture.refresh_primary_audit_surface(executor).unwrap();
    fixture.inject_primary_market_rollback(executor).unwrap();
    let analysis_replay = fixture.replay_for_branch(analysis.clone());
    fixture.capture_active_checkpoint(executor).unwrap();

    fixture
        .switch_branch(main_checkpoint.branch.clone())
        .unwrap();
    fixture.restore_checkpoint(&main_checkpoint).unwrap();
    let correction = fixture.open_branch("correction-drift").unwrap();
    fixture.seed_market(MarketSeed::fx_dislocation(29)).unwrap();
    let correction_audit = fixture.refresh_primary_audit_surface(executor).unwrap();
    fixture.inject_primary_market_rollback(executor).unwrap();
    let correction_replay = fixture.replay_for_branch(correction.clone());
    fixture.capture_active_checkpoint(executor).unwrap();
    let correction_lineage = fixture.main_risk_lineage();

    fixture
        .switch_branch(main_checkpoint.branch.clone())
        .unwrap();
    fixture.restore_checkpoint(&main_checkpoint).unwrap();
    let main_audit = fixture.read_primary_audit_surface(executor).unwrap();

    BranchDivergenceOutcome {
        main_audit,
        analysis_audit,
        correction_audit,
        analysis_replay,
        correction_replay,
        correction_lineage,
        branch_heads: BTreeMap::from([
            (
                "main",
                fixture.branch_head_snapshot_id(main_checkpoint.branch),
            ),
            ("analysis", fixture.branch_head_snapshot_id(analysis)),
            ("correction", fixture.branch_head_snapshot_id(correction)),
        ]),
    }
}

#[cfg(feature = "parallel")]
#[test]
fn fintech_serial_parallel_branch_divergence_keeps_overlap_honest_after_hostility() {
    let serial = run_parallel_drift_workflow(StageExecutor::Serial);
    let parallel = run_parallel_drift_workflow(StageExecutor::aggressive_parallel());

    assert_eq!(serial.main_audit, parallel.main_audit);
    assert_eq!(serial.analysis_audit, parallel.analysis_audit);
    assert_eq!(serial.correction_audit, parallel.correction_audit);
    assert_eq!(serial.branch_heads, parallel.branch_heads);

    let analysis_diff = compare_replay_slices(&serial.analysis_replay, &parallel.analysis_replay);
    assert!(analysis_diff.mismatches.is_empty());
    let correction_diff =
        compare_replay_slices(&serial.correction_replay, &parallel.correction_replay);
    assert!(correction_diff.mismatches.is_empty());
    let lineage_diff =
        compare_lineage_records(&serial.correction_lineage, &parallel.correction_lineage);
    assert!(lineage_diff.mismatches.is_empty());
}
