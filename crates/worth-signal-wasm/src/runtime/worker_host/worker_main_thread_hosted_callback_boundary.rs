use serde::{Deserialize, Serialize};

use crate::boundary::errors::WorthSignalJsError;
use crate::expression::model::SignalValue;
use crate::runtime::compute_callbacks::CapturedHostCapabilityRead;
use crate::runtime::core::{
    MainThreadHostedCallbackAdmission, MainThreadHostedCallbackClosedInput,
};

use super::worker_main_thread_hosted_callback_validation::validate_main_thread_hosted_callback_result;
use super::{
    canonical_worker_certification_digest, committed_truth_digest_for_runtime,
    WorkerHostBoundaryCausality, WorkerHostBoundaryPerformanceEnvelope,
    WorkerMainThreadHostedCallbackExecutionCertificationPackage, WorkerRuntimeShell,
};

type CurrentHostedCallbackBoundaryBasis = worth_proof::FreshnessScopedBasis<
    worth_proof::CurrentValidity,
    worth_proof::AssumptionBasis<HostedCallbackBoundaryBasis>,
>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerMainThreadHostedCallbackInput {
    pub id: String,
    pub value: SignalValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerMainThreadHostedCallbackRequestEnvelope {
    pub envelope_family: String,
    pub causality: WorkerHostBoundaryCausality,
    pub callback_id: String,
    pub request_digest: String,
    pub closed_input_ids: Vec<String>,
    pub closed_input_count: u64,
    pub host_capability_read_count: u64,
    pub closed_payload_digest: String,
    pub host_execution_boundary: String,
    pub performance: WorkerHostBoundaryPerformanceEnvelope,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerMainThreadHostedCallbackResult {
    pub request_digest: String,
    pub callback_id: String,
    pub outcome: WorkerMainThreadHostedCallbackOutcome,
    pub artifact_identity: String,
    #[serde(default)]
    pub value: Option<SignalValue>,
    #[serde(default)]
    pub captured_read_ids: Vec<String>,
    #[serde(default)]
    pub captured_host_capability_reads: Vec<CapturedHostCapabilityRead>,
    #[serde(default)]
    pub runtime_read_breadth: u64,
    #[serde(default)]
    pub return_serialization_breadth: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WorkerMainThreadHostedCallbackOutcome {
    Completed,
    Failed,
    Denied,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerMainThreadHostedCallbackResultReport {
    pub envelope_family: String,
    pub causality: WorkerHostBoundaryCausality,
    pub callback_id: String,
    pub acknowledged_request_digest: String,
    pub result_digest: String,
    pub callback_execution_artifact: String,
    pub closed_request_result_digest: String,
    pub runtime_admitted_result_count: u64,
    pub runtime_mutation_breadth: u32,
    pub worker_first_truth_digest: String,
    pub performance: WorkerHostBoundaryPerformanceEnvelope,
    pub host_result_is_authoritative: bool,
    pub worker_readmission_required: bool,
    pub ambient_graph_read_denied: bool,
    pub fallback_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::runtime::worker_host) struct HostedCallbackBoundaryPayload;
impl worth_proof::PhaseMarker for HostedCallbackBoundaryPayload {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HostedCallbackReadmissionAuthority;
impl worth_proof::AuthorityMarker for HostedCallbackReadmissionAuthority {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::runtime::worker_host) struct HostedCallbackBoundaryBasis {
    runtime_admitted_result_count: u64,
}

pub(in crate::runtime::worker_host) type CurrentMainThreadHostedCallbackResult =
    worth_proof::Artifact<
        HostedCallbackBoundaryPayload,
        WorkerMainThreadHostedCallbackResult,
        worth_proof::NoProofs,
        CurrentHostedCallbackBoundaryBasis,
    >;

pub(in crate::runtime::worker_host) type BridgedMainThreadHostedCallbackResult =
    worth_proof::Artifact<
        HostedCallbackBoundaryPayload,
        WorkerMainThreadHostedCallbackResult,
        worth_proof::NoProofs,
        worth_proof::BoundaryBridgedAuthorityRevalidationRequiredBasis<HostedCallbackBoundaryBasis>,
    >;

pub(in crate::runtime::worker_host) type ReadmittedMainThreadHostedCallbackResult =
    worth_proof::Artifact<
        HostedCallbackBoundaryPayload,
        WorkerMainThreadHostedCallbackResult,
        worth_proof::NoProofs,
        CurrentHostedCallbackBoundaryBasis,
    >;

impl WorkerMainThreadHostedCallbackRequestEnvelope {
    fn from_closed_inputs(
        callback_id: String,
        closed_inputs: Vec<MainThreadHostedCallbackClosedInput>,
        host_capability_read_count: u64,
        causality: WorkerHostBoundaryCausality,
    ) -> Result<Self, WorthSignalJsError> {
        let closed_input_ids = closed_inputs
            .iter()
            .map(|input| input.id.clone())
            .collect::<Vec<_>>();
        let bridged_inputs = closed_inputs
            .into_iter()
            .map(WorkerMainThreadHostedCallbackInput::from)
            .collect::<Vec<_>>();
        let closed_payload_digest =
            canonical_worker_certification_digest(&("closedCallbackInputs", &bridged_inputs))?;
        let request_digest = canonical_worker_certification_digest(&(
            "mainThreadHostedCallbackRequest",
            callback_id.as_str(),
            &closed_input_ids,
            host_capability_read_count,
            closed_payload_digest.as_str(),
        ))?;
        Ok(Self {
            envelope_family: "mainThreadHostedCallbackExecution".to_owned(),
            causality,
            callback_id,
            request_digest,
            closed_input_count: closed_input_ids.len() as u64,
            closed_input_ids,
            host_capability_read_count,
            closed_payload_digest: closed_payload_digest.clone(),
            host_execution_boundary: "mainThreadHostedCallback".to_owned(),
            performance:
                WorkerHostBoundaryPerformanceEnvelope::main_thread_hosted_callback_request(
                    bridged_inputs.len() as u64,
                    closed_payload_digest.as_str(),
                )?,
        })
    }
}

impl WorkerMainThreadHostedCallbackResultReport {
    fn from_readmitted_result(
        request: &WorkerMainThreadHostedCallbackRequestEnvelope,
        result: ReadmittedMainThreadHostedCallbackResult,
        causality: WorkerHostBoundaryCausality,
        runtime_admitted_result_count: u64,
        runtime_mutation_breadth: u32,
        worker_first_truth_digest: String,
    ) -> Result<Self, WorthSignalJsError> {
        Ok(Self {
            envelope_family: "mainThreadHostedCallbackExecution".to_owned(),
            causality,
            callback_id: result.payload().callback_id.clone(),
            acknowledged_request_digest: result.payload().request_digest.clone(),
            result_digest: canonical_worker_certification_digest(result.payload())?,
            callback_execution_artifact: result
                .payload()
                .outcome
                .callback_execution_artifact()
                .to_owned(),
            closed_request_result_digest: canonical_worker_certification_digest(&(
                request.request_digest.as_str(),
                result.payload().artifact_identity.as_str(),
                runtime_admitted_result_count,
            ))?,
            runtime_admitted_result_count,
            runtime_mutation_breadth,
            worker_first_truth_digest,
            performance: WorkerHostBoundaryPerformanceEnvelope::main_thread_hosted_callback_result(
                result.payload().artifact_identity.as_str(),
                runtime_admitted_result_count,
                runtime_mutation_breadth,
            )?,
            host_result_is_authoritative: false,
            worker_readmission_required: runtime_admitted_result_count == 0,
            ambient_graph_read_denied: true,
            fallback_count: 0,
        })
    }
}

impl WorkerMainThreadHostedCallbackOutcome {
    fn callback_execution_artifact(self) -> &'static str {
        match self {
            Self::Completed => "mainThreadHostedCallbackCompleted",
            Self::Failed => "mainThreadHostedCallbackFailed",
            Self::Denied => "mainThreadHostedCallbackDenied",
            Self::Unavailable => "mainThreadHostedCallbackUnavailable",
        }
    }
}

impl WorkerRuntimeShell {
    pub fn issue_main_thread_hosted_callback_request(
        &mut self,
        callback_id: &str,
    ) -> Result<WorkerMainThreadHostedCallbackRequestEnvelope, WorthSignalJsError> {
        let request = self
            .core
            .main_thread_hosted_callback_closed_request(callback_id)?;
        let envelope = WorkerMainThreadHostedCallbackRequestEnvelope::from_closed_inputs(
            request.callback_id,
            request.closed_inputs,
            request.host_capability_read_count,
            self.next_host_boundary_causality(),
        )?;
        self.latest_main_thread_hosted_callback_request = Some(envelope.clone());
        self.latest_main_thread_hosted_callback_report = None;
        Ok(envelope)
    }

    pub fn admit_main_thread_hosted_callback_result(
        &mut self,
        request: WorkerMainThreadHostedCallbackRequestEnvelope,
        result: WorkerMainThreadHostedCallbackResult,
    ) -> Result<WorkerMainThreadHostedCallbackResultReport, WorthSignalJsError> {
        validate_main_thread_hosted_callback_result(&request, &result)?;
        let readmitted_result = readmit_main_thread_hosted_callback_result(&request, result)?;
        let runtime_admitted_result_count = u64::from(
            readmitted_result.payload().outcome == WorkerMainThreadHostedCallbackOutcome::Completed,
        );
        let runtime_mutation_breadth = if runtime_admitted_result_count == 0 {
            0
        } else {
            self.core.admit_main_thread_hosted_callback_result(
                MainThreadHostedCallbackAdmission {
                    callback_id: readmitted_result.payload().callback_id.clone(),
                    value: readmitted_result.payload().value.clone().ok_or_else(|| {
                        WorthSignalJsError::invalid_input(
                            "completed main-thread-hosted callback result requires a value",
                        )
                    })?,
                    captured_read_ids: readmitted_result.payload().captured_read_ids.clone(),
                    captured_host_capability_reads: readmitted_result
                        .payload()
                        .captured_host_capability_reads
                        .clone(),
                    runtime_read_breadth: readmitted_result.payload().runtime_read_breadth,
                },
            )?
        };
        let worker_first_truth_digest = committed_truth_digest_for_runtime(&self.core)?;
        self.clear_worker_boundary_certification_evidence();
        let report = WorkerMainThreadHostedCallbackResultReport::from_readmitted_result(
            &request,
            readmitted_result,
            self.next_host_boundary_causality(),
            runtime_admitted_result_count,
            runtime_mutation_breadth,
            worker_first_truth_digest,
        )?;
        self.latest_main_thread_hosted_callback_request = Some(request);
        self.latest_main_thread_hosted_callback_report = Some(report.clone());
        Ok(report)
    }

    pub fn certify_main_thread_hosted_callback_execution(
        &self,
    ) -> Result<WorkerMainThreadHostedCallbackExecutionCertificationPackage, WorthSignalJsError>
    {
        WorkerMainThreadHostedCallbackExecutionCertificationPackage::from_execution_evidence(
            &self.core,
            self.latest_main_thread_hosted_callback_request()?,
            self.latest_main_thread_hosted_callback_report()?,
        )
    }

    #[cfg(test)]
    pub fn define_main_thread_hosted_callback_for_test(
        &mut self,
        id: String,
        callback: Box<
            dyn Fn() -> Result<
                crate::runtime::compute_callbacks::ComputeCallbackInvocationResult,
                crate::runtime::compute_callbacks::ComputeCallbackFailure,
            >,
        >,
    ) -> Result<(), WorthSignalJsError> {
        self.core.define_web_computed_native_callback(id, callback)
    }

    fn latest_main_thread_hosted_callback_request(
        &self,
    ) -> Result<&WorkerMainThreadHostedCallbackRequestEnvelope, WorthSignalJsError> {
        self.latest_main_thread_hosted_callback_request
            .as_ref()
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(
                    "main-thread-hosted callback certification requires retained request evidence",
                )
            })
    }

    fn latest_main_thread_hosted_callback_report(
        &self,
    ) -> Result<&WorkerMainThreadHostedCallbackResultReport, WorthSignalJsError> {
        self.latest_main_thread_hosted_callback_report
            .as_ref()
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(
                    "main-thread-hosted callback certification requires retained result evidence",
                )
            })
    }
}

