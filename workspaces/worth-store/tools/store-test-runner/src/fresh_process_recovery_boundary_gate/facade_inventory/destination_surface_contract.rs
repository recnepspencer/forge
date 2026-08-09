pub(super) const BACKEND_RECOVERY_SURFACES: &[(&str, &str, &str)] = &[
    (
        "PhysicalRecoveryMediaGeneration",
        "worth-store-physical-backend/recovery-media/generation",
        "phase-2",
    ),
    (
        "QualifiedPhysicalBackendProfile",
        "worth-store-physical-backend/recovery-media/profile",
        "phase-2",
    ),
    (
        "QualifiedRecoveryFilesystemMedia",
        "worth-store-physical-backend/recovery-media/qualified",
        "phase-2",
    ),
    (
        "QualifiedRecoveryFilesystemMedia::qualify_existing",
        "worth-store-physical-backend/recovery-media/qualified",
        "phase-2",
    ),
    (
        "QualifiedRecoveryFilesystemMedia::backend_profile",
        "worth-store-physical-backend/recovery-media/qualified",
        "phase-2",
    ),
    (
        "QualifiedRecoveryFilesystemMedia::media_generation",
        "worth-store-physical-backend/recovery-media/qualified",
        "phase-2",
    ),
    (
        "QualifiedRecoveryFilesystemMedia::admit_persisted_store",
        "worth-store-physical-backend/recovery-media/qualified",
        "phase-2",
    ),
    (
        "RecoveryFilesystemQualificationError",
        "worth-store-physical-backend/recovery-media/qualification",
        "phase-2",
    ),
    (
        "AdmittedRecoveryFilesystemMedia",
        "worth-store-physical-backend/recovery-media/admitted",
        "phase-2",
    ),
    (
        "AdmittedRecoveryFilesystemMedia::store_identity",
        "worth-store-physical-backend/recovery-media/admitted",
        "phase-2",
    ),
    (
        "AdmittedRecoveryFilesystemMedia::bounded_discovery",
        "worth-store-physical-backend/recovery-media/admitted",
        "phase-2",
    ),
    (
        "BoundedRecoveryFilesystemDiscovery",
        "worth-store-physical-backend/recovery-media/discovery",
        "phase-2",
    ),
];

#[test]
fn backend_recovery_facade_is_complete_and_store_identity_is_post_admission() {
    use std::collections::BTreeSet;

    use super::super::documents::{
        read_repository_document, split_csv, API_INVENTORY, DESTINATION_TOPOLOGY,
    };

    let api = read_repository_document(API_INVENTORY).expect("read C.8 API inventory");
    let rows = super::parse_inventory(&api).expect("parse C.8 API inventory");
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
        ])
    );
}
