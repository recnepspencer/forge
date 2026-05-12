use super::super::super::support::*;

#[test]
fn runtime_support_profiles_expose_facade_family_posture() {
    let primary_runtime = stateful_bridge_task_runtime();
    let bridge_runtime = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .build_backend_from_parts()
        .build()
        .expect("complete backend parts should build");

    for family in [
        ForgeQueryRuntimeFacadeFamily::Read,
        ForgeQueryRuntimeFacadeFamily::Live,
        ForgeQueryRuntimeFacadeFamily::Computed,
        ForgeQueryRuntimeFacadeFamily::Effect,
        ForgeQueryRuntimeFacadeFamily::BranchPreview,
        ForgeQueryRuntimeFacadeFamily::Write,
        ForgeQueryRuntimeFacadeFamily::Inspect,
    ] {
        assert_eq!(
            primary_runtime
                .support_profile()
                .support_for(family)
                .expect("primary support row should exist")
                .status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
        assert_eq!(
            bridge_runtime
                .support_profile()
                .support_for(family)
                .expect("bridge-backed support row should exist")
                .status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
    }

    assert_eq!(
        primary_runtime.support_profile().posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );
    assert_eq!(
        bridge_runtime.support_profile().posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );

    let bridge_support_profile = bridge_runtime.support_profile();
    let write_support = bridge_support_profile
        .support_for(ForgeQueryRuntimeFacadeFamily::Write)
        .expect("write support row should exist");
    assert!(write_support
        .evidence()
        .iter()
        .any(|evidence| evidence == "authoritative-mutation-evidence"));

    let inspect_support = bridge_support_profile
        .support_for(ForgeQueryRuntimeFacadeFamily::Inspect)
        .expect("inspect support row should exist");
    assert!(inspect_support
        .authority_lanes()
        .contains(&ForgeQueryAuthorityLane::BranchLocalTruth));
    assert!(inspect_support
        .authority_lanes()
        .contains(&ForgeQueryAuthorityLane::PendingWriteIntent));
}

#[test]
fn runtime_public_api_contract_marks_future_async_surfaces_as_deferred() {
    let runtime = stateful_bridge_task_runtime();
    let contract = runtime.public_api_contract();

    assert_eq!(
        contract.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );
    assert_eq!(contract.deferred_family_count(), 5);
    assert!(!contract.contract_digest().is_empty());

    for (family, expected_reason) in [
        (ForgeQueryRuntimeFacadeFamily::Temporal, "Milestone 9.4"),
        (
            ForgeQueryRuntimeFacadeFamily::AsyncResource,
            "Milestone 9.5",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
            "Milestone 9.6",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
            "Milestone 10",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
            "Milestone 11",
        ),
    ] {
        let row = contract
            .family(family)
            .expect("future support gate row should exist");
        assert_eq!(
            row.status(),
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt
        );
        assert!(row
            .reason()
            .is_some_and(|reason| reason.contains(expected_reason)));
        assert!(row.authority_lanes().is_empty());
        assert!(row.evidence().is_empty());
    }
}

#[test]
fn runtime_public_support_matrix_freezes_stable_deferred_and_unsupported_rows() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.support-matrix")
        .expect("task runtime should open a named workspace");
    let matrix = workspace.public_support_matrix();
    let contract = workspace.public_api_contract();

    assert_eq!(
        matrix.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );
    assert_eq!(
        matrix.stable_row_count(),
        contract.stable_family_count() + 1
    );
    assert_eq!(
        matrix.deferred_row_count(),
        contract.deferred_family_count() + 1
    );
    assert_eq!(
        matrix.unsupported_row_count(),
        contract.unsupported_family_count()
    );
    assert_eq!(
        matrix.parallel_api_forbidden_row_count(),
        matrix.rows().len()
    );
    assert_eq!(
        matrix.fail_closed_row_count(),
        matrix.deferred_row_count() + matrix.unsupported_row_count()
    );

    let certification = matrix
        .row("authoritative-mutation-evidence-certification")
        .expect("authority-evidence gate row must be explicit");
    assert_eq!(certification.facade_family(), None);
    assert_eq!(
        certification.status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );
    assert_eq!(
        certification.owner_milestone(),
        "Runtime Authoritative Mutation Evidence Gate"
    );
    assert!(certification.parallel_api_forbidden());
    assert!(!certification.admission_fail_closed());
    assert!(certification.support_contract_digest().is_some());
}

#[test]
fn runtime_public_support_gate_denies_deferred_and_unsupported_families_before_use() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.support-gate")
        .expect("task runtime should open a named workspace");

    let read = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Read)
        .expect("supported read family should admit");
    assert_eq!(read.family(), ForgeQueryRuntimeFacadeFamily::Read);
    assert_eq!(
        read.status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );

    for (family, expected_reason) in [
        (ForgeQueryRuntimeFacadeFamily::Temporal, "Milestone 9.4"),
        (
            ForgeQueryRuntimeFacadeFamily::AsyncResource,
            "Milestone 9.5",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
            "Milestone 9.6",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
            "Milestone 10",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
            "Milestone 11",
        ),
        (
            ForgeQueryRuntimeFacadeFamily::Intent,
            "intent commit strategies",
        ),
    ] {
        let error = workspace
            .admit_public_api_family(family)
            .expect_err("unsupported or deferred public API family should fail closed");
        match error {
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
                assert_eq!(denial.family(), family);
                assert!(denial.reason().contains(expected_reason));
            }
            other => panic!("expected typed public support denial, got {other:?}"),
        }
    }
}

#[test]
fn runtime_public_api_naming_contract_prefers_workspace_surface_names() {
    let contract = ForgeQueryRuntime::public_api_naming_contract();

    assert_eq!(contract.preferred_name_for("workspace"), Some("workspace"));
    assert_eq!(contract.preferred_name_for("insert"), Some("insert"));
    assert_eq!(contract.preferred_name_for("update"), Some("update"));
    assert_eq!(contract.preferred_name_for("delete"), Some("delete"));
    assert_eq!(contract.preferred_name_for("batch"), Some("batch"));
    assert_eq!(contract.preferred_name_for("inspect"), Some("inspect"));
    assert!(contract.rows().iter().any(|row| {
        row.concept() == "insert"
            && row
                .alternate_names()
                .iter()
                .any(|name| name.contains("ForgeQueryWriteCommand::InsertAspects"))
    }));
    assert!(contract
        .rows()
        .iter()
        .all(|row| row.preferred_name() != "surface"));
}

#[test]
fn runtime_public_mutation_surface_report_lists_only_live_lower_level_command_surfaces() {
    let workspace = stateful_bridge_task_runtime()
        .workspace("task.mutation-surface")
        .expect("task runtime should open a named workspace");
    let report = workspace.public_mutation_surface_report();

    assert_eq!(report.lower_level_stable_count(), 5);
    assert_eq!(report.support_gated_count(), 2);
    assert!(report
        .row_by_surface("ForgeQueryWriteCommand::Insert")
        .is_none());
    assert_eq!(
        report
            .row_by_surface("ForgeQueryWriteCommand::InsertAspects")
            .expect("aspect insert command row should exist")
            .posture(),
        ForgeQueryMutationSurfacePosture::LowerLevelStable
    );
}
