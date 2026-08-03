use crate::runtime::{
    WorthQueryAuthorityLane, WorthQueryEffectPolicy, WorthQueryRuntimeBackendPosture,
    WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeFamilySupport, WorthQueryRuntimeSupportProfile,
};

pub(super) fn in_memory_test_backend_support_profile() -> WorthQueryRuntimeSupportProfile {
    WorthQueryRuntimeSupportProfile::new([
        supported_read(),
        supported_live(),
        supported_submission(),
        supported_branch_preview(),
        supported_write(),
        unsupported(
            WorthQueryRuntimeFacadeFamily::Computed,
            "in-memory consumer test backend does not host computed view maintainers",
        ),
        unsupported(
            WorthQueryRuntimeFacadeFamily::SharedRead,
            "in-memory consumer test backend has no published shared-read artifact store",
        ),
        unsupported(
            WorthQueryRuntimeFacadeFamily::Replay,
            "in-memory consumer test backend does not persist replayable journals",
        ),
        unsupported(
            WorthQueryRuntimeFacadeFamily::Effect,
            "in-memory consumer test backend does not deliver runtime effects",
        ),
        unsupported(
            WorthQueryRuntimeFacadeFamily::Intent,
            "in-memory consumer test backend does not execute declared intent strategies",
        ),
        supported_inspect(),
        unsupported(
            WorthQueryRuntimeFacadeFamily::Temporal,
            "in-memory consumer test backend does not run temporal execution state",
        ),
        unsupported(
            WorthQueryRuntimeFacadeFamily::AsyncResource,
            "in-memory consumer test backend does not run async resource state",
        ),
        unsupported(
            WorthQueryRuntimeFacadeFamily::MixedCauseDelivery,
            "in-memory consumer test backend does not bridge external mixed-cause delivery",
        ),
        WorthQueryRuntimeFamilySupport::deferred(
            WorthQueryRuntimeFacadeFamily::StoreBackedExecution,
            "store-backed execution parity is deferred to Milestone 10",
        ),
        WorthQueryRuntimeFamilySupport::deferred(
            WorthQueryRuntimeFacadeFamily::DurableArtifacts,
            "durable restart and artifact reload are deferred to Milestone 11",
        ),
    ])
    .with_posture(WorthQueryRuntimeBackendPosture::Scaffold)
    .with_direct_atomic_batch_authority()
}

fn supported_read() -> WorthQueryRuntimeFamilySupport {
    WorthQueryRuntimeFamilySupport::supported(
        WorthQueryRuntimeFacadeFamily::Read,
        [WorthQueryAuthorityLane::AuthoritativeTruth],
        [],
        ["consumer-kit-in-memory-test-read"],
    )
}

fn supported_live() -> WorthQueryRuntimeFamilySupport {
    WorthQueryRuntimeFamilySupport::supported(
        WorthQueryRuntimeFacadeFamily::Live,
        [WorthQueryAuthorityLane::AuthoritativeTruth],
        [],
        ["consumer-kit-in-memory-test-live"],
    )
}

fn supported_submission() -> WorthQueryRuntimeFamilySupport {
    WorthQueryRuntimeFamilySupport::supported(
        WorthQueryRuntimeFacadeFamily::Submission,
        [
            WorthQueryAuthorityLane::PendingWriteIntent,
            WorthQueryAuthorityLane::AuthoritativeTruth,
        ],
        [],
        ["consumer-kit-in-memory-test-submission"],
    )
}

fn supported_branch_preview() -> WorthQueryRuntimeFamilySupport {
    WorthQueryRuntimeFamilySupport::supported(
        WorthQueryRuntimeFacadeFamily::BranchPreview,
        [
            WorthQueryAuthorityLane::PreviewTruth,
            WorthQueryAuthorityLane::BranchLocalTruth,
        ],
        [
            WorthQueryEffectPolicy::DeriveOnly,
            WorthQueryEffectPolicy::SandboxedWriteIntent,
        ],
        ["consumer-kit-in-memory-test-preview-basis"],
    )
}

fn supported_write() -> WorthQueryRuntimeFamilySupport {
    WorthQueryRuntimeFamilySupport::supported(
        WorthQueryRuntimeFacadeFamily::Write,
        [WorthQueryAuthorityLane::AuthoritativeTruth],
        [],
        ["consumer-kit-in-memory-test-write-authority"],
    )
}

fn supported_inspect() -> WorthQueryRuntimeFamilySupport {
    WorthQueryRuntimeFamilySupport::supported(
        WorthQueryRuntimeFacadeFamily::Inspect,
        [
            WorthQueryAuthorityLane::AuthoritativeTruth,
            WorthQueryAuthorityLane::BranchLocalTruth,
            WorthQueryAuthorityLane::PendingWriteIntent,
            WorthQueryAuthorityLane::PreviewTruth,
        ],
        [],
        ["consumer-kit-in-memory-test-inspection"],
    )
}

fn unsupported(
    family: WorthQueryRuntimeFacadeFamily,
    reason: impl Into<String>,
) -> WorthQueryRuntimeFamilySupport {
    WorthQueryRuntimeFamilySupport::unsupported(family, reason)
}
