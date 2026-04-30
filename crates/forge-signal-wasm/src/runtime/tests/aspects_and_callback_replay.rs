use super::support::*;

#[test]
fn diagnostics_tier_changes_richness_only_not_merge_truth() {
    let development = RuntimePolicySpec {
        preset: RuntimePolicyPreset::WebDevelopment,
    };
    let kernel = RuntimePolicySpec {
        preset: RuntimePolicyPreset::Kernel,
    };

    let (mut development_runtime, development_main, development_feature, _) =
        build_adversarial_merge_runtime(development);
    let (mut kernel_runtime, kernel_main, kernel_feature, _) =
        build_adversarial_merge_runtime(kernel);

    let development_plan = development_runtime
        .plan_merge_branches_with_proof(development_feature, development_main)
        .unwrap();
    let kernel_plan = kernel_runtime
        .plan_merge_branches_with_proof(kernel_feature, kernel_main)
        .unwrap();
    assert_eq!(
        development_plan.proof.plan_digest,
        kernel_plan.proof.plan_digest
    );
    assert_eq!(
        development_plan.proof.semantics_digest,
        kernel_plan.proof.semantics_digest
    );

    let development_result = development_runtime
        .merge_branches_with_proof(development_feature, development_main)
        .unwrap();
    let kernel_result = kernel_runtime
        .merge_branches_with_proof(kernel_feature, kernel_main)
        .unwrap();

    assert_eq!(
        development_result.proof.result_digest,
        kernel_result.proof.result_digest
    );
    assert_eq!(
        development_result.result.selected_semantics,
        kernel_result.result.selected_semantics
    );

    let development_state = development_runtime
        .branch_state_proof(development_main)
        .unwrap();
    let kernel_state = kernel_runtime.branch_state_proof(kernel_main).unwrap();
    assert_eq!(development_state.state_digest, kernel_state.state_digest);
}

#[test]
fn aspect_filtered_reads_ignore_irrelevant_aspect_updates() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "sensor".to_owned(),
            initial: SignalValue::Number(10.0),
            produces_aspects: Some(vec![1, 2]),
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "display".to_owned(),
            reads: vec![RecipeReadSpec::Signal(
                crate::recipe::model::RecipeReadSignalSpec {
                    id: "sensor".to_owned(),
                    scope: None,
                    aspects: crate::recipe::model::AspectSelectionSpec {
                        aspect: Some(1),
                        aspects: None,
                    },
                },
            )],
            expr: read("sensor"),
            when: None,
            identity: None,
            produces_aspects: None,
        })
        .unwrap();

    assert_eq!(
        runtime.read_value("display").unwrap(),
        SignalValue::Number(10.0)
    );
    assert_eq!(
        runtime.read_versions(vec!["display".to_owned()]).unwrap()[0].version,
        1
    );

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "sensor".to_owned(),
            value: SignalValue::Number(99.0),
            aspect: None,
            aspects: Some(vec![2]),
        }])
        .unwrap();

    assert_eq!(
        runtime.read_value("display").unwrap(),
        SignalValue::Number(10.0),
        "display should not recompute when only an unread aspect changes"
    );
    assert_eq!(
        runtime.read_versions(vec!["display".to_owned()]).unwrap()[0].version,
        1,
        "unread aspect churn must not advance the derived node version"
    );

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "sensor".to_owned(),
            value: SignalValue::Number(42.0),
            aspect: None,
            aspects: Some(vec![1]),
        }])
        .unwrap();

    assert_eq!(
        runtime.read_value("display").unwrap(),
        SignalValue::Number(42.0)
    );
    assert_eq!(
        runtime.read_versions(vec!["display".to_owned()]).unwrap()[0].version,
        2
    );
}

#[test]
fn multi_aspect_versions_survive_snapshot_round_trip() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "sensor".to_owned(),
            initial: SignalValue::Number(10.0),
            produces_aspects: Some(vec![1, 2]),
        })
        .unwrap();

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "sensor".to_owned(),
            value: SignalValue::Number(15.0),
            aspect: None,
            aspects: Some(vec![2]),
        }])
        .unwrap();

    let before = runtime.read_versions(vec!["sensor".to_owned()]).unwrap();
    assert_eq!(before[0].aspect_versions.len(), 2);
    assert_eq!(before[0].aspect_versions[0].aspect, 1);
    assert_eq!(before[0].aspect_versions[0].version, 1);
    assert_eq!(before[0].aspect_versions[1].aspect, 2);
    assert_eq!(before[0].aspect_versions[1].version, 2);

    let snapshot = runtime.snapshot().unwrap();

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "sensor".to_owned(),
            value: SignalValue::Number(25.0),
            aspect: None,
            aspects: Some(vec![1]),
        }])
        .unwrap();

    runtime.restore_snapshot(snapshot).unwrap();

    let restored = runtime.read_versions(vec!["sensor".to_owned()]).unwrap();
    assert_eq!(restored[0].aspect_versions, before[0].aspect_versions);
}

