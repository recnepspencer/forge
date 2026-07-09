use super::{AdmittedSourceContract, BridgeSourceCapability, MaterializedTruthViewPacketSet};
use crate::snapshot::MaterializedTruthViewObservation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceMaterializationCounters {
    source_declaration_count: usize,
    source_contract_count: usize,
    source_packet_count: usize,
    source_packet_member_count: usize,
    source_materialization_count: usize,
    source_snapshot_read_count: usize,
    source_historical_read_count: usize,
    source_branch_read_count: usize,
    source_facet_read_count: usize,
    source_capability_rejection_count: usize,
    source_contract_mismatch_count: usize,
    source_adapter_non_native_escape_count: usize,
    source_builder_configuration_conflict_count: usize,
}

impl SourceMaterializationCounters {
    pub(crate) fn from_observation(
        contract: &AdmittedSourceContract,
        observation: &MaterializedTruthViewObservation,
    ) -> Self {
        Self {
            source_declaration_count: 1,
            source_contract_count: 1,
            source_packet_count: 1,
            source_packet_member_count: observation.read_packet().reads().len(),
            source_materialization_count: 1,
            source_snapshot_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::SnapshotRead),
            ),
            source_historical_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::HistoricalRead),
            ),
            source_branch_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::BranchRead),
            ),
            source_facet_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::FacetRead),
            ),
            source_capability_rejection_count: 0,
            source_contract_mismatch_count: 0,
            source_adapter_non_native_escape_count: 0,
            source_builder_configuration_conflict_count: 0,
        }
    }

    pub(crate) fn from_packet_set(
        contract: &AdmittedSourceContract,
        materialized_packet_set: &MaterializedTruthViewPacketSet,
    ) -> Self {
        Self {
            source_declaration_count: 1,
            source_contract_count: 1,
            source_packet_count: materialized_packet_set.planned_packet_set().packet_count(),
            source_packet_member_count: materialized_packet_set
                .planned_packet_set()
                .packet_member_count(),
            source_materialization_count: materialized_packet_set.materialization_count(),
            source_snapshot_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::SnapshotRead),
            ),
            source_historical_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::HistoricalRead),
            ),
            source_branch_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::BranchRead),
            ),
            source_facet_read_count: usize::from(
                contract
                    .required_capabilities()
                    .contains(BridgeSourceCapability::FacetRead),
            ),
            source_capability_rejection_count: 0,
            source_contract_mismatch_count: 0,
            source_adapter_non_native_escape_count: 0,
            source_builder_configuration_conflict_count: 0,
        }
    }

    pub fn source_declaration_count(&self) -> usize {
        self.source_declaration_count
    }

    pub fn source_contract_count(&self) -> usize {
        self.source_contract_count
    }

    pub fn source_packet_count(&self) -> usize {
        self.source_packet_count
    }

    pub fn source_packet_member_count(&self) -> usize {
        self.source_packet_member_count
    }

    pub fn source_materialization_count(&self) -> usize {
        self.source_materialization_count
    }

    pub fn source_snapshot_read_count(&self) -> usize {
        self.source_snapshot_read_count
    }

    pub fn source_historical_read_count(&self) -> usize {
        self.source_historical_read_count
    }

    pub fn source_branch_read_count(&self) -> usize {
        self.source_branch_read_count
    }

    pub fn source_facet_read_count(&self) -> usize {
        self.source_facet_read_count
    }

    pub fn source_capability_rejection_count(&self) -> usize {
        self.source_capability_rejection_count
    }

    pub fn source_contract_mismatch_count(&self) -> usize {
        self.source_contract_mismatch_count
    }

    pub fn source_adapter_non_native_escape_count(&self) -> usize {
        self.source_adapter_non_native_escape_count
    }

    pub fn source_builder_configuration_conflict_count(&self) -> usize {
        self.source_builder_configuration_conflict_count
    }
}
