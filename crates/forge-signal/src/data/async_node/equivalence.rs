use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::data::handle::NodeId;
use crate::data::resource::{
    ResourceBoundaryPerformanceEnvelope, ResourcePayloadContractDigest, ResourcePolicyDigest,
};
use crate::diagnostics::DiagnosticsAvailability;

use super::{
    AsyncNodeCapabilityAliasLoweringProof, AsyncNodeCapabilityDeclaration,
    AsyncNodeHistoricalParityReport, DeniedAsyncNodeHistoricalParity,
};

const ASYNC_NODE_CAPABILITY_EQUIVALENCE_REPORT_SCHEMA_VERSION: &str =
    "forge-signal-async-node-capability-equivalence-report-v1";
const DENIED_ASYNC_NODE_CAPABILITY_EQUIVALENCE_SCHEMA_VERSION: &str =
    "forge-signal-denied-async-node-capability-equivalence-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AsyncNodeCapabilityEquivalenceDenialClass {
    HandleDeclarationNodeMismatch,
    HandleDeclarationDigestMismatch,
    HistoricalParityDenied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeniedAsyncNodeCapabilityEquivalence {
    denial_class: AsyncNodeCapabilityEquivalenceDenialClass,
    handle_node: NodeId,
    declaration_node: NodeId,
    historical_parity_denial: Option<DeniedAsyncNodeHistoricalParity>,
    performance: ResourceBoundaryPerformanceEnvelope,
    denial_digest: String,
}

impl DeniedAsyncNodeCapabilityEquivalence {
    pub(crate) fn handle_declaration_node_mismatch(
        handle_node: NodeId,
        declaration_node: NodeId,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let denial_class = AsyncNodeCapabilityEquivalenceDenialClass::HandleDeclarationNodeMismatch;
        let denial_digest = canonical_digest(&DeniedAsyncNodeCapabilityEquivalenceDigestBasis {
            schema_version: DENIED_ASYNC_NODE_CAPABILITY_EQUIVALENCE_SCHEMA_VERSION,
            denial_class,
            handle_node,
            declaration_node,
            historical_parity_denial_digest: None,
            performance,
        });
        Self {
            denial_class,
            handle_node,
            declaration_node,
            historical_parity_denial: None,
            performance,
            denial_digest,
        }
    }

    pub(crate) fn historical_parity_denied(
        handle_node: NodeId,
        declaration_node: NodeId,
        historical_parity_denial: DeniedAsyncNodeHistoricalParity,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let denial_class = AsyncNodeCapabilityEquivalenceDenialClass::HistoricalParityDenied;
        let denial_digest = canonical_digest(&DeniedAsyncNodeCapabilityEquivalenceDigestBasis {
            schema_version: DENIED_ASYNC_NODE_CAPABILITY_EQUIVALENCE_SCHEMA_VERSION,
            denial_class,
            handle_node,
            declaration_node,
            historical_parity_denial_digest: Some(historical_parity_denial.denial_digest()),
            performance,
        });
        Self {
            denial_class,
            handle_node,
            declaration_node,
            historical_parity_denial: Some(historical_parity_denial),
            performance,
            denial_digest,
        }
    }

    pub(crate) fn handle_declaration_digest_mismatch(
        handle_node: NodeId,
        declaration_node: NodeId,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let denial_class =
            AsyncNodeCapabilityEquivalenceDenialClass::HandleDeclarationDigestMismatch;
        let denial_digest = canonical_digest(&DeniedAsyncNodeCapabilityEquivalenceDigestBasis {
            schema_version: DENIED_ASYNC_NODE_CAPABILITY_EQUIVALENCE_SCHEMA_VERSION,
            denial_class,
            handle_node,
            declaration_node,
            historical_parity_denial_digest: None,
            performance,
        });
        Self {
            denial_class,
            handle_node,
            declaration_node,
            historical_parity_denial: None,
            performance,
            denial_digest,
        }
    }

    pub fn denial_class(&self) -> AsyncNodeCapabilityEquivalenceDenialClass {
        self.denial_class
    }

    pub fn handle_node(&self) -> NodeId {
        self.handle_node
    }

    pub fn declaration_node(&self) -> NodeId {
        self.declaration_node
    }

