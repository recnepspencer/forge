use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::replay_undo_semantic_graph::{
    current_boolean_event_ledger_spatial_boundary, current_boolean_split_spatial_boundary,
    current_projection_receipt_spatial_boundary, CurrentReplayUndoSpatialBoundary,
};

use crate::replay_undo_consumer_cutover::current_replay_undo_forbidden_surface_denial_ledger;
use crate::replay_undo_inventory::{
    current_replay_undo_inventory_report, current_replay_undo_source_firewall_report,
    ReplayUndoInventoryCategory, ReplayUndoInventoryDisposition, ReplayUndoInventorySourceIdentity,
    ReplayUndoInventorySourceKind,
};

use super::current_cutover::{
    WorthWorkloadOrdinaryConsumerCutoverError, WorthWorkloadOrdinaryConsumerCutoverErrorKind,
};
use super::current_replay_undo_boundary_proof::current_replay_undo_boundary_proof;
use crate::workload_composition::planner_owned_routing::current_replay_undo_transaction_route_packet;

#[derive(Clone, Debug)]
pub(crate) enum WorthWorkloadCurrentOrdinaryRouteAuthority {
    LookupConsumed(WorthWorkloadCurrentLookupConsumedRouteAuthority),
    CompletedSplit(WorthWorkloadCurrentCompletedSplitRouteAuthority),
    ReplayUndoBoundary(WorthWorkloadCurrentReplayUndoBoundaryRouteAuthority),
}

#[derive(Clone)]
pub(crate) struct WorthWorkloadCurrentLookupConsumedRouteAuthority {
    left_boundary: CurrentReplayUndoSpatialBoundary,
    right_boundary: CurrentReplayUndoSpatialBoundary,
    route_authority_digest: String,
}

#[derive(Clone, Debug)]
pub(crate) struct WorthWorkloadCurrentCompletedSplitRouteAuthority {
    split_boundary: CurrentReplayUndoSpatialBoundary,
    lookup_route_authority: WorthWorkloadCurrentLookupConsumedRouteAuthority,
    route_authority_digest: String,
}

#[derive(Clone, Debug)]
pub(crate) struct WorthWorkloadCurrentReplayUndoBoundaryRouteAuthority {
    lookup_route_authority: WorthWorkloadCurrentLookupConsumedRouteAuthority,
    route_authority_digest: String,
    route_packet_identity: String,
    route_family: String,
    boundary_proof_digest: String,
    transaction_packet_identity: String,
    replay_scope_identity: String,
    undo_scope_identity: String,
    source_identity: ReplayUndoInventorySourceIdentity,
    source_path: String,
    inventory_row_count: usize,
    forbidden_surface_denial_count: usize,
}

pub(crate) fn current_lookup_consumed_route_authority() -> Result<
    WorthWorkloadCurrentLookupConsumedRouteAuthority,
    WorthWorkloadOrdinaryConsumerCutoverError,
> {
    WorthWorkloadCurrentLookupConsumedRouteAuthority::current()
}

pub(crate) fn current_completed_split_route_authority() -> Result<
    WorthWorkloadCurrentCompletedSplitRouteAuthority,
    WorthWorkloadOrdinaryConsumerCutoverError,
> {
    WorthWorkloadCurrentCompletedSplitRouteAuthority::current()
}

pub(crate) fn current_replay_undo_boundary_route_authority() -> Result<
    WorthWorkloadCurrentReplayUndoBoundaryRouteAuthority,
    WorthWorkloadOrdinaryConsumerCutoverError,
> {
    WorthWorkloadCurrentReplayUndoBoundaryRouteAuthority::current()
}

impl WorthWorkloadCurrentOrdinaryRouteAuthority {
    pub(crate) fn lookup_route_authority(
        &self,
    ) -> &WorthWorkloadCurrentLookupConsumedRouteAuthority {
        match self {
            Self::LookupConsumed(authority) => authority,
            Self::CompletedSplit(authority) => authority.lookup_route_authority(),
            Self::ReplayUndoBoundary(authority) => authority.lookup_route_authority(),
        }
    }

    pub(crate) fn route_authority_digest(&self) -> &str {
        match self {
            Self::LookupConsumed(authority) => authority.route_authority_digest(),
            Self::CompletedSplit(authority) => authority.route_authority_digest(),
            Self::ReplayUndoBoundary(authority) => authority.route_authority_digest(),
        }
    }
}

