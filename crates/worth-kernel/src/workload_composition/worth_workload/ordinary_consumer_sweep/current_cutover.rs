use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::current_cutover_proof::current_worth_workload_ordinary_consumer_batch_execution_receipt;
use super::current_route_witness::{
    current_completed_split_batch_execution_cluster_witness,
    current_lookup_consumed_batch_execution_cluster_witness,
    current_replay_undo_boundary_batch_execution_cluster_witness,
    WorthWorkloadOrdinaryConsumerCurrentRouteWitness, WorthWorkloadOrdinaryConsumerRouteKind,
};
use crate::workload_composition::{
    current_conflict_batch_admission_inventory, BatchAdmissionExecutionReceipt,
    ConflictBatchAdmissionCertificationPosture, ConflictBatchAdmissionDisposition,
    ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionReplacementPhase,
    ConflictBatchAdmissionSurfaceIdentity,
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

pub fn current_worth_workload_ordinary_consumer_cutover(
) -> Result<WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverError> {
    let inventory = current_conflict_batch_admission_inventory().map_err(|error| {
        WorthWorkloadOrdinaryConsumerCutoverError::new(
            WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingInventory,
            format!("phase 13 ordinary-consumer cutover inventory did not load: {error:?}"),
        )
    })?;
    ordinary_consumer_cutover_from_inventory(&inventory)
}

pub(super) fn ordinary_consumer_cutover_from_inventory(
    inventory: &crate::workload_composition::ConflictBatchAdmissionInventory,
) -> Result<WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverError> {
    let lowered_rows = inventory
        .rows()
        .iter()
        .filter(|row| {
            row.replacement_phase()
                == ConflictBatchAdmissionReplacementPhase::PhaseElevenConsumerSweep
        })
        .cloned()
        .map(PendingWorthWorkloadOrdinaryConsumerCutoverRow::from_phase_eleven_inventory_row)
        .collect::<Result<Vec<_>, _>>()?;
    if lowered_rows.is_empty() {
        return Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
            WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingInventory,
            "phase 13 ordinary-consumer cutover requires phase-11 consumer sweep rows",
        ));
    }
    let route_witnesses = lowered_rows
        .iter()
        .filter_map(PendingWorthWorkloadOrdinaryConsumerCutoverRow::route_witness)
        .collect::<Vec<_>>();
    let batch_execution_receipt =
        current_worth_workload_ordinary_consumer_batch_execution_receipt(&route_witnesses)?;
    let rows = lowered_rows
        .into_iter()
        .map(|row| row.bind_receipt(&batch_execution_receipt))
        .collect();
    Ok(WorthWorkloadOrdinaryConsumerCutover::new(
        batch_execution_receipt,
        rows,
    ))
}

