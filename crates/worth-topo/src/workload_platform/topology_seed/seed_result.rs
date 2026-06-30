use super::{
    TopologySeedCounters, TopologySeedEntityIdentities, TopologySeedKind,
    TopologySeedNeighborhoodReceipt, TopologySeedQueryReceipts, TopologySeedTopologyPosture,
};
use crate::brep::topology_graph::TopologyView;
use crate::validation::TopologyValidationReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySeedValidationReceipt {
    validator_names: Vec<String>,
}

impl TopologySeedValidationReceipt {
    pub(crate) fn from_report(report: &TopologyValidationReport) -> Self {
        Self {
            validator_names: report
                .rows
                .iter()
                .map(|row| row.validator.clone())
                .collect(),
        }
    }

    pub fn validator_names(&self) -> &[String] {
        &self.validator_names
    }

    pub fn row_count(&self) -> usize {
        self.validator_names.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySeedReceipt {
    kind: TopologySeedKind,
    topology_posture: TopologySeedTopologyPosture,
    query_receipts: TopologySeedQueryReceipts,
    entity_identities: TopologySeedEntityIdentities,
    counters: TopologySeedCounters,
    validation: TopologySeedValidationReceipt,
    neighborhood: Option<TopologySeedNeighborhoodReceipt>,
}

impl TopologySeedReceipt {
    pub(crate) fn new(
        kind: TopologySeedKind,
        query_receipts: TopologySeedQueryReceipts,
        entity_identities: TopologySeedEntityIdentities,
        counters: TopologySeedCounters,
        validation: TopologySeedValidationReceipt,
        neighborhood: Option<TopologySeedNeighborhoodReceipt>,
    ) -> Self {
        Self {
            kind,
            topology_posture: kind.topology_posture(),
            query_receipts,
            entity_identities,
            counters,
            validation,
            neighborhood,
        }
    }

    pub fn kind(&self) -> TopologySeedKind {
        self.kind
    }

    pub fn topology_posture(&self) -> TopologySeedTopologyPosture {
        self.topology_posture
    }

    pub fn query_receipts(&self) -> &TopologySeedQueryReceipts {
        &self.query_receipts
    }

    pub fn entity_identities(&self) -> &TopologySeedEntityIdentities {
        &self.entity_identities
    }

    pub fn counters(&self) -> TopologySeedCounters {
        self.counters
    }

    pub fn validation(&self) -> &TopologySeedValidationReceipt {
        &self.validation
    }

    pub fn neighborhood(&self) -> Option<&TopologySeedNeighborhoodReceipt> {
        self.neighborhood.as_ref()
    }

    pub fn can_enter_spatial_binding(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySeedBuiltTopology {
    topology: TopologyView,
    receipt: TopologySeedReceipt,
}

impl TopologySeedBuiltTopology {
    pub fn new(topology: TopologyView, receipt: TopologySeedReceipt) -> Self {
        Self { topology, receipt }
    }

    pub fn topology(&self) -> &TopologyView {
        &self.topology
    }

    pub fn receipt(&self) -> &TopologySeedReceipt {
        &self.receipt
    }

    pub fn into_parts(self) -> (TopologyView, TopologySeedReceipt) {
        (self.topology, self.receipt)
    }
}
