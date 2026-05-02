use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::data::handle::NodeId;
use crate::data::resource::ResourceBoundaryPerformanceEnvelope;

use super::{
    AsyncNodeHierarchyReplaySummary, AsyncNodeHistoricalParityReport,
    DeniedAsyncNodeHistoricalParity,
};

const ASYNC_NODE_HIERARCHY_HISTORICAL_PARITY_REPORT_SCHEMA_VERSION: &str =
    "forge-signal-async-node-hierarchy-historical-parity-report-v1";
const DENIED_ASYNC_NODE_HIERARCHY_HISTORICAL_PARITY_SCHEMA_VERSION: &str =
    "forge-signal-denied-async-node-hierarchy-historical-parity-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AsyncNodeHierarchyHistoricalParityDenialClass {
    NotHierarchicalRoot,
    HistoricalParityDenied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeniedAsyncNodeHierarchyHistoricalParity {
    denial_class: AsyncNodeHierarchyHistoricalParityDenialClass,
    root_node: NodeId,
    historical_parity_denial: Option<DeniedAsyncNodeHistoricalParity>,
    performance: ResourceBoundaryPerformanceEnvelope,
    denial_digest: String,
}

impl DeniedAsyncNodeHierarchyHistoricalParity {
    pub(crate) fn not_hierarchical_root(
        root_node: NodeId,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self::new(
            AsyncNodeHierarchyHistoricalParityDenialClass::NotHierarchicalRoot,
            root_node,
            None,
            performance,
        )
    }

    pub(crate) fn historical_parity_denied(
        root_node: NodeId,
        historical_parity_denial: DeniedAsyncNodeHistoricalParity,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self::new(
            AsyncNodeHierarchyHistoricalParityDenialClass::HistoricalParityDenied,
            root_node,
            Some(historical_parity_denial),
            performance,
        )
    }

    fn new(
        denial_class: AsyncNodeHierarchyHistoricalParityDenialClass,
        root_node: NodeId,
        historical_parity_denial: Option<DeniedAsyncNodeHistoricalParity>,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let denial_digest =
            hierarchy_history_digest(&DeniedAsyncNodeHierarchyHistoricalParityDigestBasis {
                schema_version: DENIED_ASYNC_NODE_HIERARCHY_HISTORICAL_PARITY_SCHEMA_VERSION,
                denial_class,
                root_node,
                historical_parity_denial_digest: historical_parity_denial
                    .as_ref()
                    .map(|denial| denial.denial_digest()),
                performance,
            });
        Self {
            denial_class,
            root_node,
            historical_parity_denial,
            performance,
            denial_digest,
        }
    }

    pub fn denial_class(&self) -> AsyncNodeHierarchyHistoricalParityDenialClass {
        self.denial_class
    }

    pub fn root_node(&self) -> NodeId {
        self.root_node
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
pub struct AsyncNodeHierarchyHistoricalParityReport {
    root_node: NodeId,
    hierarchy_replay_summary: AsyncNodeHierarchyReplaySummary,
    historical_parity_report: AsyncNodeHistoricalParityReport,
    performance: ResourceBoundaryPerformanceEnvelope,
    parity_digest: String,
}

impl AsyncNodeHierarchyHistoricalParityReport {
    pub(crate) fn new(
        root_node: NodeId,
        hierarchy_replay_summary: AsyncNodeHierarchyReplaySummary,
        historical_parity_report: AsyncNodeHistoricalParityReport,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        let parity_digest =
            hierarchy_history_digest(&AsyncNodeHierarchyHistoricalParityDigestBasis {
                schema_version: ASYNC_NODE_HIERARCHY_HISTORICAL_PARITY_REPORT_SCHEMA_VERSION,
                root_node,
                hierarchy_replay_digest: hierarchy_replay_summary.replay_digest(),
                historical_parity_digest: historical_parity_report.parity_digest(),
                performance,
            });
        Self {
            root_node,
            hierarchy_replay_summary,
            historical_parity_report,
            performance,
            parity_digest,
        }
    }

    pub fn root_node(&self) -> NodeId {
        self.root_node
    }

    pub fn hierarchy_replay_summary(&self) -> &AsyncNodeHierarchyReplaySummary {
        &self.hierarchy_replay_summary
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
struct DeniedAsyncNodeHierarchyHistoricalParityDigestBasis<'a> {
    schema_version: &'static str,
    denial_class: AsyncNodeHierarchyHistoricalParityDenialClass,
    root_node: NodeId,
    historical_parity_denial_digest: Option<&'a str>,
    performance: ResourceBoundaryPerformanceEnvelope,
}

#[derive(Debug, Serialize)]
struct AsyncNodeHierarchyHistoricalParityDigestBasis<'a> {
    schema_version: &'static str,
    root_node: NodeId,
    hierarchy_replay_digest: &'a str,
    historical_parity_digest: &'a str,
    performance: ResourceBoundaryPerformanceEnvelope,
}

fn hierarchy_history_digest<T: Serialize>(basis: &T) -> String {
    let bytes =
        serde_json::to_vec(basis).expect("async node hierarchy history digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
