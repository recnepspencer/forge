use super::support::{
    collect_subscriber_patches, collect_subscriber_patches_from_head,
    expected_patch_suffix_after_checkpoint, run_seeded_cdc_scenario,
    sampled_checkpoints_from_patches,
};
use crate::facade::publication::{SubscriberRecoverySource, SubscriberResumeRequest};
use crate::tests::harness::certify::assertions::{
    assert_multi_subscriber_converges, assert_visible_truth_matches, assert_window_matrix_matches,
};
use crate::tests::harness::fixtures::runtime::RuntimeHarnessMode;
use crate::tests::harness::model::truth_model::VisibleTruthSummary;
use crate::tests::harness::observe::patch_stream::collect_patch_stream_from_head;
use crate::tests::harness::observe::subscriber_stream::{
    collect_fuzzed_subscriber_views, collect_multi_subscriber_views,
    random_checkpoints_from_patches,
};
use crate::tests::harness::scenario::operation::ScenarioOperation;
use crate::tests::harness::scenario::profiles::CertificationPressureProfile;
use crate::tests::harness::scenario::runner::{
    build_property_runtime, run_property_scenario, run_seeded_scenario, SeededScenarioConfig,
};
use crate::tests::support::*;
use proptest::collection::vec;
use proptest::prelude::*;

#[test]
fn cdc_certification_snapshot_pinning_is_neutral_under_rewrite_churn() {
    let mut pinned_runtime =
        runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
    let mut unpinned_runtime =
        runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);

    let pinned_left =
        create_entity_in_partition(&mut pinned_runtime, "baseline-left", PartitionId(7));
    let pinned_right =
        create_entity_in_partition(&mut pinned_runtime, "baseline-right", PartitionId(11));
    let unpinned_left =
        create_entity_in_partition(&mut unpinned_runtime, "baseline-left", PartitionId(7));
    let unpinned_right =
        create_entity_in_partition(&mut unpinned_runtime, "baseline-right", PartitionId(11));
    let baseline_checkpoint =
        checkpoint_for_schema_version(PatchStreamPosition(2), SchemaVersionId(1));
    let pinned_snapshot = pinned_runtime.visibility_authority().snapshot();

    for step in 0..48 {
        let left_name = format!("left-rewrite-{step}");
        let right_name = format!("right-rewrite-{step}");
        let churn_name = format!("churn-{step}");

        let _ = update_entity(&mut pinned_runtime, pinned_left, &left_name);
        let _ = update_entity(&mut pinned_runtime, pinned_right, &right_name);
        let _ = update_entity(&mut unpinned_runtime, unpinned_left, &left_name);
        let _ = update_entity(&mut unpinned_runtime, unpinned_right, &right_name);

        if step % 3 == 0 {
            let partition = match step % 4 {
                0 => PartitionId(7),
                1 => PartitionId(11),
                2 => PartitionId(29),
                _ => PartitionId(31),
            };
            let _ = create_entity_in_partition(&mut pinned_runtime, &churn_name, partition);
            let _ = create_entity_in_partition(&mut unpinned_runtime, &churn_name, partition);
        }
    }

    let pinned_full = collect_subscriber_patches(&pinned_runtime, baseline_checkpoint.clone(), 512);
    let unpinned_full =
        collect_subscriber_patches(&unpinned_runtime, baseline_checkpoint.clone(), 512);
    assert_eq!(pinned_full, unpinned_full);

    for window_size in [1_usize, 2, 3, 5, 8, 13] {
        let pinned =
            collect_subscriber_patches(&pinned_runtime, baseline_checkpoint.clone(), window_size);
        let unpinned =
            collect_subscriber_patches(&unpinned_runtime, baseline_checkpoint.clone(), window_size);
        assert_eq!(pinned, pinned_full, "pinned window {window_size} drifted");
        assert_eq!(
            unpinned, unpinned_full,
            "unpinned window {window_size} drifted"
        );
        assert_eq!(
            pinned, unpinned,
            "window {window_size} diverged under pinning"
        );
    }

    let pinned_batch = pinned_runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(
            baseline_checkpoint,
            8,
        ))
        .unwrap();
    let unpinned_batch = unpinned_runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(
            checkpoint_for_schema_version(PatchStreamPosition(2), SchemaVersionId(1)),
            8,
        ))
        .unwrap();
    assert_eq!(
        pinned_batch.recovery_decision,
        unpinned_batch.recovery_decision
    );

    let latest_snapshot = pinned_runtime.visibility_authority().snapshot();
    let pinned_snapshot_read = pinned_runtime
        .visibility_reads()
        .read_snapshot(&pinned_snapshot)
        .unwrap();
    let pinned_latest_read = pinned_runtime
        .visibility_reads()
        .read_snapshot(&latest_snapshot)
        .unwrap();

    assert_eq!(
        read_entity_name(pinned_snapshot_read.get_entity(pinned_left).unwrap()),
        Some("baseline-left")
    );
    assert_eq!(
        read_entity_name(pinned_snapshot_read.get_entity(pinned_right).unwrap()),
        Some("baseline-right")
    );
    assert_eq!(
        read_entity_name(pinned_latest_read.get_entity(pinned_left).unwrap()),
        Some("left-rewrite-47")
    );
    assert_eq!(
        read_entity_name(pinned_latest_read.get_entity(pinned_right).unwrap()),
        Some("right-rewrite-47")
    );

    let retention = pinned_runtime.retention_authority().inspect_plan();
    assert!(retention.snapshot_pinned_entities >= 2);
    assert!(pinned_runtime
        .visibility_authority()
        .release_snapshot(&pinned_snapshot));
}

