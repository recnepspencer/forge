use crate::facade::BranchId;

use super::actions::{
    build_branch_scoped_case_index, capture_world_snapshot, checkpoint_world,
    commit_case_trade_after_savepoint, compact_world_store, correct_seeded_trade_candidate,
    diverge_case_trade_on_branch, emit_trade_correction_audit_record, merge_branch_into_main,
    open_analysis_branch, open_audit_branch, promote_case_correspondence, recover_persisted_world,
    recover_runtime_from_plan, refresh_risk_views, register_case_book_index,
    release_snapshot_handle, repair_seeded_failed_settlement, shock_market_on_branch,
    stress_seeded_intraday_risk,
};
use super::comparisons::{compare_case_truth, compare_replay_probe};
use super::complexity::{
    assert_counter_at_most, contract_ids, measure_world_action, workflow_budgets, ComplexityBudget,
};
use super::failure_injection::{
    corrupt_latest_checkpoint_file, drop_latest_parent_envelope_for_replay,
    invalid_savepoint_rollback_code, replay_latest_commit_on_wrong_branch,
    rollback_seeded_trade_correction_after_savepoint,
};
use super::fixture::FintechWorld;
use super::invariants::{
    assert_correction_case_transition, assert_cross_context_relations, assert_fixture_shape,
    assert_intraday_risk_case_transition, assert_merge_metadata_preserved,
    assert_metadata_preserved_after_recovery, assert_named_truth_world,
    assert_observability_overlap_stable, assert_observability_surfaces_agree,
    assert_partitioned_payloads, assert_recovery_matches_truth, assert_replay_targets_branch,
    assert_settlement_repair_case_transition, assert_snapshot_release_contract,
};
use super::naming::{
    artifact_alias, invariant_id, read_alias, replay_alias, scenario_name, workflow_name,
};
use super::probes::{
    capture_case_truth_probe, capture_observability_probe, capture_recovery_probe,
    capture_replay_probe, read_snapshot_probe, read_version_probe, ProbeStage,
};
use super::scales::FintechScale;
use super::scenarios::{
    setup_selected_world, setup_world, setup_world_for, setup_world_for_failed_settlement_repair,
    setup_world_for_historical_visibility, setup_world_for_intraday_risk,
    setup_world_for_late_trade_correction, setup_world_with, FintechScenario,
};

fn setup_smoke_world() -> FintechWorld {
    let world = setup_world_for(FintechScenario::SmokeBook);
    assert_fixture_shape(&world, FintechScale::smoke());
    world
}

#[test]
fn fintech_fixture_builds_partitioned_truth_with_cross_context_relations() {
    let world = setup_world();
    let read = world.read_latest();

    assert_named_truth_world(&world);
    assert_partitioned_payloads(&read);
    assert_cross_context_relations(&read);
}

#[test]
fn fintech_world_setup_helpers_build_expected_scales() {
    let default_world = setup_world();
    let scaled_world = setup_world_with(FintechScale::smoke());
    let history_world = setup_world_for_historical_visibility();
    let intraday_world = setup_world_for_intraday_risk();
    let correction_world = setup_world_for_late_trade_correction();
    let settlement_world = setup_world_for_failed_settlement_repair();

    assert_fixture_shape(&default_world, FintechScale::smoke());
    assert_fixture_shape(&scaled_world, FintechScale::smoke());
    assert_named_truth_world(&history_world);
    assert_named_truth_world(&intraday_world);
    assert_named_truth_world(&correction_world);
    assert_named_truth_world(&settlement_world);
}

#[test]
fn fintech_scenario_selectors_expose_canonical_cases_and_expected_invariants() {
    let (_world, selection) = setup_selected_world(FintechScenario::LateTradeCorrection);

    assert!(matches!(
        selection.scenario,
        FintechScenario::LateTradeCorrection
    ));
    assert_eq!(
        selection.canonical_case,
        super::fixture::FintechCaseRole::LateTradeCorrection
    );
    assert_eq!(selection.scenario_key, "late-trade-correction");
    assert!(!selection.expected_invariants.is_empty());
    assert!(selection.expected_artifacts.contains(&"diagnostics"));
    assert_eq!(
        selection.expected_read_alias,
        "trade-correction.read.post-mutation"
    );
    assert_eq!(selection.probe_prefix, "trade-correction");
    assert!(!selection.persisted);
    assert_eq!(
        scenario_name("trade-correction", "baseline"),
        "fintech.trade-correction.baseline"
    );
    assert_eq!(
        workflow_name("trade-correction", "replay"),
        "fintech.trade-correction.replay"
    );
    assert_eq!(
        artifact_alias("trade-correction", "read", "post-mutation"),
        "trade-correction.read.post-mutation"
    );
    assert_eq!(
        read_alias("trade-correction", "post-mutation"),
        "trade-correction.read.post-mutation"
    );
    assert_eq!(
        replay_alias("trade-correction", "analysis"),
        "trade-correction.replay.analysis"
    );
    assert_eq!(
        invariant_id("trade-correction", "analysis_branch_local"),
        "trade-correction:analysis_branch_local"
    );
}

