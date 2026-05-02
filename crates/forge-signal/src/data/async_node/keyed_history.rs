use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::data::output::{ComputationFamily, ComputationKey};
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;

use super::{
    AsyncKeyedNodeCapabilityBinding, AsyncNodeHistoricalParityReport,
    DeniedAsyncNodeHistoricalParity,
};

const ASYNC_KEYED_NODE_HISTORICAL_PARITY_REPORT_SCHEMA_VERSION: &str =
    "forge-signal-async-keyed-node-historical-parity-report-v1";
const DENIED_ASYNC_KEYED_NODE_HISTORICAL_PARITY_SCHEMA_VERSION: &str =
    "forge-signal-denied-async-keyed-node-historical-parity-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AsyncKeyedNodeHistoricalParityDenialClass {
    BindingHandleNodeMismatch,
    BindingHandleDigestMismatch,
    HistoricalParityDenied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeniedAsyncKeyedNodeHistoricalParity {
    denial_class: AsyncKeyedNodeHistoricalParityDenialClass,
    family: ComputationFamily,
    key: ComputationKey,
    binding_node: crate::data::handle::NodeId,
    handle_node: crate::data::handle::NodeId,
    historical_parity_denial: Option<DeniedAsyncNodeHistoricalParity>,
    performance: ResourceBoundaryPerformanceEnvelope,
    denial_digest: String,
}

impl DeniedAsyncKeyedNodeHistoricalParity {
    pub(crate) fn binding_handle_node_mismatch(
        binding: &AsyncKeyedNodeCapabilityBinding,
        handle_node: crate::data::handle::NodeId,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self::new(
            AsyncKeyedNodeHistoricalParityDenialClass::BindingHandleNodeMismatch,
            binding,
            handle_node,
            None,
            performance,
        )
    }

    pub(crate) fn binding_handle_digest_mismatch(
        binding: &AsyncKeyedNodeCapabilityBinding,
        handle_node: crate::data::handle::NodeId,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self::new(
            AsyncKeyedNodeHistoricalParityDenialClass::BindingHandleDigestMismatch,
            binding,
            handle_node,
            None,
            performance,
        )
    }

    pub(crate) fn historical_parity_denied(
        binding: &AsyncKeyedNodeCapabilityBinding,
        handle_node: crate::data::handle::NodeId,
        historical_parity_denial: DeniedAsyncNodeHistoricalParity,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self::new(
            AsyncKeyedNodeHistoricalParityDenialClass::HistoricalParityDenied,
            binding,
            handle_node,
            Some(historical_parity_denial),
            performance,
        )
    }

    fn new(
        denial_class: AsyncKeyedNodeHistoricalParityDenialClass,
        binding: &AsyncKeyedNodeCapabilityBinding,
        handle_node: crate::data::handle::NodeId,
        historical_parity_denial: Option<DeniedAsyncNodeHistoricalParity>,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let denial_digest =
            keyed_history_digest(&DeniedAsyncKeyedNodeHistoricalParityDigestBasis {
                schema_version: DENIED_ASYNC_KEYED_NODE_HISTORICAL_PARITY_SCHEMA_VERSION,
                denial_class,
                family: binding.family(),
                key: binding.key(),
                binding_node: binding.node(),
                handle_node,
                historical_parity_denial_digest: historical_parity_denial
                    .as_ref()
                    .map(|denial| denial.denial_digest()),
                performance,
            });
        Self {
            denial_class,
            family: binding.family().clone(),
            key: binding.key().clone(),
            binding_node: binding.node(),
            handle_node,
            historical_parity_denial,
            performance,
            denial_digest,
        }
    }

    pub fn denial_class(&self) -> AsyncKeyedNodeHistoricalParityDenialClass {
        self.denial_class
    }

    pub fn family(&self) -> &ComputationFamily {
        &self.family
    }

    pub fn key(&self) -> &ComputationKey {
        &self.key
    }

