use std::mem::size_of;
use std::sync::Arc;

use worth_query_installation::facade::WorthQueryArtifactGovernanceContract;
use worth_relational::facade::runtime::RelationalExecutionBasisIdentity;

use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryProviderCheckpointExport;
use crate::domain_computation::{
    WorthQueryExecutionResourceAttemptEvidence, WorthQueryWorkflowArtifactRegistryEvidence,
};
use crate::execution_digest::hash_protocol_parts;

use super::super::{WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun};

pub const WORTH_QUERY_CHECKPOINT_EXPORT_PROTOCOL_IDENTITY: &str =
    "worth_query_checkpoint_export_handoff";
pub const WORTH_QUERY_CHECKPOINT_EXPORT_PROTOCOL_VERSION: u64 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryCheckpointExportCost {
    binding_material_bytes: usize,
    provider_payload_bytes: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCheckpointExportHandoff {
    logical_run_identity: Arc<str>,
    yielded_attempt_identity: Arc<str>,
    operation_binding_identity: Arc<str>,
    installed_operation_identity: Arc<str>,
    installation_generation: u64,
    semantic_basis_identity: Arc<str>,
    resource_attempt: WorthQueryExecutionResourceAttemptEvidence,
    bridge_basis_identity: Arc<str>,
    bridge_request_identity: Arc<str>,
    relational_basis_identity: RelationalExecutionBasisIdentity,
    provider_generation: u64,
    checkpoint_occurrence_identity: Arc<str>,
    artifact_run_identity: Option<Arc<str>>,
    artifact_evidence: Option<WorthQueryWorkflowArtifactRegistryEvidence>,
    provider: WorthQueryProviderCheckpointExport,
    binding_digest: Arc<str>,
    cost: WorthQueryCheckpointExportCost,
}

struct WorthQueryCheckpointExportBinding {
    logical_run_identity: Arc<str>,
    yielded_attempt_identity: Arc<str>,
    operation_binding_identity: Arc<str>,
    installed_operation_identity: Arc<str>,
    installation_generation: u64,
    semantic_basis_identity: Arc<str>,
    resource_attempt: WorthQueryExecutionResourceAttemptEvidence,
    bridge_basis_identity: Arc<str>,
    bridge_request_identity: Arc<str>,
    relational_basis_identity: RelationalExecutionBasisIdentity,
    provider_generation: u64,
    checkpoint_occurrence_identity: Arc<str>,
    artifact_run_identity: Option<Arc<str>>,
    artifact_evidence: Option<WorthQueryWorkflowArtifactRegistryEvidence>,
    provider: WorthQueryProviderCheckpointExport,
}

impl WorthQueryCheckpointExportCost {
    pub const fn binding_material_bytes(self) -> usize {
        self.binding_material_bytes
    }

    pub const fn provider_payload_bytes(self) -> usize {
        self.provider_payload_bytes
    }

    pub const fn total_bound_bytes(self) -> usize {
        self.binding_material_bytes
            .saturating_add(self.provider_payload_bytes)
    }
}

impl WorthQueryCheckpointExportHandoff {
    pub const fn protocol_identity(&self) -> &'static str {
        WORTH_QUERY_CHECKPOINT_EXPORT_PROTOCOL_IDENTITY
    }

    pub const fn protocol_version(&self) -> u64 {
        WORTH_QUERY_CHECKPOINT_EXPORT_PROTOCOL_VERSION
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub const fn cost(&self) -> WorthQueryCheckpointExportCost {
        self.cost
    }

    pub fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub fn yielded_attempt_identity(&self) -> &str {
        &self.yielded_attempt_identity
    }

    pub fn operation_binding_identity(&self) -> &str {
        &self.operation_binding_identity
    }

    pub fn installed_operation_identity(&self) -> &str {
        &self.installed_operation_identity
    }

    pub const fn installation_generation(&self) -> u64 {
        self.installation_generation
    }

    pub fn semantic_basis_identity(&self) -> &str {
        &self.semantic_basis_identity
    }

    pub fn resource_attempt_evidence(&self) -> &WorthQueryExecutionResourceAttemptEvidence {
        &self.resource_attempt
    }

    pub fn bridge_basis_identity(&self) -> &str {
        &self.bridge_basis_identity
    }

    pub fn bridge_request_identity(&self) -> &str {
        &self.bridge_request_identity
    }

    pub fn relational_basis_identity(&self) -> &RelationalExecutionBasisIdentity {
        &self.relational_basis_identity
    }

    pub const fn provider_generation(&self) -> u64 {
        self.provider_generation
    }

    pub fn checkpoint_occurrence_identity(&self) -> &str {
        &self.checkpoint_occurrence_identity
    }

    pub fn artifact_run_identity(&self) -> Option<&str> {
        self.artifact_run_identity.as_deref()
    }

    pub const fn artifact_production_generation(&self) -> Option<u64> {
        match self.artifact_evidence {
            Some(evidence) => Some(evidence.production_generation()),
            None => None,
        }
    }

    pub const fn artifact_evidence(&self) -> Option<WorthQueryWorkflowArtifactRegistryEvidence> {
        self.artifact_evidence
    }

    pub fn governance(&self) -> &WorthQueryArtifactGovernanceContract {
        self.provider.governance()
    }

    pub fn provider_export(&self) -> &WorthQueryProviderCheckpointExport {
        &self.provider
    }

    pub(super) fn bind_direct(
        yielded: &WorthQueryYieldedDirectRun,
        provider: WorthQueryProviderCheckpointExport,
    ) -> Self {
        Self::bind(WorthQueryCheckpointExportBinding {
            logical_run_identity: Arc::from(yielded.logical_run_identity()),
            yielded_attempt_identity: Arc::from(yielded.yielded_attempt_identity()),
            operation_binding_identity: Arc::from(yielded.operation_binding_identity()),
            installed_operation_identity: Arc::from(yielded.installed_operation_identity()),
            installation_generation: yielded.installation_generation().ordinal(),
            semantic_basis_identity: Arc::from(yielded.semantic_basis_identity()),
            resource_attempt: yielded.resource_attempt_evidence().clone(),
            bridge_basis_identity: Arc::from(yielded.bridge().basis_identity().as_str()),
            bridge_request_identity: Arc::from(yielded.bridge_request_identity()),
            relational_basis_identity: yielded.relational_basis_identity().clone(),
            provider_generation: yielded.checkpoint().provider_generation(),
            checkpoint_occurrence_identity: Arc::from(yielded.checkpoint().identity()),
            artifact_run_identity: None,
            artifact_evidence: None,
            provider,
        })
    }

    pub(super) fn bind_workflow(
        yielded: &WorthQueryYieldedWorkflowRun,
        provider: WorthQueryProviderCheckpointExport,
    ) -> Self {
        Self::bind(WorthQueryCheckpointExportBinding {
            logical_run_identity: Arc::from(yielded.logical_run_identity()),
            yielded_attempt_identity: Arc::from(yielded.yielded_attempt_identity()),
            operation_binding_identity: Arc::from(yielded.operation_binding_identity()),
            installed_operation_identity: Arc::from(yielded.installed_operation_identity()),
            installation_generation: yielded.installation_generation().ordinal(),
            semantic_basis_identity: Arc::from(yielded.semantic_basis_identity()),
            resource_attempt: yielded.resource_attempt_evidence().clone(),
            bridge_basis_identity: Arc::from(yielded.bridge().basis_identity().as_str()),
            bridge_request_identity: Arc::from(yielded.bridge_request_identity()),
            relational_basis_identity: yielded.relational_basis_identity().clone(),
            provider_generation: yielded.checkpoint().provider_generation(),
            checkpoint_occurrence_identity: Arc::from(yielded.checkpoint().identity()),
            artifact_run_identity: Some(Arc::from(yielded.artifact_run_identity())),
            artifact_evidence: Some(yielded.artifact_evidence()),
            provider,
        })
    }

    fn bind(binding: WorthQueryCheckpointExportBinding) -> Self {
        let material = binding_material(&binding);
        let binding_material_bytes = material.iter().fold(0usize, |total, part| {
            total
                .saturating_add(size_of::<u64>())
                .saturating_add(part.len())
        });
        let cost = WorthQueryCheckpointExportCost {
            binding_material_bytes,
            provider_payload_bytes: binding.provider.payload_bytes(),
        };
        Self {
            logical_run_identity: binding.logical_run_identity,
            yielded_attempt_identity: binding.yielded_attempt_identity,
            operation_binding_identity: binding.operation_binding_identity,
            installed_operation_identity: binding.installed_operation_identity,
            installation_generation: binding.installation_generation,
            semantic_basis_identity: binding.semantic_basis_identity,
            resource_attempt: binding.resource_attempt,
            bridge_basis_identity: binding.bridge_basis_identity,
            bridge_request_identity: binding.bridge_request_identity,
            relational_basis_identity: binding.relational_basis_identity,
            provider_generation: binding.provider_generation,
            checkpoint_occurrence_identity: binding.checkpoint_occurrence_identity,
            artifact_run_identity: binding.artifact_run_identity,
            artifact_evidence: binding.artifact_evidence,
            provider: binding.provider,
            binding_digest: Arc::from(hash_protocol_parts(&material)),
            cost,
        }
    }
}

fn binding_material(binding: &WorthQueryCheckpointExportBinding) -> Vec<String> {
    let artifact = binding.artifact_evidence.unwrap_or_default();
    vec![
        WORTH_QUERY_CHECKPOINT_EXPORT_PROTOCOL_IDENTITY.to_owned(),
        format!("protocol-version:{WORTH_QUERY_CHECKPOINT_EXPORT_PROTOCOL_VERSION}"),
        format!("logical-run:{}", binding.logical_run_identity),
        format!("yielded-attempt:{}", binding.yielded_attempt_identity),
        format!("operation-binding:{}", binding.operation_binding_identity),
        format!(
            "installed-operation:{}",
            binding.installed_operation_identity
        ),
        format!(
            "installation-generation:{}",
            binding.installation_generation
        ),
        format!("semantic-basis:{}", binding.semantic_basis_identity),
        format!("resource-evidence:{}", binding.resource_attempt.identity()),
        format!(
            "resource-envelope:{}",
            binding.resource_attempt.envelope_identity()
        ),
        format!("bridge-basis:{}", binding.bridge_basis_identity),
        format!("bridge-request:{}", binding.bridge_request_identity),
        format!(
            "relational-runtime:{}",
            binding.relational_basis_identity.runtime_instance_id()
        ),
        format!(
            "relational-snapshot:{}",
            binding.relational_basis_identity.snapshot_id().0
        ),
        format!(
            "relational-lease:{}",
            binding.relational_basis_identity.lease_ordinal()
        ),
        format!("provider-generation:{}", binding.provider_generation),
        format!("checkpoint:{}", binding.checkpoint_occurrence_identity),
        format!("provider-contract:{}", binding.provider.contract_digest()),
        format!(
            "artifact-run:{}",
            binding.artifact_run_identity.as_deref().unwrap_or("none")
        ),
        format!(
            "artifact-production-generation:{}",
            artifact.production_generation()
        ),
        format!("artifact-produced:{}", artifact.produced_artifact_count()),
        format!("artifact-retained:{}", artifact.retained_artifact_count()),
        format!("artifact-disposed:{}", artifact.disposed_artifact_count()),
        format!("artifact-retained-bytes:{}", artifact.retained_bytes()),
    ]
}
