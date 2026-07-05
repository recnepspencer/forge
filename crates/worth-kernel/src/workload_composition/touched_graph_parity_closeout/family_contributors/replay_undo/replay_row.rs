use crate::workload_composition::planner_owned_routing::WorthTouchedGraphConflictSelectedRoutePacket;
use crate::workload_composition::planner_owned_routing::WorthWorkloadOrdinaryConsumerCutover;

use super::error::{
    ReplayUndoFamilyContributorCatalogError, ReplayUndoFamilyContributorCatalogErrorKind,
};
use super::row::{ReplayUndoContributorRowKind, ReplayUndoFamilyContributorCatalogRow};

const CURRENT_SOURCE: &str = "current_replay_undo_boundary_route_authority";
const CARRIED_SCOPE_SOURCE: &str =
    "current_replay_undo_boundary_route_authority::replay_scope_identity";
const CARRIED_BOUNDARY_SOURCE: &str =
    "current_replay_undo_boundary_route_authority::boundary_proof_digest";
const ORDINARY_CALLER_SURFACE: &str = "admit_boolean_split_replay_undo_boundary";
const ORDINARY_CALLER_PATH: &str =
    "crates/worth-kernel/src/workload_composition/worth_workload/replay_undo_boundary/boolean_split_boundary_admission.rs";
const SELECTED_FIELDS: &[&str] = &[
    "route_packet_identity",
    "transaction_packet_identity",
    "replay_scope_identity",
    "boundary_proof_digest",
    "route_authority_digest",
];

pub(super) fn current_replay_contributor_row(
) -> Result<ReplayUndoFamilyContributorCatalogRow, ReplayUndoFamilyContributorCatalogError> {
    let selected_route = crate::workload_composition::planner_owned_routing::
        current_worth_touched_graph_conflict_selected_route_packet()
    .map_err(|error| {
        ReplayUndoFamilyContributorCatalogError::new(
            ReplayUndoFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
            error.detail(),
        )
    })?;
    let cutover = crate::workload_composition::planner_owned_routing::
        current_worth_workload_ordinary_consumer_cutover()
    .map_err(|error| {
        ReplayUndoFamilyContributorCatalogError::new(
            ReplayUndoFamilyContributorCatalogErrorKind::CurrentSurfaceUnavailable,
            format!("{error:?}"),
        )
    })?;
    replay_contributor_row_from_authorities(&selected_route, &cutover)
}

pub(super) fn replay_contributor_row_from_authorities(
    selected_route: &WorthTouchedGraphConflictSelectedRoutePacket,
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
) -> Result<ReplayUndoFamilyContributorCatalogRow, ReplayUndoFamilyContributorCatalogError> {
    ReplayUndoFamilyContributorCatalogRow::new(
        ReplayUndoContributorRowKind::Replay,
        CURRENT_SOURCE,
        CARRIED_SCOPE_SOURCE,
        CARRIED_BOUNDARY_SOURCE,
        ORDINARY_CALLER_SURFACE,
        ORDINARY_CALLER_PATH,
        selected_route.replay_undo_route_family(),
        selected_route
            .replay_undo_route_packet_identity()
            .to_string(),
        shared_identity(
            selected_route.transaction_packet_identities(),
            &cutover.transaction_packet_identities(),
            "transaction packet identity",
        )?,
        shared_identity(
            selected_route.replay_scope_identities(),
            &cutover.replay_scope_identities(),
            "replay scope identity",
        )?,
        shared_identity(
            selected_route.replay_undo_boundary_proof_digests(),
            &cutover.replay_undo_boundary_proof_digests(),
            "replay/undo boundary proof digest",
        )?,
        SELECTED_FIELDS,
    )
}

fn shared_identity(
    selected_route_values: &[String],
    cutover_values: &[String],
    label: &'static str,
) -> Result<String, ReplayUndoFamilyContributorCatalogError> {
    selected_route_values
        .iter()
        .find(|value| cutover_values.iter().any(|candidate| candidate == *value))
        .cloned()
        .ok_or_else(|| {
            ReplayUndoFamilyContributorCatalogError::new(
                ReplayUndoFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                format!(
                    "replay/undo contributor row requires a {label} carried by both selected-route and ordinary cutover authorities",
                ),
            )
        })
}