fn readmit_main_thread_hosted_callback_result(
    request: &WorkerMainThreadHostedCallbackRequestEnvelope,
    result: WorkerMainThreadHostedCallbackResult,
) -> Result<ReadmittedMainThreadHostedCallbackResult, WorthSignalJsError> {
    validate_closed_callback_frontier(request, &result)?;
    let runtime_admitted_result_count =
        u64::from(result.outcome == WorkerMainThreadHostedCallbackOutcome::Completed);
    let current = CurrentMainThreadHostedCallbackResult::with_current_basis(
        result,
        HostedCallbackBoundaryBasis {
            runtime_admitted_result_count,
        },
        hosted_callback_readmission_authority(),
    );
    let bridged: BridgedMainThreadHostedCallbackResult = current.bridge_trust_boundary();
    Ok(bridged.readmit_with_authority(
        HostedCallbackBoundaryBasis {
            runtime_admitted_result_count,
        },
        hosted_callback_readmission_authority(),
    ))
}

fn validate_closed_callback_frontier(
    request: &WorkerMainThreadHostedCallbackRequestEnvelope,
    result: &WorkerMainThreadHostedCallbackResult,
) -> Result<(), WorthSignalJsError> {
    if result.outcome != WorkerMainThreadHostedCallbackOutcome::Completed {
        return Ok(());
    }
    let mut captured = result.captured_read_ids.clone();
    captured.sort();
    captured.dedup();
    if captured != request.closed_input_ids {
        return Err(WorthSignalJsError::invalid_input(
            "main-thread-hosted callback result cannot read outside the closed worker-issued input frontier",
        ));
    }

    Ok(())
}

fn hosted_callback_readmission_authority(
) -> worth_proof::AuthorityWitness<HostedCallbackReadmissionAuthority> {
    worth_proof::AuthorityWitness::from_authority_marker(HostedCallbackReadmissionAuthority)
}

impl From<MainThreadHostedCallbackClosedInput> for WorkerMainThreadHostedCallbackInput {
    fn from(input: MainThreadHostedCallbackClosedInput) -> Self {
        Self {
            id: input.id,
            value: input.value,
        }
    }
}
