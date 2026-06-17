use super::declaration::PlanarBooleanSplitEdgeChainLedgerDeclaration;
use super::denial::PlanarBooleanSplitEdgeChainLedgerDenial;
use super::input::PlanarBooleanSplitEdgeChainLedgerInput;
use super::ledger::PlanarBooleanSplitEdgeChainLedger;
use super::receipt::PlanarBooleanSplitEdgeChainLedgerReceipt;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet, PlanarBooleanOverlapEdgeChainSet,
    PlanarBooleanSplitChainValidationReceipt, PlanarBooleanSplitDecisionLogQueryResult,
    PlanarBooleanSplitEdgeFragmentSet, PlanarBooleanSplitPersistentNamingReceipt,
    PlanarBooleanSplitVertexIdentitySet,
};

pub struct PlanarBooleanSplitEdgeChainLedgerQueryDomain;

pub struct PlanarBooleanSplitEdgeChainLedgerQueryInput<'a> {
    split_request: &'a PlanarBooleanEdgeSplitRequest,
    endpoint_boundary_schedules: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_subdivision_schedules: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
    split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
    split_chain_validation: &'a PlanarBooleanSplitChainValidationReceipt,
    split_persistent_names: &'a PlanarBooleanSplitPersistentNamingReceipt,
    split_decision_log: &'a PlanarBooleanSplitDecisionLogQueryResult,
}

pub struct PlanarBooleanSplitEdgeChainLedgerLoweredPlan<'a> {
    declaration: PlanarBooleanSplitEdgeChainLedgerDeclaration,
    input: PlanarBooleanSplitEdgeChainLedgerQueryInput<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitEdgeChainLedgerQueryResult {
    ledger: PlanarBooleanSplitEdgeChainLedger,
    receipt: PlanarBooleanSplitEdgeChainLedgerReceipt,
}

impl<'a> PlanarBooleanSplitEdgeChainLedgerQueryInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        split_request: &'a PlanarBooleanEdgeSplitRequest,
        endpoint_boundary_schedules: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
        interval_subdivision_schedules: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
        split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
        split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
        split_chain_validation: &'a PlanarBooleanSplitChainValidationReceipt,
        split_persistent_names: &'a PlanarBooleanSplitPersistentNamingReceipt,
        split_decision_log: &'a PlanarBooleanSplitDecisionLogQueryResult,
    ) -> Self {
        Self {
            split_request,
            endpoint_boundary_schedules,
            interval_subdivision_schedules,
            split_vertices,
            split_fragments,
            overlap_chains,
            split_chain_validation,
            split_persistent_names,
            split_decision_log,
        }
    }
}

impl PlanarBooleanSplitEdgeChainLedgerQueryDomain {
    pub fn declare<'a>(
        input: PlanarBooleanSplitEdgeChainLedgerQueryInput<'a>,
    ) -> Result<
        PlanarBooleanSplitEdgeChainLedgerLoweredPlan<'a>,
        PlanarBooleanSplitEdgeChainLedgerDenial,
    > {
        let declaration = PlanarBooleanSplitEdgeChainLedgerDeclaration::from_query_products(
            input.split_request,
            input.split_chain_validation,
            input.split_persistent_names,
            input.split_decision_log,
        )?;
        Ok(PlanarBooleanSplitEdgeChainLedgerLoweredPlan { declaration, input })
    }
}

impl PlanarBooleanSplitEdgeChainLedgerLoweredPlan<'_> {
    pub fn declaration(&self) -> &PlanarBooleanSplitEdgeChainLedgerDeclaration {
        &self.declaration
    }

    pub fn execute(
        self,
    ) -> Result<PlanarBooleanSplitEdgeChainLedgerQueryResult, PlanarBooleanSplitEdgeChainLedgerDenial>
    {
        let input = PlanarBooleanSplitEdgeChainLedgerInput::from_query_products(
            self.declaration,
            self.input.split_request,
            self.input.endpoint_boundary_schedules,
            self.input.interval_subdivision_schedules,
            self.input.split_vertices,
            self.input.split_fragments,
            self.input.overlap_chains,
            self.input.split_chain_validation,
            self.input.split_persistent_names,
            self.input.split_decision_log,
        );
        let (ledger, receipt) = PlanarBooleanSplitEdgeChainLedger::assemble(input)?;
        Ok(PlanarBooleanSplitEdgeChainLedgerQueryResult { ledger, receipt })
    }
}

impl PlanarBooleanSplitEdgeChainLedgerQueryResult {
    pub fn ledger(&self) -> &PlanarBooleanSplitEdgeChainLedger {
        &self.ledger
    }
    pub fn receipt(&self) -> &PlanarBooleanSplitEdgeChainLedgerReceipt {
        &self.receipt
    }
    pub fn into_receipt(self) -> PlanarBooleanSplitEdgeChainLedgerReceipt {
        self.receipt
    }
    pub fn certifies_query_owned_split_edge_chain_ledger(&self) -> bool {
        self.receipt.certifies_split_edge_chain_ledger()
            && self.receipt.ledger_identity() == self.ledger.ledger_identity()
            && self.receipt.chain_identities().len() == self.ledger.chains().len()
    }
}
