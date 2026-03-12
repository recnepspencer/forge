use super::regimes::MarketRegime;
use super::scales::FintechScale;
use super::scenarios::setup_seeded_world_with;
use crate::facade::*;

#[test]
fn fintech_high_fanout_tolerance_session_recovers_after_masking_pressure() {
    let mut fixture = setup_seeded_world_with(FintechScale::fanout(), MarketRegime::Calm, 7);
    fixture.assert_shape(FintechScale::fanout());

    let baseline_checkpoint = fixture
        .capture_active_checkpoint(StageExecutor::Serial)
        .unwrap();
    let baseline_threshold = fixture
        .read_primary_threshold_with_executor(StageExecutor::Serial)
        .unwrap();
    let baseline_market = fixture
        .read_primary_market_source_with_executor(StageExecutor::Serial)
        .unwrap();
    let baseline_bucket = fixture
        .read_bucket_aggregate_with_executor(0, StageExecutor::Serial)
        .unwrap();
    let baseline_scenario = fixture
        .read_scenario_aggregate_with_executor(0, StageExecutor::Serial)
        .unwrap();

    fixture
        .bump_primary_market(1, 0, 0, 0, StageExecutor::Serial)
        .unwrap();
    let threshold_after_small = fixture
        .read_primary_threshold_with_executor(StageExecutor::Serial)
        .unwrap();
    assert_eq!(threshold_after_small, baseline_threshold);

    fixture
        .bump_primary_market(6, 3, 2, 2, StageExecutor::Serial)
        .unwrap();
    let hot_market = fixture
        .read_primary_market_source_with_executor(StageExecutor::Serial)
        .unwrap();
    let hot_threshold = fixture
        .read_primary_threshold_with_executor(StageExecutor::Serial)
        .unwrap();
    let hot_audit = fixture
        .refresh_primary_audit_surface(StageExecutor::Serial)
        .unwrap();
    let hot_bucket = fixture
        .read_bucket_aggregate_with_executor(0, StageExecutor::Serial)
        .unwrap();
    let hot_scenario = fixture
        .read_scenario_aggregate_with_executor(0, StageExecutor::Serial)
        .unwrap();
    assert_ne!(hot_market, baseline_market);
    assert_ne!(hot_threshold, baseline_threshold);
    assert!(hot_audit.desk.get(super::aspects::RISK) > 0);
    assert!(hot_bucket.get(super::aspects::RISK) > 0);
    assert!(hot_scenario.get(super::aspects::RISK) > 0);

    fixture
        .inject_primary_market_rollback(StageExecutor::Serial)
        .unwrap();
    fixture.restore_checkpoint(&baseline_checkpoint).unwrap();
    let restored_bucket = fixture
        .read_bucket_aggregate_with_executor(0, StageExecutor::Serial)
        .unwrap();
    let restored_scenario = fixture
        .read_scenario_aggregate_with_executor(0, StageExecutor::Serial)
        .unwrap();
    let restored_threshold = fixture
        .read_primary_threshold_with_executor(StageExecutor::Serial)
        .unwrap();

    assert_eq!(restored_bucket, baseline_bucket);
    assert_eq!(restored_scenario, baseline_scenario);
    assert_eq!(restored_threshold, baseline_threshold);
    assert_eq!(
        fixture
            .read_primary_audit_surface(StageExecutor::Serial)
            .unwrap(),
        baseline_checkpoint.audit
    );
}