#[test]
fn callback_snapshot_restore_denies_missing_callback_registrations() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "count".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_web_computed_native_callback(
            "doubled".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(2.0),
                    captured_read_ids: vec!["count".to_owned()],
                    runtime_read_breadth: 1,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();

    let snapshot = runtime.snapshot().unwrap();
    assert!(runtime
        .dispose_web_computed_callback_for_test("doubled")
        .unwrap());

    let err = runtime.restore_snapshot(snapshot).unwrap_err();
    assert_eq!(err.code, "computeCallbackUnavailableForRestore");
    assert!(err.message.contains("doubled"));
    let summary = runtime.web_performance_summary();
    assert_eq!(summary.compute_callback_missing_unavailability_count, 1);
}

#[test]
fn callback_replay_and_lineage_surfaces_report_callback_availability() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "count".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_web_computed_native_callback(
            "doubled".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(2.0),
                    captured_read_ids: vec!["count".to_owned()],
                    runtime_read_breadth: 1,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();

    let _ = runtime.read_value("doubled").unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "count".to_owned(),
            value: SignalValue::Number(2.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    let replay = runtime.replay_for_id("doubled").unwrap();
    assert!(replay.frames.iter().any(|frame| frame
        .callback
        .as_ref()
        .map(|callback| callback.registered)
        == Some(true)));
    let branch_replay = runtime
        .replay_for_branch(runtime.current_branch().id.0)
        .unwrap();
    assert!(branch_replay.frames.iter().any(|frame| frame
        .callback
        .as_ref()
        .map(|callback| callback.registered)
        == Some(true)));
    let lineage = runtime.lineage_for_id("doubled").unwrap();
    assert!(lineage.events.iter().any(|event| event
        .callback
        .as_ref()
        .map(|callback| callback.registered)
        == Some(true)));

    assert!(runtime
        .dispose_web_computed_callback_for_test("doubled")
        .unwrap());

    let replay = runtime.replay_for_id("doubled").unwrap();
    assert!(replay.frames.iter().any(|frame| {
        frame.callback.as_ref().map(|callback| {
            !callback.registered
                && callback.unavailable_reason.as_deref()
                    == Some("computeCallbackUnavailableForReplay")
        }) == Some(true)
    }));
    let branch_replay = runtime
        .replay_for_branch(runtime.current_branch().id.0)
        .unwrap();
    assert!(branch_replay.frames.iter().any(|frame| {
        frame.callback.as_ref().map(|callback| {
            !callback.registered
                && callback.unavailable_reason.as_deref()
                    == Some("computeCallbackUnavailableForReplay")
        }) == Some(true)
    }));
    let lineage = runtime.lineage_for_id("doubled").unwrap();
    assert!(lineage.events.iter().any(|event| {
        event.callback.as_ref().map(|callback| {
            !callback.registered
                && callback.unavailable_reason.as_deref()
                    == Some("computeCallbackUnavailableForReplay")
        }) == Some(true)
    }));
}

#[cfg(feature = "profile-extended")]
#[test]
fn extended_profile_accepts_aspect_slot_fifteen() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "sensor".to_owned(),
            initial: SignalValue::Number(10.0),
            produces_aspects: Some(vec![15]),
        })
        .unwrap();

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "sensor".to_owned(),
            value: SignalValue::Number(15.0),
            aspect: None,
            aspects: Some(vec![15]),
        }])
        .unwrap();

    let versions = runtime.read_versions(vec!["sensor".to_owned()]).unwrap();
    assert_eq!(versions[0].aspect_versions.len(), 1);
    assert_eq!(versions[0].aspect_versions[0].aspect, 15);
    assert_eq!(versions[0].aspect_versions[0].version, 2);
}

#[test]
fn replay_artifact_proof_reports_typed_mismatch_classes() {
    let (mut runtime, main_branch_id, feature_branch_id, _) =
        build_adversarial_merge_runtime(RuntimePolicySpec::default());

    let expected_plan = runtime
        .plan_merge_branches_with_proof(feature_branch_id, main_branch_id)
        .unwrap();
    let expected_result = runtime
        .merge_branches_with_proof(feature_branch_id, main_branch_id)
        .unwrap();
    let expected_state = runtime.branch_state_proof(main_branch_id).unwrap();

    let replayed_branch = runtime
        .create_branch("replayed-divergent".to_owned())
        .unwrap();
    runtime.switch_branch(replayed_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(7.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    let report = runtime
        .replay_artifact_proof(
            forge_signal::facade::adapters::ReplayArtifactProofInput {
                proof_schema_version: expected_result.proof.proof_schema_version.clone(),
                registry_bundle_digest: Some(expected_result.proof.registry_bundle_digest.clone()),
                lowered_strategy_bundle_digest: Some(
                    expected_result.proof.lowered_strategy_bundle_digest.clone(),
                ),
                merge_plan_digest: Some(expected_plan.proof.plan_digest.clone()),
                merge_result_digest: Some(expected_result.proof.result_digest.clone()),
                lineage_digest: Some(expected_result.proof.lineage_digest.clone()),
                branch_state_digest: expected_state.state_digest.clone(),
            },
            replayed_branch.id.0,
        )
        .unwrap();

    assert!(!report.parity);
    assert!(report
        .mismatch_classes
        .contains(&forge_signal::facade::adapters::ReplayMismatchClass::BranchStateDigestMismatch));
}
