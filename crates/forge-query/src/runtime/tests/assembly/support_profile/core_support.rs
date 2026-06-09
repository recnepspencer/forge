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
fn runtime_public_api_contract_closes_runtime_backed_temporal_async_surfaces() {
    let runtime = stateful_bridge_task_runtime();
    let contract = runtime.public_api_contract();

    assert_eq!(
        contract.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );
    assert_eq!(contract.deferred_family_count(), 2);
    assert!(!contract.contract_digest().is_empty());

    for family in [
        ForgeQueryRuntimeFacadeFamily::Temporal,
        ForgeQueryRuntimeFacadeFamily::AsyncResource,
        ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
    ] {
        let row = contract
            .family(family)
            .expect("runtime-backed support row should exist");
        assert_eq!(
            row.status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
        assert_eq!(
            row.teaching_posture(),
            ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly
        );
        assert!(!row.ordinary_downstream_dx());
        assert!(row.parallel_api_forbidden());
        assert!(row.admission_fail_closed());
        assert_eq!(row.owner_closure(), "Milestone 9.4");
        assert_eq!(
            row.extension_rule(),
            "must-extend-stabilized-handle-state-lane-aspect-inspection-facade"
        );
        assert!(row.reason().is_none());
        assert_eq!(row.authority_lanes().len(), 1);
        assert_eq!(row.evidence().len(), 1);
    }

    for (family, expected_reason) in [
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
            .expect("deferred support gate row should exist");
        assert_eq!(
            row.status(),
            ForgeQueryRuntimeFamilySupportStatus::DeferredDebt
        );
        assert_eq!(
            row.teaching_posture(),
            ForgeQueryRuntimeFamilyTeachingPosture::VisibleButDeferred
        );
        assert!(!row.ordinary_downstream_dx());
        assert!(row.parallel_api_forbidden());
        assert!(row.admission_fail_closed());
        assert_eq!(row.owner_closure(), expected_reason);
        assert!(row
            .reason()
            .is_some_and(|reason| reason.contains(expected_reason)));
        assert!(row.authority_lanes().is_empty());
        assert!(row.evidence().is_empty());
    }

    let read = contract
        .family(ForgeQueryRuntimeFacadeFamily::Read)
        .expect("supported read family should exist");
    assert_eq!(
        read.teaching_posture(),
        ForgeQueryRuntimeFamilyTeachingPosture::OrdinaryRuntimeDx
    );
    assert!(read.ordinary_downstream_dx());
    assert!(!read.admission_fail_closed());

    let intent = contract
        .family(ForgeQueryRuntimeFacadeFamily::Intent)
        .expect("intent family should remain visible in the public contract");
    assert_eq!(
        intent.status(),
        ForgeQueryRuntimeFamilySupportStatus::Unsupported
    );
    assert_eq!(
        intent.teaching_posture(),
        ForgeQueryRuntimeFamilyTeachingPosture::VisibleVocabularyOnly
    );
    assert!(!intent.ordinary_downstream_dx());
    assert!(intent.parallel_api_forbidden());
    assert!(intent.admission_fail_closed());
    assert_eq!(
        intent.extension_rule(),
        "must-admit-through-runtime-support-profile-before-public-use"
    );
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
        contract.stable_family_count() + 4
    );
    assert_eq!(
        matrix.deferred_row_count(),
        contract.deferred_family_count()
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
        matrix.deferred_row_count() + matrix.unsupported_row_count() + 4
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
    assert_eq!(
        certification.teaching_posture(),
        ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly
    );
    assert!(!certification.ordinary_downstream_dx());
    assert_eq!(
        certification.extension_rule(),
        "must-extend-target-binding-naming-continuity-causality-provenance-contract"
    );

    let temporal = matrix
        .row_for_family(ForgeQueryRuntimeFacadeFamily::Temporal)
        .expect("temporal support row must be explicit");
    assert_eq!(
        temporal.status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );
    assert_eq!(
        temporal.teaching_posture(),
        ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly
    );
    assert!(!temporal.ordinary_downstream_dx());
    assert!(temporal.parallel_api_forbidden());
    assert!(temporal.admission_fail_closed());
    assert_eq!(temporal.owner_milestone(), "Milestone 9.4");

    let intent = matrix
        .row_for_family(ForgeQueryRuntimeFacadeFamily::Intent)
        .expect("intent vocabulary row must stay visible");
    assert_eq!(
        intent.status(),
        ForgeQueryRuntimeFamilySupportStatus::Unsupported
    );
    assert_eq!(
        intent.teaching_posture(),
        ForgeQueryRuntimeFamilyTeachingPosture::VisibleVocabularyOnly
    );
    assert!(!intent.ordinary_downstream_dx());
    assert!(intent.parallel_api_forbidden());
    assert!(intent.admission_fail_closed());
    assert_eq!(
        intent.extension_rule(),
        "must-admit-through-runtime-support-profile-before-public-use"
    );

    let temporal_async_certification = matrix
        .row("temporal-async-certification")
        .expect("temporal async certification row must stay explicit");
    assert_eq!(
        temporal_async_certification.status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );
    assert_eq!(
        temporal_async_certification.teaching_posture(),
        ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly
    );
    assert!(!temporal_async_certification.ordinary_downstream_dx());
    assert!(!temporal_async_certification.admission_fail_closed());

    let temporal_async_remask = matrix
        .row("temporal-async-remask")
        .expect("temporal async remask row must stay explicit");
    assert_eq!(
        temporal_async_remask.status(),
        ForgeQueryRuntimeFamilySupportStatus::Supported
    );
    assert_eq!(
        temporal_async_remask.teaching_posture(),
        ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly
    );
    assert!(!temporal_async_remask.ordinary_downstream_dx());
    assert!(temporal_async_remask.admission_fail_closed());
    assert_eq!(
        temporal_async_remask.extension_rule(),
        "must-remask-before-runtime-delivery-state-and-inspection-projection"
    );
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

    for family in [
        ForgeQueryRuntimeFacadeFamily::Temporal,
        ForgeQueryRuntimeFacadeFamily::AsyncResource,
        ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
    ] {
        let admitted = workspace
            .admit_public_api_family(family)
            .expect("runtime-backed family should now admit");
        assert_eq!(admitted.family(), family);
        assert_eq!(
            admitted.status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
    }

    for (family, expected_reason) in [
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