    pub fn binding_node(&self) -> crate::data::handle::NodeId {
        self.binding_node
    }

    pub fn handle_node(&self) -> crate::data::handle::NodeId {
        self.handle_node
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
pub struct AsyncKeyedNodeHistoricalParityReport {
    family: ComputationFamily,
    key: ComputationKey,
    node: crate::data::handle::NodeId,
    registry_digest: crate::data::resource::ResourcePolicyDigest,
    bundle_digest: crate::data::resource::ResourcePolicyDigest,
    payload_contract_digest: crate::data::resource::ResourcePayloadContractDigest,
    historical_parity_report: AsyncNodeHistoricalParityReport,
    performance: ResourceBoundaryPerformanceEnvelope,
    parity_digest: String,
}

impl AsyncKeyedNodeHistoricalParityReport {
    pub(crate) fn new(
        binding: &AsyncKeyedNodeCapabilityBinding,
        historical_parity_report: AsyncNodeHistoricalParityReport,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let parity_digest = keyed_history_digest(&AsyncKeyedNodeHistoricalParityDigestBasis {
            schema_version: ASYNC_KEYED_NODE_HISTORICAL_PARITY_REPORT_SCHEMA_VERSION,
            family: binding.family(),
            key: binding.key(),
            node: binding.node(),
            registry_digest: binding.registry_digest(),
            bundle_digest: binding.bundle_digest(),
            payload_contract_digest: binding.payload_contract_digest(),
            historical_parity_digest: historical_parity_report.parity_digest(),
            performance,
        });
        Self {
            family: binding.family().clone(),
            key: binding.key().clone(),
            node: binding.node(),
            registry_digest: binding.registry_digest().clone(),
            bundle_digest: binding.bundle_digest().clone(),
            payload_contract_digest: binding.payload_contract_digest().clone(),
            historical_parity_report,
            performance,
            parity_digest,
        }
    }

    pub fn family(&self) -> &ComputationFamily {
        &self.family
    }

    pub fn key(&self) -> &ComputationKey {
        &self.key
    }

    pub fn node(&self) -> crate::data::handle::NodeId {
        self.node
    }

    pub fn registry_digest(&self) -> &crate::data::resource::ResourcePolicyDigest {
        &self.registry_digest
    }

    pub fn bundle_digest(&self) -> &crate::data::resource::ResourcePolicyDigest {
        &self.bundle_digest
    }

    pub fn payload_contract_digest(&self) -> &crate::data::resource::ResourcePayloadContractDigest {
        &self.payload_contract_digest
    }

    pub fn historical_parity_report(&self) -> &AsyncNodeHistoricalParityReport {
        &self.historical_parity_report
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn parity_digest(&self) -> &str {
        &self.parity_digest
    }
}

#[derive(Debug, Serialize)]
struct DeniedAsyncKeyedNodeHistoricalParityDigestBasis<'a> {
    schema_version: &'static str,
    denial_class: AsyncKeyedNodeHistoricalParityDenialClass,
    family: &'a ComputationFamily,
    key: &'a ComputationKey,
    binding_node: crate::data::handle::NodeId,
    handle_node: crate::data::handle::NodeId,
    historical_parity_denial_digest: Option<&'a str>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct AsyncKeyedNodeHistoricalParityDigestBasis<'a> {
    schema_version: &'static str,
    family: &'a ComputationFamily,
    key: &'a ComputationKey,
    node: crate::data::handle::NodeId,
    registry_digest: &'a crate::data::resource::ResourcePolicyDigest,
    bundle_digest: &'a crate::data::resource::ResourcePolicyDigest,
    payload_contract_digest: &'a crate::data::resource::ResourcePayloadContractDigest,
    historical_parity_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

fn keyed_history_digest<T: Serialize>(basis: &T) -> String {
    let bytes =
        serde_json::to_vec(basis).expect("async keyed node historical parity digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
