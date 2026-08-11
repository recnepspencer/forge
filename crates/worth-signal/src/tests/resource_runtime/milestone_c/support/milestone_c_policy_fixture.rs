use super::super::{
    resource_milestone_c_policy_certification_builder, DeniedResourcePolicyRestoreCompatibility,
    ResourceDiagnosticsExpansionDenial, ResourceLifecycleRetentionCompactionReport,
    ResourceMilestoneCPolicyCertificationBundle, ResourcePolicyRegistryFreezeReport,
    ResourcePolicyRestoreCompatibilityProof, ResourceRetryScheduleReport,
    ResourceTimeoutHeartbeatExtensionReport,
};
use super::cancellation_supersession_evidence::{
    resource_milestone_c_cancellation_supersession_evidence,
    ResourceMilestoneCCancellationSupersessionEvidence,
};
use super::observation_evidence::{
    resource_milestone_c_observation_evidence, ResourceMilestoneCObservationEvidence,
};
use super::registry_freeze_evidence::freeze_resource_policy_registry_evidence;
use super::restore_compatibility_evidence::{
    resource_milestone_c_restore_compatibility_evidence,
    ResourceMilestoneCRestoreCompatibilityEvidence,
};
use super::retention_replay_evidence::{
    resource_milestone_c_retention_replay_evidence, ResourceMilestoneCRetentionReplayEvidence,
};
use super::retry_evidence::{resource_milestone_c_retry_evidence, ResourceMilestoneCRetryEvidence};
use super::revalidation_evidence::{
    resource_milestone_c_revalidation_evidence, ResourceMilestoneCRevalidationEvidence,
};
use super::timeout_evidence::{
    resource_milestone_c_timeout_evidence, ResourceMilestoneCTimeoutEvidence,
};

pub(in crate::tests::resource_runtime::milestone_c) struct ResourceMilestoneCPolicyFixture {
    pub(in crate::tests::resource_runtime::milestone_c) freeze_report:
        ResourcePolicyRegistryFreezeReport,
    pub(in crate::tests::resource_runtime::milestone_c) denied_retry_report:
        ResourceRetryScheduleReport,
    pub(in crate::tests::resource_runtime::milestone_c) heartbeat_denial_report:
        ResourceTimeoutHeartbeatExtensionReport,
    pub(in crate::tests::resource_runtime::milestone_c) retention_report:
        ResourceLifecycleRetentionCompactionReport,
    pub(in crate::tests::resource_runtime::milestone_c) diagnostics_denial:
        ResourceDiagnosticsExpansionDenial,
    pub(in crate::tests::resource_runtime::milestone_c) compatible_restore:
        ResourcePolicyRestoreCompatibilityProof,
    pub(in crate::tests::resource_runtime::milestone_c) incompatible_restore:
        DeniedResourcePolicyRestoreCompatibility,
    pub(in crate::tests::resource_runtime::milestone_c) missing_restore:
        DeniedResourcePolicyRestoreCompatibility,
    pub(in crate::tests::resource_runtime::milestone_c) bundle:
        ResourceMilestoneCPolicyCertificationBundle,
}

pub(in crate::tests::resource_runtime::milestone_c) fn resource_milestone_c_policy_fixture(
) -> ResourceMilestoneCPolicyFixture {
    let freeze_report = freeze_resource_policy_registry_evidence();
    let ResourceMilestoneCRetryEvidence {
        denied_retry_report,
    } = resource_milestone_c_retry_evidence();
    let ResourceMilestoneCTimeoutEvidence {
        timeout_report,
        heartbeat_denial_report,
    } = resource_milestone_c_timeout_evidence();
    let ResourceMilestoneCCancellationSupersessionEvidence {
        cancellation_report,
        overlap_admission,
        intent_coalescing,
    } = resource_milestone_c_cancellation_supersession_evidence();
    let ResourceMilestoneCRevalidationEvidence {
        revalidation_report,
    } = resource_milestone_c_revalidation_evidence();
    let ResourceMilestoneCObservationEvidence { observation_report } =
        resource_milestone_c_observation_evidence();
    let ResourceMilestoneCRetentionReplayEvidence {
        retention_report,
        replay_availability,
        diagnostics_denial,
    } = resource_milestone_c_retention_replay_evidence();
    let ResourceMilestoneCRestoreCompatibilityEvidence {
        compatible_restore,
        incompatible_restore,
        missing_restore,
    } = resource_milestone_c_restore_compatibility_evidence();

    let bundle = resource_milestone_c_policy_certification_builder()
        .with_async_resource_policy_family_certification(&freeze_report)
        .expect("policy family certification should accept freeze evidence")
        .with_async_retry_budget_and_backoff_certification(&denied_retry_report)
        .expect("retry family certification should accept retry evidence")
        .with_async_timeout_deadline_certification(&timeout_report, &heartbeat_denial_report)
        .expect("timeout family certification should accept timeout evidence")
        .with_async_cancellation_supersession_policy_certification(
            &cancellation_report,
            &overlap_admission,
            &intent_coalescing,
        )
        .expect("cancellation/supersession family certification should accept evidence")
        .with_async_revalidation_freshness_certification(&revalidation_report)
        .expect("revalidation family certification should accept evidence")
        .with_async_observation_output_continuity_certification(&observation_report)
        .expect("observation family certification should accept evidence")
        .with_async_retention_replay_policy_certification(&retention_report, &replay_availability)
        .expect("retention/replay family certification should accept evidence")
        .build()
        .expect("complete milestone C policy certification bundle should pass");

    ResourceMilestoneCPolicyFixture {
        freeze_report,
        denied_retry_report,
        heartbeat_denial_report,
        retention_report,
        diagnostics_denial,
        compatible_restore,
        incompatible_restore,
        missing_restore,
        bundle,
    }
}