    pub fn historical_parity_denial(&self) -> Option<&DeniedAsyncNodeHistoricalParity> {
        self.historical_parity_denial.as_ref()
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsyncNodeCapabilityEquivalenceReport {
    node: NodeId,
    capability_declaration_digest: String,
    legacy_declaration_digest: String,
    registry_digest: ResourcePolicyDigest,
    bundle_digest: ResourcePolicyDigest,
    payload_contract_digest: ResourcePayloadContractDigest,
    lifecycle_digest: String,
    output_continuity_digest: String,
    denial_digest: String,
    observation_digest: Option<String>,
    explanation_digest: Option<String>,
    explanation_availability: DiagnosticsAvailability,
    replay_restore_digest: String,
    alias_lowering_proof: AsyncNodeCapabilityAliasLoweringProof,
    historical_parity_report: AsyncNodeHistoricalParityReport,
    performance: ResourceBoundaryPerformanceEnvelope,
    equivalence_digest: String,
}

impl AsyncNodeCapabilityEquivalenceReport {
    pub(crate) fn new(
        declaration: &AsyncNodeCapabilityDeclaration,
        alias_lowering_proof: AsyncNodeCapabilityAliasLoweringProof,
        historical_parity_report: AsyncNodeHistoricalParityReport,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let capability_declaration_digest = canonical_digest(declaration);
        let legacy_declaration_digest =
            canonical_digest(&declaration.clone().into_legacy_resource_declaration());
        let replay = historical_parity_report.replay_reconstruction();
        let observation_digest = historical_parity_report
            .observation_batch_report()
            .map(canonical_digest);
        let explanation_digest = historical_parity_report
            .explanation_summary()
            .map(canonical_digest);
        let explanation_availability = historical_parity_report.explanation_availability();
        let replay_restore_digest = canonical_digest(&ReplayRestoreDigestBasis {
            replay_digest: replay.replay_digest(),
            branch_restore_report: historical_parity_report.branch_restore_report(),
        });
        let equivalence_digest = canonical_digest(&AsyncNodeCapabilityEquivalenceDigestBasis {
            schema_version: ASYNC_NODE_CAPABILITY_EQUIVALENCE_REPORT_SCHEMA_VERSION,
            node: declaration.node(),
            capability_declaration_digest: &capability_declaration_digest,
            legacy_declaration_digest: &legacy_declaration_digest,
            registry_digest: alias_lowering_proof.capability_registry_digest(),
            bundle_digest: alias_lowering_proof.capability_bundle_digest(),
            payload_contract_digest: alias_lowering_proof.capability_payload_contract_digest(),
            lifecycle_digest: replay.lifecycle_digest(),
            output_continuity_digest: replay.output_continuity_digest(),
            denial_digest: replay.denied_completion_digest(),
            observation_digest: observation_digest.as_deref(),
            explanation_digest: explanation_digest.as_deref(),
            explanation_availability,
            replay_restore_digest: &replay_restore_digest,
            alias_lowering_proof: &alias_lowering_proof,
            historical_parity_digest: historical_parity_report.parity_digest(),
            performance,
        });
        Self {
            node: declaration.node(),
            capability_declaration_digest,
            legacy_declaration_digest,
            registry_digest: alias_lowering_proof.capability_registry_digest().clone(),
            bundle_digest: alias_lowering_proof.capability_bundle_digest().clone(),
            payload_contract_digest: alias_lowering_proof
                .capability_payload_contract_digest()
                .clone(),
            lifecycle_digest: replay.lifecycle_digest().to_owned(),
            output_continuity_digest: replay.output_continuity_digest().to_owned(),
            denial_digest: replay.denied_completion_digest().to_owned(),
            observation_digest,
            explanation_digest,
            explanation_availability,
            replay_restore_digest,
            alias_lowering_proof,
            historical_parity_report,
            performance,
            equivalence_digest,
        }
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn capability_declaration_digest(&self) -> &str {
        &self.capability_declaration_digest
    }

    pub fn legacy_declaration_digest(&self) -> &str {
        &self.legacy_declaration_digest
    }

    pub fn registry_digest(&self) -> &ResourcePolicyDigest {
        &self.registry_digest
    }

    pub fn bundle_digest(&self) -> &ResourcePolicyDigest {
        &self.bundle_digest
    }

    pub fn payload_contract_digest(&self) -> &ResourcePayloadContractDigest {
        &self.payload_contract_digest
    }

    pub fn lifecycle_digest(&self) -> &str {
        &self.lifecycle_digest
    }

    pub fn output_continuity_digest(&self) -> &str {
        &self.output_continuity_digest
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }

    pub fn observation_digest(&self) -> Option<&str> {
        self.observation_digest.as_deref()
    }

    pub fn explanation_digest(&self) -> Option<&str> {
        self.explanation_digest.as_deref()
    }

    pub fn explanation_availability(&self) -> DiagnosticsAvailability {
        self.explanation_availability
    }

    pub fn replay_restore_digest(&self) -> &str {
        &self.replay_restore_digest
    }

    pub fn alias_lowering_proof(&self) -> &AsyncNodeCapabilityAliasLoweringProof {
        &self.alias_lowering_proof
    }

    pub fn historical_parity_report(&self) -> &AsyncNodeHistoricalParityReport {
        &self.historical_parity_report
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn equivalence_digest(&self) -> &str {
        &self.equivalence_digest
    }
}

#[derive(Debug, Serialize)]
struct AsyncNodeCapabilityEquivalenceDigestBasis<'a> {
    schema_version: &'static str,
    node: NodeId,
    capability_declaration_digest: &'a str,
    legacy_declaration_digest: &'a str,
    registry_digest: &'a ResourcePolicyDigest,
    bundle_digest: &'a ResourcePolicyDigest,
    payload_contract_digest: &'a ResourcePayloadContractDigest,
    lifecycle_digest: &'a str,
    output_continuity_digest: &'a str,
    denial_digest: &'a str,
    observation_digest: Option<&'a str>,
    explanation_digest: Option<&'a str>,
    explanation_availability: DiagnosticsAvailability,
    replay_restore_digest: &'a str,
    alias_lowering_proof: &'a AsyncNodeCapabilityAliasLoweringProof,
    historical_parity_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct DeniedAsyncNodeCapabilityEquivalenceDigestBasis<'a> {
    schema_version: &'static str,
    denial_class: AsyncNodeCapabilityEquivalenceDenialClass,
    handle_node: NodeId,
    declaration_node: NodeId,
    historical_parity_denial_digest: Option<&'a str>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct ReplayRestoreDigestBasis<'a> {
    replay_digest: &'a str,
    branch_restore_report: Option<crate::data::resource::ResourceBranchRestoreReport>,
}

fn canonical_digest<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value)
        .expect("async node capability equivalence serialization should succeed");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
