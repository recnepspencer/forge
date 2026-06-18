use crate::runtime::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryRuntimeBackendPosture,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport, ForgeQueryRuntimeSupportProfile,
};

pub(super) fn in_memory_test_backend_support_profile() -> ForgeQueryRuntimeSupportProfile {
    ForgeQueryRuntimeSupportProfile::new([
        supported_read(),
        supported_live(),
        supported_submission(),
        supported_branch_preview(),
        supported_write(),
        unsupported(
            ForgeQueryRuntimeFacadeFamily::Computed,
            "in-memory consumer test backend does not host computed view maintainers",
        ),
        unsupported(
            ForgeQueryRuntimeFacadeFamily::SharedRead,
            "in-memory consumer test backend has no published shared-read artifact store",
        ),
        unsupported(
            ForgeQueryRuntimeFacadeFamily::Replay,
            "in-memory consumer test backend does not persist replayable journals",
        ),
        unsupported(
            ForgeQueryRuntimeFacadeFamily::Effect,
            "in-memory consumer test backend does not deliver runtime effects",
        ),
        unsupported(
            ForgeQueryRuntimeFacadeFamily::Intent,
            "in-memory consumer test backend does not execute declared intent strategies",
        ),
        supported_inspect(),
        unsupported(
            ForgeQueryRuntimeFacadeFamily::Temporal,
            "in-memory consumer test backend does not run temporal execution state",
        ),
        unsupported(
            ForgeQueryRuntimeFacadeFamily::AsyncResource,
            "in-memory consumer test backend does not run async resource state",
        ),
        unsupported(
            ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
            "in-memory consumer test backend does not bridge external mixed-cause delivery",
        ),
        ForgeQueryRuntimeFamilySupport::deferred(
            ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
            "store-backed execution parity is deferred to Milestone 10",
        ),
        ForgeQueryRuntimeFamilySupport::deferred(
            ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
            "durable restart and artifact reload are deferred to Milestone 11",
        ),
    ])
    .with_posture(ForgeQueryRuntimeBackendPosture::Scaffold)
}

fn supported_read() -> ForgeQueryRuntimeFamilySupport {
    ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::Read,
        [ForgeQueryAuthorityLane::AuthoritativeTruth],
        [],
        ["consumer-kit-in-memory-test-read"],
    )
}

fn supported_live() -> ForgeQueryRuntimeFamilySupport {
    ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::Live,
        [ForgeQueryAuthorityLane::AuthoritativeTruth],
        [],
        ["consumer-kit-in-memory-test-live"],
    )
}

fn supported_submission() -> ForgeQueryRuntimeFamilySupport {
    ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::Submission,
        [
            ForgeQueryAuthorityLane::PendingWriteIntent,
            ForgeQueryAuthorityLane::AuthoritativeTruth,
        ],
        [],
        ["consumer-kit-in-memory-test-submission"],
    )
}

fn supported_branch_preview() -> ForgeQueryRuntimeFamilySupport {
    ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::BranchPreview,
        [
            ForgeQueryAuthorityLane::PreviewTruth,
            ForgeQueryAuthorityLane::BranchLocalTruth,
        ],
        [
            ForgeQueryEffectPolicy::DeriveOnly,
            ForgeQueryEffectPolicy::SandboxedWriteIntent,
        ],
        ["consumer-kit-in-memory-test-preview-basis"],
    )
}

fn supported_write() -> ForgeQueryRuntimeFamilySupport {
    ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::Write,
        [ForgeQueryAuthorityLane::AuthoritativeTruth],
        [],
        ["consumer-kit-in-memory-test-write-authority"],
    )
}

fn supported_inspect() -> ForgeQueryRuntimeFamilySupport {
    ForgeQueryRuntimeFamilySupport::supported(
        ForgeQueryRuntimeFacadeFamily::Inspect,
        [
            ForgeQueryAuthorityLane::AuthoritativeTruth,
            ForgeQueryAuthorityLane::BranchLocalTruth,
            ForgeQueryAuthorityLane::PendingWriteIntent,
            ForgeQueryAuthorityLane::PreviewTruth,
        ],
        [],
        ["consumer-kit-in-memory-test-inspection"],
    )
}

fn unsupported(
    family: ForgeQueryRuntimeFacadeFamily,
    reason: impl Into<String>,
) -> ForgeQueryRuntimeFamilySupport {
    ForgeQueryRuntimeFamilySupport::unsupported(family, reason)
}
