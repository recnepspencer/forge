use serde::Serialize;

use super::worker_boundary_causality::WorkerBoundaryCausalityModel;
use super::worker_boundary_envelope_family::{
    worker_boundary_envelope_families, WorkerBoundaryEnvelopeSummary,
};
use super::worker_boundary_proof_topology::{
    worker_boundary_proof_stages, WorkerBoundaryProofStageSummary,
};
use super::worker_deployment_posture::{
    worker_deployment_postures, WorkerDeploymentPostureSummary,
};
use super::worker_fallback_policy::{worker_fallback_policies, WorkerFallbackPolicySummary};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerBoundaryArtifactLock {
    pub artifact_lock_version: u32,
    pub envelope_families: Vec<WorkerBoundaryEnvelopeSummary>,
    pub causality_model: WorkerBoundaryCausalityModel,
    pub deployment_postures: Vec<WorkerDeploymentPostureSummary>,
    pub fallback_policies: Vec<WorkerFallbackPolicySummary>,
    pub proof_stages: Vec<WorkerBoundaryProofStageSummary>,
}

impl WorkerBoundaryArtifactLock {
    pub(in crate::runtime::worker_bridge) fn frozen_worker_boundary_contract() -> Self {
        Self {
            artifact_lock_version: 1,
            envelope_families: worker_boundary_envelope_families(),
            causality_model: WorkerBoundaryCausalityModel::transaction_sequence_then_generation(),
            deployment_postures: worker_deployment_postures(),
            fallback_policies: worker_fallback_policies(),
            proof_stages: worker_boundary_proof_stages(),
        }
    }
}