#[test]
fn cdc_certification_seeded_matrix_is_deterministic_and_resume_exact() {
    for seed in 0_u64..16 {
        let left = run_seeded_cdc_scenario(seed, 96);
        let right = run_seeded_cdc_scenario(seed, 96);

        let full_left =
            collect_subscriber_patches(&left.runtime, left.baseline_checkpoint.clone(), 512);
        let full_from_head = collect_subscriber_patches_from_head(&left.runtime, 512);
        let full_right =
            collect_subscriber_patches(&right.runtime, right.baseline_checkpoint.clone(), 512);
        assert_eq!(full_left, full_right, "seed {seed} diverged");

        let window_matrix = [1_usize, 2, 3, 5, 8, 13]
            .into_iter()
            .map(|window_size| {
                (
                    window_size,
                    collect_subscriber_patches(
                        &left.runtime,
                        left.baseline_checkpoint.clone(),
                        window_size,
                    ),
                )
            })
            .collect::<Vec<_>>();
        assert_window_matrix_matches(
            &format!("seed {seed} baseline window matrix"),
            &full_left,
            &window_matrix,
        );

        let checkpoints = sampled_checkpoints_from_patches(&full_left, 6);
        let views = collect_multi_subscriber_views(&left.runtime, &checkpoints, &[1, 2, 4, 7]);
        assert_multi_subscriber_converges(
            &format!("seed {seed} multi-subscriber convergence"),
            &views,
            &full_from_head,
        );

        for checkpoint in checkpoints {
            let expected = expected_patch_suffix_after_checkpoint(&full_left, &checkpoint);
            let resumed = collect_subscriber_patches(&left.runtime, checkpoint.clone(), 4);
            assert_eq!(
                resumed,
                expected,
                "seed {seed} resume drifted after {:?}",
                checkpoint.position()
            );
        }
    }
}

