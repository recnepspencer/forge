use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedgerReceipt;
use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionLedgerReceipt;

use super::{
    CompletedBooleanLoopReconstructionHandoff, CompletedBooleanSplitHandoff,
    PlanarBooleanLoopRuntimeRegistrationProof, WorkloadCompositionError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BooleanChainIntegrationHandoff {
    handoff_identity: String,
    split_ledger_receipt: PlanarBooleanSplitEdgeChainLedgerReceipt,
    loop_ledger_receipt: PlanarBooleanLoopReconstructionLedgerReceipt,
    runtime_registration_proof: PlanarBooleanLoopRuntimeRegistrationProof,
    workload_stage_index_identity: String,
    counters: BooleanChainIntegrationCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BooleanChainIntegrationCounters {
    declared_split_chain_breadth: usize,
    declared_loop_ledger_breadth: usize,
    ledger_receipts_consumed: usize,
    query_graph_proofs_consumed: usize,
    stage_index_lookups: usize,
    residue_rows: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BooleanChainResidueRow {
    id: &'static str,
    owner: &'static str,
    cap: usize,
    removal_trigger: &'static str,
    boundary: &'static str,
}

const BOOLEAN_CHAIN_RESIDUE: &[BooleanChainResidueRow] = &[
    BooleanChainResidueRow {
        id: "boolean-chain-prep-product-snapshot",
        owner: "cross-crate boolean chain",
        cap: 1,
        removal_trigger: "Milestone 7.5 consumes BooleanChainIntegrationHandoff and no longer needs loop products as a diagnostic snapshot",
        boundary: "non-authority snapshot; cannot satisfy BooleanChainIntegrationHandoff",
    },
    BooleanChainResidueRow {
        id: "boolean-chain-runtime-registration-ceremony",
        owner: "worth-kernel/worth-topo",
        cap: 1,
        removal_trigger: "Query graph obligation execution proof replaces the local loop operator and validator registration matrix ceremony",
        boundary: "typed Query proof accompaniment; not a split or loop ledger identity",
    },
];

impl BooleanChainIntegrationHandoff {
    pub(crate) fn from_completed_handoffs(
        split_handoff: &CompletedBooleanSplitHandoff,
        loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
    ) -> Result<Self, WorkloadCompositionError> {
        let split_lookup = split_handoff.require_boolean_split_lookup()?;
        let loop_lookup = loop_handoff.require_boolean_loop_reconstruction_lookup()?;
        let loop_workload_split_lookup = loop_handoff
            .completed_workload()
            .require_boolean_split_lookup(split_handoff.split_ledger_receipt())?;
        require_runtime_proof_matches_loop_handoff(loop_handoff)?;

        let runtime_registration_proof = loop_handoff.runtime_registration_proof().clone();
        let workload_stage_index_identity =
            loop_handoff.workload_stage_index_identity().to_string();
        let counters = BooleanChainIntegrationCounters::from_completed_handoffs(
            split_handoff,
            loop_handoff,
            split_lookup.lookup_counters().indexed_lookup_count()
                + loop_lookup.lookup_counters().indexed_lookup_count()
                + loop_workload_split_lookup
                    .lookup_counters()
                    .indexed_lookup_count(),
            BOOLEAN_CHAIN_RESIDUE.len(),
        );
        let handoff_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "boolean-chain-integration-handoff".to_string(),
                format!(
                    "split-receipt:{}",
                    split_handoff.split_ledger_receipt().receipt_identity()
                ),
                format!(
                    "split-ledger:{}",
                    split_handoff.split_ledger_receipt().ledger_identity()
                ),
                format!(
                    "loop-receipt:{}",
                    loop_handoff.loop_ledger_receipt().receipt_identity()
                ),
                format!(
                    "loop-ledger:{}",
                    loop_handoff.loop_ledger_receipt().ledger_identity()
                ),
                format!(
                    "query-proof:{}",
                    runtime_registration_proof.proof_identity()
                ),
                format!("stage-index:{workload_stage_index_identity}"),
            ],
        );

        Ok(Self {
            handoff_identity,
            split_ledger_receipt: split_handoff.split_ledger_receipt().clone(),
            loop_ledger_receipt: loop_handoff.loop_ledger_receipt().clone(),
            runtime_registration_proof,
            workload_stage_index_identity,
            counters,
        })
    }

    pub fn handoff_identity(&self) -> &str {
        &self.handoff_identity
    }

    pub fn split_ledger_receipt(&self) -> &PlanarBooleanSplitEdgeChainLedgerReceipt {
        &self.split_ledger_receipt
    }

    pub fn loop_ledger_receipt(&self) -> &PlanarBooleanLoopReconstructionLedgerReceipt {
        &self.loop_ledger_receipt
    }

    pub fn query_graph_proof_identity(&self) -> &str {
        self.runtime_registration_proof.proof_identity()
    }

    pub fn runtime_registration_proof(&self) -> &PlanarBooleanLoopRuntimeRegistrationProof {
        &self.runtime_registration_proof
    }

    pub fn workload_stage_index_identity(&self) -> &str {
        &self.workload_stage_index_identity
    }

    pub fn counters(&self) -> BooleanChainIntegrationCounters {
        self.counters
    }

    pub fn residue_manifest(&self) -> &'static [BooleanChainResidueRow] {
        BOOLEAN_CHAIN_RESIDUE
    }
}

