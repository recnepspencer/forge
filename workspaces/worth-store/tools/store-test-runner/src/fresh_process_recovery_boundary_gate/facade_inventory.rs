mod api_inventory;
mod backend_wal_durability_surface_contract;
mod blob_replay_surface_contract;
mod bounded_decode_surface_contract;
mod checkpoint_backup_surface_contract;
mod cross_file_surface_contract;
mod delivered_api;
mod destination_surface_contract;
pub(super) mod disposition_contract;
mod inventory_disposition;
mod moved_integrity_surface_contract;
mod offline_verifier_candidate_surface_contract;
mod operations_recovery_surface_contract;
mod phase_eight_owner_surface_contract;
mod physical_isolation_publication_surface_contract;
mod physical_isolation_recovery_source_surface_contract;
mod preserved_destination_surfaces;
mod reachable_api;
mod recovery_physics_destination_surfaces;
mod recovery_wal_surface_contract;
mod runtime_phase_eight_surface_contract;
mod runtime_phase_five_surface_contract;
mod runtime_phase_four_plan_surface_contract;
mod runtime_phase_four_projection_surface_contract;
mod runtime_phase_four_surface_contract;
mod runtime_phase_seven_surface_contract;
mod runtime_phase_six_surface_contract;
mod runtime_phase_three_surface_contract;
mod runtime_progression_completion_surface_contract;
mod supporting_delivery_surface_contract;

use std::collections::BTreeSet;

use super::documents::{read_repository_document, API_INVENTORY};
use api_inventory::parse_inventory;
use backend_wal_durability_surface_contract::{
    BACKEND_WAL_DURABILITY_CERTIFICATION_SURFACES, BACKEND_WAL_DURABILITY_DESTINATION_SURFACES,
};
use blob_replay_surface_contract::BLOB_REPLAY_DESTINATION_SURFACES;
use bounded_decode_surface_contract::BOUNDED_DECODE_SURFACES;
use checkpoint_backup_surface_contract::CHECKPOINT_BACKUP_DESTINATION_SURFACES;
use cross_file_surface_contract::assert_cross_file_surfaces;
use delivered_api::assert_exact_inventory;
use destination_surface_contract::{
    BACKEND_RECOVERY_EVIDENCE_SURFACES, BACKEND_RECOVERY_PUBLICATION_SURFACES,
    BACKEND_RECOVERY_SURFACES, STORE_RECOVERY_COORDINATION_SURFACES,
    STORE_RECOVERY_PUBLICATION_SURFACES,
};
use moved_integrity_surface_contract::MOVED_INTEGRITY_DESTINATION_SURFACES;
use offline_verifier_candidate_surface_contract::OFFLINE_VERIFIER_CANDIDATE_DESTINATION_SURFACES;
use operations_recovery_surface_contract::operations_recovery_destination_surfaces;
use phase_eight_owner_surface_contract::PHASE_EIGHT_OWNER_DESTINATION_SURFACES;
use physical_isolation_publication_surface_contract::PHYSICAL_ISOLATION_PUBLICATION_DESTINATION_SURFACES;
use physical_isolation_recovery_source_surface_contract::PHYSICAL_ISOLATION_RECOVERY_SOURCE_DESTINATION_SURFACES;
use preserved_destination_surfaces::PRESERVED_WAL_DESTINATION_SURFACES;
use reachable_api::current_facade;
use recovery_physics_destination_surfaces::DESTINATION_SURFACES;
use recovery_wal_surface_contract::RECOVERY_WAL_DESTINATION_SURFACES;
use runtime_phase_eight_surface_contract::PHASE_EIGHT_DELIVERY_SURFACES;
use runtime_phase_five_surface_contract::PHASE_FIVE_DELIVERY_SURFACES;
use runtime_phase_four_plan_surface_contract::PHASE_FOUR_PLAN_SURFACES;
use runtime_phase_four_projection_surface_contract::PHASE_FOUR_PROJECTION_SURFACES;
use runtime_phase_four_surface_contract::PHASE_FOUR_DELIVERY_SURFACES;
use runtime_phase_seven_surface_contract::phase_seven_delivery_surfaces;
use runtime_phase_six_surface_contract::PHASE_SIX_DELIVERY_SURFACES;
use runtime_phase_three_surface_contract::RUNTIME_PHASE_THREE_SURFACES;
use runtime_progression_completion_surface_contract::RUNTIME_PROGRESSION_COMPLETION_DESTINATION_SURFACES;
use supporting_delivery_surface_contract::SUPPORTING_DELIVERY_SURFACES;
const FACADE: &str = "workspaces/worth-store/crates/worth-store-recovery-physics/src/lib.rs";