#[test]
fn cdc_certification_savepoint_abandoned_work_never_leaks_into_stream_truth() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity_outcome(&mut runtime, "anchor-left");
    let right = create_entity_outcome(&mut runtime, "anchor-right");
    let left_entity = changed_entities(&left)[0];
    let right_entity = changed_entities(&right)[0];
    let checkpoint = checkpoint_for_schema_version(right.patch_position(), SchemaVersionId(1));

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("surviving"));
    let savepoint = txn.create_savepoint();
    txn.push_batch(batch_create("abandoned"));
    txn.push_batch(
        WorkerIntentBatch::new("abandoned-left").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: left_entity,
                payload: RecordPayload::StructuredJson(json!({"name":"abandoned-left"})),
            }),
        )),
    );
    txn.push_batch(
        WorkerIntentBatch::new("abandoned-right").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: right_entity,
                payload: RecordPayload::StructuredJson(json!({"name":"abandoned-right"})),
            }),
        )),
    );
    let rollback = txn.rollback_to_savepoint(savepoint).unwrap();
    txn.push_batch(
        WorkerIntentBatch::new("survived-left").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: left_entity,
                payload: RecordPayload::StructuredJson(json!({"name":"survived-left"})),
            }),
        )),
    );
    txn.push_batch(
        WorkerIntentBatch::new("survived-right").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: right_entity,
                payload: RecordPayload::StructuredJson(json!({"name":"survived-right"})),
            }),
        )),
    );
    let outcome = txn.commit().unwrap();

    assert!(rollback.summary().has_discarded_entity_creation());

    let subscriber = collect_subscriber_patches(&runtime, checkpoint, 1);
    assert!(subscriber
        .iter()
        .flat_map(|patch| patch.records.iter())
        .all(|record| !patch_detail_contains(record, "abandoned")));

    let patch_batch = runtime
        .publication_access()
        .read_patch_stream(PatchStreamRequest {
            after_position: Some(PatchStreamPosition(2)),
            max_commits: 32,
        })
        .unwrap();
    assert_eq!(subscriber, patch_batch.patches);

    let read = runtime
        .visibility_reads()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let names = read
        .entities()
        .iter()
        .filter_map(|record| read_entity_name(record))
        .collect::<Vec<_>>();

    assert!(names.contains(&"surviving"));
    assert!(names.contains(&"survived-left"));
    assert!(names.contains(&"survived-right"));
    assert!(!names.contains(&"abandoned"));
    assert!(!names.contains(&"abandoned-left"));
    assert!(!names.contains(&"abandoned-right"));
}

#[test]
fn cdc_certification_durable_recovery_matches_head_and_midstream_consumers() {
    let mut runtime = persisted_runtime_with_test_schema();
    for name in ["a", "b", "c", "d", "e", "f"] {
        let _ = create_entity_outcome(&mut runtime, name);
        if ["b", "d"].contains(&name) {
            runtime.durability_authority().checkpoint().unwrap();
        }
    }

    let full_head = collect_subscriber_patches_from_head(&runtime, 128);
    let mid_checkpoint = checkpoint_for_schema_version(PatchStreamPosition(3), SchemaVersionId(1));
    let expected_mid = expected_patch_suffix_after_checkpoint(&full_head, &mid_checkpoint);

    assert!(runtime
        .history_authority()
        .remove_commit_envelope_for_test(crate::history::data::CommitId(1)));
    assert!(runtime
        .history_authority()
        .remove_commit_envelope_for_test(crate::history::data::CommitId(2)));
    assert!(runtime
        .history_authority()
        .remove_commit_envelope_for_test(crate::history::data::CommitId(3)));

    let durable_mid = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(mid_checkpoint, 2))
        .unwrap();
    assert_eq!(
        durable_mid.recovery_decision.source,
        SubscriberRecoverySource::DurableCanonicalRecovery
    );

    let durable_mid_stitched = collect_subscriber_patches(
        &runtime,
        checkpoint_for_schema_version(PatchStreamPosition(3), SchemaVersionId(1)),
        2,
    );
    assert_eq!(durable_mid_stitched, expected_mid);

    let recovery_plan = runtime.durability_access().recovery_plan();
    let mut recovered = persisted_runtime_with_test_schema();
    recovered
        .durability_authority()
        .recover(recovery_plan)
        .unwrap();
    let recovered_patch_batch = recovered
        .publication_access()
        .read_patch_stream(PatchStreamRequest {
            after_position: Some(PatchStreamPosition(3)),
            max_commits: 32,
        })
        .unwrap();
    assert_eq!(durable_mid_stitched, recovered_patch_batch.patches);
}

