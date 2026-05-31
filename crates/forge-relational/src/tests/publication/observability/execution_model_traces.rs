use super::fixtures::*;

#[test]
fn aspect_traces_and_diagnostics_are_stable_across_supported_execution_models() {
    let serial_diagnostics = RelationalDiagnosticsProfile {
        detailed_traces_enabled: true,
        ..RelationalDiagnosticsProfile::default()
    };
    let mut serial = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(declared_aspect_schema_registry(
            CascadeDeletePolicy::CascadeDeleteRelations,
        ))
        .diagnostics(serial_diagnostics.clone())
        .execution_model(crate::facade::runtime::RelationalExecutionModel::SerialAuthority)
        .build();
    let mut staged = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(declared_aspect_schema_registry(
            CascadeDeletePolicy::CascadeDeleteRelations,
        ))
        .diagnostics(serial_diagnostics.clone())
        .execution_model(
            crate::facade::runtime::RelationalExecutionModel::StagedParallelPreparation,
        )
        .build();
    let mut post_commit = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(declared_aspect_schema_registry(
            CascadeDeletePolicy::CascadeDeleteRelations,
        ))
        .diagnostics(serial_diagnostics)
        .execution_model(
            crate::facade::runtime::RelationalExecutionModel::ParallelPostCommitConsumption,
        )
        .build();

    let serial_outcome = create_entity_outcome(&mut serial, "trace-stable");
    let staged_outcome = create_entity_outcome(&mut staged, "trace-stable");
    let post_commit_outcome = create_entity_outcome(&mut post_commit, "trace-stable");

    assert_eq!(
        serial_outcome.aspect_evaluation_traces(),
        staged_outcome.aspect_evaluation_traces()
    );
    assert_eq!(
        serial_outcome.aspect_evaluation_traces(),
        post_commit_outcome.aspect_evaluation_traces()
    );
    assert_eq!(
        serial_outcome.aspect_emission_traces(),
        staged_outcome.aspect_emission_traces()
    );
    assert_eq!(
        serial_outcome.aspect_emission_traces(),
        post_commit_outcome.aspect_emission_traces()
    );
    assert_eq!(serial_outcome.patch(), staged_outcome.patch());
    assert_eq!(serial_outcome.patch(), post_commit_outcome.patch());
    assert_eq!(
        aspect_relevant_diagnostics(serial_outcome.diagnostics()),
        aspect_relevant_diagnostics(staged_outcome.diagnostics())
    );
    assert_eq!(
        aspect_relevant_diagnostics(serial_outcome.diagnostics()),
        aspect_relevant_diagnostics(post_commit_outcome.diagnostics())
    );
    let _ = assert_patch_truth_invariants(&serial_outcome);
    let _ = assert_patch_truth_invariants(&staged_outcome);
    let _ = assert_patch_truth_invariants(&post_commit_outcome);
}

fn aspect_relevant_diagnostics(
    diagnostics: &[crate::facade::diagnostics::RelationalDiagnosticArtifact],
) -> Vec<crate::facade::diagnostics::RelationalDiagnosticArtifact> {
    diagnostics
        .iter()
        .filter_map(|artifact| {
            let entries = artifact
                .entries
                .iter()
                .filter(|entry| {
                    matches!(
                        entry.code,
                        DiagnosticCode::AspectEvaluationTraced
                            | DiagnosticCode::AspectEmissionTraced
                            | DiagnosticCode::EntityCreated
                            | DiagnosticCode::CommitPublished
                    )
                })
                .cloned()
                .collect::<Vec<_>>();
            (!entries.is_empty()).then_some(
                crate::facade::diagnostics::RelationalDiagnosticArtifact {
                    scope: artifact.scope.clone(),
                    kind: artifact.kind.clone(),
                    determinism: artifact.determinism.clone(),
                    entries,
                },
            )
        })
        .collect()
}

#[test]
fn geometry_operational_hot_path_policy_suppresses_detailed_traces() {
    let mut runtime = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::GeometryKernel)
        .schema_registry(test_schema_registry())
        .diagnostics(RelationalDiagnosticsProfile::geometry_operational_hot_path())
        .build();

    let _ = create_entity_outcome(&mut runtime, "geometry-hot-policy");
    let diagnostics = runtime.publication().diagnostics();

    assert!(diagnostics.artifacts().iter().any(|artifact| {
        artifact.scope == DiagnosticsScope::Transaction
            && artifact.kind == DiagnosticsArtifactKind::MinimalSummary
    }));
    assert!(!diagnostics
        .artifacts()
        .iter()
        .any(|artifact| { artifact.kind == DiagnosticsArtifactKind::DetailedTrace }));
}

#[test]
fn chip_rich_certification_policy_keeps_detailed_traces_available() {
    let mut runtime = RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::ChipSimulation)
        .schema_registry(test_schema_registry())
        .diagnostics(RelationalDiagnosticsProfile::chip_rich_certification())
        .build();

    let _ = create_entity_outcome(&mut runtime, "chip-rich-policy");
    let diagnostics = runtime.publication().diagnostics();

    assert!(diagnostics.artifacts().iter().any(|artifact| {
        artifact.scope == DiagnosticsScope::Transaction
            && artifact.kind == DiagnosticsArtifactKind::MinimalSummary
    }));
    assert!(diagnostics
        .artifacts()
        .iter()
        .any(|artifact| { artifact.kind == DiagnosticsArtifactKind::DetailedTrace }));
}

#[test]
fn geometry_operational_hot_path_policy_defers_replay_reconstructable_artifacts() {
    let profile = RelationalDiagnosticsProfile::geometry_operational_hot_path();
    let comparison_policy = profile.artifact_policy(
        DiagnosticsScope::Replay,
        DiagnosticsArtifactKind::Comparison,
    );

    assert_eq!(
        comparison_policy.delivery_class,
        DiagnosticsDeliveryClass::ReconstructableFromReplay
    );
    assert!(!comparison_policy.enabled);
    assert_eq!(comparison_policy.max_entries, 0);

    let summary_policy = profile.artifact_policy(
        DiagnosticsScope::Transaction,
        DiagnosticsArtifactKind::MinimalSummary,
    );
    assert_eq!(
        summary_policy.delivery_class,
        DiagnosticsDeliveryClass::MustBeHot
    );
    assert!(summary_policy.enabled);
    assert!(summary_policy.max_entries > 0);
}
