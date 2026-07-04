use schema::facade::platform::authority::{
    replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily,
    touched_graph_parity_closeout::TouchedGraphParityFamilyKind,
};

use crate::workload_composition::touched_graph_parity_closeout::family_contributors::{
    KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError,
    KernelTouchedGraphParityQuerySurfaceKind,
};

use super::error::{
    ReplayUndoFamilyContributorCatalogError, ReplayUndoFamilyContributorCatalogErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoContributorRowKind {
    Replay,
    Undo,
}

impl ReplayUndoContributorRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Replay => "replay",
            Self::Undo => "undo",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoFamilyContributorCatalogRow {
    kind: ReplayUndoContributorRowKind,
    family_kind: TouchedGraphParityFamilyKind,
    current_packet_or_identity_source: &'static str,
    carried_scope_identity_source: &'static str,
    carried_witness_or_boundary_source: &'static str,
    ordinary_path_live_caller_surface: &'static str,
    ordinary_path_live_caller_path: &'static str,
    route_family: ReplayUndoPlannerRouteFamily,
    route_packet_identity: String,
    transaction_packet_identity: String,
    carried_scope_identity: String,
    carried_boundary_proof_digest: String,
    selected_identity_fields_produced: &'static [&'static str],
    coverage_contributor: KernelTouchedGraphParityCoverageContributor,
}

impl ReplayUndoFamilyContributorCatalogRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        kind: ReplayUndoContributorRowKind,
        current_packet_or_identity_source: &'static str,
        carried_scope_identity_source: &'static str,
        carried_witness_or_boundary_source: &'static str,
        ordinary_path_live_caller_surface: &'static str,
        ordinary_path_live_caller_path: &'static str,
        route_family: ReplayUndoPlannerRouteFamily,
        route_packet_identity: String,
        transaction_packet_identity: String,
        carried_scope_identity: String,
        carried_boundary_proof_digest: String,
        selected_identity_fields_produced: &'static [&'static str],
    ) -> Result<Self, ReplayUndoFamilyContributorCatalogError> {
        if current_packet_or_identity_source.is_empty()
            || carried_scope_identity_source.is_empty()
            || carried_witness_or_boundary_source.is_empty()
            || ordinary_path_live_caller_surface.is_empty()
            || ordinary_path_live_caller_path.is_empty()
            || route_packet_identity.is_empty()
            || transaction_packet_identity.is_empty()
            || carried_scope_identity.is_empty()
            || carried_boundary_proof_digest.is_empty()
            || selected_identity_fields_produced.is_empty()
        {
            return Err(ReplayUndoFamilyContributorCatalogError::new(
                ReplayUndoFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                "replay/undo contributor row requires exact current, carried scope, boundary proof, and ordinary caller identities",
            ));
        }

        let current_surface = match kind {
            ReplayUndoContributorRowKind::Replay => {
                "current_replay_undo_boundary_route_authority::replay_scope_identity"
            }
            ReplayUndoContributorRowKind::Undo => {
                "current_replay_undo_boundary_route_authority::undo_scope_identity"
            }
        };
        Ok(Self {
            kind,
            family_kind: TouchedGraphParityFamilyKind::ReplayUndo,
            current_packet_or_identity_source,
            carried_scope_identity_source,
            carried_witness_or_boundary_source,
            ordinary_path_live_caller_surface,
            ordinary_path_live_caller_path,
            route_family,
            route_packet_identity,
            transaction_packet_identity,
            carried_scope_identity,
            carried_boundary_proof_digest,
            selected_identity_fields_produced,
            coverage_contributor: KernelTouchedGraphParityCoverageContributor::new(
                current_surface,
                "crates/worth-kernel/src/workload_composition/planner_owned_routing/ordinary_consumer_authority/replay_undo_route_authority.rs",
                current_packet_or_identity_source,
                "current_worth_workload_ordinary_consumer_cutover::{replay_undo_boundary_proof_digests,transaction_packet_identities,replay_scope_identities,undo_scope_identities}",
                "carried_scope_contract",
                "crates/worth-kernel/src/workload_composition/planner_owned_routing/ordinary_consumer_authority/",
                selected_identity_fields_produced,
                KernelTouchedGraphParityQuerySurfaceKind::NotQuery,
                ordinary_path_live_caller_surface,
                ordinary_path_live_caller_path,
            ),
        })
    }

    pub const fn kind(&self) -> ReplayUndoContributorRowKind {
        self.kind
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub const fn current_packet_or_identity_source(&self) -> &'static str {
        self.current_packet_or_identity_source
    }

    pub const fn carried_scope_identity_source(&self) -> &'static str {
        self.carried_scope_identity_source
    }

    pub const fn carried_witness_or_boundary_source(&self) -> &'static str {
        self.carried_witness_or_boundary_source
    }

    pub const fn ordinary_path_live_caller_surface(&self) -> &'static str {
        self.ordinary_path_live_caller_surface
    }

    pub const fn ordinary_path_live_caller_path(&self) -> &'static str {
        self.ordinary_path_live_caller_path
    }

    pub const fn route_family(&self) -> ReplayUndoPlannerRouteFamily {
        self.route_family
    }

    pub fn route_packet_identity(&self) -> &str {
        &self.route_packet_identity
    }

    pub fn transaction_packet_identity(&self) -> &str {
        &self.transaction_packet_identity
    }

    pub fn carried_scope_identity(&self) -> &str {
        &self.carried_scope_identity
    }

    pub fn carried_boundary_proof_digest(&self) -> &str {
        &self.carried_boundary_proof_digest
    }

    pub const fn selected_identity_fields_produced(&self) -> &'static [&'static str] {
        self.selected_identity_fields_produced
    }

    pub const fn coverage_contributor(&self) -> &KernelTouchedGraphParityCoverageContributor {
        &self.coverage_contributor
    }

    #[cfg(test)]
    pub(crate) fn with_test_identity_override(
        mut self,
        route_packet_identity: &str,
        transaction_packet_identity: &str,
        carried_scope_identity: &str,
        carried_boundary_proof_digest: &str,
    ) -> Self {
        self.route_packet_identity = route_packet_identity.to_string();
        self.transaction_packet_identity = transaction_packet_identity.to_string();
        self.carried_scope_identity = carried_scope_identity.to_string();
        self.carried_boundary_proof_digest = carried_boundary_proof_digest.to_string();
        self
    }
}

pub(crate) fn replay_undo_coverage_contributor_rows_from_catalog(
    rows: &[ReplayUndoFamilyContributorCatalogRow],
) -> Result<Vec<KernelTouchedGraphParityCoverageContributor>, KernelTouchedGraphParityCoverageError>
{
    Ok(rows
        .iter()
        .map(|row| row.coverage_contributor().clone())
        .collect())
}
