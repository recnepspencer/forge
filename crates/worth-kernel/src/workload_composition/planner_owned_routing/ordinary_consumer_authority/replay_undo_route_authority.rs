use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::cutover::{
    WorthWorkloadOrdinaryConsumerCutoverError, WorthWorkloadOrdinaryConsumerCutoverErrorKind,
};
use super::lookup_route_authority::{
    current_completed_split_route_authority, WorthWorkloadCurrentLookupConsumedRouteAuthority,
};
use super::replay_undo_boundary_proof::lower_current_replay_undo_boundary_proof;
use crate::replay_undo_consumer_cutover::current_replay_undo_forbidden_surface_denial_ledger;
use crate::replay_undo_inventory::{
    current_replay_undo_inventory_report, current_replay_undo_source_firewall_report,
    ReplayUndoInventoryCategory, ReplayUndoInventoryDisposition, ReplayUndoInventorySourceIdentity,
    ReplayUndoInventorySourceKind,
};
use crate::workload_composition::performance_trace::trace_scope;
use crate::workload_composition::planner_owned_routing::current_replay_undo_transaction_route_packet;

#[derive(Clone, Debug)]
pub(crate) struct WorthWorkloadCurrentReplayUndoBoundaryRouteAuthority {
    lookup_route_authority: WorthWorkloadCurrentLookupConsumedRouteAuthority,
    route_authority_digest: String,
    boundary_proof_digest: String,
    transaction_packet_identity: String,
    replay_scope_identity: String,
    undo_scope_identity: String,
    #[cfg(test)]
    source_identity: ReplayUndoInventorySourceIdentity,
    #[cfg(test)]
    source_path: String,
    #[cfg(test)]
    inventory_row_count: usize,
    #[cfg(test)]
    forbidden_surface_denial_count: usize,
}

#[derive(Clone, Debug)]
pub(crate) enum WorthWorkloadCurrentOrdinaryRouteAuthority {
    LookupConsumed(WorthWorkloadCurrentLookupConsumedRouteAuthority),
    CompletedSplit(super::lookup_route_authority::WorthWorkloadCurrentCompletedSplitRouteAuthority),
    ReplayUndoBoundary(WorthWorkloadCurrentReplayUndoBoundaryRouteAuthority),
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

impl WorthWorkloadCurrentReplayUndoBoundaryRouteAuthority {
    fn current() -> Result<Self, WorthWorkloadOrdinaryConsumerCutoverError> {
        trace_scope("current_replay_undo_boundary_route_authority", || {
            let completed_split_route_authority = current_completed_split_route_authority()?;
            let replay_undo_route_packet =
                trace_scope("current_replay_undo_transaction_route_packet", || {
                    current_replay_undo_transaction_route_packet().map_err(current_route_error)
                })?;
            let replay_undo_boundary_proof = lower_current_replay_undo_boundary_proof(
                &replay_undo_route_packet,
                completed_split_route_authority.split_boundary(),
            )?;
            let inventory = trace_scope("current_replay_undo_inventory_report", || {
                current_replay_undo_inventory_report().map_err(current_route_error)
            })?;
            inventory
                .require_full_declared_coverage()
                .map_err(current_route_error)?;
            let source_row = inventory
                .require_source(
                    ReplayUndoInventorySourceIdentity::KernelBooleanSplitReplayUndoBoundaryAdmission,
                )
                .map_err(current_route_error)?;
            require_replay_undo_boundary_row(source_row)?;
            let source_firewall = trace_scope("current_replay_undo_source_firewall_report", || {
                current_replay_undo_source_firewall_report()
            });
            if !source_firewall.require_no_undeclared_receipt_consumers() {
                return Err(WorthWorkloadOrdinaryConsumerCutoverError::new(
                    WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
                    "phase 13 replay/undo route authority found an undeclared replay/undo receipt consumer",
                ));
            }
            let forbidden_surface_denials = trace_scope(
                "current_replay_undo_forbidden_surface_denial_ledger",
                current_replay_undo_forbidden_surface_denial_ledger,
            );
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
                #[cfg(test)]
                source_identity: source_row.source_identity(),
                #[cfg(test)]
                source_path: source_row.source_path().to_string(),
                #[cfg(test)]
                inventory_row_count: inventory.rows().len(),
                #[cfg(test)]
                forbidden_surface_denial_count: forbidden_surface_denials.row_count(),
            })
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

    pub(crate) fn transaction_packet_identity(&self) -> &str {
        &self.transaction_packet_identity
    }

    pub(crate) fn replay_scope_identity(&self) -> &str {
        &self.replay_scope_identity
    }

    pub(crate) fn undo_scope_identity(&self) -> &str {
        &self.undo_scope_identity
    }

    #[cfg(test)]
    pub(crate) const fn source_identity(&self) -> ReplayUndoInventorySourceIdentity {
        self.source_identity
    }

    #[cfg(test)]
    pub(crate) fn source_path(&self) -> &str {
        &self.source_path
    }

    #[cfg(test)]
    pub(crate) const fn inventory_row_count(&self) -> usize {
        self.inventory_row_count
    }

    #[cfg(test)]
    pub(crate) const fn forbidden_surface_denial_count(&self) -> usize {
        self.forbidden_surface_denial_count
    }

    #[cfg(test)]
    pub(crate) fn with_test_replay_undo_identity_override(
        mut self,
        boundary_proof_digest: &str,
        transaction_packet_identity: &str,
        replay_scope_identity: &str,
        undo_scope_identity: &str,
    ) -> Self {
        self.boundary_proof_digest = boundary_proof_digest.to_string();
        self.transaction_packet_identity = transaction_packet_identity.to_string();
        self.replay_scope_identity = replay_scope_identity.to_string();
        self.undo_scope_identity = undo_scope_identity.to_string();
        self
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
