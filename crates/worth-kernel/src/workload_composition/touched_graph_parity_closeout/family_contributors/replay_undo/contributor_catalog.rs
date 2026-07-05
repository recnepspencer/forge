use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_selected_route_packet,
    current_worth_workload_ordinary_consumer_cutover, WorthTouchedGraphConflictSelectedRoutePacket,
    WorthWorkloadOrdinaryConsumerCutover,
};
use crate::workload_composition::touched_graph_parity_closeout::family_contributors::{
    KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError,
};
use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use super::error::{
    ReplayUndoFamilyContributorCatalogError, ReplayUndoFamilyContributorCatalogErrorKind,
};
use super::replay_row::replay_contributor_row_from_authorities;
use super::row::{
    replay_undo_coverage_contributor_rows_from_catalog, ReplayUndoContributorRowKind,
    ReplayUndoFamilyContributorCatalogRow,
};
use super::undo_row::undo_contributor_row_from_authorities;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoFamilyContributorCatalog {
    rows: Vec<ReplayUndoFamilyContributorCatalogRow>,
}

pub fn current_replay_undo_family_contributor_catalog(
) -> Result<ReplayUndoFamilyContributorCatalog, ReplayUndoFamilyContributorCatalogError> {
    let selected_route =
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|error| {
            ReplayUndoFamilyContributorCatalogError::new(
                ReplayUndoFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
                error.detail(),
            )
        })?;
    let cutover = current_worth_workload_ordinary_consumer_cutover().map_err(|error| {
        ReplayUndoFamilyContributorCatalogError::new(
            ReplayUndoFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
            format!("{error:?}"),
        )
    })?;
    replay_undo_family_contributor_catalog_from_authorities(&selected_route, &cutover)
}

pub(crate) fn replay_undo_family_contributor_catalog_from_authorities(
    selected_route: &WorthTouchedGraphConflictSelectedRoutePacket,
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
) -> Result<ReplayUndoFamilyContributorCatalog, ReplayUndoFamilyContributorCatalogError> {
    ReplayUndoFamilyContributorCatalog::new_with_authorities(
        vec![
            replay_contributor_row_from_authorities(selected_route, cutover)?,
            undo_contributor_row_from_authorities(selected_route, cutover)?,
        ],
        selected_route,
        cutover,
    )
}

pub(crate) fn replay_undo_coverage_contributor_rows(
) -> Result<Vec<KernelTouchedGraphParityCoverageContributor>, KernelTouchedGraphParityCoverageError>
{
    let catalog = current_replay_undo_family_contributor_catalog()
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))?;
    replay_undo_coverage_contributor_rows_from_catalog(catalog.rows())
}

pub(crate) fn replay_undo_coverage_contributor_rows_from_authorities(
    selected_route: &WorthTouchedGraphConflictSelectedRoutePacket,
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
) -> Result<Vec<KernelTouchedGraphParityCoverageContributor>, KernelTouchedGraphParityCoverageError>
{
    let catalog = replay_undo_family_contributor_catalog_from_authorities(selected_route, cutover)
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))?;
    replay_undo_coverage_contributor_rows_from_catalog(catalog.rows())
}

pub(crate) fn current_replay_coverage_contributor(
) -> Result<KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError> {
    current_replay_undo_family_contributor_catalog()
        .map(|catalog| {
            catalog
                .rows()
                .iter()
                .find(|row| row.kind() == ReplayUndoContributorRowKind::Replay)
                .expect("replay contributor row")
                .coverage_contributor()
                .clone()
        })
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))
}

pub(crate) fn current_undo_coverage_contributor(
) -> Result<KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError> {
    current_replay_undo_family_contributor_catalog()
        .map(|catalog| {
            catalog
                .rows()
                .iter()
                .find(|row| row.kind() == ReplayUndoContributorRowKind::Undo)
                .expect("undo contributor row")
                .coverage_contributor()
                .clone()
        })
        .map_err(|error| KernelTouchedGraphParityCoverageError::new(error.detail()))
}

impl ReplayUndoFamilyContributorCatalog {
    pub fn new(
        rows: Vec<ReplayUndoFamilyContributorCatalogRow>,
    ) -> Result<Self, ReplayUndoFamilyContributorCatalogError> {
        validate_catalog_rows(&rows)?;
        Ok(Self { rows })
    }

    pub(crate) fn new_with_authorities(
        rows: Vec<ReplayUndoFamilyContributorCatalogRow>,
        selected_route: &WorthTouchedGraphConflictSelectedRoutePacket,
        cutover: &WorthWorkloadOrdinaryConsumerCutover,
    ) -> Result<Self, ReplayUndoFamilyContributorCatalogError> {
        validate_catalog_rows_against_authorities(&rows, selected_route, cutover)?;
        Ok(Self { rows })
    }

    pub fn rows(&self) -> &[ReplayUndoFamilyContributorCatalogRow] {
        &self.rows
    }

    #[cfg(test)]
    pub(crate) fn new_unvalidated_for_testing(
        rows: Vec<ReplayUndoFamilyContributorCatalogRow>,
    ) -> Self {
        Self { rows }
    }
}

