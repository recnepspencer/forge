use crate::runtime::placement::placement_category::WorkerPlacementCategory;

use crate::runtime::tests::support::*;

fn runtime() -> RuntimeCore {
    RuntimeCore::new(RuntimePolicySpec::default()).unwrap()
}

#[test]
fn expression_with_constantized_callback_is_worker_executable() {
    let mut runtime = runtime();
    runtime
        .define_source(SourceSpec {
            id: "base".to_owned(),
            initial: SignalValue::Number(2.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "exprDerived".to_owned(),
            reads: vec![RecipeReadSpec::LegacyId("base".to_owned())],
            expr: read("base"),
            when: None,
            identity: None,
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_web_computed_native_callback(
            "constantCallback".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(5.0),
                    captured_read_ids: Vec::new(),
                    captured_host_capability_reads: Vec::new(),
                    runtime_read_breadth: 0,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();

    let summary = runtime.worker_placement_summary().unwrap();

    assert_eq!(summary.total_declaration_count, 2);
    assert_eq!(summary.worker_executable_count, 2);
    assert_eq!(summary.main_thread_hosted_count, 0);
    assert_eq!(summary.denied_count, 0);
    assert_eq!(summary.raw_proof_count, 2);
    assert_eq!(summary.classified_outcome_count, 2);
    assert_eq!(
        summary
            .declarations
            .iter()
            .map(|declaration| (
                declaration.id.as_str(),
                declaration.category,
                declaration.declaration_origin.as_str(),
                declaration.outcome.as_str(),
                declaration.proof_stage.as_str(),
            ))
            .collect::<Vec<_>>(),
        [
            (
                "constantCallback",
                WorkerPlacementCategory::WorkerExecutable,
                "callbackConstantizedNoSignalReads",
                "success",
                "placementClassified",
            ),
            (
                "exprDerived",
                WorkerPlacementCategory::WorkerExecutable,
                "exprSpec",
                "success",
                "placementClassified",
            ),
        ]
    );
}

#[test]
fn signal_tracked_callback_is_denied_into_main_thread_hosted_lane() {
    let mut runtime = runtime();
    runtime
        .define_source(SourceSpec {
            id: "base".to_owned(),
            initial: SignalValue::Number(2.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_web_computed_native_callback(
            "trackedCallback".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(7.0),
                    captured_read_ids: vec!["base".to_owned()],
                    captured_host_capability_reads: Vec::new(),
                    runtime_read_breadth: 1,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();

    let summary = runtime.worker_placement_summary().unwrap();

    assert_eq!(summary.total_declaration_count, 1);
    assert_eq!(summary.worker_executable_count, 0);
    assert_eq!(summary.main_thread_hosted_count, 1);
    assert_eq!(summary.denied_count, 1);
    let tracked = summary
        .declarations
        .iter()
        .find(|declaration| declaration.id == "trackedCallback")
        .expect("tracked callback placement declaration exists");
    assert_eq!(tracked.category, WorkerPlacementCategory::MainThreadHosted);
    assert_eq!(tracked.signal_kind, "computed");
    assert_eq!(tracked.declaration_origin, "callbackSignalTracked");
    assert_eq!(tracked.outcome, "denied");
    assert_eq!(tracked.proof_stage, "rawDenied");
    assert!(tracked.reason.contains("process-local"));
}

#[test]
fn callback_placement_eligibility_certifies_closed_request_lane_without_fallback() {
    let mut runtime = runtime();
    runtime
        .define_source(SourceSpec {
            id: "base".to_owned(),
            initial: SignalValue::Number(2.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_web_computed_native_callback(
            "signalOnlyCallback".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(7.0),
                    captured_read_ids: vec!["base".to_owned()],
                    captured_host_capability_reads: Vec::new(),
                    runtime_read_breadth: 1,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();
    runtime
        .define_web_computed_native_callback(
            "hostReadingCallback".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::String("visible".to_owned()),
                    captured_read_ids: Vec::new(),
                    captured_host_capability_reads: vec![
                        compute_callbacks::CapturedHostCapabilityRead {
                            family: "visibility".to_owned(),
                            registration_id: "documentVisibility".to_owned(),
                            compatibility: "mainThreadOnly".to_owned(),
                        },
                    ],
                    runtime_read_breadth: 0,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();

    let package = runtime.worker_callback_placement_eligibility().unwrap();

    assert_eq!(package.certification_family, "callbackPlacementEligibility");
    assert_eq!(package.callback_declaration_count, 2);
    assert_eq!(package.worker_executable_callback_count, 0);
    assert_eq!(package.main_thread_hosted_callback_count, 2);
    assert_eq!(package.unavailable_callback_count, 0);
    assert_eq!(package.fallback_count, 0);
    assert!(package.raw_callback_transport_denied);
    assert!(package.broad_placement_collapse_denied);
    let signal_only = package
        .rows
        .iter()
        .find(|row| row.declaration_id == "signalOnlyCallback")
        .expect("signal-only callback row exists");
    assert_eq!(signal_only.callback_runtime_read_count, 1);
    assert_eq!(signal_only.host_capability_read_count, 0);
    assert!(signal_only.main_thread_hosted_lane_requires_closed_request);
    let host_reading = package
        .rows
        .iter()
        .find(|row| row.declaration_id == "hostReadingCallback")
        .expect("host-reading callback row exists");
    assert_eq!(host_reading.host_capability_read_count, 1);
    assert!(host_reading.reason.contains("typed host capabilities"));
    assert_digest_shape(&package.placement_digest);
    assert_digest_shape(&package.denial_digest);
    assert_digest_shape(&package.fallback_digest);
    assert_digest_shape(&package.capability_availability_digest);
    assert_digest_shape(&package.replay_import_compatibility_digest);
    assert_digest_shape(&package.placement_identity_digest);
    assert_digest_shape(&package.performance_digest);
}

#[test]
fn callback_placement_eligibility_preserves_shared_debug_shape_postures() {
    let mut runtime = runtime();
    runtime
        .define_web_computed_native_callback(
            "sharedDebugShapePortable".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(1.0),
                    captured_read_ids: Vec::new(),
                    captured_host_capability_reads: Vec::new(),
                    runtime_read_breadth: 0,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();
    runtime
        .define_web_computed_native_callback(
            "sharedDebugShapeHostBound".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(1.0),
                    captured_read_ids: Vec::new(),
                    captured_host_capability_reads: vec![
                        compute_callbacks::CapturedHostCapabilityRead {
                            family: "debugVisibility".to_owned(),
                            registration_id: "sharedDebugShape".to_owned(),
                            compatibility: "mainThreadOnly".to_owned(),
                        },
                    ],
                    runtime_read_breadth: 0,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();

    let package = runtime.worker_callback_placement_eligibility().unwrap();

    assert_eq!(package.callback_declaration_count, 2);
    assert_eq!(package.worker_executable_callback_count, 1);
    assert_eq!(package.main_thread_hosted_callback_count, 1);
    assert_eq!(
        package
            .rows
            .iter()
            .map(|row| row.declaration_id.as_str())
            .collect::<Vec<_>>(),
        ["sharedDebugShapeHostBound", "sharedDebugShapePortable"]
    );
    assert_eq!(
        package
            .rows
            .iter()
            .map(|row| (row.declaration_id.as_str(), row.category))
            .collect::<Vec<_>>(),
        [
            (
                "sharedDebugShapeHostBound",
                WorkerPlacementCategory::MainThreadHosted,
            ),
            (
                "sharedDebugShapePortable",
                WorkerPlacementCategory::WorkerExecutable,
            ),
        ]
    );
}
