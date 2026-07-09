use serde::Serialize;

use crate::boundary::errors::WORTHSignalJsError;
use crate::runtime::core::RuntimeCore;
use crate::runtime::placement::placement_category::WorkerPlacementCategory;

use super::worker_main_thread_hosted_callback_boundary::{
    WorkerMainThreadHostedCallbackRequestEnvelope, WorkerMainThreadHostedCallbackResultReport,
};
use super::worker_main_thread_hosted_callback_validation::validate_main_thread_hosted_callback_request_envelope;
use super::{canonical_worker_certification_digest, committed_truth_digest_for_runtime};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerMainThreadHostedCallbackExecutionCertificationPackage {
    pub certification_family: &'static str,
    pub covered_suite_count: u64,
    pub callback_id: String,
    pub request_digest: String,
    pub result_digest: String,
    pub placement_digest: String,
    pub denial_digest: String,
    pub fallback_digest: String,
    pub capability_availability_digest: String,
    pub replay_import_compatibility_digest: String,
    pub placement_identity_digest: String,
    pub hosted_execution_digest: String,
    pub worker_first_truth_digest: String,
    pub runtime_admitted_result_count: u64,
    pub runtime_mutation_breadth: u32,
    pub ambient_graph_read_denied: bool,
    pub host_result_is_authoritative: bool,
    pub fallback_count: u64,
    pub certification_digest: String,
}

impl WorkerMainThreadHostedCallbackExecutionCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_execution_evidence(
        core: &RuntimeCore,
        request: &WorkerMainThreadHostedCallbackRequestEnvelope,
        report: &WorkerMainThreadHostedCallbackResultReport,
    ) -> Result<Self, WORTHSignalJsError> {
        validate_execution_evidence_pair(request, report)?;
        let placement = core.worker_callback_placement_eligibility()?;
        let row = placement
            .rows
            .iter()
            .find(|row| row.declaration_id == request.callback_id)
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(
                    "main-thread-hosted callback certification requires placement evidence for the callback",
                )
            })?;
        if row.category != WorkerPlacementCategory::MainThreadHosted
            || !row.main_thread_hosted_lane_requires_closed_request
        {
            return Err(WORTHSignalJsError::invalid_input(
                "main-thread-hosted callback certification requires a main-thread-hosted placement row",
            ));
        }
        let current_truth_digest = committed_truth_digest_for_runtime(core)?;
        if current_truth_digest != report.worker_first_truth_digest {
            return Err(WORTHSignalJsError::invalid_input(
                "main-thread-hosted callback certification evidence is stale for the current worker truth",
            ));
        }

        let hosted_execution_digest = canonical_worker_certification_digest(&(
            "mainThreadHostedCallbackExecution",
            request.request_digest.as_str(),
            report.result_digest.as_str(),
            report.closed_request_result_digest.as_str(),
            report.runtime_admitted_result_count,
            report.runtime_mutation_breadth,
            report.ambient_graph_read_denied,
            report.host_result_is_authoritative,
            report.fallback_count,
        ))?;
        let certification_digest = canonical_worker_certification_digest(&(
            "mainThreadHostedCallbackExecutionCertification",
            placement.placement_digest.as_str(),
            placement.denial_digest.as_str(),
            placement.fallback_digest.as_str(),
            placement.capability_availability_digest.as_str(),
            placement.replay_import_compatibility_digest.as_str(),
            placement.placement_identity_digest.as_str(),
            hosted_execution_digest.as_str(),
            current_truth_digest.as_str(),
        ))?;

        Ok(Self {
            certification_family: "mainThreadHostedCallbackExecutionCertification",
            covered_suite_count: 1,
            callback_id: request.callback_id.clone(),
            request_digest: request.request_digest.clone(),
            result_digest: report.result_digest.clone(),
            placement_digest: placement.placement_digest,
            denial_digest: placement.denial_digest,
            fallback_digest: placement.fallback_digest,
            capability_availability_digest: placement.capability_availability_digest,
            replay_import_compatibility_digest: placement.replay_import_compatibility_digest,
            placement_identity_digest: placement.placement_identity_digest,
            hosted_execution_digest,
            worker_first_truth_digest: current_truth_digest,
            runtime_admitted_result_count: report.runtime_admitted_result_count,
            runtime_mutation_breadth: report.runtime_mutation_breadth,
            ambient_graph_read_denied: report.ambient_graph_read_denied,
            host_result_is_authoritative: report.host_result_is_authoritative,
            fallback_count: report.fallback_count,
            certification_digest,
        })
    }
}

fn validate_execution_evidence_pair(
    request: &WorkerMainThreadHostedCallbackRequestEnvelope,
    report: &WorkerMainThreadHostedCallbackResultReport,
) -> Result<(), WORTHSignalJsError> {
    validate_main_thread_hosted_callback_request_envelope(request)?;
    if request.envelope_family != "mainThreadHostedCallbackExecution"
        || report.envelope_family != "mainThreadHostedCallbackExecution"
    {
        return Err(WORTHSignalJsError::invalid_input(
            "main-thread-hosted callback certification requires callback execution evidence",
        ));
    }
    if report.callback_id != request.callback_id
        || report.acknowledged_request_digest != request.request_digest
    {
        return Err(WORTHSignalJsError::invalid_input(
            "main-thread-hosted callback certification request and result evidence must match",
        ));
    }
    if report.fallback_count != 0 || report.host_result_is_authoritative {
        return Err(WORTHSignalJsError::invalid_input(
            "main-thread-hosted callback certification requires zero fallback and non-authoritative host result posture",
        ));
    }
    if !report.ambient_graph_read_denied {
        return Err(WORTHSignalJsError::invalid_input(
            "main-thread-hosted callback certification requires ambient graph read denial",
        ));
    }

    Ok(())
}