pub(crate) fn validate_replay_undo_family_contributor_catalog(
    catalog: &ReplayUndoFamilyContributorCatalog,
) -> Result<(), ReplayUndoFamilyContributorCatalogError> {
    validate_catalog_rows(catalog.rows())
}

fn validate_catalog_rows(
    rows: &[ReplayUndoFamilyContributorCatalogRow],
) -> Result<(), ReplayUndoFamilyContributorCatalogError> {
    if rows.len() != 2 {
        return Err(ReplayUndoFamilyContributorCatalogError::new(
            ReplayUndoFamilyContributorCatalogErrorKind::MissingRequiredRow,
            "replay/undo contributor catalog requires exactly replay and undo rows",
        ));
    }
    let cutover = current_worth_workload_ordinary_consumer_cutover().map_err(|error| {
        ReplayUndoFamilyContributorCatalogError::new(
            ReplayUndoFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
            format!("{error:?}"),
        )
    })?;
    let selected_route =
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|error| {
            ReplayUndoFamilyContributorCatalogError::new(
                ReplayUndoFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
                error.detail(),
            )
        })?;
    validate_catalog_rows_against_authorities(rows, &selected_route, &cutover)
}

fn validate_catalog_rows_against_authorities(
    rows: &[ReplayUndoFamilyContributorCatalogRow],
    selected_route: &WorthTouchedGraphConflictSelectedRoutePacket,
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
) -> Result<(), ReplayUndoFamilyContributorCatalogError> {
    let mut has_replay = false;
    let mut has_undo = false;
    for row in rows {
        match row.kind() {
            ReplayUndoContributorRowKind::Replay => has_replay = true,
            ReplayUndoContributorRowKind::Undo => has_undo = true,
        }
        if row.family_kind() != TouchedGraphParityFamilyKind::ReplayUndo {
            return Err(ReplayUndoFamilyContributorCatalogError::new(
                ReplayUndoFamilyContributorCatalogErrorKind::MismatchedRouteFamily,
                "replay/undo contributor row must remain in the shared ReplayUndo family kind",
            ));
        }
        if row.route_family() != selected_route.replay_undo_route_family()
            || row.route_packet_identity() != selected_route.replay_undo_route_packet_identity()
        {
            return Err(ReplayUndoFamilyContributorCatalogError::new(
                ReplayUndoFamilyContributorCatalogErrorKind::MismatchedRouteFamily,
                "replay/undo contributor row route family diverged from the carried replay/undo planner route family",
            ));
        }
        if row.route_packet_identity().is_empty()
            || row.transaction_packet_identity().is_empty()
            || row.carried_boundary_proof_digest().is_empty()
            || row.carried_scope_identity().is_empty()
        {
            return Err(ReplayUndoFamilyContributorCatalogError::new(
                ReplayUndoFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                "replay/undo contributor row must carry explicit route packet, transaction packet, boundary proof, and scope identities",
            ));
        }
        if !selected_route
            .transaction_packet_identities()
            .iter()
            .any(|identity| identity == row.transaction_packet_identity())
            || !cutover
                .transaction_packet_identities()
                .iter()
                .any(|identity| identity == row.transaction_packet_identity())
            || !selected_route
                .replay_undo_boundary_proof_digests()
                .iter()
                .any(|digest| digest == row.carried_boundary_proof_digest())
            || !cutover
                .replay_undo_boundary_proof_digests()
                .iter()
                .any(|digest| digest == row.carried_boundary_proof_digest())
        {
            return Err(ReplayUndoFamilyContributorCatalogError::new(
                ReplayUndoFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                "replay/undo contributor row must carry the exact shared transaction packet and boundary proof identities admitted by selected-route and ordinary cutover authorities",
            ));
        }
        let scope_matches = match row.kind() {
            ReplayUndoContributorRowKind::Replay => {
                selected_route
                    .replay_scope_identities()
                    .iter()
                    .any(|identity| identity == row.carried_scope_identity())
                    && cutover
                        .replay_scope_identities()
                        .iter()
                        .any(|identity| identity == row.carried_scope_identity())
            }
            ReplayUndoContributorRowKind::Undo => {
                selected_route
                    .undo_scope_identities()
                    .iter()
                    .any(|identity| identity == row.carried_scope_identity())
                    && cutover
                        .undo_scope_identities()
                        .iter()
                        .any(|identity| identity == row.carried_scope_identity())
            }
        };
        if !scope_matches {
            return Err(ReplayUndoFamilyContributorCatalogError::new(
                ReplayUndoFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                "replay/undo contributor row must carry the exact shared replay or undo scope identity admitted by selected-route and ordinary cutover authorities",
            ));
        }
        if row.ordinary_path_live_caller_path()
            != "crates/worth-kernel/src/workload_composition/worth_workload/replay_undo_boundary/boolean_split_boundary_admission.rs"
        {
            return Err(ReplayUndoFamilyContributorCatalogError::new(
                ReplayUndoFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                "replay/undo contributor row must name the exact admitted ordinary caller path carried by the replay/undo boundary admission surface",
            ));
        }
    }

    if !(has_replay && has_undo) {
        return Err(ReplayUndoFamilyContributorCatalogError::new(
            ReplayUndoFamilyContributorCatalogErrorKind::MissingRequiredRow,
            "replay/undo contributor catalog requires one replay row and one undo row",
        ));
    }
    Ok(())
}
