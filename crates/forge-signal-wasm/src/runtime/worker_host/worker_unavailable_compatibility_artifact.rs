use serde::Serialize;

use crate::boundary::errors::ForgeSignalJsError;
use crate::runtime::core::RuntimeCore;

use super::{
    canonical_worker_certification_digest, certify_worker_compatibility,
    publish_definition_envelope_into_worker_runtime, WorkerCompatibilityCertificationReport,
    WorkerCompatibilityCertificationScenario,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerUnavailableCompatibilityCertificationPackage {
    pub certification_family: &'static str,
    pub covered_suite_count: u64,
    pub worker_support_posture: &'static str,
    pub selected_deployment_posture: &'static str,
    pub runtime_authority: &'static str,
    pub compatibility_artifact: &'static str,
    pub incompatibility_artifact: &'static str,
    pub fallback_policy: &'static str,
    pub hidden_fallback_allowed: bool,
    pub denial_artifact_required: bool,
    pub fallback_count: u64,
    pub worker_first_reference_truth_digest: String,
    pub compatibility_mode_truth_digest: String,
    pub compatibility_truth_digest: String,
    pub deployment_posture_digest: String,
    pub fallback_policy_digest: String,
    pub denial_digest: String,
    pub fallback_digest: String,
    pub callback_declaration_count: u64,
    pub main_thread_hosted_callback_count: u64,
    pub unavailable_callback_count: u64,
    pub capability_availability_digest: String,
    pub replay_import_compatibility_digest: String,
    pub placement_identity_digest: String,
    pub historical_capability_digest: String,
    pub certification_digest: String,
}

pub fn certify_worker_unavailable_compatibility_artifact(
    scenario: WorkerCompatibilityCertificationScenario,
) -> Result<WorkerUnavailableCompatibilityCertificationPackage, ForgeSignalJsError> {
    let compatibility_report = certify_worker_compatibility(scenario.clone())?;
    reject_non_convergent_compatibility_report(&compatibility_report)?;
    let placement = compatibility_mode_placement_package(&scenario)?;
    if placement.fallback_count != 0 {
        return Err(ForgeSignalJsError::invalid_input(
            "worker-unavailable compatibility certification requires zero placement fallback",
        ));
    }
    let policy = worker_unavailable_policy_summary()?;
    let worker_first_reference_truth_digest = compatibility_report
        .committed_truth_report
        .worker_first_truth_digest
        .clone();
    let compatibility_mode_truth_digest = compatibility_report
        .committed_truth_report
        .compatibility_mode_truth_digest
        .clone();
    let compatibility_truth_digest = canonical_worker_certification_digest(&(
        "workerUnavailableCompatibilityTruth",
        &compatibility_report,
    ))?;
    let deployment_posture_digest = canonical_worker_certification_digest(&(
        "workerUnavailableDeploymentPosture",
        policy.worker_support_posture,
        policy.selected_deployment_posture,
        policy.runtime_authority,
        policy.compatibility_artifact,
        policy.incompatibility_artifact,
    ))?;
    let fallback_policy_digest = canonical_worker_certification_digest(&(
        "workerUnavailableFallbackPolicy",
        policy.fallback_policy,
        policy.hidden_fallback_allowed,
        policy.denial_artifact_required,
        0_u64,
        placement.denial_digest.as_str(),
        placement.fallback_digest.as_str(),
    ))?;
    let historical_capability_digest = canonical_worker_certification_digest(&(
        "workerUnavailableHistoricalCapabilityPosture",
        placement.callback_declaration_count,
        placement.main_thread_hosted_callback_count,
        placement.unavailable_callback_count,
        placement.fallback_count,
        placement.denial_digest.as_str(),
        placement.fallback_digest.as_str(),
        placement.capability_availability_digest.as_str(),
        placement.replay_import_compatibility_digest.as_str(),
        placement.placement_identity_digest.as_str(),
        policy.incompatibility_artifact,
    ))?;
    let certification_digest = canonical_worker_certification_digest(&(
        "workerUnavailableCompatibilityCertification",
        compatibility_truth_digest.as_str(),
        deployment_posture_digest.as_str(),
        fallback_policy_digest.as_str(),
        historical_capability_digest.as_str(),
        placement.denial_digest.as_str(),
        placement.fallback_digest.as_str(),
        worker_first_reference_truth_digest.as_str(),
        compatibility_mode_truth_digest.as_str(),
    ))?;

    Ok(WorkerUnavailableCompatibilityCertificationPackage {
        certification_family: "workerUnavailableCompatibilityCertification",
        covered_suite_count: 1,
        worker_support_posture: policy.worker_support_posture,
        selected_deployment_posture: policy.selected_deployment_posture,
        runtime_authority: policy.runtime_authority,
        compatibility_artifact: policy.compatibility_artifact,
        incompatibility_artifact: policy.incompatibility_artifact,
        fallback_policy: policy.fallback_policy,
        hidden_fallback_allowed: policy.hidden_fallback_allowed,
        denial_artifact_required: policy.denial_artifact_required,
        fallback_count: 0,
        worker_first_reference_truth_digest,
        compatibility_mode_truth_digest,
        compatibility_truth_digest,
        deployment_posture_digest,
        fallback_policy_digest,
        denial_digest: placement.denial_digest,
        fallback_digest: placement.fallback_digest,
        callback_declaration_count: placement.callback_declaration_count,
        main_thread_hosted_callback_count: placement.main_thread_hosted_callback_count,
        unavailable_callback_count: placement.unavailable_callback_count,
        capability_availability_digest: placement.capability_availability_digest,
        replay_import_compatibility_digest: placement.replay_import_compatibility_digest,
        placement_identity_digest: placement.placement_identity_digest,
        historical_capability_digest,
        certification_digest,
    })
}

fn reject_non_convergent_compatibility_report(
    report: &WorkerCompatibilityCertificationReport,
) -> Result<(), ForgeSignalJsError> {
    if !report.committed_truth_report.committed_truth_matches
        || !report
            .branch_lifecycle_report
            .main_branch_report
            .branch_truth_matches
        || !report
            .branch_lifecycle_report
            .restored_branch_report
            .branch_truth_matches
        || !report.observation_report.observation_truth_matches
        || !report.diagnostics_report.diagnostics_truth_matches
        || !report.async_lifecycle_report.async_lifecycle_truth_matches
        || !report.isolation_report.all_regions_remain_worker_owned
        || report.isolation_report.broad_placement_collapse_detected
    {
        return Err(ForgeSignalJsError::invalid_input(
            "worker-unavailable compatibility certification requires compatibility truth convergence",
        ));
    }
    Ok(())
}

fn compatibility_mode_placement_package(
    scenario: &WorkerCompatibilityCertificationScenario,
) -> Result<crate::runtime::placement::WorkerCallbackPlacementEligibilityPackage, ForgeSignalJsError>
{
    let mut runtime = RuntimeCore::new(scenario.publication.policy.clone())?;
    publish_definition_envelope_into_worker_runtime(
        &mut runtime,
        scenario.publication.clone().into_definition_envelope(),
    )?;
    runtime.worker_callback_placement_eligibility()
}

struct WorkerUnavailablePolicySummary {
    worker_support_posture: &'static str,
    selected_deployment_posture: &'static str,
    runtime_authority: &'static str,
    compatibility_artifact: &'static str,
    incompatibility_artifact: &'static str,
    fallback_policy: &'static str,
    hidden_fallback_allowed: bool,
    denial_artifact_required: bool,
}

fn worker_unavailable_policy_summary() -> Result<WorkerUnavailablePolicySummary, ForgeSignalJsError>
{
    let runtime = RuntimeCore::new(Default::default())?;
    let artifact_lock = runtime.worker_boundary_artifact_lock();
    let main_thread_posture = artifact_lock
        .deployment_postures
        .iter()
        .find(|posture| posture.label == "mainThreadCompatibility")
        .ok_or_else(|| {
            ForgeSignalJsError::internal(
                "worker boundary artifact lock is missing main-thread compatibility posture",
            )
        })?;
    let declared_fallback_policy = artifact_lock
        .fallback_policies
        .iter()
        .find(|policy| policy.label == "productDeclaredFallbackOnly")
        .ok_or_else(|| {
            ForgeSignalJsError::internal(
                "worker boundary artifact lock is missing product-declared fallback policy",
            )
        })?;
    if main_thread_posture.preferred_for_heavy_apps
        || declared_fallback_policy.hidden_fallback_allowed
        || !declared_fallback_policy.denial_artifact_required
    {
        return Err(ForgeSignalJsError::internal(
            "worker boundary artifact lock permits hidden worker-unavailable fallback",
        ));
    }
    Ok(WorkerUnavailablePolicySummary {
        worker_support_posture: "workerUnavailable",
        selected_deployment_posture: main_thread_posture.label,
        runtime_authority: main_thread_posture.runtime_authority,
        compatibility_artifact: "explicitMainThreadCompatibilityRuntime",
        incompatibility_artifact: "dedicatedWorkerUnavailable",
        fallback_policy: declared_fallback_policy.label,
        hidden_fallback_allowed: declared_fallback_policy.hidden_fallback_allowed,
        denial_artifact_required: declared_fallback_policy.denial_artifact_required,
    })
}
