use serde::Serialize;

use crate::boundary::errors::WorthSignalJsError;

use super::{
    canonical_worker_certification_digest, WorkerBrowserHistoryIngressReport,
    WorkerHostBoundaryPerformanceEnvelope, WorkerHostCapabilityIngressReport,
    WorkerHostEffectAcknowledgementReport, WorkerHostEffectRequestEnvelope,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerMainThreadHostBridgeCertificationPackage {
    pub certification_family: &'static str,
    pub covered_suite_count: u64,
    pub host_capability_envelope_digest: String,
    pub host_capability_lifecycle_digest: String,
    pub host_capability_truth_digest: String,
    pub host_capability_coalescing_digest: String,
    pub host_capability_artifact_digest: String,
    pub browser_history_envelope_digest: String,
    pub browser_history_route_truth_digest: String,
    pub browser_history_continuity_digest: String,
    pub browser_history_replay_restore_digest: String,
    pub host_effect_request_digest: String,
    pub host_effect_acknowledged_request_digest: String,
    pub host_effect_acknowledgement_digest: String,
    pub host_effect_lifecycle_artifact: String,
    pub host_effect_lifecycle_integrity_digest: String,
    pub worth_proof_readmission_digest: String,
    pub host_boundary_causality_digest: String,
    pub boundary_performance_digest: String,
    pub worker_first_truth_digest: String,
    pub ambient_host_read_denied: bool,
    pub host_acknowledgement_is_authoritative: bool,
    pub certification_digest: String,
}

impl WorkerMainThreadHostBridgeCertificationPackage {
    pub fn from_boundary_reports(
        host_capability_report: &WorkerHostCapabilityIngressReport,
        browser_history_report: &WorkerBrowserHistoryIngressReport,
        host_effect_request: &WorkerHostEffectRequestEnvelope,
        host_effect_acknowledgement: &WorkerHostEffectAcknowledgementReport,
    ) -> Result<Self, WorthSignalJsError> {
        validate_main_thread_host_bridge_report_families(
            host_capability_report,
            browser_history_report,
            host_effect_request,
            host_effect_acknowledgement,
        )?;
        validate_main_thread_host_bridge_causality(
            host_capability_report,
            browser_history_report,
            host_effect_request,
            host_effect_acknowledgement,
        )?;
        validate_main_thread_host_effect_pairing(host_effect_request, host_effect_acknowledgement)?;

        let performance_digest = main_thread_host_bridge_performance_digest(
            &host_capability_report.performance,
            &browser_history_report.performance,
            &host_effect_request.performance,
            &host_effect_acknowledgement.performance,
        )?;
        let causality_digest = canonical_worker_certification_digest(&(
            host_capability_report.causality.clone(),
            browser_history_report.causality.clone(),
            host_effect_request.causality.clone(),
            host_effect_acknowledgement.causality.clone(),
        ))?;
        let certification_digest =
            canonical_worker_certification_digest(&MainThreadHostBridgeCertificationDigestSeed {
                certification_family: "mainThreadHostBridgeCertification",
                host_capability_envelope_digest: host_capability_report
                    .host_capability_envelope_digest
                    .as_str(),
                host_capability_lifecycle_digest: host_capability_report.lifecycle_digest.as_str(),
                host_capability_truth_digest: host_capability_report.truth_digest.as_str(),
                host_capability_coalescing_digest: host_capability_report
                    .coalescing_digest
                    .as_str(),
                host_capability_artifact_digest: host_capability_report
                    .host_boundary_artifact_digest
                    .as_str(),
                browser_history_envelope_digest: browser_history_report
                    .browser_history_envelope_digest
                    .as_str(),
                browser_history_route_truth_digest: browser_history_report
                    .route_truth_digest
                    .as_str(),
                browser_history_continuity_digest: browser_history_report
                    .continuity_digest
                    .as_str(),
                browser_history_replay_restore_digest: browser_history_report
                    .replay_restore_digest
                    .as_str(),
                host_effect_request_digest: host_effect_request.request_digest.as_str(),
                host_effect_acknowledged_request_digest: host_effect_acknowledgement
                    .acknowledged_request_digest
                    .as_str(),
                host_effect_acknowledgement_digest: host_effect_acknowledgement
                    .acknowledgement_digest
                    .as_str(),
                host_effect_lifecycle_artifact: host_effect_acknowledgement
                    .host_effect_lifecycle_artifact
                    .as_str(),
                host_effect_lifecycle_integrity_digest: host_effect_acknowledgement
                    .lifecycle_integrity_digest
                    .as_str(),
                worth_proof_readmission_digest: host_effect_acknowledgement
                    .worth_proof_readmission_digest
                    .as_str(),
                host_boundary_causality_digest: causality_digest.as_str(),
                boundary_performance_digest: performance_digest.as_str(),
                worker_first_truth_digest: host_effect_acknowledgement
                    .worker_first_truth_digest
                    .as_str(),
                host_capability_ambient_read_denied: host_capability_report
                    .ambient_worker_read_denied,
                browser_history_ambient_read_denied: browser_history_report
                    .ambient_location_read_denied,
                host_acknowledgement_is_authoritative: host_effect_acknowledgement
                    .host_acknowledgement_is_authoritative,
            })?;

        Ok(Self {
            certification_family: "mainThreadHostBridgeCertification",
            covered_suite_count: 3,
            host_capability_envelope_digest: host_capability_report
                .host_capability_envelope_digest
                .clone(),
            host_capability_lifecycle_digest: host_capability_report.lifecycle_digest.clone(),
            host_capability_truth_digest: host_capability_report.truth_digest.clone(),
            host_capability_coalescing_digest: host_capability_report.coalescing_digest.clone(),
            host_capability_artifact_digest: host_capability_report
                .host_boundary_artifact_digest
                .clone(),
            browser_history_envelope_digest: browser_history_report
                .browser_history_envelope_digest
                .clone(),
            browser_history_route_truth_digest: browser_history_report.route_truth_digest.clone(),
            browser_history_continuity_digest: browser_history_report.continuity_digest.clone(),
            browser_history_replay_restore_digest: browser_history_report
                .replay_restore_digest
                .clone(),
            host_effect_request_digest: host_effect_request.request_digest.clone(),
            host_effect_acknowledged_request_digest: host_effect_acknowledgement
                .acknowledged_request_digest
                .clone(),
            host_effect_acknowledgement_digest: host_effect_acknowledgement
                .acknowledgement_digest
                .clone(),
            host_effect_lifecycle_artifact: host_effect_acknowledgement
                .host_effect_lifecycle_artifact
                .clone(),
            host_effect_lifecycle_integrity_digest: host_effect_acknowledgement
                .lifecycle_integrity_digest
                .clone(),
            worth_proof_readmission_digest: host_effect_acknowledgement
                .worth_proof_readmission_digest
                .clone(),
            host_boundary_causality_digest: causality_digest,
            boundary_performance_digest: performance_digest,
            worker_first_truth_digest: host_effect_acknowledgement
                .worker_first_truth_digest
                .clone(),
            ambient_host_read_denied: host_capability_report.ambient_worker_read_denied
                && browser_history_report.ambient_location_read_denied,
            host_acknowledgement_is_authoritative: host_effect_acknowledgement
                .host_acknowledgement_is_authoritative,
            certification_digest,
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MainThreadHostBridgeCertificationDigestSeed<'a> {
    certification_family: &'static str,
    host_capability_envelope_digest: &'a str,
    host_capability_lifecycle_digest: &'a str,
    host_capability_truth_digest: &'a str,
    host_capability_coalescing_digest: &'a str,
    host_capability_artifact_digest: &'a str,
    browser_history_envelope_digest: &'a str,
    browser_history_route_truth_digest: &'a str,
    browser_history_continuity_digest: &'a str,
    browser_history_replay_restore_digest: &'a str,
    host_effect_request_digest: &'a str,
    host_effect_acknowledged_request_digest: &'a str,
    host_effect_acknowledgement_digest: &'a str,
    host_effect_lifecycle_artifact: &'a str,
    host_effect_lifecycle_integrity_digest: &'a str,
    worth_proof_readmission_digest: &'a str,
    host_boundary_causality_digest: &'a str,
    boundary_performance_digest: &'a str,
    worker_first_truth_digest: &'a str,
    host_capability_ambient_read_denied: bool,
    browser_history_ambient_read_denied: bool,
    host_acknowledgement_is_authoritative: bool,
}

fn validate_main_thread_host_bridge_report_families(
    host_capability_report: &WorkerHostCapabilityIngressReport,
    browser_history_report: &WorkerBrowserHistoryIngressReport,
    host_effect_request: &WorkerHostEffectRequestEnvelope,
    host_effect_acknowledgement: &WorkerHostEffectAcknowledgementReport,
) -> Result<(), WorthSignalJsError> {
    if host_capability_report.envelope_family != "hostCapabilityIngress" {
        return Err(WorthSignalJsError::invalid_input(
            "main thread host bridge certification requires host capability ingress evidence",
        ));
    }
    if browser_history_report.envelope_family != "browserHistoryIngress" {
        return Err(WorthSignalJsError::invalid_input(
            "main thread host bridge certification requires browser history ingress evidence",
        ));
    }
    if host_effect_request.envelope_family != "hostEffectEgress"
        || host_effect_acknowledgement.envelope_family != "hostEffectEgress"
    {
        return Err(WorthSignalJsError::invalid_input(
            "main thread host bridge certification requires host effect request and acknowledgement evidence",
        ));
    }

    Ok(())
}

fn validate_main_thread_host_bridge_causality(
    host_capability_report: &WorkerHostCapabilityIngressReport,
    browser_history_report: &WorkerBrowserHistoryIngressReport,
    host_effect_request: &WorkerHostEffectRequestEnvelope,
    host_effect_acknowledgement: &WorkerHostEffectAcknowledgementReport,
) -> Result<(), WorthSignalJsError> {
    let transaction_sequences = [
        host_capability_report.causality.transaction_sequence,
        browser_history_report.causality.transaction_sequence,
        host_effect_request.causality.transaction_sequence,
        host_effect_acknowledgement.causality.transaction_sequence,
    ];

    if !transaction_sequences
        .windows(2)
        .all(|window| window[0] < window[1])
    {
        return Err(WorthSignalJsError::invalid_input(
            "main thread host bridge certification requires monotonically ordered boundary causality",
        ));
    }

    Ok(())
}

fn validate_main_thread_host_effect_pairing(
    host_effect_request: &WorkerHostEffectRequestEnvelope,
    host_effect_acknowledgement: &WorkerHostEffectAcknowledgementReport,
) -> Result<(), WorthSignalJsError> {
    if host_effect_acknowledgement.acknowledged_request_digest != host_effect_request.request_digest
    {
        return Err(WorthSignalJsError::invalid_input(
            "main thread host bridge certification requires acknowledgement request digest to match the issued host effect request",
        ));
    }

    Ok(())
}

fn main_thread_host_bridge_performance_digest(
    host_capability_performance: &WorkerHostBoundaryPerformanceEnvelope,
    browser_history_performance: &WorkerHostBoundaryPerformanceEnvelope,
    host_effect_request_performance: &WorkerHostBoundaryPerformanceEnvelope,
    host_effect_acknowledgement_performance: &WorkerHostBoundaryPerformanceEnvelope,
) -> Result<String, WorthSignalJsError> {
    canonical_worker_certification_digest(&(
        "mainThreadHostBridgePerformance",
        host_capability_performance,
        browser_history_performance,
        host_effect_request_performance,
        host_effect_acknowledgement_performance,
    ))
}
