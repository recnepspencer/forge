use super::*;

pub(in crate::runtime::tests) fn bridge_runtime_with_support(
    profile: ForgeQueryRuntimeSupportProfile,
) -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
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
        .build()
        .expect("complete backend parts should build")
}

pub(in crate::runtime::tests) fn bridge_runtime_with_support_and_existing_truth_verification(
    profile: ForgeQueryRuntimeSupportProfile,
    adapter: TestExistingTruthVerificationAdapter,
) -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .existing_truth_verification(adapter)
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
        .support_profile(profile)
        .build_backend_from_parts()
        .build()
        .expect("complete backend parts should build")
}

pub(in crate::runtime::tests) fn task_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([ForgeQueryCollection::new(
            "Task",
            [
                crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
            ],
        )])
        .build()
        .expect("runtime should build")
}

pub(in crate::runtime::tests) fn task_issue_memory_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([
            ForgeQueryCollection::new(
                "Task",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
                ],
            ),
            ForgeQueryCollection::new(
                "Issue",
                [
                    crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                    crate::memory_workspace::ForgeQueryAspect::new(
                        "summary.value",
                        "summary.value",
                    ),
                ],
            ),
        ])
        .build()
        .expect("runtime should build")
}

pub(in crate::runtime::tests) fn grouped_task_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([ForgeQueryCollection::new(
            "Task",
            [
                crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
                crate::memory_workspace::ForgeQueryAspect::new("status.value", "status.value"),
            ],
        )])
        .build()
        .expect("runtime should build")
}

pub(in crate::runtime::tests) fn intent_support_profile() -> ForgeQueryRuntimeSupportProfile {
    ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    )
    .with_family_support(ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::Intent,
        [
            ForgeQueryAuthorityLane::AuthoritativeTruth,
            ForgeQueryAuthorityLane::BranchLocalTruth,
            ForgeQueryAuthorityLane::PreviewTruth,
        ],
        [],
        ["test-intent-authority"],
    ))
}

pub(in crate::runtime::tests) fn bridge_verified_direct_relation_profile(
    operation_family: &str,
) -> ForgeQueryRuntimeSupportProfile {
    ForgeQueryRuntimeSupportProfile::bridge_backed(
        "test-subscription-activation",
        "test-preview-basis",
        "test-inspector-evidence",
    )
    .with_bridge_backed_verification_support(
        operation_family,
        "direct_relation_identity",
        true,
        true,
        None,
    )
}
