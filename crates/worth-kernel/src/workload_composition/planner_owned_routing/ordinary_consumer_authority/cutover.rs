mod current;
mod inventory_lowering;
mod model;
mod row;
#[cfg(test)]
mod test_support;
mod witness;

use crate::workload_composition::{
    BatchAdmissionExecutionReceipt, ConflictBatchAdmissionInventoryRow,
};

use super::route_witness::{
    WorthWorkloadOrdinaryConsumerCurrentRouteWitness, WorthWorkloadOrdinaryConsumerRouteKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthWorkloadOrdinaryConsumerCutoverPosture {
    SelectedPlanDrivenOrdinaryConsumer,
    QueryProofAccompanimentOnly,
    ReplayUndoCloseoutOnly,
    CoveredOrdinaryConsumerDependency,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthWorkloadOrdinaryConsumerCutoverErrorKind {
    MissingInventory,
    MissingCurrentProofChain,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkloadOrdinaryConsumerCutoverError {
    kind: WorthWorkloadOrdinaryConsumerCutoverErrorKind,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkloadOrdinaryConsumerCutoverRow {
    surface_name: String,
    owner: String,
    blocker: String,
    removal_trigger: String,
    posture: WorthWorkloadOrdinaryConsumerCutoverPosture,
    selected_plan_witness: Option<WorthWorkloadOrdinaryConsumerSelectedPlanWitness>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkloadOrdinaryConsumerCutover {
    batch_execution_receipt: BatchAdmissionExecutionReceipt,
    rows: Vec<WorthWorkloadOrdinaryConsumerCutoverRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthWorkloadOrdinaryConsumerSelectedPlanWitness {
    route_kind: WorthWorkloadOrdinaryConsumerRouteKind,
    route_lineage_digest: String,
    route_authority_digest: String,
    replay_undo_boundary_proof_digest: Option<String>,
    transaction_packet_identity: Option<String>,
    replay_scope_identity: Option<String>,
    undo_scope_identity: Option<String>,
    batch_execution_receipt_digest: String,
}

#[derive(Clone, Debug)]
struct PendingWorthWorkloadOrdinaryConsumerCutoverRow {
    surface_name: String,
    owner: String,
    blocker: String,
    removal_trigger: String,
    posture: WorthWorkloadOrdinaryConsumerCutoverPosture,
    route_witness: Option<WorthWorkloadOrdinaryConsumerCurrentRouteWitness>,
}

pub use current::current_worth_workload_ordinary_consumer_cutover;
pub(crate) use inventory_lowering::ordinary_consumer_cutover_from_inventory;

impl WorthWorkloadOrdinaryConsumerCutoverError {
    pub(crate) fn new(
        kind: WorthWorkloadOrdinaryConsumerCutoverErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }
}

impl WorthWorkloadOrdinaryConsumerCutoverPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedPlanDrivenOrdinaryConsumer => "selected-plan-driven-ordinary-consumer",
            Self::QueryProofAccompanimentOnly => "query-proof-accompaniment-only",
            Self::ReplayUndoCloseoutOnly => "replay-undo-closeout-only",
            Self::CoveredOrdinaryConsumerDependency => "covered-ordinary-consumer-dependency",
        }
    }
}

fn owner_name(owner: crate::workload_composition::ConflictBatchAdmissionOwner) -> &'static str {
    match owner {
        crate::workload_composition::ConflictBatchAdmissionOwner::WorthKernel => "worth-kernel",
        crate::workload_composition::ConflictBatchAdmissionOwner::WorthTopo => "worth-topo",
        crate::workload_composition::ConflictBatchAdmissionOwner::WorthSpatial => "worth-spatial",
        crate::workload_composition::ConflictBatchAdmissionOwner::ForgeQuery => "forge-query",
    }
}

fn phase_eleven_consumer_sweep_rows(
    inventory: &crate::workload_composition::ConflictBatchAdmissionInventory,
) -> impl Iterator<Item = ConflictBatchAdmissionInventoryRow> + '_ {
    inventory.rows().iter().filter(|row| {
        row.replacement_phase()
            == crate::workload_composition::ConflictBatchAdmissionReplacementPhase::PhaseElevenConsumerSweep
    }).cloned()
}

#[cfg(test)]
pub(crate) use test_support::{
    ordinary_consumer_cutover_from_inventory_for_tests,
    ordinary_consumer_cutover_from_inventory_with_test_replay_undo_identity_override,
};