#[test]
fn fintech_snapshot_release_preserves_historical_visibility_and_invalidates_handle_reads() {
    let mut world = setup_world_for(FintechScenario::HistoricalVisibility);
    let analysis = open_analysis_branch(&mut world);
    let baseline_snapshot = capture_world_snapshot(&mut world);
    let baseline_probe = read_snapshot_probe(
        &world,
        super::fixture::FintechCaseRole::LateTradeCorrection,
        &baseline_snapshot,
        ProbeStage::Baseline,
    );

    let _correction = correct_seeded_trade_candidate(&mut world, analysis.clone());
    let _audit = emit_trade_correction_audit_record(&mut world, analysis);

    let historical_after_mutation = read_snapshot_probe(
        &world,
        super::fixture::FintechCaseRole::LateTradeCorrection,
        &baseline_snapshot,
        ProbeStage::PostMutation,
    );
    let historical_after_release = {
        let version_id = baseline_snapshot.version_id;
        assert!(release_snapshot_handle(&mut world, &baseline_snapshot));
        assert!(world.runtime.read_snapshot(&baseline_snapshot).is_none());
        read_version_probe(
            &world,
            super::fixture::FintechCaseRole::LateTradeCorrection,
            version_id,
            ProbeStage::PostRecovery,
        )
    };

    assert_snapshot_release_contract(
        &baseline_probe,
        &historical_after_mutation,
        &historical_after_release,
    );
}

#[test]
fn fintech_world_exposes_named_domain_probes_for_correction_risk_and_settlement() {
    let world = setup_world();
    let snapshot = world.latest_snapshot();

    let correction = world
        .runtime
        .execute_read_packet(&snapshot, &world.packet_for_correction_probe())
        .unwrap();
    let risk = world
        .runtime
        .execute_read_packet(&snapshot, &world.packet_for_intraday_risk_probe())
        .unwrap();
    let settlement = world
        .runtime
        .execute_read_packet(&snapshot, &world.packet_for_settlement_repair_probe())
        .unwrap();

    assert_eq!(correction.entities.len(), 3);
    assert_eq!(risk.entities.len(), 4);
    assert_eq!(settlement.entities.len(), 4);
}

#[test]
fn fintech_analysis_workflow_preserves_branching_snapshots_and_trade_correction() {
    let mut world = setup_smoke_world();
    let analysis = open_analysis_branch(&mut world);
    let baseline_snapshot = world.runtime.snapshot();
    let baseline_probe = read_snapshot_probe(
        &world,
        super::fixture::FintechCaseRole::LateTradeCorrection,
        &baseline_snapshot,
        ProbeStage::Baseline,
    );
    let baseline_observability = capture_observability_probe(&world, ProbeStage::Baseline);

    let _shock = shock_market_on_branch(&mut world, analysis.clone());
    let _correction = correct_seeded_trade_candidate(&mut world, analysis.clone());
    let _audit = emit_trade_correction_audit_record(&mut world, analysis.clone());
    let _risk = refresh_risk_views(&mut world, analysis.clone());

    let main_read = world.runtime.read_snapshot(&baseline_snapshot).unwrap();
    let analysis_read = world.read_latest();
    assert_ne!(main_read.entities().len(), 0);
    assert!(analysis_read.entities().iter().any(|entity| matches!(
        &entity.payload,
        crate::facade::RecordPayload::StructuredJson(value)
            if value.get("corrected").and_then(|flag| flag.as_bool()) == Some(true)
    )));
    assert_eq!(
        world
            .runtime
            .branch_head(&BranchId("analysis".to_string()))
            .cloned(),
        world.runtime.latest_commit().cloned()
    );
    let post_probe = capture_case_truth_probe(
        &world,
        super::fixture::FintechCaseRole::LateTradeCorrection,
        ProbeStage::PostMutation,
    );
    let post_observability = capture_observability_probe(&world, ProbeStage::PostMutation);
    assert_correction_case_transition(&baseline_probe, &post_probe);
    assert_observability_overlap_stable(&baseline_observability, &post_observability);
}

