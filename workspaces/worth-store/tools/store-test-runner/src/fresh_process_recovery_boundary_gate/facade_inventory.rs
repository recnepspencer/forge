mod bounded_decode_surface_contract;
mod cross_file_surface_contract;
mod delivered_api;
mod destination_surface_contract;
pub(super) mod disposition_contract;
mod inventory_disposition;
mod reachable_api;
mod runtime_phase_eight_surface_contract;
mod runtime_phase_five_surface_contract;
mod runtime_phase_four_plan_surface_contract;
mod runtime_phase_four_projection_surface_contract;
mod runtime_phase_four_surface_contract;
mod runtime_phase_seven_surface_contract;
mod runtime_phase_six_surface_contract;
mod runtime_phase_three_surface_contract;
mod supporting_delivery_surface_contract;

use std::collections::BTreeSet;

use super::documents::{read_repository_document, split_csv, API_INVENTORY};
use bounded_decode_surface_contract::BOUNDED_DECODE_SURFACES;
use cross_file_surface_contract::assert_cross_file_surfaces;
use delivered_api::assert_exact_inventory;
use destination_surface_contract::{
    BACKEND_RECOVERY_PUBLICATION_SURFACES, BACKEND_RECOVERY_SURFACES,
    STORE_RECOVERY_COORDINATION_SURFACES, STORE_RECOVERY_PUBLICATION_SURFACES,
};
use reachable_api::current_facade;
use runtime_phase_eight_surface_contract::PHASE_EIGHT_DELIVERY_SURFACES;
use runtime_phase_five_surface_contract::PHASE_FIVE_DELIVERY_SURFACES;
use runtime_phase_four_plan_surface_contract::PHASE_FOUR_PLAN_SURFACES;
use runtime_phase_four_projection_surface_contract::PHASE_FOUR_PROJECTION_SURFACES;
use runtime_phase_four_surface_contract::PHASE_FOUR_DELIVERY_SURFACES;
use runtime_phase_seven_surface_contract::phase_seven_delivery_surfaces;
use runtime_phase_six_surface_contract::PHASE_SIX_DELIVERY_SURFACES;
use runtime_phase_three_surface_contract::RUNTIME_PHASE_THREE_SURFACES;
use supporting_delivery_surface_contract::SUPPORTING_DELIVERY_SURFACES;
const HEADER: &str = "scope,surface,source_owner,disposition,destination_owner,phase";
const FACADE: &str = "workspaces/worth-store/crates/worth-store-recovery-physics/src/lib.rs";

