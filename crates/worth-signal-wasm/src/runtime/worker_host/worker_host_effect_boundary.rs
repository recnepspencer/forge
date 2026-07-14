use serde::{Deserialize, Serialize};

use crate::boundary::errors::WorthSignalJsError;
use crate::expression::model::SignalValue;
use crate::recipe::model::SetValue;

use super::{
    canonical_worker_certification_digest, WorkerHostBoundaryCausality,
    WorkerHostBoundaryPerformanceEnvelope,
};

type CurrentHostEffectBoundaryBasis = worth_proof::FreshnessScopedBasis<
    worth_proof::CurrentValidity,
    worth_proof::AssumptionBasis<HostEffectBoundaryBasis>,
>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHostEffectRequest {
    pub effect_id: String,
    pub host_capability_family: String,
    pub closed_payload_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHostEffectRequestEnvelope {
    pub envelope_family: &'static str,
    pub causality: WorkerHostBoundaryCausality,
    pub request_digest: String,
    pub host_execution_boundary: &'static str,
    pub performance: WorkerHostBoundaryPerformanceEnvelope,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHostEffectAcknowledgement {
    pub request_digest: String,
    pub outcome: WorkerHostEffectOutcome,
    pub artifact_identity: String,
    #[serde(default)]
    pub runtime_lifecycle_source_id: Option<String>,
    #[serde(default)]
    pub lifecycle_value: Option<SignalValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WorkerHostEffectOutcome {
    Completed,
    Failed,
    Detached,
    Unavailable,
    Denied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHostEffectAcknowledgementReport {
    pub envelope_family: &'static str,
    pub causality: WorkerHostBoundaryCausality,
    pub acknowledged_request_digest: String,
    pub acknowledgement_digest: String,
    pub host_effect_lifecycle_artifact: String,
    pub lifecycle_integrity_digest: String,
    pub worth_proof_readmission_digest: String,
    pub runtime_admitted_lifecycle_count: u64,
    pub runtime_mutation_breadth: u32,
    pub worker_first_truth_digest: String,
    pub performance: WorkerHostBoundaryPerformanceEnvelope,
    pub host_acknowledgement_is_authoritative: bool,
    pub worker_readmission_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::runtime::worker_host) struct HostEffectAcknowledgementBoundaryPayload;
impl worth_proof::PhaseMarker for HostEffectAcknowledgementBoundaryPayload {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HostEffectReadmissionAuthority;
impl worth_proof::AuthorityMarker for HostEffectReadmissionAuthority {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::runtime::worker_host) struct HostEffectBoundaryBasis {
    lifecycle_fact_count: u64,
}

pub(in crate::runtime::worker_host) type CurrentHostEffectAcknowledgement = worth_proof::Artifact<
    HostEffectAcknowledgementBoundaryPayload,
    WorkerHostEffectAcknowledgement,
    worth_proof::NoProofs,
    CurrentHostEffectBoundaryBasis,
>;

pub(in crate::runtime::worker_host) type BridgedHostEffectAcknowledgement = worth_proof::Artifact<
    HostEffectAcknowledgementBoundaryPayload,
    WorkerHostEffectAcknowledgement,
    worth_proof::NoProofs,
    worth_proof::BoundaryBridgedAuthorityRevalidationRequiredBasis<HostEffectBoundaryBasis>,
>;

pub(in crate::runtime::worker_host) type ReadmittedHostEffectAcknowledgement =
    worth_proof::Artifact<
        HostEffectAcknowledgementBoundaryPayload,
        WorkerHostEffectAcknowledgement,
        worth_proof::NoProofs,
        CurrentHostEffectBoundaryBasis,
    >;

pub(in crate::runtime::worker_host) struct WorkerHostEffectReadmission {
    pub acknowledgement: ReadmittedHostEffectAcknowledgement,
    pub runtime_values: Vec<SetValue>,
}

impl WorkerHostEffectRequestEnvelope {
    pub(in crate::runtime::worker_host) fn from_request(
        request: WorkerHostEffectRequest,
        causality: WorkerHostBoundaryCausality,
    ) -> Result<Self, WorthSignalJsError> {
        Ok(Self {
            envelope_family: "hostEffectEgress",
            causality: causality.clone(),
            request_digest: canonical_worker_certification_digest(&request)?,
            host_execution_boundary: "mainThreadHostEffect",
            performance: WorkerHostBoundaryPerformanceEnvelope::host_effect_request(
                request.closed_payload_identity.as_str(),
            )?,
        })
    }
}

impl WorkerHostEffectAcknowledgementReport {
    pub(in crate::runtime::worker_host) fn from_acknowledgement(
        acknowledgement: ReadmittedHostEffectAcknowledgement,
        causality: WorkerHostBoundaryCausality,
        runtime_admitted_lifecycle_count: u64,
        runtime_mutation_breadth: u32,
        worker_first_truth_digest: String,
    ) -> Result<Self, WorthSignalJsError> {
        let host_effect_lifecycle_artifact = acknowledgement
            .payload()
            .outcome
            .host_effect_lifecycle_artifact();

        Ok(Self {
            envelope_family: "hostEffectEgress",
            causality: causality.clone(),
            acknowledged_request_digest: acknowledgement.payload().request_digest.clone(),
            acknowledgement_digest: canonical_worker_certification_digest(
                acknowledgement.payload(),
            )?,
            host_effect_lifecycle_artifact: host_effect_lifecycle_artifact.to_owned(),
            lifecycle_integrity_digest: canonical_worker_certification_digest(&(
                acknowledgement.payload().request_digest.as_str(),
                host_effect_lifecycle_artifact,
                causality,
            ))?,
            worth_proof_readmission_digest: canonical_worker_certification_digest(&(
                "WorthProofHostEffectBoundaryReadmission",
                std::any::type_name::<BridgedHostEffectAcknowledgement>(),
                std::any::type_name::<ReadmittedHostEffectAcknowledgement>(),
                acknowledgement.basis().basis().value().lifecycle_fact_count,
            ))?,
            runtime_admitted_lifecycle_count,
            runtime_mutation_breadth,
            worker_first_truth_digest,
            performance: WorkerHostBoundaryPerformanceEnvelope::host_effect_acknowledgement(
                acknowledgement.payload().artifact_identity.as_str(),
                runtime_admitted_lifecycle_count,
                runtime_mutation_breadth,
            )?,
            host_acknowledgement_is_authoritative: false,
            worker_readmission_required: runtime_admitted_lifecycle_count == 0,
        })
    }
}

impl WorkerHostEffectOutcome {
    fn host_effect_lifecycle_artifact(self) -> &'static str {
        match self {
            Self::Completed => "hostEffectCompleted",
            Self::Failed => "hostEffectFailed",
            Self::Detached => "hostEffectDetached",
            Self::Unavailable => "hostEffectUnavailable",
            Self::Denied => "hostEffectDenied",
        }
    }

    fn allows_lifecycle_readmission(self) -> bool {
        matches!(self, Self::Completed | Self::Failed)
    }
}

pub(in crate::runtime::worker_host) fn bridge_host_effect_acknowledgement(
    acknowledgement: WorkerHostEffectAcknowledgement,
) -> BridgedHostEffectAcknowledgement {
    let lifecycle_fact_count = host_effect_lifecycle_fact_count(&acknowledgement);
    let current_acknowledgement = CurrentHostEffectAcknowledgement::with_current_basis(
        acknowledgement,
        HostEffectBoundaryBasis {
            lifecycle_fact_count,
        },
        host_effect_readmission_authority(),
    );

    current_acknowledgement.bridge_trust_boundary()
}

pub(in crate::runtime::worker_host) fn readmit_host_effect_acknowledgement(
    acknowledgement: BridgedHostEffectAcknowledgement,
) -> Result<WorkerHostEffectReadmission, WorthSignalJsError> {
    let runtime_values = lifecycle_facts_for_host_effect_readmission(&acknowledgement)?;
    let lifecycle_fact_count = runtime_values.len() as u64;
    let readmitted_acknowledgement = acknowledgement.readmit_with_authority(
        HostEffectBoundaryBasis {
            lifecycle_fact_count,
        },
        host_effect_readmission_authority(),
    );

    Ok(WorkerHostEffectReadmission {
        acknowledgement: readmitted_acknowledgement,
        runtime_values,
    })
}

fn lifecycle_facts_for_host_effect_readmission(
    acknowledgement: &BridgedHostEffectAcknowledgement,
) -> Result<Vec<SetValue>, WorthSignalJsError> {
    if !acknowledgement
        .payload()
        .outcome
        .allows_lifecycle_readmission()
        && (acknowledgement
            .payload()
            .runtime_lifecycle_source_id
            .is_some()
            || acknowledgement.payload().lifecycle_value.is_some())
    {
        return Err(WorthSignalJsError::invalid_input(
            "detached unavailable or denied host effect artifacts cannot mutate worker runtime truth",
        ));
    }

    match (
        &acknowledgement.payload().runtime_lifecycle_source_id,
        &acknowledgement.payload().lifecycle_value,
    ) {
        (Some(runtime_lifecycle_source_id), Some(lifecycle_value)) => Ok(vec![SetValue {
            id: runtime_lifecycle_source_id.clone(),
            value: lifecycle_value.clone(),
            aspect: None,
            aspects: None,
        }]),
        (None, None) => Ok(Vec::new()),
        _ => Err(WorthSignalJsError::invalid_input(
            "host effect lifecycle readmission requires a paired runtime lifecycle source id with lifecycle value",
        )),
    }
}

fn host_effect_lifecycle_fact_count(acknowledgement: &WorkerHostEffectAcknowledgement) -> u64 {
    u64::from(
        acknowledgement.runtime_lifecycle_source_id.is_some()
            && acknowledgement.lifecycle_value.is_some(),
    )
}

fn host_effect_readmission_authority(
) -> worth_proof::AuthorityWitness<HostEffectReadmissionAuthority> {
    worth_proof::AuthorityWitness::from_authority_marker(HostEffectReadmissionAuthority)
}
