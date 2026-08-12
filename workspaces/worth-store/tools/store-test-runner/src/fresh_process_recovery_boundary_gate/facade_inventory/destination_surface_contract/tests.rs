use std::collections::BTreeSet;

use super::super::super::documents::{
    read_repository_document, split_csv, API_INVENTORY, DESTINATION_TOPOLOGY,
};
use super::super::parse_inventory;

#[test]
fn backend_recovery_facade_is_complete_and_store_identity_is_post_admission() {
    let api = read_repository_document(API_INVENTORY).expect("read C.8 API inventory");
    let rows = parse_inventory(&api).expect("parse C.8 API inventory");
    let api_owners = rows
        .iter()
        .filter(|row| row.scope == "destination")
        .map(|row| row.source_owner.as_str())
        .collect::<BTreeSet<_>>();
    let topology = read_repository_document(DESTINATION_TOPOLOGY).expect("read C.8 topology");
    let backend_owners = topology
        .lines()
        .skip(1)
        .map(|line| split_csv(line, 6).expect("parse topology row"))
        .filter(|columns| {
            columns[0].contains("physical-backend/src/recovery_media/")
                && !columns[0].ends_with("/mod.rs")
        })
        .map(|columns| format!("worth-store-{}", columns[1]))
        .collect::<BTreeSet<_>>();
    assert!(backend_owners
        .iter()
        .all(|owner| api_owners.contains(owner.as_str())));
    let store_identity_surfaces = rows
        .iter()
        .filter(|row| row.scope == "destination" && row.surface.ends_with("::store_identity"))
        .map(|row| row.surface.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        store_identity_surfaces,
        BTreeSet::from([
            "AdmittedPhysicalRecovery::store_identity",
            "AdmittedRecoveryFilesystemMedia::store_identity",
            "BoundedRecoveryFilesystemDiscovery::store_identity",
            "DurableRootSelector::store_identity",
            "NamespaceDurablePhysicalRecovery::store_identity",
            "PhysicalRecoveryBlock::store_identity",
            "PhysicalRecoveryPublicationIndeterminate::store_identity",
            "PlannedPhysicalRecovery::store_identity",
            "RecoveredPhysicalRuntimeCore::store_identity",
            "RecoveryPublicationExpectation::store_identity",
            "RecoveryReportEnvelope::store_identity",
            "StagedPhysicalRecovery::store_identity",
            "RecoveryPublicationPlan::store_identity",
            "ReopenedPhysicalRecovery::store_identity",
            "SelectedPhysicalRecovery::store_identity",
            "StoreRecoveryBindingFreshnessSample::store_identity",
            "StoreRecoveryCleanupFreshnessSample::store_identity",
        ])
    );
}
