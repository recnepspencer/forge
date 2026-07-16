use super::current_root_fixture::*;
use super::support::*;

#[test]
fn phase_one_ledger_is_inert_complete_and_records_the_hard_cutover() {
    let ledger = OperationalRecoveryBoundaryLedger::current();
    assert!(
        ledger
            .entries()
            .iter()
            .all(|entry| !entry.authority_owner.is_empty()
                && !entry.construction_authority.is_empty())
    );
    assert!(ledger
        .entries()
        .iter()
        .any(|entry| entry.artifact == "blob custody"
            && entry.mutation_owner == "worth-store-blob-chunks"));
    assert!(ledger
        .shared_vocabulary()
        .entries()
        .iter()
        .all(|entry| !entry.reverse_flow_denial.is_empty()));
    assert!(CurrentRecoverySurfaceGapReport::current()
        .gaps()
        .iter()
        .any(|gap| gap.surface == "behavioral operations vocabulary"));
}

#[test]
fn production_current_root_source_protects_one_page_allocation_for_multiple_slots() {
    let scenario = BackupScenario::new("runtime-root-closure");
    let fixture = current_root_fixture_with_shared_page();
    let source = fixture.source();
    assert_eq!(source.manifest().page_slots().len(), 2);
    assert_eq!(source.page_cells().len(), 1);

    let mut artifacts = scenario
        .references()
        .iter()
        .filter(|artifact| {
            !matches!(
                artifact.family(),
                BackupArtifactFamily::RootManifest
                    | BackupArtifactFamily::Page
                    | BackupArtifactFamily::Extent
            )
        })
        .cloned()
        .collect::<Vec<_>>();
    artifacts.extend(core_references_for_source(&fixture, &scenario.source));
    let manifest = BackupCutManifest::from_current_root_source(source, artifacts.clone())
        .expect("runtime-issued reachability must admit");
    assert_eq!(
        manifest
            .artifacts()
            .iter()
            .filter(|artifact| artifact.family() == BackupArtifactFamily::Page)
            .count(),
        1
    );

    let coordinates = BackupCutCoordinates::new(
        "lineage-a",
        source.manifest().root_publication().generation().get(),
        1,
        scenario.checkpoint_identity(),
        10,
        10,
        12,
        12,
        "worth-physical-format-v1",
        "posix-file-fsync-dir-sync",
    )
    .expect("runtime cut coordinates");
    let authority = crate::backup::export::current_authority("store.physical.default_instance");
    let admitted = OnlineBackupIntent::new(
        OperationalOperationId::new("runtime-root-source-admission").expect("operation"),
        coordinates,
        manifest,
        backup_custody(&authority),
    )
    .admit_cut(&authority, &scenario.control_store(), &scenario.leases)
    .expect("current runtime authority must admit its own physical source");
    assert_eq!(
        admitted.cut().authority_identity(),
        source.store_authority_identity()
    );

    let page = artifacts
        .iter()
        .position(|artifact| artifact.family() == BackupArtifactFamily::Page)
        .expect("page artifact");
    artifacts.remove(page);
    assert!(matches!(
        BackupCutManifest::from_current_root_source(source, artifacts),
        Err(BackupCutManifestDenial::MissingOwnerReachability)
    ));
}

#[test]
fn backup_cut_manifest_rejects_two_names_for_one_physical_allocation() {
    let directory = tempfile::tempdir().expect("temp directory");
    let first_path = directory.path().join("first.page");
    let alias_path = directory.path().join("alias.extent");
    std::fs::write(&first_path, b"shared-allocation").expect("media");
    std::fs::hard_link(&first_path, &alias_path).expect("hard-link alias");
    let first = BackupArtifactReference::declare_untrusted_physical_observation(
        BackupArtifactFamily::Page,
        artifact_format(BackupArtifactFamily::Page),
        "page-a",
        1,
        artifact_coverage(BackupArtifactFamily::Page),
        observe_physical_backup_artifact(first_path, 4).expect("first observation"),
        reclaim_reference(BackupArtifactFamily::Page, 20),
    )
    .expect("reference");
    let alias = BackupArtifactReference::declare_untrusted_physical_observation(
        BackupArtifactFamily::Extent,
        artifact_format(BackupArtifactFamily::Extent),
        "extent-a",
        1,
        artifact_coverage(BackupArtifactFamily::Extent),
        observe_physical_backup_artifact(alias_path, 4).expect("alias observation"),
        reclaim_reference(BackupArtifactFamily::Extent, 21),
    )
    .expect("reference");
    assert!(matches!(
        BackupCutManifest::canonical([first, alias]),
        Err(BackupCutManifestDenial::DuplicatePhysicalArtifact)
    ));
}

