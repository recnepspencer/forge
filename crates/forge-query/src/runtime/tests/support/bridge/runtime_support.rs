use crate::runtime::tests::support::*;

pub(in crate::runtime::tests) fn bridge_backed_runtime_with_support(
    profile: ForgeQueryRuntimeSupportProfile,
) -> ForgeQueryRuntime {
    bridge_backed_runtime_with_support_and_intent_authority(profile, TestIntentAuthority)
}

pub(in crate::runtime::tests) fn bridge_runtime_with_support(
    profile: ForgeQueryRuntimeSupportProfile,
) -> ForgeQueryRuntime {
    bridge_backed_runtime_with_support(profile)
}

pub(in crate::runtime::tests) fn bridge_backed_runtime_with_support_and_intent_authority<
    T: ForgeQueryIntentAuthorityAdapter + 'static,
>(
    profile: ForgeQueryRuntimeSupportProfile,
    intent_authority: T,
) -> ForgeQueryRuntime {
    bridge_backed_runtime_builder(profile)
        .intent_authority(intent_authority)
        .build_backend_from_parts()
        .build()
        .expect("complete bridge-backed runtime test support should build")
}

pub(in crate::runtime::tests) fn bridge_runtime_with_support_and_intent_authority<
    T: ForgeQueryIntentAuthorityAdapter + 'static,
>(
    profile: ForgeQueryRuntimeSupportProfile,
    intent_authority: T,
) -> ForgeQueryRuntime {
    bridge_backed_runtime_with_support_and_intent_authority(profile, intent_authority)
}

pub(in crate::runtime::tests) fn bridge_backed_runtime_with_existing_truth_verification(
    profile: ForgeQueryRuntimeSupportProfile,
    adapter: TestExistingTruthVerificationAdapter,
) -> ForgeQueryRuntime {
    bridge_backed_runtime_builder(profile)
        .existing_truth_verification(adapter)
        .build_backend_from_parts()
        .build()
        .expect("complete bridge-backed runtime test support should build")
}

pub(in crate::runtime::tests) fn bridge_runtime_with_support_and_existing_truth_verification(
    profile: ForgeQueryRuntimeSupportProfile,
    adapter: TestExistingTruthVerificationAdapter,
) -> ForgeQueryRuntime {
    bridge_backed_runtime_with_existing_truth_verification(profile, adapter)
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

fn bridge_backed_runtime_builder(
    profile: ForgeQueryRuntimeSupportProfile,
) -> ForgeQueryRuntimeBuilder {
    complete_backend_from_parts_builder().support_profile(profile)
}

pub(in crate::runtime::tests) fn complete_backend_from_parts_builder() -> ForgeQueryRuntimeBuilder {
    ForgeQueryRuntime::builder()
        .runtime_bridge(test_bridge())
        .schema_adapter(TestSchemaAdapter)
        .source_adapter(TestSourceAdapter::default())
        .snapshot_identity(TestSnapshotIdentityAdapter)
        .write_authority(TestWriteAuthority)
        .signal_sink(TestSignalSink)
        .subscription_activation(TestSubscriptionActivation)
        .preview_basis(TestPreviewBasis)
        .inspector_evidence(TestInspectorEvidence)
}
