pub(super) mod disposition_contract;
mod reachable_api;

use std::collections::{BTreeMap, BTreeSet};

use super::documents::{read_repository_document, split_csv, API_INVENTORY};
use disposition_contract::expected_current_disposition;
use reachable_api::current_facade;

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
        "PhysicalRecoveryPlatformAuthority",
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
        "DiscoveredPhysicalRecovery",
        "progression/discovered",
        "phase-3",
    ),
    (
        "SelectedPhysicalRecovery",
        "progression/selected",
        "phase-3",
    ),
    ("PlannedPhysicalRecovery", "progression/planned", "phase-4"),
    ("StagedPhysicalRecovery", "progression/staged", "phase-5"),
    (
        "NamespaceDurablePhysicalRecovery",
        "progression/published",
        "phase-6",
    ),
    (
        "ReopenedPhysicalRecovery",
        "progression/reopened",
        "phase-6",
    ),
    (
        "RecoveredPhysicalRuntimeHandoff",
        "worth-store/recovery-construction/handoff",
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
        "PhysicalRecoveryFreshnessPort",
        "worth-store/recovery-freshness/port",
        "phase-2",
    ),
    (
        "PhysicalRecoveryFreshnessPort::sample_binding",
        "worth-store/recovery-freshness/port",
        "phase-4",
    ),
    (
        "PhysicalRecoveryFreshnessPort::sample_cleanup",
        "worth-store/recovery-freshness/port",
        "phase-7",
    ),
    (
        "PhysicalRecoveryConstructionAuthority",
        "worth-store/recovery-construction/authority",
        "phase-6",
    ),
    ("PhysicalRecoveryRefusal", "entry/outcome", "phase-2"),
    ("PhysicalRecoveryBlock", "entry/outcome", "phase-3"),
    (
        "PhysicalRecoveryPublicationIndeterminate",
        "entry/outcome",
        "phase-6",
    ),
    (
        "RecoveryOperationFateSet",
        "handoff/operation-fates",
        "phase-4",
    ),
    (
        "RecoveryCleanupPosture",
        "handoff/cleanup-posture",
        "phase-7",
    ),
    ("RecoveryReportEnvelope", "observation/report", "phase-6"),
    (
        "RecoveryObserverReport",
        "worth-store-offline-verifier/c8-recovery-observation/report",
        "phase-6",
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
    for cross_file_surface in [
        "RecoveryCompletion::execute_publication_recovery_replay",
        "IntegrityDamageMap::admit_corruption_readmission",
        "RecoveryPhysicsTimelineAuthority::resolve_candidates",
        "layout_projection::BoundedWalTailLayoutReport::lookup_tail_range",
    ] {
        assert!(
            actual
                .iter()
                .any(|(_, surface, _)| surface == cross_file_surface),
            "cross-file public impl `{cross_file_surface}` escaped API discovery"
        );
    }
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
    assert_eq!(
        destination,
        DESTINATION_SURFACES.iter().copied().collect(),
        "C.8 destination facade inventory is not exact"
    );
}

#[test]
fn dispositions_name_one_real_destination_owner() {
    let document = read_repository_document(API_INVENTORY).expect("read C.8 API inventory");
    let rows = parse_inventory(&document).expect("parse C.8 API inventory");
    let mut surfaces = BTreeMap::new();
    for row in rows {
        assert!(
            surfaces
                .insert((row.scope.clone(), row.surface.clone()), ())
                .is_none(),
            "duplicate C.8 API row for {} {}",
            row.scope,
            row.surface
        );
        match row.scope.as_str() {
            "current" | "current-certification" => assert_eq!(
                (
                    row.disposition.as_str(),
                    row.destination_owner.as_str(),
                    row.phase.as_str(),
                ),
                expected_current_disposition(&row.source_owner, &row.surface),
                "wrong C.8 disposition for {}",
                row.surface
            ),
            "destination" => {
                assert_eq!(row.disposition, "create");
                let expected_owner = if row.source_owner.starts_with("worth-") {
                    row.source_owner.clone()
                } else {
                    format!("worth-store-recovery-runtime/{}", row.source_owner)
                };
                assert_eq!(row.destination_owner, expected_owner);
            }
            other => panic!("unknown C.8 API scope `{other}`"),
        }
        assert_valid_owner(&row.destination_owner, row.disposition == "delete");
        assert!(matches!(
            row.phase.as_str(),
            "phase-2"
                | "phase-3"
                | "phase-4"
                | "phase-5"
                | "phase-6"
                | "phase-7"
                | "phase-8"
                | "phase-9"
        ));
    }
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

fn assert_valid_owner(owner: &str, deletion: bool) {
    if deletion {
        assert_eq!(owner, "none");
        return;
    }
    assert_ne!(owner, "none");
    let leaf = owner.rsplit('/').next().unwrap_or(owner);
    assert!(
        !matches!(
            leaf,
            "recovery" | "physics" | "support" | "evidence" | "utility"
        ),
        "generic C.8 API destination owner `{owner}`"
    );
}

struct ApiRow {
    scope: String,
    surface: String,
    source_owner: String,
    disposition: String,
    destination_owner: String,
    phase: String,
}