#[test]
fn current_facade_and_destination_contract_have_exact_inventory_rows() {
    let document = read_repository_document(API_INVENTORY).expect("read C.8 API inventory");
    let rows = parse_inventory(&document).expect("parse C.8 API inventory");
    let exact_rows = rows
        .iter()
        .map(|row| {
            (
                row.scope.as_str(),
                row.surface.as_str(),
                row.source_owner.as_str(),
                row.disposition.as_str(),
                row.destination_owner.as_str(),
                row.phase.as_str(),
            )
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        exact_rows.len(),
        rows.len(),
        "C.8 API inventory contains duplicate rows"
    );
    let current = rows
        .iter()
        .filter(|row| row.scope.starts_with("current"))
        .map(|row| {
            (
                row.scope.clone(),
                row.surface.clone(),
                row.source_owner.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    let actual = current_facade().expect("parse recovery-physics facade");
    assert_cross_file_surfaces(&actual);
    let omitted = actual.difference(&current).collect::<Vec<_>>();
    let stale = current.difference(&actual).collect::<Vec<_>>();
    assert!(
        omitted.is_empty() && stale.is_empty(),
        "C.8 API inventory omitted {omitted:?} or retained stale {stale:?}"
    );

    let destination = rows
        .iter()
        .filter(|row| row.scope == "destination")
        .map(|row| {
            (
                row.surface.as_str(),
                row.source_owner.as_str(),
                row.phase.as_str(),
            )
        })
        .collect::<BTreeSet<_>>();
    let contract = DESTINATION_SURFACES
        .iter()
        .chain(BOUNDED_DECODE_SURFACES)
        .chain(RUNTIME_PHASE_THREE_SURFACES)
        .chain(PHASE_FOUR_DELIVERY_SURFACES)
        .chain(PHASE_FOUR_PLAN_SURFACES)
        .chain(PHASE_FOUR_PROJECTION_SURFACES)
        .chain(PHASE_FIVE_DELIVERY_SURFACES)
        .chain(PHASE_SIX_DELIVERY_SURFACES)
        .chain(phase_seven_delivery_surfaces())
        .chain(PHASE_EIGHT_DELIVERY_SURFACES)
        .chain(BACKEND_RECOVERY_SURFACES)
        .chain(BACKEND_RECOVERY_EVIDENCE_SURFACES)
        .chain(BACKEND_RECOVERY_PUBLICATION_SURFACES)
        .chain(BACKEND_WAL_DURABILITY_DESTINATION_SURFACES)
        .chain(STORE_RECOVERY_COORDINATION_SURFACES)
        .chain(STORE_RECOVERY_PUBLICATION_SURFACES)
        .chain(SUPPORTING_DELIVERY_SURFACES)
        .chain(PHASE_EIGHT_OWNER_DESTINATION_SURFACES)
        .chain(BLOB_REPLAY_DESTINATION_SURFACES)
        .chain(CHECKPOINT_BACKUP_DESTINATION_SURFACES)
        .chain(operations_recovery_destination_surfaces())
        .chain(OFFLINE_VERIFIER_CANDIDATE_DESTINATION_SURFACES)
        .chain(PHYSICAL_ISOLATION_PUBLICATION_DESTINATION_SURFACES)
        .chain(PHYSICAL_ISOLATION_RECOVERY_SOURCE_DESTINATION_SURFACES)
        .chain(RECOVERY_WAL_DESTINATION_SURFACES)
        .chain(RUNTIME_PROGRESSION_COMPLETION_DESTINATION_SURFACES)
        .chain(MOVED_INTEGRITY_DESTINATION_SURFACES)
        .copied()
        .collect::<BTreeSet<_>>();
    let omitted = contract.difference(&destination).collect::<Vec<_>>();
    let stale = destination.difference(&contract).collect::<Vec<_>>();
    assert!(
        omitted.is_empty() && stale.is_empty(),
        "C.8 destination facade inventory omitted {omitted:?} or retained stale {stale:?}"
    );

    let delivered_inventory = rows
        .iter()
        .filter(|row| row.scope == "destination")
        .filter(|row| {
            !row.source_owner
                .starts_with("worth-store-offline-verifier/")
        })
        // These Phase 8 owner surfaces are validated by the destination
        // contract above. Layout-indexes and security metadata are deliberately
        // not members of the existing shipped facade families, while the
        // format-native checkpoint wire surface is part of the physical-format
        // facade and remains in the delivered inventory.
        .filter(|row| {
            if row.source_owner.starts_with("worth-store-physical-format/") {
                return true;
            }
            !PHASE_EIGHT_OWNER_DESTINATION_SURFACES
                .iter()
                .any(|(surface, source_owner, phase)| {
                    row.surface == *surface
                        && row.source_owner == *source_owner
                        && row.phase == *phase
                })
        })
        .filter(|row| {
            !operations_recovery_destination_surfaces().any(|(surface, source_owner, phase)| {
                row.surface == *surface && row.source_owner == *source_owner && row.phase == *phase
            })
        })
        .filter(|row| {
            !OFFLINE_VERIFIER_CANDIDATE_DESTINATION_SURFACES.iter().any(
                |(surface, source_owner, phase)| {
                    row.surface == *surface
                        && row.source_owner == *source_owner
                        && row.phase == *phase
                },
            )
        })
        .filter(|row| {
            !PHYSICAL_ISOLATION_PUBLICATION_DESTINATION_SURFACES
                .iter()
                .any(|(surface, source_owner, phase)| {
                    row.surface == *surface
                        && row.source_owner == *source_owner
                        && row.phase == *phase
                })
        })
        .filter(|row| {
            !PHYSICAL_ISOLATION_RECOVERY_SOURCE_DESTINATION_SURFACES
                .iter()
                .any(|(surface, source_owner, phase)| {
                    row.surface == *surface
                        && row.source_owner == *source_owner
                        && row.phase == *phase
                })
        })
        .filter(|row| {
            !MOVED_INTEGRITY_DESTINATION_SURFACES
                .iter()
                .any(|(surface, source_owner, phase)| {
                    row.surface == *surface
                        && row.source_owner == *source_owner
                        && row.phase == *phase
                })
        })
        .filter(|row| {
            !matches!(
                row.surface.as_str(),
                "RecoveryCompletionDenial" | "complete_recovery"
            )
        })
        .filter(|row| {
            !BLOB_REPLAY_DESTINATION_SURFACES
                .iter()
                .any(|(surface, source_owner, phase)| {
                    row.surface == *surface
                        && row.source_owner == *source_owner
                        && row.phase == *phase
                })
        })
        .filter(|row| {
            !BACKEND_WAL_DURABILITY_CERTIFICATION_SURFACES.contains(&row.surface.as_str())
        })
        .filter(|row| {
            !PRESERVED_WAL_DESTINATION_SURFACES
                .iter()
                .any(|(surface, source_owner, phase)| {
                    row.surface == *surface
                        && row.source_owner == *source_owner
                        && row.phase == *phase
                })
        })
        .filter(|row| {
            matches!(
                row.phase.as_str(),
                "phase-2" | "phase-3" | "phase-4" | "phase-5" | "phase-6" | "phase-7" | "phase-8"
            )
        })
        .map(|row| (row.surface.clone(), row.source_owner.clone()))
        .collect::<BTreeSet<_>>();
    assert_exact_inventory(delivered_inventory).expect("exact delivered C.8 facade inventory");
}