impl WorthWorkloadCurrentLookupConsumedRouteAuthority {
    fn current() -> Result<Self, WorthWorkloadOrdinaryConsumerCutoverError> {
        let left_boundary =
            current_boolean_event_ledger_spatial_boundary().map_err(current_route_error)?;
        let right_boundary =
            current_projection_receipt_spatial_boundary().map_err(current_route_error)?;
        let route_authority_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-kernel:ordinary-consumer-lookup-route-authority:v2".to_string(),
                format!(
                    "left-stage:{}",
                    left_boundary.workload_handoff().stage_receipt_identity()
                ),
                format!(
                    "left-lookup:{}",
                    left_boundary
                        .workload_handoff()
                        .lookup_execution_receipt_digest()
                ),
                format!(
                    "left-authority:{}",
                    left_boundary.authority().stage_index_identity()
                ),
                format!(
                    "right-stage:{}",
                    right_boundary.workload_handoff().stage_receipt_identity()
                ),
                format!(
                    "right-lookup:{}",
                    right_boundary
                        .workload_handoff()
                        .lookup_execution_receipt_digest()
                ),
                format!(
                    "right-authority:{}",
                    right_boundary.authority().stage_index_identity()
                ),
            ],
        );
        Ok(Self {
            left_boundary,
            right_boundary,
            route_authority_digest,
        })
    }

    pub(crate) fn left_boundary(&self) -> &CurrentReplayUndoSpatialBoundary {
        &self.left_boundary
    }

    pub(crate) fn right_boundary(&self) -> &CurrentReplayUndoSpatialBoundary {
        &self.right_boundary
    }

    pub(crate) fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }
}

impl WorthWorkloadCurrentCompletedSplitRouteAuthority {
    fn current() -> Result<Self, WorthWorkloadOrdinaryConsumerCutoverError> {
        let lookup_route_authority = current_lookup_consumed_route_authority()?;
        let split_boundary =
            current_boolean_split_spatial_boundary().map_err(current_route_error)?;
        let retained_replay_identity = split_boundary
            .retained_replay_receipt()
            .map(|receipt| receipt.identity().receipt_identity().to_string())
            .unwrap_or("not-required".to_string());
        let route_authority_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-kernel:ordinary-consumer-completed-split-route-authority:v1".to_string(),
                format!(
                    "lookup-authority:{}",
                    lookup_route_authority.route_authority_digest()
                ),
                format!(
                    "split-stage:{}",
                    split_boundary.workload_handoff().stage_receipt_identity()
                ),
                format!(
                    "split-lookup:{}",
                    split_boundary
                        .workload_handoff()
                        .lookup_execution_receipt_digest()
                ),
                format!(
                    "split-authority:{}",
                    split_boundary.authority().stage_index_identity()
                ),
                format!("retained-replay:{retained_replay_identity}"),
            ],
        );
        Ok(Self {
            split_boundary,
            lookup_route_authority,
            route_authority_digest,
        })
    }

    pub(crate) fn split_boundary(&self) -> &CurrentReplayUndoSpatialBoundary {
        &self.split_boundary
    }

    pub(crate) fn lookup_route_authority(
        &self,
    ) -> &WorthWorkloadCurrentLookupConsumedRouteAuthority {
        &self.lookup_route_authority
    }

    pub(crate) fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }
}