#[test]
fn fintech_intraday_risk_workflow_exposes_open_breach_on_analysis_branch() {
    let mut world = setup_smoke_world();
    let analysis = open_analysis_branch(&mut world);
    let baseline_probe = capture_case_truth_probe(
        &world,
        super::fixture::FintechCaseRole::IntradayRisk,
        ProbeStage::Baseline,
    );

    let _stress = stress_seeded_intraday_risk(&mut world, analysis.clone());
    let post_probe = capture_case_truth_probe(
        &world,
        super::fixture::FintechCaseRole::IntradayRisk,
        ProbeStage::PostMutation,
    );
    let analysis_read = world.read_latest();
    let replay_probe = capture_replay_probe(&mut world, analysis.clone());
    let post_replay_probe = capture_case_truth_probe(
        &world,
        super::fixture::FintechCaseRole::IntradayRisk,
        ProbeStage::PostReplay,
    );

    assert!(analysis_read.entities().iter().any(|entity| matches!(
        &entity.payload,
        crate::facade::RecordPayload::StructuredJson(value)
            if value.get("entity_type").and_then(|value| value.as_str()) == Some("limit_breach")
                && value.get("status").and_then(|value| value.as_str()) == Some("open")
    )));
    assert_eq!(
        world
            .runtime
            .branch_head(&BranchId("analysis".to_string()))
            .cloned(),
        world.runtime.latest_commit().cloned()
    );
    assert_intraday_risk_case_transition(&baseline_probe, &post_probe);
    assert!(compare_case_truth(&post_probe, &post_replay_probe).is_empty());
    assert!(compare_replay_probe(&replay_probe, &replay_probe).is_empty());
    assert_replay_targets_branch(&replay_probe, &analysis);
}

#[test]
fn fintech_settlement_repair_workflow_exposes_repaired_settlement_on_analysis_branch() {
    let mut world = setup_smoke_world();
    let analysis = open_analysis_branch(&mut world);
    let baseline_probe = capture_case_truth_probe(
        &world,
        super::fixture::FintechCaseRole::FailedSettlementRepair,
        ProbeStage::Baseline,
    );

    let _repair = repair_seeded_failed_settlement(&mut world, analysis.clone());
    let post_probe = capture_case_truth_probe(
        &world,
        super::fixture::FintechCaseRole::FailedSettlementRepair,
        ProbeStage::PostMutation,
    );
    let analysis_read = world.read_latest();

    assert!(analysis_read.entities().iter().any(|entity| matches!(
        &entity.payload,
        crate::facade::RecordPayload::StructuredJson(value)
            if value.get("entity_type").and_then(|value| value.as_str()) == Some("settlement")
                && value.get("status").and_then(|value| value.as_str()) == Some("repaired")
    )));
    assert_eq!(
        world
            .runtime
            .branch_head(&BranchId("analysis".to_string()))
            .cloned(),
        world.runtime.latest_commit().cloned()
    );
    assert_settlement_repair_case_transition(&baseline_probe, &post_probe);
}

#[test]
fn fintech_persisted_workflow_recovers_checkpoint_tail_and_keeps_queryable_portfolio_state() {
    let mut world = setup_world_for(FintechScenario::PersistedSmokeBook);
    let analysis = open_analysis_branch(&mut world);
    let _shock = shock_market_on_branch(&mut world, analysis.clone());
    let _checkpoint = checkpoint_world(&mut world).unwrap();
    let _correction = correct_seeded_trade_candidate(&mut world, analysis);
    let expected = {
        let plan = world.runtime.recovery_plan();
        let mut recovered = FintechWorld::setup_persisted_world().runtime;
        let outcome = recovered.recover(plan).unwrap();
        capture_recovery_probe(&recovered, &outcome)
    };

    let (recovered, outcome) = recover_persisted_world(&world).unwrap();
    let recovered_snapshot = recovered
        .latest_publication_bundle()
        .unwrap()
        .snapshot
        .clone();

    let packet = world.packet_for_portfolio_probe();
    let result = recovered
        .execute_read_packet(&recovered_snapshot, &packet)
        .unwrap();
    let before_probe = capture_case_truth_probe(
        &world,
        super::fixture::FintechCaseRole::BaselinePortfolio,
        ProbeStage::PostMutation,
    );
    let recovery_probe = capture_recovery_probe(&recovered, &outcome);

    assert_eq!(result.entities.len(), 3);
    assert_eq!(
        recovered
            .branch_head(&BranchId("analysis".to_string()))
            .cloned(),
        recovered.latest_commit().cloned()
    );
    let after_probe = read_snapshot_probe(
        &world,
        super::fixture::FintechCaseRole::BaselinePortfolio,
        &world.latest_snapshot(),
        ProbeStage::PostRecovery,
    );
    assert!(compare_case_truth(&before_probe, &after_probe).is_empty());
    assert_recovery_matches_truth(&expected, &recovery_probe);
}