impl WorthWorkloadOrdinaryConsumerCutover {
    fn new(
        batch_execution_receipt: BatchAdmissionExecutionReceipt,
        mut rows: Vec<WorthWorkloadOrdinaryConsumerCutoverRow>,
    ) -> Self {
        rows.sort_by(|left, right| left.surface_name.cmp(&right.surface_name));
        let _cutover_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &rows
                .iter()
                .map(|row| {
                    format!(
                        "{}:{}:{}:{}:{}",
                        row.surface_name,
                        row.owner,
                        row.blocker,
                        row.removal_trigger,
                        row.posture.as_str()
                    )
                })
                .chain(rows.iter().filter_map(|row| {
                    row.selected_plan_witness.as_ref().map(|witness| {
                        format!(
                            "selected-plan-witness:{}:{}:{}:{}:{}:{}:{}:{}:{}",
                            row.surface_name,
                            witness.route_kind().as_str(),
                            witness.route_lineage_digest(),
                            witness.route_authority_digest(),
                            witness
                                .replay_undo_boundary_proof_digest()
                                .unwrap_or("not-applicable"),
                            witness
                                .transaction_packet_identity()
                                .unwrap_or("not-applicable"),
                            witness.replay_scope_identity().unwrap_or("not-applicable"),
                            witness.undo_scope_identity().unwrap_or("not-applicable"),
                            witness.batch_execution_receipt_digest()
                        )
                    })
                }))
                .chain(std::iter::once(format!(
                    "batch-execution:{}",
                    batch_execution_receipt.execution_receipt_digest()
                )))
                .chain(std::iter::once(
                    "worth-kernel:ordinary-consumer-cutover:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        Self {
            batch_execution_receipt,
            rows,
        }
    }

    pub fn batch_execution_receipt(&self) -> &BatchAdmissionExecutionReceipt {
        &self.batch_execution_receipt
    }

    pub fn rows(&self) -> &[WorthWorkloadOrdinaryConsumerCutoverRow] {
        &self.rows
    }

    pub(crate) fn replay_undo_boundary_proof_digests(&self) -> Vec<String> {
        sorted_unique_selected_plan_values(self, |witness| {
            witness.replay_undo_boundary_proof_digest()
        })
    }

    pub(crate) fn transaction_packet_identities(&self) -> Vec<String> {
        sorted_unique_selected_plan_values(self, |witness| witness.transaction_packet_identity())
    }

    pub(crate) fn replay_scope_identities(&self) -> Vec<String> {
        sorted_unique_selected_plan_values(self, |witness| witness.replay_scope_identity())
    }

    pub(crate) fn undo_scope_identities(&self) -> Vec<String> {
        sorted_unique_selected_plan_values(self, |witness| witness.undo_scope_identity())
    }

    pub(crate) fn replay_undo_selected_plan_witness_count(&self) -> usize {
        self.rows
            .iter()
            .filter_map(WorthWorkloadOrdinaryConsumerCutoverRow::selected_plan_witness)
            .filter(|witness| witness.replay_undo_boundary_proof_digest().is_some())
            .count()
    }
}

impl WorthWorkloadOrdinaryConsumerCutoverError {
    pub(super) fn new(
        kind: WorthWorkloadOrdinaryConsumerCutoverErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }
}

impl WorthWorkloadOrdinaryConsumerCutoverRow {
    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn posture(&self) -> WorthWorkloadOrdinaryConsumerCutoverPosture {
        self.posture
    }

