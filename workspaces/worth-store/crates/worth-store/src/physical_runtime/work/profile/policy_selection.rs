use sha2::{Digest, Sha256};
use worth_signal::facade::{
    AsyncNodeCapabilityDeclaration, ResourceCancellationPolicyDeclaration,
    ResourceDiagnosticsPolicyDeclaration, ResourceLifecyclePolicyDeclaration,
    ResourceObservationPolicyDeclaration, ResourceOutputContinuityPolicyDeclaration,
    ResourceReplayPolicyDeclaration, ResourceRetentionPolicyDeclaration,
    ResourceRetryPolicyDeclaration, ResourceRevalidationPolicyDeclaration,
    ResourceStaleAfterPolicyDeclaration, ResourceSupersessionPolicyDeclaration,
    ResourceTimeoutPolicyDeclaration,
};

/// The complete Signal policy posture installed for physical work capabilities.
///
/// Lowering and identity bytes intentionally live together so a policy change
/// cannot hide behind Signal defaults or a serialization-derived fingerprint.
pub(in crate::physical_runtime::work) struct PhysicalSignalPolicySelection;

impl PhysicalSignalPolicySelection {
    pub(in crate::physical_runtime::work) fn apply(
        declaration: AsyncNodeCapabilityDeclaration,
    ) -> AsyncNodeCapabilityDeclaration {
        declaration
            .with_lifecycle_policy(ResourceLifecyclePolicyDeclaration::default())
            .with_retry_policy(ResourceRetryPolicyDeclaration::Disabled)
            .with_timeout_policy(ResourceTimeoutPolicyDeclaration::Disabled)
            .with_cancellation_policy(ResourceCancellationPolicyDeclaration::RuntimeDenialOnly)
            .with_stale_after_policy(ResourceStaleAfterPolicyDeclaration::Disabled)
            .with_supersession_policy(
                ResourceSupersessionPolicyDeclaration::NewGenerationSupersedesPrior,
            )
            .with_revalidation_policy(
                ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrActiveHandleForced,
            )
            .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleOnly)
            .with_output_continuity_policy(
                ResourceOutputContinuityPolicyDeclaration::PreserveLifecycleOutputSeparation,
            )
            .with_retention_policy(ResourceRetentionPolicyDeclaration::TerminalSummariesOnly)
            .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly)
            .with_replay_policy(ResourceReplayPolicyDeclaration::DenyOnUnknownOrMissing)
    }

    pub(in crate::physical_runtime::work) fn update_profile_identity(digest: &mut Sha256) {
        digest.update(b"worth-store.physical-signal-policy-selection.v1");
        digest.update([
            1, // unrequested lifecycle
            1, // retry disabled
            1, // timeout disabled
            1, // runtime-denial-only cancellation
            1, // stale-after disabled
            1, // new generation supersedes prior
            6, // dependency change or active-handle forced revalidation
            1, // lifecycle-only observation
            1, // lifecycle/output separation
            3, // terminal summaries only
            1, // retained diagnostics only
            9, // deny replay on unknown or missing policy
        ]);
    }
}