impl BooleanChainIntegrationCounters {
    fn from_completed_handoffs(
        split_handoff: &CompletedBooleanSplitHandoff,
        loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
        stage_index_lookups: usize,
        residue_rows: usize,
    ) -> Self {
        Self {
            declared_split_chain_breadth: split_handoff
                .split_ledger_receipt()
                .counters()
                .ledger_chains_emitted(),
            declared_loop_ledger_breadth: loop_handoff
                .loop_ledger_receipt()
                .counters()
                .ledger_rows_emitted(),
            ledger_receipts_consumed: 2,
            query_graph_proofs_consumed: 1,
            stage_index_lookups,
            residue_rows,
        }
    }

    pub fn declared_split_chain_breadth(self) -> usize {
        self.declared_split_chain_breadth
    }

    pub fn declared_loop_ledger_breadth(self) -> usize {
        self.declared_loop_ledger_breadth
    }

    pub fn ledger_receipts_consumed(self) -> usize {
        self.ledger_receipts_consumed
    }

    pub fn query_graph_proofs_consumed(self) -> usize {
        self.query_graph_proofs_consumed
    }

    pub fn stage_index_lookups(self) -> usize {
        self.stage_index_lookups
    }

    pub fn residue_rows(self) -> usize {
        self.residue_rows
    }
}

impl BooleanChainResidueRow {
    pub fn id(self) -> &'static str {
        self.id
    }

    pub fn owner(self) -> &'static str {
        self.owner
    }

    pub fn cap(self) -> usize {
        self.cap
    }

    pub fn removal_trigger(self) -> &'static str {
        self.removal_trigger
    }

    pub fn boundary(self) -> &'static str {
        self.boundary
    }
}

impl CompletedBooleanLoopReconstructionHandoff {
    pub fn complete_boolean_chain_integration_handoff(
        &self,
        split_handoff: &CompletedBooleanSplitHandoff,
    ) -> Result<BooleanChainIntegrationHandoff, WorkloadCompositionError> {
        BooleanChainIntegrationHandoff::from_completed_handoffs(split_handoff, self)
    }
}

fn require_runtime_proof_matches_loop_handoff(
    loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
) -> Result<(), WorkloadCompositionError> {
    let proof = loop_handoff.runtime_registration_proof();
    let receipt = loop_handoff.loop_ledger_receipt();
    if proof.loop_receipt_identity() != receipt.receipt_identity()
        || proof.loop_ledger_identity() != receipt.ledger_identity()
        || proof.downstream_consumption_identity() != receipt.downstream_consumption_identity()
        || proof.stage_index_identity() != loop_handoff.workload_stage_index_identity()
    {
        return Err(WorkloadCompositionError::BooleanChainHandoff(
            "boolean chain integration requires Query runtime proof bound to the exact loop ledger receipt and workload stage index".to_string(),
        ));
    }
    Ok(())
}
