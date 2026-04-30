use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::data::handle::NodeId;
use crate::data::resource::{
    ResourceBoundaryPerformanceEnvelope, ResourceBranchRestoreReport,
    ResourceDiagnosticsExpansionDenial, ResourceDiagnosticsSummary, ResourceObservationBatchReport,
    ResourcePayloadContractDigest, ResourcePolicyDigest, ResourceReplayReconstructionReport,
};
use crate::diagnostics::{DiagnosticsAvailability, ExplanationSummary};

const ASYNC_NODE_HISTORICAL_PARITY_REPORT_SCHEMA_VERSION: &str =
    "forge-signal-async-node-historical-parity-report-v1";
const DENIED_ASYNC_NODE_HISTORICAL_PARITY_SCHEMA_VERSION: &str =
    "forge-signal-denied-async-node-historical-parity-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AsyncNodeHistoricalParityDenialClass {
    NonLiveOwner,
    UndeclaredCapability,
    RegistryDigestDrift,
    BundleDigestDrift,
    PayloadContractDigestDrift,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeniedAsyncNodeHistoricalParity {
    node: NodeId,
    denial_class: AsyncNodeHistoricalParityDenialClass,
    expected_registry_digest: ResourcePolicyDigest,
    current_registry_digest: Option<ResourcePolicyDigest>,
    expected_bundle_digest: ResourcePolicyDigest,
    current_bundle_digest: Option<ResourcePolicyDigest>,
    expected_payload_contract_digest: ResourcePayloadContractDigest,
    current_payload_contract_digest: Option<ResourcePayloadContractDigest>,
    performance: ResourceBoundaryPerformanceEnvelope,
    denial_digest: String,
}