#[test]
fn cdc_certification_thousand_step_random_resume_matrix_converges() {
    for seed in 0_u64..3 {
        let world = run_seeded_scenario(SeededScenarioConfig::geometry_kernel(
            seed,
            CertificationPressureProfile::ThousandStep,
        ));
        assert!(
            !world.trace.operations.is_empty(),
            "seed {seed} produced no operations"
        );
        assert_eq!(world.trace.seed, seed);

        let full_from_head = collect_subscriber_patches_from_head(&world.runtime, 2048);
        let checkpoints = random_checkpoints_from_patches(&full_from_head, seed ^ 0xA5A5_A5A5, 12);
        let views = collect_multi_subscriber_views(
            &world.runtime,
            &checkpoints,
            CertificationPressureProfile::ThousandStep.default_windows(),
        );

        assert_multi_subscriber_converges(
            &format!(
                "seed {} thousand-step random resume matrix over {} checkpoints",
                seed,
                world.checkpoints.len()
            ),
            &views,
            &full_from_head,
        );
    }
}

#[test]
fn cdc_certification_explicit_dependency_graph_resume_is_exact() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
    let source = create_entity_in_partition(&mut runtime, "source-a", PartitionId(7));
    let target = create_entity_in_partition(&mut runtime, "target-b", PartitionId(11));
    let _dependency =
        create_relation_in_partition(&mut runtime, source, target, "depends-on", PartitionId(29));
    let baseline_checkpoint = checkpoint_for_schema_version(
        runtime
            .publication_access()
            .latest_patch()
            .unwrap()
            .position,
        SchemaVersionId(1),
    );

    for step in 0..48 {
        let _ = update_entity(&mut runtime, target, &format!("target-b-{step}"));
        if step % 4 == 0 {
            let _ = update_entity(&mut runtime, source, &format!("source-a-{step}"));
        }
        if step % 6 == 0 {
            let _ = create_entity_in_partition(
                &mut runtime,
                &format!("churn-{step}"),
                match step % 3 {
                    0 => PartitionId(7),
                    1 => PartitionId(11),
                    _ => PartitionId(31),
                },
            );
        }
    }

    let full = collect_subscriber_patches(&runtime, baseline_checkpoint.clone(), 512);
    let dependency_checkpoints = random_checkpoints_from_patches(&full, 0xDEADBEEF, 10);
    let views = collect_multi_subscriber_views(&runtime, &dependency_checkpoints, &[1, 2, 3, 5, 8]);
    assert_multi_subscriber_converges(
        "explicit dependency graph resume exactness",
        &views,
        &collect_subscriber_patches_from_head(&runtime, 512),
    );
}

#[test]
fn cdc_certification_persisted_seeded_matrix_survives_checkpoint_compaction_and_recovery() {
    for seed in 0_u64..3 {
        let world = run_seeded_scenario(SeededScenarioConfig::persisted_geometry(
            seed,
            CertificationPressureProfile::HistoryPressure,
        ));
        let full_from_head = collect_subscriber_patches_from_head(&world.runtime, 4096);
        let checkpoints = random_checkpoints_from_patches(&full_from_head, seed ^ 0x55AA55AA, 10);
        let views = collect_multi_subscriber_views(&world.runtime, &checkpoints, &[1, 2, 4, 8, 16]);
        assert_multi_subscriber_converges(
            &format!("persisted seed {seed} pre-recovery convergence"),
            &views,
            &full_from_head,
        );

        let recovery_plan = world.runtime.durability_access().recovery_plan();
        let mut recovered = build_property_runtime(RuntimeHarnessMode::Persisted);
        recovered
            .durability_authority()
            .recover(recovery_plan)
            .unwrap();
        let recovered_patch_stream = collect_patch_stream_from_head(&recovered, 4096);
        assert_eq!(
            recovered_patch_stream, full_from_head,
            "persisted seed {seed} drifted after recovery"
        );
    }
}