#[test]
fn fintech_branch_divergence_merge_and_savepoint_verbs_stay_case_local() {
    let mut world = setup_smoke_world();
    let analysis = open_analysis_branch(&mut world);
    let audit = open_audit_branch(&mut world);

    let rollback = rollback_seeded_trade_correction_after_savepoint(&mut world, analysis.clone());
    assert!(!rollback.restored_records.is_empty());

    let _saved = commit_case_trade_after_savepoint(&mut world, analysis.clone());
    let _diverged = diverge_case_trade_on_branch(
        &mut world,
        audit.clone(),
        super::fixture::FintechCaseRole::LateTradeCorrection,
        1_610_000,
    );
    let merged = merge_branch_into_main(&mut world, audit.clone());
    let (merge_parent_branches, merge_base_count, parent_count) = {
        let envelope = world
            .runtime
            .canonical_commit_envelope(merged.commit.commit_id)
            .expect("merged commit should have canonical envelope");
        (
            envelope.merge_parent_branches.clone(),
            envelope.merge_base_commits.len(),
            envelope.commit.parents.len(),
        )
    };

    assert_eq!(merged.commit.branch_id, BranchId("main".to_string()));
    assert_eq!(
        world
            .runtime
            .branch_head(&audit)
            .map(|commit| commit.commit_id),
        Some(merged.commit.parents[1])
    );
    assert_ne!(
        world
            .runtime
            .branch_head(&BranchId("analysis".to_string()))
            .map(|commit| commit.commit_id),
        Some(merged.commit.commit_id)
    );
    assert_merge_metadata_preserved(
        &merge_parent_branches,
        merge_base_count,
        parent_count,
        &audit,
        2,
    );
}

#[test]
fn fintech_failure_injection_helpers_cover_savepoints_replay_and_checkpoint_corruption() {
    let mut world = setup_smoke_world();
    let analysis = open_analysis_branch(&mut world);
    let rollback = rollback_seeded_trade_correction_after_savepoint(&mut world, analysis.clone());
    let invalid_code = invalid_savepoint_rollback_code(&mut world, analysis.clone());

    assert!(!rollback.restored_records.is_empty());
    assert_eq!(
        invalid_code,
        crate::facade::DiagnosticCode::InvalidSavepoint
    );

    let _correction = correct_seeded_trade_candidate(&mut world, analysis);
    let removed = drop_latest_parent_envelope_for_replay(&mut world.runtime);
    assert!(removed.is_some());
    let wrong_branch = replay_latest_commit_on_wrong_branch(&mut world.runtime).unwrap();
    assert_eq!(
        wrong_branch.failure,
        Some(crate::facade::ReplayFailureClass::BranchMismatch)
    );

    let mut persisted = setup_world_for(FintechScenario::PersistedSmokeBook);
    checkpoint_world(&mut persisted).unwrap();
    let path = corrupt_latest_checkpoint_file(&persisted.runtime.recovery_plan());
    assert!(path.is_some());
}

#[test]
fn fintech_complexity_hooks_measure_seeded_workflows() {
    let mut world = setup_smoke_world();
    let analysis = open_analysis_branch(&mut world);
    let contracts = contract_ids(&world);
    let clone_budget = workflow_budgets()
        .into_iter()
        .find(|budget| budget.label == "full_state_clones")
        .unwrap_or(ComplexityBudget {
            label: "full_state_clones",
            max: 0,
            selector: |counters| counters.full_state_clones,
        });

    assert!(contracts.contains(&"runtime.partition_local_commit"));

    let counters = measure_world_action(&mut world, |world| {
        let _ = correct_seeded_trade_candidate(world, analysis.clone());
    });
    assert_counter_at_most(
        &counters,
        clone_budget.selector,
        clone_budget.max,
        clone_budget.label,
    );
}

