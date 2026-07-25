use sha2::{Digest, Sha256};
use worth_signal::facade::{
    AsyncNodeCapabilityDeclaration, AsyncNodePayloadContract, AsyncNodePayloadContractId, NodeId,
    ResourceCancellationPolicyDeclaration, ResourceDiagnosticsPolicyDeclaration,
    ResourceLifecyclePolicyDeclaration, ResourceObservationPolicyDeclaration,
    ResourceOutputContinuityPolicyDeclaration, ResourcePolicyDigest,
    ResourceReplayPolicyDeclaration, ResourceRetentionPolicyDeclaration,
    ResourceRetryPolicyDeclaration, ResourceRevalidationPolicyDeclaration,
    ResourceStaleAfterPolicyDeclaration, ResourceSupersessionPolicyDeclaration,
    ResourceTimeoutPolicyDeclaration, TemporalDuration,
};

/// The complete Signal policy posture installed for physical work capabilities.
///
/// Lowering and identity intentionally share the same selected declaration.
pub(in crate::physical_runtime) struct PhysicalSignalPolicySelection;

impl PhysicalSignalPolicySelection {
    pub(in crate::physical_runtime) fn apply(
        declaration: AsyncNodeCapabilityDeclaration,
    ) -> AsyncNodeCapabilityDeclaration {
        declaration
            .with_lifecycle_policy(ResourceLifecyclePolicyDeclaration::default())
            .with_retry_policy(ResourceRetryPolicyDeclaration::FixedDelay {
                delay: TemporalDuration::temporal_duration(1)
                    .expect("physical retry delay is positive"),
            })
            .with_timeout_policy(ResourceTimeoutPolicyDeclaration::PerAttemptTimeout {
                timeout: TemporalDuration::temporal_duration(1_000)
                    .expect("physical attempt timeout is positive"),
            })
            .with_cancellation_policy(
                ResourceCancellationPolicyDeclaration::BestEffortHostSignalAndRuntimeDenial,
            )
            .with_stale_after_policy(ResourceStaleAfterPolicyDeclaration::Disabled)
            .with_supersession_policy(
                ResourceSupersessionPolicyDeclaration::OverlappingGenerationRetainsOldHostWork,
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
        let bundle = Self::canonical_bundle_digest();
        let canonical = bundle.as_str().as_bytes();
        digest.update(b"worth-store.physical-signal-policy-selection.v4");
        digest.update((canonical.len() as u64).to_le_bytes());
        digest.update(canonical);
    }

    fn canonical_bundle_digest() -> ResourcePolicyDigest {
        let template = AsyncNodeCapabilityDeclaration::new(
            NodeId::new(0, 0),
            AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(0)),
        );
        let selected = Self::apply(template);
        let lowered = selected
            .canonical_policy_bundle()
            .expect("the frozen physical Signal policy selection must resolve");
        lowered.bundle_digest().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_identity_consumes_the_digest_of_the_applied_signal_policy() {
        let declaration = AsyncNodeCapabilityDeclaration::new(
            NodeId::new(7, 0),
            AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(17)),
        );
        let applied = PhysicalSignalPolicySelection::apply(declaration);
        let actual = applied.canonical_policy_bundle().unwrap();

        assert_eq!(
            PhysicalSignalPolicySelection::canonical_bundle_digest(),
            actual.bundle_digest().clone()
        );
    }
}