#[test]
fn cdc_certification_subscriber_api_fuzz_matrix_stays_consistent_under_hostile_history_pressure() {
    for (profile, seed_range) in [
        (CertificationPressureProfile::Smoke, 0_u64..2),
        (CertificationPressureProfile::HistoryPressure, 2_u64..4),
    ] {
        for seed in seed_range {
            let world = run_seeded_scenario(SeededScenarioConfig::hostile_geometry(
                seed ^ 0xBAD5EED,
                profile,
            ));
            let head = collect_subscriber_patches_from_head(&world.runtime, 4096);
            let patch_head = collect_patch_stream_from_head(&world.runtime, 4096);
            assert_eq!(
                head, patch_head,
                "hostile seed {seed} patch/subscriber drifted"
            );

            let views = collect_fuzzed_subscriber_views(&world.runtime, &head, seed ^ 0xC0DEC0DE);
            assert_multi_subscriber_converges(
                &format!("hostile subscriber api fuzz seed {seed}"),
                &views,
                &head,
            );
        }
    }
}

#[test]
fn cdc_certification_rewrite_storm_preserves_exact_suffix_under_tiny_windows() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
    let entity = create_entity_in_partition(&mut runtime, "rewrite-storm-0", PartitionId(7));
    let profile = CertificationPressureProfile::RewriteStorm;
    let baseline_checkpoint = checkpoint_for_schema_version(
        runtime
            .publication_access()
            .latest_patch()
            .unwrap()
            .position,
        SchemaVersionId(1),
    );

    for step in 1..=profile.steps() {
        let _ = update_entity(&mut runtime, entity, &format!("rewrite-storm-{step}"));
        if step % 16 == 0 {
            let partition = match step % 4 {
                0 => PartitionId(7),
                1 => PartitionId(11),
                2 => PartitionId(29),
                _ => PartitionId(31),
            };
            let _ =
                create_entity_in_partition(&mut runtime, &format!("storm-churn-{step}"), partition);
        }
    }

    let head = collect_subscriber_patches_from_head(&runtime, 8192);
    let resumed = collect_subscriber_patches(&runtime, baseline_checkpoint.clone(), 1);
    let expected = expected_patch_suffix_after_checkpoint(&head, &baseline_checkpoint);
    assert_eq!(resumed, expected);

    let checkpoint_samples = random_checkpoints_from_patches(&head, 0x515E515E, 24);
    let views =
        collect_multi_subscriber_views(&runtime, &checkpoint_samples, profile.default_windows());
    assert_multi_subscriber_converges("rewrite storm tiny-window convergence", &views, &head);
}

#[test]
fn cdc_certification_restart_loops_do_not_leak_resume_state_across_sessions() {
    let world = run_seeded_scenario(SeededScenarioConfig::hostile_geometry(
        0xABCD1234,
        CertificationPressureProfile::HistoryPressure,
    ));
    let head = collect_subscriber_patches_from_head(&world.runtime, 4096);
    let checkpoints = random_checkpoints_from_patches(&head, 0xD15C0C7, 20);

    for checkpoint in checkpoints {
        let expected = expected_patch_suffix_after_checkpoint(&head, &checkpoint);
        let mut session_checkpoint = checkpoint.clone();
        let mut stitched = Vec::new();

        for cycle in 0..1024 {
            let batch = world
                .runtime
                .publication_access()
                .read_subscriber_stream(SubscriberResumeRequest::resume_after(
                    session_checkpoint.clone(),
                    (cycle % 3) + 1,
                ))
                .unwrap();
            if batch.patches.is_empty() {
                break;
            }
            stitched.extend(batch.patches.clone());
            let Some(next_checkpoint) = batch.next_checkpoint else {
                break;
            };
            if next_checkpoint == session_checkpoint {
                break;
            }
            session_checkpoint = next_checkpoint;
        }

        assert_eq!(stitched, expected);
    }
}