#[test]
fn fintech_persisted_workflow_supports_compaction_after_checkpoint() {
    let mut world = setup_world_for(FintechScenario::PersistedSmokeBook);
    let analysis = open_analysis_branch(&mut world);
    let _shock = shock_market_on_branch(&mut world, analysis);
    checkpoint_world(&mut world).unwrap();
    let pre_compaction_probe = capture_case_truth_probe(
        &world,
        super::fixture::FintechCaseRole::BaselinePortfolio,
        ProbeStage::PostMutation,
    );

    let compaction = compact_world_store(&mut world).unwrap();
    let (recovered, outcome) = recover_persisted_world(&world).unwrap();
    let recovered_probe = capture_recovery_probe(&recovered, &outcome);

    assert!(
        !compaction.removed_segments.is_empty() || !compaction.retained_segments.is_empty(),
        "compaction should report at least one segment decision"
    );
    assert_eq!(
        recovered
            .branch_head(&BranchId("analysis".to_string()))
            .cloned(),
        recovered.latest_commit().cloned()
    );
    let packet = world.packet_for_portfolio_probe();
    let version_id = recovered
        .latest_commit()
        .expect("recovered runtime should restore a latest commit")
        .version_id;
    let result = recovered.read_version(version_id).execute_packet(&packet);
    assert_eq!(result.entities.len(), pre_compaction_probe.entity_count);
    assert!(recovered_probe.latest_commit_id.is_some());
}

#[test]
fn fintech_metadata_survives_hostile_workflow_recovery() {
    let mut world = setup_world_for(FintechScenario::PersistedSmokeBook);
    let analysis = open_analysis_branch(&mut world);
    let correction = correct_seeded_trade_candidate(&mut world, analysis.clone());
    let _audit = emit_trade_correction_audit_record(&mut world, analysis.clone());
    let resolution = promote_case_correspondence(
        &mut world,
        super::fixture::FintechCaseRole::BaselinePortfolio,
        super::fixture::FintechCaseRole::LateTradeCorrection,
        correction.commit.clone(),
    );
    let index = register_case_book_index(&mut world);
    let build = build_branch_scoped_case_index(
        &mut world,
        index.index_id,
        analysis,
        correction.commit.clone(),
    );
    checkpoint_world(&mut world).unwrap();

    let (recovered, _outcome) = recover_persisted_world(&world).unwrap();
    let graph = recovered.lineage_graph(&BranchId("main".to_string()));
    let generation = recovered
        .latest_index_generation(index.index_id, &BranchId("analysis".to_string()))
        .expect("analysis index generation should recover");

    assert!(build.failed_indexes.is_empty());
    assert_eq!(generation.generation_id, build.generations[0].generation_id);
    assert_eq!(generation.source_commit_id, correction.commit.commit_id);
    assert_metadata_preserved_after_recovery(resolution, &graph, generation);
}

#[test]
fn fintech_observability_surfaces_agree_for_hostile_correction_workflow() {
    let mut world = setup_smoke_world();
    let analysis = open_analysis_branch(&mut world);

    let _shock = shock_market_on_branch(&mut world, analysis.clone());
    let _correction = correct_seeded_trade_candidate(&mut world, analysis.clone());
    let _audit = emit_trade_correction_audit_record(&mut world, analysis.clone());
    let _risk = refresh_risk_views(&mut world, analysis);

    assert_observability_surfaces_agree(&world);
}

#[test]
fn fintech_recovery_falls_back_from_corrupt_latest_checkpoint_and_keeps_truth() {
    let mut world = setup_world_for(FintechScenario::PersistedSmokeBook);
    let analysis = open_analysis_branch(&mut world);
    let _shock = shock_market_on_branch(&mut world, analysis.clone());
    checkpoint_world(&mut world).unwrap();
    let correction = correct_seeded_trade_candidate(&mut world, analysis);
    checkpoint_world(&mut world).unwrap();
    let plan = world.runtime.recovery_plan();
    let corrupted = corrupt_latest_checkpoint_file(&plan);
    assert!(corrupted.is_some());

    let (recovered, outcome) = recover_runtime_from_plan(world.runtime.recovery_plan()).unwrap();
    let packet = world.packet_for_correction_probe();
    let version_id = recovered
        .latest_commit()
        .expect("recovered runtime should restore a latest commit")
        .version_id;
    let result = recovered.read_version(version_id).execute_packet(&packet);

    assert_eq!(outcome.latest_commit, Some(correction.commit.clone()));
    assert!(!outcome
        .integrity_report
        .skipped_corrupt_checkpoints
        .is_empty());
    assert!(result.entities.iter().any(|entity| matches!(
        &entity.payload,
        crate::facade::RecordPayload::StructuredJson(value)
            if value.get("corrected").and_then(|value| value.as_bool()) == Some(true)
    )));
}