impl DeniedAsyncNodeHistoricalParity {
    pub(crate) fn new(
        node: NodeId,
        denial_class: AsyncNodeHistoricalParityDenialClass,
        expected_registry_digest: ResourcePolicyDigest,
        current_registry_digest: Option<ResourcePolicyDigest>,
        expected_bundle_digest: ResourcePolicyDigest,
        current_bundle_digest: Option<ResourcePolicyDigest>,
        expected_payload_contract_digest: ResourcePayloadContractDigest,
        current_payload_contract_digest: Option<ResourcePayloadContractDigest>,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let denial_digest =
            async_node_history_digest(&DeniedAsyncNodeHistoricalParityDigestBasis {
                schema_version: DENIED_ASYNC_NODE_HISTORICAL_PARITY_SCHEMA_VERSION,
                node,
                denial_class,
                expected_registry_digest: &expected_registry_digest,
                current_registry_digest: current_registry_digest.as_ref(),
                expected_bundle_digest: &expected_bundle_digest,
                current_bundle_digest: current_bundle_digest.as_ref(),
                expected_payload_contract_digest: &expected_payload_contract_digest,
                current_payload_contract_digest: current_payload_contract_digest.as_ref(),
                performance,
            });
        Self {
            node,
            denial_class,
            expected_registry_digest,
            current_registry_digest,
            expected_bundle_digest,
            current_bundle_digest,
            expected_payload_contract_digest,
            current_payload_contract_digest,
            performance,
            denial_digest,
        }
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn denial_class(&self) -> AsyncNodeHistoricalParityDenialClass {
        self.denial_class
    }

    pub fn expected_registry_digest(&self) -> &ResourcePolicyDigest {
        &self.expected_registry_digest
    }

    pub fn current_registry_digest(&self) -> Option<&ResourcePolicyDigest> {
        self.current_registry_digest.as_ref()
    }

    pub fn expected_bundle_digest(&self) -> &ResourcePolicyDigest {
        &self.expected_bundle_digest
    }

    pub fn current_bundle_digest(&self) -> Option<&ResourcePolicyDigest> {
        self.current_bundle_digest.as_ref()
    }

    pub fn expected_payload_contract_digest(&self) -> &ResourcePayloadContractDigest {
        &self.expected_payload_contract_digest
    }

    pub fn current_payload_contract_digest(&self) -> Option<&ResourcePayloadContractDigest> {
        self.current_payload_contract_digest.as_ref()
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
pub struct AsyncNodeHistoricalParityReport {
    node: NodeId,
    registry_digest: ResourcePolicyDigest,
    bundle_digest: ResourcePolicyDigest,
    payload_contract_digest: ResourcePayloadContractDigest,
    branch_restore_report: Option<ResourceBranchRestoreReport>,
    replay_reconstruction: ResourceReplayReconstructionReport,
    observation_batch_report: Option<ResourceObservationBatchReport>,
    explanation_summary: Option<ExplanationSummary>,
    explanation_availability: DiagnosticsAvailability,
    diagnostics_summary: Option<ResourceDiagnosticsSummary>,
    diagnostics_denial: Option<ResourceDiagnosticsExpansionDenial>,
    performance: ResourceBoundaryPerformanceEnvelope,
    parity_digest: String,
}

impl AsyncNodeHistoricalParityReport {
    pub(crate) fn new(
        node: NodeId,
        registry_digest: ResourcePolicyDigest,
        bundle_digest: ResourcePolicyDigest,
        payload_contract_digest: ResourcePayloadContractDigest,
        branch_restore_report: Option<ResourceBranchRestoreReport>,
        replay_reconstruction: ResourceReplayReconstructionReport,
        observation_batch_report: Option<ResourceObservationBatchReport>,
        explanation_summary: Option<ExplanationSummary>,
        explanation_availability: DiagnosticsAvailability,
        diagnostics_summary: Option<ResourceDiagnosticsSummary>,
        diagnostics_denial: Option<ResourceDiagnosticsExpansionDenial>,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        assert!(
            diagnostics_summary.is_some() ^ diagnostics_denial.is_some(),
            "historical parity report must carry exactly one diagnostics outcome"
        );
        assert_eq!(
            explanation_summary.is_some(),
            explanation_availability.is_available(),
            "explanation availability must match whether explanation lineage is present"
        );
        let parity_digest = async_node_history_digest(&AsyncNodeHistoricalParityDigestBasis {
            schema_version: ASYNC_NODE_HISTORICAL_PARITY_REPORT_SCHEMA_VERSION,
            node,
            registry_digest: &registry_digest,
            bundle_digest: &bundle_digest,
            payload_contract_digest: &payload_contract_digest,
            branch_restore_report,
            replay_reconstruction: &replay_reconstruction,
            observation_batch_report: observation_batch_report.as_ref(),
            explanation_summary: explanation_summary.as_ref(),
            explanation_availability,
            diagnostics_summary: diagnostics_summary.as_ref(),
            diagnostics_denial: diagnostics_denial.as_ref(),
            performance,
        });
        Self {
            node,
            registry_digest,
            bundle_digest,
            payload_contract_digest,
            branch_restore_report,
            replay_reconstruction,
            observation_batch_report,
            explanation_summary,
            explanation_availability,
            diagnostics_summary,
            diagnostics_denial,
            performance,
            parity_digest,
        }
    }

    pub fn node(&self) -> NodeId {
        self.node
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

    pub fn branch_restore_report(&self) -> Option<ResourceBranchRestoreReport> {
        self.branch_restore_report
    }

    pub fn replay_reconstruction(&self) -> &ResourceReplayReconstructionReport {
        &self.replay_reconstruction
    }

    pub fn observation_batch_report(&self) -> Option<&ResourceObservationBatchReport> {
        self.observation_batch_report.as_ref()
    }

    pub fn explanation_summary(&self) -> Option<&ExplanationSummary> {
        self.explanation_summary.as_ref()
    }

    pub fn explanation_availability(&self) -> DiagnosticsAvailability {
        self.explanation_availability
    }

    pub fn diagnostics_summary(&self) -> Option<&ResourceDiagnosticsSummary> {
        self.diagnostics_summary.as_ref()
    }

    pub fn diagnostics_denial(&self) -> Option<&ResourceDiagnosticsExpansionDenial> {
        self.diagnostics_denial.as_ref()
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn parity_digest(&self) -> &str {
        &self.parity_digest
    }
}

#[derive(Debug, Serialize)]
struct DeniedAsyncNodeHistoricalParityDigestBasis<'a> {
    schema_version: &'static str,
    node: NodeId,
    denial_class: AsyncNodeHistoricalParityDenialClass,
    expected_registry_digest: &'a ResourcePolicyDigest,
    current_registry_digest: Option<&'a ResourcePolicyDigest>,
    expected_bundle_digest: &'a ResourcePolicyDigest,
    current_bundle_digest: Option<&'a ResourcePolicyDigest>,
    expected_payload_contract_digest: &'a ResourcePayloadContractDigest,
    current_payload_contract_digest: Option<&'a ResourcePayloadContractDigest>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct AsyncNodeHistoricalParityDigestBasis<'a> {
    schema_version: &'static str,
    node: NodeId,
    registry_digest: &'a ResourcePolicyDigest,
    bundle_digest: &'a ResourcePolicyDigest,
    payload_contract_digest: &'a ResourcePayloadContractDigest,
    branch_restore_report: Option<ResourceBranchRestoreReport>,
    replay_reconstruction: &'a ResourceReplayReconstructionReport,
    observation_batch_report: Option<&'a ResourceObservationBatchReport>,
    explanation_summary: Option<&'a ExplanationSummary>,
    explanation_availability: DiagnosticsAvailability,
    diagnostics_summary: Option<&'a ResourceDiagnosticsSummary>,
    diagnostics_denial: Option<&'a ResourceDiagnosticsExpansionDenial>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

fn async_node_history_digest<T: Serialize>(basis: &T) -> String {
    let bytes =
        serde_json::to_vec(basis).expect("async node history digest serialization should succeed");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