#[test]
fn artifact_family_cannot_be_paired_with_an_unrelated_reclaim_domain() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("page.media");
    std::fs::write(&path, b"page").expect("media");
    assert!(
        BackupArtifactReference::declare_untrusted_physical_observation(
            BackupArtifactFamily::Page,
            artifact_format(BackupArtifactFamily::Page),
            "page-a",
            1,
            BackupArtifactCoverage::physical_reachability(),
            observe_physical_backup_artifact(path, 4).expect("observation"),
            reclaim_reference(BackupArtifactFamily::Extent, 8),
        )
        .is_none()
    );
}

#[test]
fn cut_identity_binds_the_exact_physical_lease_owners() {
    let scenario = BackupScenario::new("lease-owner-identity");
    let authority = crate::backup::export::current_authority("s10-lease-owner-identity");
    let control = scenario.control_store();
    let first = OnlineBackupIntent::new(
        OperationalOperationId::new("backup-owner-one").expect("operation"),
        scenario.coordinates(),
        scenario.cut_manifest(),
        backup_custody(&authority),
    )
    .admit_cut(&authority, &control, &scenario.leases)
    .expect("first cut");

    let mut changed = scenario.references().to_vec();
    let page_index = changed
        .iter()
        .position(|artifact| artifact.family() == BackupArtifactFamily::Page)
        .expect("page artifact");
    let page = &changed[page_index];
    let changed_owner = reclaim_reference(BackupArtifactFamily::Page, 99);
    let changed_path = scenario.source.join("changed-owner.page");
    super::backup_artifact_fixture::copy_page_to_owner(
        page.source_path(),
        &changed_path,
        changed_owner,
    );
    changed[page_index] = BackupArtifactReference::declare_untrusted_physical_observation(
        BackupArtifactFamily::Page,
        page.format(),
        page.identity(),
        page.generation(),
        page.coverage().clone(),
        observe_physical_backup_artifact(changed_path, 5).expect("observation"),
        changed_owner,
    )
    .expect("changed physical owner");
    let second = OnlineBackupIntent::new(
        OperationalOperationId::new("backup-owner-two").expect("operation"),
        scenario.coordinates(),
        BackupCutManifest::canonical(changed).expect("changed manifest"),
        backup_custody(&authority),
    )
    .admit_cut(&authority, &control, &scenario.leases)
    .expect("second cut");
    assert_ne!(first.cut().identity(), second.cut().identity());
}

#[test]
fn cut_identity_binds_acknowledged_frontier_and_rejects_unproven_profiles() {
    let scenario = BackupScenario::new("cut-coordinate-identity");
    let authority = crate::backup::export::current_authority("s10-cut-coordinate-identity");
    let control = scenario.control_store();
    let admit = |operation: &str, acknowledged_frontier, format, backend| {
        OnlineBackupIntent::new(
            OperationalOperationId::new(operation).expect("operation"),
            BackupCutCoordinates::new(
                "lineage-a",
                1,
                1,
                scenario.checkpoint_identity(),
                10,
                10,
                12,
                acknowledged_frontier,
                format,
                backend,
            )
            .expect("coordinates"),
            scenario.cut_manifest(),
            backup_custody(&authority),
        )
        .admit_cut(&authority, &control, &scenario.leases)
        .expect("cut")
        .cut()
        .identity()
    };

    let baseline = admit(
        "cut-coordinate-baseline",
        12,
        "worth-physical-format-v1",
        "posix-file-fsync-dir-sync",
    );
    let later_ack = admit(
        "cut-coordinate-ack",
        13,
        "worth-physical-format-v1",
        "posix-file-fsync-dir-sync",
    );

    assert_ne!(baseline, later_ack);

    for (operation, format, backend, expected) in [
        (
            "cut-coordinate-format",
            "invented-format",
            "posix-file-fsync-dir-sync",
            BackupCutAdmissionDenial::FormatProfileMismatch,
        ),
        (
            "cut-coordinate-backend",
            "worth-physical-format-v1",
            "invented-backend",
            BackupCutAdmissionDenial::BackendProfileMismatch,
        ),
    ] {
        let coordinates = BackupCutCoordinates::new(
            "lineage-a",
            1,
            1,
            scenario.checkpoint_identity(),
            10,
            10,
            12,
            12,
            format,
            backend,
        )
        .expect("individually representable coordinates");
        let denial = OnlineBackupIntent::new(
            OperationalOperationId::new(operation).expect("operation"),
            coordinates,
            scenario.cut_manifest(),
            backup_custody(&authority),
        )
        .admit_cut(&authority, &control, &scenario.leases)
        .expect_err("unproven profile must fail before source admission");
        assert!(matches!(denial, OnlineBackupAdmissionDenial::Cut(actual) if actual == expected));
    }
}