#[test]
fn cdc_certification_retention_truncation_recovers_exact_suffix_from_old_checkpoint() {
    let mut world = run_seeded_scenario(SeededScenarioConfig::persisted_geometry(
        0xFACE515E,
        CertificationPressureProfile::HistoryPressure,
    ));
    let head = collect_subscriber_patches_from_head(&world.runtime, 4096);
    let old_checkpoint = random_checkpoints_from_patches(&head, 0x0DCAFE, 1)
        .into_iter()
        .next()
        .unwrap_or_else(|| world.baseline_checkpoint.clone());

    for _ in 0..16 {
        let _ = world.runtime.retention_authority().run_pass();
    }

    let resumed = collect_subscriber_patches(&world.runtime, old_checkpoint.clone(), 3);
    let expected = expected_patch_suffix_after_checkpoint(&head, &old_checkpoint);
    assert_eq!(resumed, expected);
}

#[test]
fn cdc_certification_concurrent_branch_merge_pressure_keeps_subscriber_order_stable() {
    let world = run_seeded_scenario(SeededScenarioConfig {
        seed: 0xB12A6E55,
        steps: CertificationPressureProfile::HistoryPressure.steps(),
        checkpoint_stride: 8,
        runtime_mode: RuntimeHarnessMode::InMemory(RelationalRuntimeProfile::GeometryKernel),
        relation_pressure: true,
        durable_checkpoint_every: None,
        durable_compact_every: None,
        retention_pass_every: Some(4),
        branch_pressure: true,
        replacement_pressure: true,
    });
    let head = collect_subscriber_patches_from_head(&world.runtime, 4096);
    let patch_head = collect_patch_stream_from_head(&world.runtime, 4096);
    assert_eq!(head, patch_head);

    let views = collect_fuzzed_subscriber_views(&world.runtime, &head, 0xB12A6E55);
    assert_multi_subscriber_converges("branch merge pressure convergence", &views, &head);
}

prop_compose! {
    fn arb_scenario_operation()(
        tag in 0usize..9,
        a in 0usize..16,
        b in 0usize..16,
        _c in 0usize..16,
        partition in 0usize..4,
    ) -> ScenarioOperation {
        let partition = match partition {
            0 => PartitionId(7),
            1 => PartitionId(11),
            2 => PartitionId(29),
            _ => PartitionId(31),
        };
        match tag {
            0 => ScenarioOperation::CreateEntity { partition, name: String::new() },
            1 => ScenarioOperation::UpdateEntity { entity_slot: a, name: String::new(), branch_slot: 0 },
            2 => ScenarioOperation::ReplaceEntity { entity_slot: a, name: String::new(), branch_slot: 0, partition },
            3 => ScenarioOperation::DeleteEntity { entity_slot: a, branch_slot: 0 },
            4 => ScenarioOperation::CreateRelation {
                source_slot: a,
                target_slot: b,
                client_key: String::new(),
                partition,
            },
            5 => ScenarioOperation::DeleteRelation { relation_slot: a },
            6 => ScenarioOperation::CaptureSnapshot,
            7 => ScenarioOperation::ReleaseSnapshot { snapshot_slot: a },
            8 => ScenarioOperation::RunRetentionPass,
            _ => ScenarioOperation::DurableCheckpoint,
        }
    }
}

prop_compose! {
    fn arb_branch_operation()(
        tag in 0usize..6,
        a in 0usize..12,
    ) -> ScenarioOperation {
        match tag {
            0 => ScenarioOperation::CreateBranch { branch_name: String::new(), from_branch_slot: 0 },
            1 => ScenarioOperation::UpdateEntity { entity_slot: a, name: String::new(), branch_slot: a },
            2 => ScenarioOperation::MergeBranchIntoMain { branch_slot: a },
            3 => ScenarioOperation::CaptureSnapshot,
            4 => ScenarioOperation::ReleaseSnapshot { snapshot_slot: a },
            5 => ScenarioOperation::RunRetentionPass,
            _ => ScenarioOperation::DurableCheckpoint,
        }
    }
}

