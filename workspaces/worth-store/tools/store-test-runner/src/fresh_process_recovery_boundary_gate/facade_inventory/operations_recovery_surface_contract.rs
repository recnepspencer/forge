//! Destination contract for successor-owned recovery workflow surfaces.

#[path = "operations_point_in_time_surface_contract.rs"]
mod point_in_time_recovery_surface_contract;
#[path = "operations_recovery_replay_surface_contract.rs"]
mod recovery_replay_surface_contract;
#[path = "operations_restore_surface_contract.rs"]
mod restore_surface_contract;
#[path = "operations_rollback_surface_contract.rs"]
mod rollback_surface_contract;

pub(super) fn operations_recovery_destination_surfaces(
) -> impl Iterator<Item = &'static (&'static str, &'static str, &'static str)> {
    restore_surface_contract::DESTINATION_SURFACES
        .iter()
        .chain(point_in_time_recovery_surface_contract::DESTINATION_SURFACES.iter())
        .chain(rollback_surface_contract::DESTINATION_SURFACES.iter())
        .chain(recovery_replay_surface_contract::DESTINATION_SURFACES.iter())
}