#[test]
fn source_replacement_after_cut_admission_fails_before_output_allocation() {
    let scenario = BackupScenario::new("source-replacement");
    let authority = crate::backup::export::current_authority("s10-source-replacement");
    let control = scenario.control_store();
    let admitted = OnlineBackupIntent::new(
        OperationalOperationId::new("backup-source-replacement").expect("operation"),
        scenario.coordinates(),
        scenario.cut_manifest(),
        backup_custody(&authority),
    )
    .admit_cut(&authority, &control, &scenario.leases)
    .expect("cut");
    let source = scenario.references()[0].source_path().to_path_buf();
    let identical_bytes = std::fs::read(&source).expect("source bytes");
    std::fs::remove_file(&source).expect("remove admitted allocation");
    std::fs::write(&source, identical_bytes).expect("replace with identical bytes");
    assert!(matches!(
        admitted.materialize(&scenario.target, 7, &control),
        Err(BackupMaterializationDenial::Physical(
            worth_store_physical_backend::PhysicalBackupMaterializationDenial::SourceIdentityMismatch { .. }
        ))
    ));
    assert_eq!(
        std::fs::read_dir(&scenario.target).expect("target").count(),
        0
    );
}

#[test]
fn backup_cut_admission_rejects_manifest_coordinates_that_name_another_root() {
    let scenario = BackupScenario::new("root-mismatch");
    let authority = crate::backup::export::current_authority("s10-root-mismatch");
    let control = scenario.control_store();
    let mismatched = BackupCutCoordinates::new(
        "lineage-a",
        2,
        1,
        scenario.checkpoint_identity(),
        10,
        10,
        12,
        12,
        "worth-physical-format-v1",
        "posix-file-fsync-dir-sync",
    )
    .expect("individually valid coordinates");

    let operation = OperationalOperationId::new("backup-root-mismatch").expect("operation");
    let denial = OnlineBackupIntent::new(
        operation.clone(),
        mismatched,
        scenario.cut_manifest(),
        backup_custody(&authority),
    )
    .admit_cut(&authority, &control, &scenario.leases)
    .expect_err("cross-generation cut must fail");
    assert!(matches!(
        denial,
        OnlineBackupAdmissionDenial::Cut(BackupCutAdmissionDenial::RootGenerationMismatch)
    ));
    assert_eq!(
        control
            .observe_selection_coordinates()
            .expect("control observation"),
        None,
        "a rejected cut must not durably open an unrecoverable workflow"
    );

    let admitted = OnlineBackupIntent::new(
        operation,
        scenario.coordinates(),
        scenario.cut_manifest(),
        backup_custody(&authority),
    )
    .admit_cut(&authority, &control, &scenario.leases)
    .expect("the same operation identity remains usable after preflight denial");
    admitted
        .abandon("test closeout", &control, &scenario.leases)
        .expect("durable closeout");
}

#[test]
fn custody_admitted_by_another_store_authority_cannot_open_a_backup_cut() {
    let scenario = BackupScenario::new("foreign-custody-authority");
    let current = crate::backup::export::current_authority("s10-current-custody-authority");
    let foreign = crate::backup::export::current_authority("s10-foreign-custody-authority");
    let control = scenario.control_store();

    let denial = OnlineBackupIntent::new(
        OperationalOperationId::new("foreign-custody-authority").expect("operation"),
        scenario.coordinates(),
        scenario.cut_manifest(),
        backup_custody(&foreign),
    )
    .admit_cut(&current, &control, &scenario.leases)
    .expect_err("scope labels cannot launder another Store's custody authority");

    assert!(matches!(
        denial,
        OnlineBackupAdmissionDenial::Cut(BackupCutAdmissionDenial::SecurityScopeAuthorityMismatch)
    ));
    assert_eq!(
        control
            .observe_selection_coordinates()
            .expect("control observation"),
        None
    );
}
