use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::data::output::{ComputationFamily, ComputationKey};
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;

use super::{
    AsyncKeyedNodeCapabilityBinding, AsyncNodeCapabilityEquivalenceReport,
    DeniedAsyncNodeCapabilityEquivalence,
};

const ASYNC_KEYED_NODE_CAPABILITY_EQUIVALENCE_REPORT_SCHEMA_VERSION: &str =
    "worth-signal-async-keyed-node-capability-equivalence-report-v1";
const DENIED_ASYNC_KEYED_NODE_CAPABILITY_EQUIVALENCE_SCHEMA_VERSION: &str =
    "worth-signal-denied-async-keyed-node-capability-equivalence-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AsyncKeyedNodeCapabilityEquivalenceDenialClass {
    BindingHandleNodeMismatch,
    BindingHandleDigestMismatch,
    CapabilityEquivalenceDenied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeniedAsyncKeyedNodeCapabilityEquivalence {
    denial_class: AsyncKeyedNodeCapabilityEquivalenceDenialClass,
    family: ComputationFamily,
    key: ComputationKey,
    binding_node: crate::data::handle::NodeId,
    handle_node: crate::data::handle::NodeId,
    capability_equivalence_denial: Option<DeniedAsyncNodeCapabilityEquivalence>,
    performance: ResourceBoundaryPerformanceEnvelope,
    denial_digest: String,
}

impl DeniedAsyncKeyedNodeCapabilityEquivalence {
    pub(crate) fn binding_handle_node_mismatch(
        binding: &AsyncKeyedNodeCapabilityBinding,
        handle_node: crate::data::handle::NodeId,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self::new(
            AsyncKeyedNodeCapabilityEquivalenceDenialClass::BindingHandleNodeMismatch,
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
            AsyncKeyedNodeCapabilityEquivalenceDenialClass::BindingHandleDigestMismatch,
            binding,
            handle_node,
            None,
            performance,
        )
    }

    pub(crate) fn capability_equivalence_denied(
        binding: &AsyncKeyedNodeCapabilityBinding,
        handle_node: crate::data::handle::NodeId,
        capability_equivalence_denial: DeniedAsyncNodeCapabilityEquivalence,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self::new(
            AsyncKeyedNodeCapabilityEquivalenceDenialClass::CapabilityEquivalenceDenied,
            binding,
            handle_node,
            Some(capability_equivalence_denial),
            performance,
        )
    }

    fn new(
        denial_class: AsyncKeyedNodeCapabilityEquivalenceDenialClass,
        binding: &AsyncKeyedNodeCapabilityBinding,
        handle_node: crate::data::handle::NodeId,
        capability_equivalence_denial: Option<DeniedAsyncNodeCapabilityEquivalence>,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let denial_digest =
            keyed_equivalence_digest(&DeniedAsyncKeyedNodeCapabilityEquivalenceDigestBasis {
                schema_version: DENIED_ASYNC_KEYED_NODE_CAPABILITY_EQUIVALENCE_SCHEMA_VERSION,
                denial_class,
                family: binding.family(),
                key: binding.key(),
                binding_node: binding.node(),
                handle_node,
                capability_equivalence_denial_digest: capability_equivalence_denial
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
            capability_equivalence_denial,
            performance,
            denial_digest,
        }
    }

    pub fn denial_class(&self) -> AsyncKeyedNodeCapabilityEquivalenceDenialClass {
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

    pub fn capability_equivalence_denial(&self) -> Option<&DeniedAsyncNodeCapabilityEquivalence> {
        self.capability_equivalence_denial.as_ref()
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
pub struct AsyncKeyedNodeCapabilityEquivalenceReport {
    family: ComputationFamily,
    key: ComputationKey,
    node: crate::data::handle::NodeId,
    equivalence_report: AsyncNodeCapabilityEquivalenceReport,
    performance: ResourceBoundaryPerformanceEnvelope,
    equivalence_digest: String,
}

impl AsyncKeyedNodeCapabilityEquivalenceReport {
    pub(crate) fn new(
        binding: &AsyncKeyedNodeCapabilityBinding,
        equivalence_report: AsyncNodeCapabilityEquivalenceReport,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let equivalence_digest =
            keyed_equivalence_digest(&AsyncKeyedNodeCapabilityEquivalenceDigestBasis {
                schema_version: ASYNC_KEYED_NODE_CAPABILITY_EQUIVALENCE_REPORT_SCHEMA_VERSION,
                family: binding.family(),
                key: binding.key(),
                node: binding.node(),
                equivalence_digest: equivalence_report.equivalence_digest(),
                performance,
            });
        Self {
            family: binding.family().clone(),
            key: binding.key().clone(),
            node: binding.node(),
            equivalence_report,
            performance,
            equivalence_digest,
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

    pub fn equivalence_report(&self) -> &AsyncNodeCapabilityEquivalenceReport {
        &self.equivalence_report
    }

    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }

    pub fn equivalence_digest(&self) -> &str {
        &self.equivalence_digest
    }
}

#[derive(Debug, Serialize)]
struct DeniedAsyncKeyedNodeCapabilityEquivalenceDigestBasis<'a> {
    schema_version: &'static str,
    denial_class: AsyncKeyedNodeCapabilityEquivalenceDenialClass,
    family: &'a ComputationFamily,
    key: &'a ComputationKey,
    binding_node: crate::data::handle::NodeId,
    handle_node: crate::data::handle::NodeId,
    capability_equivalence_denial_digest: Option<&'a str>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct AsyncKeyedNodeCapabilityEquivalenceDigestBasis<'a> {
    schema_version: &'static str,
    family: &'a ComputationFamily,
    key: &'a ComputationKey,
    node: crate::data::handle::NodeId,
    equivalence_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

fn keyed_equivalence_digest<T: Serialize>(basis: &T) -> String {
    let bytes = serde_json::to_vec(basis)
        .expect("async keyed node capability equivalence digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