impl WorthWorkloadCurrentReplayUndoBoundaryRouteAuthority {
    fn current() -> Result<Self, WorthWorkloadOrdinaryConsumerCutoverError> {
        let completed_split_route_authority = current_completed_split_route_authority()?;
        let replay_undo_route_packet =
            current_replay_undo_transaction_route_packet().map_err(current_route_error)?;
        let replay_undo_boundary_proof =
            current_replay_undo_boundary_proof(completed_split_route_authority.split_boundary())?;
        let inventory = current_replay_undo_inventory_report().map_err(current_route_error)?;
        inventory
            .require_full_declared_coverage()
            .map_err(current_route_error)?;
        let source_row = inventory
            .require_source(
                ReplayUndoInventorySourceIdentity::KernelBooleanSplitReplayUndoBoundaryAdmission,
            )
            .map_err(current_route_error)?;
        require_replay_undo_boundary_row(source_row)?;
        let source_firewall = current_replay_undo_source_firewall_report();
        if !source_firewall.require_no_undeclared_receipt_consumers() {
            return Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
                WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
                "phase 13 replay/undo route authority found an undeclared replay/undo receipt consumer",
            ));
        }
        let forbidden_surface_denials = current_replay_undo_forbidden_surface_denial_ledger();
        forbidden_surface_denials
            .require_phase_eleven_denials()
            .map_err(current_route_error)?;
        let route_authority_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-kernel:ordinary-consumer-replay-undo-route-authority:v3".to_string(),
                format!(
                    "completed-split-authority:{}",
                    completed_split_route_authority.route_authority_digest()
                ),
                format!(
                    "planner-route-packet:{}",
                    replay_undo_route_packet.route_packet_identity()
                ),
                format!(
                    "planner-route-family:{}",
                    replay_undo_route_packet.family().as_str()
                ),
                format!(
                    "boundary-proof:{}",
                    replay_undo_boundary_proof.boundary_proof_digest()
                ),
                format!(
                    "packet:{}",
                    replay_undo_boundary_proof.transaction_packet_identity()
                ),
                format!(
                    "replay-scope:{}",
                    replay_undo_boundary_proof.replay_scope_identity()
                ),
                format!(
                    "undo-scope:{}",
                    replay_undo_boundary_proof.undo_scope_identity()
                ),
                format!("source:{}", source_row.source_identity().as_str()),
                format!("source-path:{}", source_row.source_path()),
                format!("source-kind:{:?}", source_row.source_kind()),
                format!("source-owner:{:?}", source_row.owner()),
                format!("source-category:{:?}", source_row.category()),
                format!("source-disposition:{:?}", source_row.disposition()),
                format!("inventory-rows:{}", inventory.rows().len()),
                format!(
                    "forbidden-surface-denials:{}",
                    forbidden_surface_denials.row_count()
                ),
                "admission-surface:public-function".to_string(),
                "source-firewall:clean".to_string(),
            ],
        );
        Ok(Self {
            lookup_route_authority: completed_split_route_authority
                .lookup_route_authority()
                .clone(),
            route_authority_digest,
            route_packet_identity: replay_undo_route_packet.route_packet_identity().to_string(),
            route_family: replay_undo_route_packet.family().as_str().to_string(),
            boundary_proof_digest: replay_undo_boundary_proof
                .boundary_proof_digest()
                .to_string(),
            transaction_packet_identity: replay_undo_boundary_proof
                .transaction_packet_identity()
                .to_string(),
            replay_scope_identity: replay_undo_boundary_proof
                .replay_scope_identity()
                .to_string(),
            undo_scope_identity: replay_undo_boundary_proof.undo_scope_identity().to_string(),
            source_identity: source_row.source_identity(),
            source_path: source_row.source_path().to_string(),
            inventory_row_count: inventory.rows().len(),
            forbidden_surface_denial_count: forbidden_surface_denials.row_count(),
        })
    }

    pub(crate) fn lookup_route_authority(
        &self,
    ) -> &WorthWorkloadCurrentLookupConsumedRouteAuthority {
        &self.lookup_route_authority
    }

    pub(crate) fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }

    pub(crate) fn boundary_proof_digest(&self) -> &str {
        &self.boundary_proof_digest
    }

    pub(crate) fn route_packet_identity(&self) -> &str {
        &self.route_packet_identity
    }

    pub(crate) fn route_family(&self) -> &str {
        &self.route_family
    }

    pub(crate) fn transaction_packet_identity(&self) -> &str {
        &self.transaction_packet_identity
    }

    pub(crate) fn replay_scope_identity(&self) -> &str {
        &self.replay_scope_identity
    }

    pub(crate) fn undo_scope_identity(&self) -> &str {
        &self.undo_scope_identity
    }

    pub(crate) const fn source_identity(&self) -> ReplayUndoInventorySourceIdentity {
        self.source_identity
    }

    pub(crate) fn source_path(&self) -> &str {
        &self.source_path
    }

    pub(crate) const fn inventory_row_count(&self) -> usize {
        self.inventory_row_count
    }

    pub(crate) const fn forbidden_surface_denial_count(&self) -> usize {
        self.forbidden_surface_denial_count
    }
}

impl PartialEq for WorthWorkloadCurrentLookupConsumedRouteAuthority {
    fn eq(&self, other: &Self) -> bool {
        self.route_authority_digest == other.route_authority_digest
    }
}

impl Eq for WorthWorkloadCurrentLookupConsumedRouteAuthority {}

impl std::fmt::Debug for WorthWorkloadCurrentLookupConsumedRouteAuthority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorthWorkloadCurrentLookupConsumedRouteAuthority")
            .field("route_authority_digest", &self.route_authority_digest)
            .finish()
    }
}

fn current_route_error<E: std::fmt::Debug>(error: E) -> WorthWorkloadOrdinaryConsumerCutoverError {
    WorthWorkloadOrdinaryConsumerCutoverError::new(
        WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
        format!("phase 13 current ordinary route authority did not assemble: {error:?}"),
    )
}

fn require_replay_undo_boundary_row(
    row: &crate::replay_undo_inventory::ReplayUndoInventoryReportRow,
) -> Result<(), WorthWorkloadOrdinaryConsumerCutoverError> {
    if row.category() != ReplayUndoInventoryCategory::UndoScope {
        return Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
            WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
            "phase 13 replay/undo route authority requires the kernel replay/undo admission row to stay in the undo-scope category",
        ));
    }
    if row.disposition() != ReplayUndoInventoryDisposition::Migrate {
        return Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
            WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
            "phase 13 replay/undo route authority requires the kernel replay/undo admission row to be migrated into the ordinary lane",
        ));
    }
    if row.source_kind() != ReplayUndoInventorySourceKind::PublicFunction {
        return Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
            WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
            "phase 13 replay/undo route authority requires the kernel replay/undo admission row to name the public admission function, not a wrapper surface",
        ));
    }
    if row.source_path()
        != "crates/worth-kernel/src/workload_composition/worth_workload/replay_undo_boundary/boolean_split_boundary_admission.rs"
    {
        return Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
            WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
            "phase 13 replay/undo route authority requires the kernel replay/undo admission row to stay anchored to the admitted boundary surface",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod test_support;
