use super::super::support::*;
use crate::memory_workspace::ForgeQueryAspect;

#[test]
fn runtime_support_profiles_expose_facade_family_posture() {
    let memory_runtime = task_runtime();
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
            memory_runtime
                .support_profile()
                .support_for(family)
                .expect("memory support row should exist")
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
        memory_runtime.support_profile().posture(),
        ForgeQueryRuntimeBackendPosture::Compatibility
    );
    assert_eq!(
        bridge_runtime.support_profile().posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );

    assert_eq!(
        bridge_runtime
            .support_profile()
            .support_for(ForgeQueryRuntimeFacadeFamily::Intent)
            .expect("intent support row should exist")
            .status(),
        ForgeQueryRuntimeFamilySupportStatus::Unsupported
    );
    assert!(bridge_runtime
        .support_profile()
        .support_for(ForgeQueryRuntimeFacadeFamily::Live)
        .expect("live support row should exist")
        .evidence()
        .iter()
        .any(|evidence| evidence == "test-subscription-activation"));
    let support_profile = bridge_runtime.support_profile();
    let inspect_support = support_profile
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
    let runtime = task_runtime();
    let contract = runtime.public_api_contract();

    assert_eq!(
        contract.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Compatibility
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
        assert!(
            row.reason()
                .is_some_and(|reason| reason.contains(expected_reason)),
            "deferred row for {family:?} must name its owning milestone"
        );
        assert!(row.authority_lanes().is_empty());
        assert!(
            row.evidence().is_empty(),
            "deferred future gates must not pretend runtime evidence exists"
        );
    }
}

#[test]
fn runtime_state_snapshot_is_digest_bound_to_basis_shape_lane_and_state() {
    let ready = ForgeQueryRuntimeStateSnapshot::ready(
        "basis:current",
        "shape:table",
        ForgeQueryAuthorityLane::AuthoritativeTruth,
        "sync runtime-backed rows are ready",
    );
    let pending = ForgeQueryRuntimeStateSnapshot::deferred(
        ForgeQueryRuntimeStateKind::Pending,
        "basis:current",
        "shape:table",
        ForgeQueryAuthorityLane::BridgeExternalState,
        "async/resource family is deferred",
    );

    assert_eq!(ready.kind(), ForgeQueryRuntimeStateKind::Ready);
    assert_eq!(ready.basis_digest(), "basis:current");
    assert_eq!(ready.result_shape_digest(), "shape:table");
    assert_eq!(
        ready.authority_lane(),
        ForgeQueryAuthorityLane::AuthoritativeTruth
    );
    assert_eq!(pending.kind(), ForgeQueryRuntimeStateKind::Pending);
    assert_eq!(
        pending.authority_lane(),
        ForgeQueryAuthorityLane::BridgeExternalState
    );
    assert_ne!(ready.state_digest(), pending.state_digest());
    assert!(
        pending.explanation().contains("deferred"),
        "pending/deferred state must carry a localizable explanation"
    );
}

#[test]
fn compatibility_memory_backend_constructor_is_explicit_and_runtime_builder_matches_it() {
    let backend = ForgeQueryMemoryApp::compatibility_backend([ForgeQueryCollection::new(
        "Task",
        [ForgeQueryAspect::new("title", "title.value")],
    )])
    .expect("compatibility backend should build");
    assert_eq!(
        crate::runtime::ForgeQueryRuntimeBackend::support_profile(&backend).posture(),
        ForgeQueryRuntimeBackendPosture::Compatibility
    );

    let runtime = ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([ForgeQueryCollection::new(
            "Task",
            [ForgeQueryAspect::new("title", "title.value")],
        )])
        .build()
        .expect("compatibility in-memory runtime should build");
    assert_eq!(
        runtime.support_profile().posture(),
        ForgeQueryRuntimeBackendPosture::Compatibility
    );
}

#[test]
fn runtime_support_denies_unsupported_write_family_before_execution() {
    let mut runtime = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Write,
                "test backend disabled write authority",
            ),
        ),
    );

    let error = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "external-1" },
                "title": { "value": "Should not write" },
            }),
        })
        .expect_err("unsupported write family should deny before write authority");

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Write);
            assert_eq!(denial.reason(), "test backend disabled write authority");
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_builder_rejects_support_profiles_that_overclaim_unimplemented_families() {
    let profile = ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
        ForgeQueryRuntimeFamilySupport::supported(
            ForgeQueryRuntimeFacadeFamily::Intent,
            [ForgeQueryAuthorityLane::PendingWriteIntent],
            [ForgeQueryEffectPolicy::AuthoritativeAllowed],
            ["fake-intent-adapter"],
        ),
    );

    let error = ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .support_profile(profile)
        .build_backend_from_parts()
        .build();
    let error = match error {
        Ok(_) => panic!("support profile must not claim unimplemented facade support"),
        Err(error) => error,
    };

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Intent);
            assert!(denial.reason().contains("intent authority adapter"));
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_support_denies_unsupported_computed_family_before_registration() {
    let mut runtime = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::Computed,
                "test backend disabled computed resources",
            ),
        ),
    );

    let error = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("task_titles.unsupported", ["title".to_string()]),
            TitleListMaintainer,
        )
        .expect_err("unsupported computed family should deny before registration");

    match error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(denial.family(), ForgeQueryRuntimeFacadeFamily::Computed);
            assert_eq!(denial.reason(), "test backend disabled computed resources");
        }
        other => panic!("expected unsupported facade family denial, got {other:?}"),
    }
}

#[test]
fn runtime_support_denies_unsupported_preview_and_branch_sessions_without_panicking() {
    let mut runtime = bridge_runtime_with_support(
        ForgeQueryRuntimeSupportProfile::compatibility_backend().with_family_support(
            ForgeQueryRuntimeFamilySupport::unsupported(
                ForgeQueryRuntimeFacadeFamily::BranchPreview,
                "test backend disabled branch and preview sessions",
            ),
        ),
    );

    let preview_error = match runtime.preview("unsupported preview") {
        Ok(_) => panic!("unsupported preview should return a typed denial"),
        Err(error) => error,
    };
    match preview_error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(
                denial.family(),
                ForgeQueryRuntimeFacadeFamily::BranchPreview
            );
            assert_eq!(
                denial.reason(),
                "test backend disabled branch and preview sessions"
            );
        }
        other => panic!("expected unsupported preview family denial, got {other:?}"),
    }

    let branch_error = match runtime.branch("unsupported branch") {
        Ok(_) => panic!("unsupported branch should return a typed denial"),
        Err(error) => error,
    };
    match branch_error {
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(denial) => {
            assert_eq!(
                denial.family(),
                ForgeQueryRuntimeFacadeFamily::BranchPreview
            );
            assert_eq!(
                denial.reason(),
                "test backend disabled branch and preview sessions"
            );
        }
        other => panic!("expected unsupported branch family denial, got {other:?}"),
    }
}