prop_compose! {
    fn arb_windows()(windows in vec(1usize..10, 3..8)) -> Vec<usize> {
        let mut windows = windows;
        windows.sort_unstable();
        windows.dedup();
        windows
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 16,
        max_global_rejects: 4096,
        .. ProptestConfig::default()
    })]

    #[test]
    fn cdc_property_random_operation_matrix_converges(
        operations in vec(arb_scenario_operation(), 24..96),
        windows in arb_windows(),
        checkpoint_seed in any::<u64>(),
    ) {
        let world = run_property_scenario(
            operations.clone(),
            RuntimeHarnessMode::InMemory(RelationalRuntimeProfile::GeometryKernel),
        );
        let mut replay_world = run_property_scenario(
            operations,
            RuntimeHarnessMode::InMemory(RelationalRuntimeProfile::GeometryKernel),
        );
        let head = collect_subscriber_patches_from_head(&world.runtime, 4096);
        let patch_head = collect_patch_stream_from_head(&world.runtime, 4096);
        prop_assert_eq!(&head, &patch_head);
        prop_assert_eq!(&head, &collect_subscriber_patches_from_head(&replay_world.runtime, 4096));

        let checkpoints = random_checkpoints_from_patches(&head, checkpoint_seed, 12);
        let views = collect_multi_subscriber_views(&world.runtime, &checkpoints, &windows);
        assert_multi_subscriber_converges(
            "property random operation matrix",
            &views,
            &head,
        );

        let mut world_for_truth = world;
        let truth = VisibleTruthSummary::capture(&mut world_for_truth.runtime);
        let replay_truth = VisibleTruthSummary::capture(&mut replay_world.runtime);
        assert_visible_truth_matches(
            "property random operation matrix replay truth",
            &truth,
            &replay_truth,
        );
    }

    #[test]
    fn cdc_property_persisted_random_operation_matrix_recovers(
        operations in vec(arb_scenario_operation(), 16..64),
        checkpoint_seed in any::<u64>(),
    ) {
        let mut world = run_property_scenario(
            operations,
            RuntimeHarnessMode::Persisted,
        );
        let head = collect_subscriber_patches_from_head(&world.runtime, 4096);
        let checkpoints = random_checkpoints_from_patches(&head, checkpoint_seed, 8);
        let views = collect_multi_subscriber_views(&world.runtime, &checkpoints, &[1, 2, 4, 8]);
        assert_multi_subscriber_converges(
            "property persisted random operation matrix",
            &views,
            &head,
        );

        world.runtime.durability_authority().checkpoint().unwrap();
        let recovery_plan = world.runtime.durability_access().recovery_plan();
        let mut recovered = build_property_runtime(RuntimeHarnessMode::Persisted);
        recovered.durability_authority().recover(recovery_plan).unwrap();
        let recovered_patch_stream = collect_patch_stream_from_head(&recovered, 4096);
        prop_assert_eq!(recovered_patch_stream, head);
        let truth = VisibleTruthSummary::capture(&mut world.runtime);
        let recovered_truth = VisibleTruthSummary::capture(&mut recovered);
        assert_visible_truth_matches(
            "property persisted random operation matrix recovered truth",
            &truth,
            &recovered_truth,
        );
    }

    #[test]
    fn cdc_property_branch_local_update_and_merge_matrix_converges(
        operations in vec(arb_branch_operation(), 16..72),
        checkpoint_seed in any::<u64>(),
    ) {
        let world = run_property_scenario(
            operations,
            RuntimeHarnessMode::InMemory(RelationalRuntimeProfile::GeometryKernel),
        );
        let head = collect_subscriber_patches_from_head(&world.runtime, 4096);
        let checkpoints = random_checkpoints_from_patches(&head, checkpoint_seed, 8);
        let views = collect_multi_subscriber_views(&world.runtime, &checkpoints, &[1, 2, 4, 8]);
        assert_multi_subscriber_converges(
            "property branch-local update and merge matrix",
            &views,
            &head,
        );
    }
}

fn patch_detail_contains(record: &crate::facade::publication::PatchRecord, needle: &str) -> bool {
    match &record.detail {
        PatchDetail::StructuredJson(value) => value.to_string().contains(needle),
        PatchDetail::Payload(payload) => payload
            .as_json()
            .map(|value| value.to_string().contains(needle))
            .unwrap_or(false),
        PatchDetail::DenseBitset(_) => false,
    }
}