    pub(crate) fn selected_plan_witness(
        &self,
    ) -> Option<&WorthWorkloadOrdinaryConsumerSelectedPlanWitness> {
        self.selected_plan_witness.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn test_posture_from_phase_eleven_inventory_row(
        row: ConflictBatchAdmissionInventoryRow,
    ) -> WorthWorkloadOrdinaryConsumerCutoverPosture {
        PendingWorthWorkloadOrdinaryConsumerCutoverRow::from_phase_eleven_inventory_row(row)
            .expect("test row should lower")
            .posture
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

impl WorthWorkloadOrdinaryConsumerSelectedPlanWitness {
    fn new(
        route_witness: &WorthWorkloadOrdinaryConsumerCurrentRouteWitness,
        batch_execution_receipt_digest: &str,
    ) -> Self {
        Self {
            route_kind: route_witness.route_kind(),
            route_lineage_digest: route_witness.route_lineage_digest().to_string(),
            route_authority_digest: route_witness.route_authority_digest().to_string(),
            replay_undo_boundary_proof_digest: route_witness
                .replay_undo_boundary_proof_digest()
                .map(str::to_string),
            transaction_packet_identity: route_witness
                .transaction_packet_identity()
                .map(str::to_string),
            replay_scope_identity: route_witness.replay_scope_identity().map(str::to_string),
            undo_scope_identity: route_witness.undo_scope_identity().map(str::to_string),
            batch_execution_receipt_digest: batch_execution_receipt_digest.to_string(),
        }
    }

    pub const fn route_kind(&self) -> WorthWorkloadOrdinaryConsumerRouteKind {
        self.route_kind
    }

    pub fn batch_execution_receipt_digest(&self) -> &str {
        &self.batch_execution_receipt_digest
    }

    pub fn route_lineage_digest(&self) -> &str {
        &self.route_lineage_digest
    }

    pub fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }

    pub fn replay_undo_boundary_proof_digest(&self) -> Option<&str> {
        self.replay_undo_boundary_proof_digest.as_deref()
    }

    pub fn transaction_packet_identity(&self) -> Option<&str> {
        self.transaction_packet_identity.as_deref()
    }

    pub fn replay_scope_identity(&self) -> Option<&str> {
        self.replay_scope_identity.as_deref()
    }

    pub fn undo_scope_identity(&self) -> Option<&str> {
        self.undo_scope_identity.as_deref()
    }
}

#[cfg(test)]
mod test_support;
#[cfg(test)]
pub(super) use test_support::{
    ordinary_consumer_cutover_from_inventory_for_tests,
    ordinary_consumer_cutover_from_inventory_with_test_replay_undo_identity_override,
};

#[derive(Clone, Debug)]
struct PendingWorthWorkloadOrdinaryConsumerCutoverRow {
    surface_name: String,
    owner: String,
    blocker: String,
    removal_trigger: String,
    posture: WorthWorkloadOrdinaryConsumerCutoverPosture,
    route_witness: Option<WorthWorkloadOrdinaryConsumerCurrentRouteWitness>,
}

impl PendingWorthWorkloadOrdinaryConsumerCutoverRow {
    fn from_phase_eleven_inventory_row(
        row: ConflictBatchAdmissionInventoryRow,
    ) -> Result<Self, WorthWorkloadOrdinaryConsumerCutoverError> {
        let (posture, route_witness) = match row.surface_identity() {
            ConflictBatchAdmissionSurfaceIdentity::WorthWorkloadAdmitLookupConsumedWorkload
                if row.disposition() == ConflictBatchAdmissionDisposition::Migrate
                    && row.certification_posture()
                        == ConflictBatchAdmissionCertificationPosture::OrdinaryProductionReachable =>
            {
                (
                    WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer,
                    Some(current_lookup_consumed_batch_execution_cluster_witness()?),
                )
            }
            ConflictBatchAdmissionSurfaceIdentity::CompletedBooleanSplitHandoffAdmitDownstreamSplitConsumption
                if row.disposition() == ConflictBatchAdmissionDisposition::Migrate
                    && row.certification_posture()
                        == ConflictBatchAdmissionCertificationPosture::OrdinaryProductionReachable =>
            {
                (
                    WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer,
                    Some(current_completed_split_batch_execution_cluster_witness()?),
                )
            }
            ConflictBatchAdmissionSurfaceIdentity::BooleanSplitReplayUndoBoundaryAdmission
                if row.disposition() == ConflictBatchAdmissionDisposition::Migrate
                    && row.certification_posture()
                        == ConflictBatchAdmissionCertificationPosture::OrdinaryProductionReachable =>
            {
                (
                    WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer,
                    Some(current_replay_undo_boundary_batch_execution_cluster_witness()?),
                )
            }
            ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanLoopRuntimeRegistrationProof
                if row.disposition() == ConflictBatchAdmissionDisposition::Cap =>
            {
                (
                    WorthWorkloadOrdinaryConsumerCutoverPosture::QueryProofAccompanimentOnly,
                    None,
                )
            }
            ConflictBatchAdmissionSurfaceIdentity::BooleanChainIntegrationHandoff
                if row.disposition() == ConflictBatchAdmissionDisposition::Cap =>
            {
                (
                    WorthWorkloadOrdinaryConsumerCutoverPosture::ReplayUndoCloseoutOnly,
                    None,
                )
            }
            _ => (
                WorthWorkloadOrdinaryConsumerCutoverPosture::CoveredOrdinaryConsumerDependency,
                None,
            ),
        };
        Ok(Self {
            surface_name: row.surface_name().to_string(),
            owner: owner_name(row.owner()).to_string(),
            blocker: row.blocker().to_string(),
            removal_trigger: row.removal_trigger().to_string(),
            posture,
            route_witness,
        })
    }

    fn route_witness(&self) -> Option<WorthWorkloadOrdinaryConsumerCurrentRouteWitness> {
        self.route_witness.clone()
    }

    fn bind_receipt(
        self,
        batch_execution_receipt: &BatchAdmissionExecutionReceipt,
    ) -> WorthWorkloadOrdinaryConsumerCutoverRow {
        WorthWorkloadOrdinaryConsumerCutoverRow {
            surface_name: self.surface_name,
            owner: self.owner,
            blocker: self.blocker,
            removal_trigger: self.removal_trigger,
            posture: self.posture,
            selected_plan_witness: self.route_witness.as_ref().map(|route_witness| {
                WorthWorkloadOrdinaryConsumerSelectedPlanWitness::new(
                    route_witness,
                    batch_execution_receipt.execution_receipt_digest(),
                )
            }),
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

fn sorted_unique_selected_plan_values(
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
    select: impl Fn(&WorthWorkloadOrdinaryConsumerSelectedPlanWitness) -> Option<&str>,
) -> Vec<String> {
    let mut values = cutover
        .rows()
        .iter()
        .filter_map(WorthWorkloadOrdinaryConsumerCutoverRow::selected_plan_witness)
        .filter_map(select)
        .map(str::to_string)
        .collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}