const DESTINATION_SURFACES: &[(&str, &str, &str)] = &[
    ("PhysicalRecoveryOpenRequest", "entry/request", "phase-2"),
    (
        "PhysicalRecoveryOpenRequest::declare",
        "entry/request",
        "phase-2",
    ),
    (
        "PhysicalRecoveryOpenRequest::admit",
        "entry/request",
        "phase-2",
    ),
    (
        "PhysicalRecoveryStaticConfiguration",
        "entry/configuration",
        "phase-2",
    ),
    (
        "PhysicalRecoveryStaticConfiguration::current",
        "entry/configuration",
        "phase-2",
    ),
    (
        "PhysicalRecoveryLimitDeclaration",
        "entry/limits",
        "phase-2",
    ),
    ("PhysicalRecoveryLimits", "entry/limits", "phase-2"),
    ("PhysicalRecoveryLimits::admit", "entry/limits", "phase-2"),
    ("PhysicalRecoveryLimitDenial", "entry/limits", "phase-2"),
    (
        "PhysicalRecoveryAdmissionCounters",
        "entry/counters",
        "phase-2",
    ),
    (
        "PhysicalRecoverySessionIdentity",
        "entry/session",
        "phase-2",
    ),
    (
        "PhysicalRecoveryEntryBindingDrift",
        "entry/authority-binding",
        "phase-2",
    ),
    (
        "PhysicalRecoveryPlatformAuthority",
        "entry/authority",
        "phase-2",
    ),
    (
        "PhysicalRecoveryPlatformAuthority::acquire",
        "entry/authority",
        "phase-2",
    ),
    (
        "PhysicalRecoveryPlatformAuthority::qualified_backend_profile",
        "entry/authority",
        "phase-2",
    ),
    (
        "PhysicalRecoveryPlatformAuthority::session_identity",
        "entry/authority",
        "phase-2",
    ),
    (
        "PhysicalRecoveryPlatformAuthority::process_counters",
        "entry/authority",
        "phase-2",
    ),
    (
        "PhysicalRecoveryPlatformAdmissionError",
        "entry/authority",
        "phase-2",
    ),
    ("PhysicalRecoveryOutcome", "entry/outcome", "phase-2"),
    ("WorthStoreRecovery", "lib", "phase-6"),
    ("WorthStoreRecovery::recover", "lib", "phase-6"),
    (
        "AdmittedPhysicalRecovery",
        "progression/admitted",
        "phase-2",
    ),
    (
        "AdmittedPhysicalRecovery::store_identity",
        "progression/admitted",
        "phase-2",
    ),
    (
        "AdmittedPhysicalRecovery::session_identity",
        "progression/admitted",
        "phase-2",
    ),
    (
        "AdmittedPhysicalRecovery::limits",
        "progression/admitted",
        "phase-2",
    ),
    (
        "AdmittedPhysicalRecovery::counters",
        "progression/admitted",
        "phase-2",
    ),
    (
        "AdmittedPhysicalRecovery::cancel_before_discovery",
        "progression/admitted",
        "phase-2",
    ),
    ("PlannedPhysicalRecovery", "progression/planned", "phase-4"),
    (
        "NamespaceDurablePhysicalRecovery",
        "progression/namespace_durable",
        "phase-6",
    ),
    (
        "ReopenedPhysicalRecovery",
        "progression/reopened",
        "phase-6",
    ),
    (
        "RecoveredPhysicalRuntimeHandoff",
        "handoff/recovered",
        "phase-6",
    ),
    (
        "PhysicalRecoveryConstructionPort",
        "worth-store/recovery-construction/port",
        "phase-6",
    ),
    (
        "PhysicalRecoveryFreshnessAuthority",
        "worth-store/recovery-freshness/authority",
        "phase-2",
    ),
    (
        "PhysicalRecoveryFreshnessAuthority::register_session",
        "worth-store/recovery-freshness/registration",
        "phase-2",
    ),
    (
        "PhysicalRecoveryRegisteredSessionAuthority",
        "worth-store/recovery-freshness/registration",
        "phase-2",
    ),
    (
        "PhysicalRecoveryRegisteredSessionAuthority::session_identity_bytes",
        "worth-store/recovery-freshness/registration",
        "phase-2",
    ),
    (
        "PhysicalRecoveryFreshnessPort",
        "worth-store/recovery-freshness/port",
        "phase-2",
    ),
    (
        "PhysicalRecoveryFreshnessPort::admit",
        "worth-store/recovery-freshness/port",
        "phase-2",
    ),
    (
        "PhysicalRecoveryFreshnessPort::sample_binding",
        "worth-store/recovery-freshness/port",
        "phase-4",
    ),
    (
        "PhysicalRecoveryConstructionAuthority",
        "worth-store/recovery-construction/authority",
        "phase-6",
    ),
    ("PhysicalRecoveryRefusal", "entry/outcome", "phase-2"),
    (
        "PhysicalRecoveryRefusal::recovery_effects",
        "entry/outcome",
        "phase-2",
    ),
    ("PhysicalRecoveryRefusalKind", "entry/outcome", "phase-2"),
    ("PhysicalRecoveryBlock", "entry/outcome", "phase-3"),
    (
        "PhysicalRecoveryPublicationIndeterminate",
        "entry/outcome",
        "phase-6",
    ),
    (
        "RecoveryOperationFateSet",
        "handoff/operation_fates",
        "phase-4",
    ),
    (
        "RecoveryCleanupPosture",
        "handoff/cleanup_posture",
        "phase-7",
    ),
    ("RecoveryReportEnvelope", "observation/report", "phase-8"),
    (
        "RecoveryObserverReport",
        "worth-store-offline-verifier/c8-recovery-observation/report",
        "phase-8",
    ),
    (
        "StoreRecoveryBindingFreshnessSample",
        "worth-store/recovery-freshness/binding",
        "phase-4",
    ),
    (
        "StoreRecoveryCleanupFreshnessSample",
        "worth-store/recovery-freshness/cleanup",
        "phase-7",
    ),
];

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
        .chain(BACKEND_RECOVERY_PUBLICATION_SURFACES)
        .chain(STORE_RECOVERY_COORDINATION_SURFACES)
        .chain(STORE_RECOVERY_PUBLICATION_SURFACES)
        .chain(SUPPORTING_DELIVERY_SURFACES)
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

fn parse_inventory(document: &str) -> Result<Vec<ApiRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.8 API inventory has an invalid schema".into());
    }
    lines
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| {
            let columns = split_csv(line, 6)
                .map_err(|error| format!("C.8 API row {}: {error}", index + 2))?;
            Ok(ApiRow {
                scope: columns[0].to_owned(),
                surface: columns[1].to_owned(),
                source_owner: columns[2].to_owned(),
                disposition: columns[3].to_owned(),
                destination_owner: columns[4].to_owned(),
                phase: columns[5].to_owned(),
            })
        })
        .collect()
}

pub(super) struct ApiRow {
    scope: String,
    surface: String,
    source_owner: String,
    disposition: String,
    destination_owner: String,
    phase: String,
}
