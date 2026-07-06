use super::classification::assemble_ledger_bundle;
use super::counters::PlanarBooleanOverlapRegionLedgerAssemblyCounters;
use super::denial::PlanarBooleanOverlapRegionLedgerAssemblyDenial;
use super::input::PlanarBooleanOverlapRegionLedgerAssemblyInput;
use super::rows::{PlanarBooleanOverlapRegionDecisionLogRow, PlanarBooleanOverlapRegionLedgerRow};
use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionIdentityLineageBundle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionDecisionLog {
    decision_log_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanOverlapRegionDecisionLogRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionLedger {
    ledger_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanOverlapRegionLedgerRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionLedgerReceipt {
    receipt_identity: String,
    request_identity: String,
    decision_log_identity: String,
    ledger_identity: String,
    overlap_region_identity_map_identity: String,
    persistent_name_propagation_map_identity: String,
    subshape_signature_map_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionLedgerAssemblyBundle {
    bundle_identity: String,
    decision_log: PlanarBooleanOverlapRegionDecisionLog,
    ledger: PlanarBooleanOverlapRegionLedger,
    receipt: PlanarBooleanOverlapRegionLedgerReceipt,
    counters: PlanarBooleanOverlapRegionLedgerAssemblyCounters,
}

impl PlanarBooleanOverlapRegionDecisionLog {
    pub(crate) fn new(
        decision_log_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanOverlapRegionDecisionLogRow>,
    ) -> Self {
        Self {
            decision_log_identity,
            request_identity,
            rows,
        }
    }

    pub fn decision_log_identity(&self) -> &str {
        &self.decision_log_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanOverlapRegionDecisionLogRow] {
        &self.rows
    }
}

impl PlanarBooleanOverlapRegionLedger {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        ledger_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanOverlapRegionLedgerRow>,
    ) -> Self {
        Self {
            ledger_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
        }
    }

    pub fn ledger_identity(&self) -> &str {
        &self.ledger_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn arrangement_graph_identity(&self) -> &str {
        &self.arrangement_graph_identity
    }

    pub fn cell_set_identity(&self) -> &str {
        &self.cell_set_identity
    }

    pub fn ordering_basis_identity(&self) -> &str {
        &self.ordering_basis_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanOverlapRegionLedgerRow] {
        &self.rows
    }
}

impl PlanarBooleanOverlapRegionLedgerReceipt {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        receipt_identity: String,
        request_identity: String,
        decision_log_identity: String,
        ledger_identity: String,
        overlap_region_identity_map_identity: String,
        persistent_name_propagation_map_identity: String,
        subshape_signature_map_identity: String,
    ) -> Self {
        Self {
            receipt_identity,
            request_identity,
            decision_log_identity,
            ledger_identity,
            overlap_region_identity_map_identity,
            persistent_name_propagation_map_identity,
            subshape_signature_map_identity,
        }
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn decision_log_identity(&self) -> &str {
        &self.decision_log_identity
    }

    pub fn ledger_identity(&self) -> &str {
        &self.ledger_identity
    }

    pub fn overlap_region_identity_map_identity(&self) -> &str {
        &self.overlap_region_identity_map_identity
    }

    pub fn persistent_name_propagation_map_identity(&self) -> &str {
        &self.persistent_name_propagation_map_identity
    }

    pub fn subshape_signature_map_identity(&self) -> &str {
        &self.subshape_signature_map_identity
    }
}

impl PlanarBooleanOverlapRegionLedgerAssemblyBundle {
    pub fn from_identity_lineage(
        identity_lineage: &PlanarBooleanOverlapRegionIdentityLineageBundle,
    ) -> Result<Self, PlanarBooleanOverlapRegionLedgerAssemblyDenial> {
        Self::admit(
            PlanarBooleanOverlapRegionLedgerAssemblyInput::from_identity_lineage(identity_lineage),
        )
    }

    pub fn admit(
        input: PlanarBooleanOverlapRegionLedgerAssemblyInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapRegionLedgerAssemblyDenial> {
        assemble_ledger_bundle(input)
    }

    pub(crate) fn new(
        bundle_identity: String,
        decision_log: PlanarBooleanOverlapRegionDecisionLog,
        ledger: PlanarBooleanOverlapRegionLedger,
        receipt: PlanarBooleanOverlapRegionLedgerReceipt,
        counters: PlanarBooleanOverlapRegionLedgerAssemblyCounters,
    ) -> Self {
        Self {
            bundle_identity,
            decision_log,
            ledger,
            receipt,
            counters,
        }
    }

    pub fn decision_log(&self) -> &PlanarBooleanOverlapRegionDecisionLog {
        &self.decision_log
    }

    pub fn ledger(&self) -> &PlanarBooleanOverlapRegionLedger {
        &self.ledger
    }

    pub fn receipt(&self) -> &PlanarBooleanOverlapRegionLedgerReceipt {
        &self.receipt
    }

    pub fn counters(&self) -> PlanarBooleanOverlapRegionLedgerAssemblyCounters {
        self.counters
    }
}
