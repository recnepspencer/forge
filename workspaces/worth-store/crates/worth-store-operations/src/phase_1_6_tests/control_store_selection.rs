use super::support::*;

#[test]
fn only_the_exact_control_media_and_generation_selected_by_fencing_can_be_current() {
    let directory = tempfile::tempdir().expect("temp directory");
    let first = control_store(directory.path().join("first.log"), "control-a");
    let second = control_store(directory.path().join("second.log"), "control-b");
    let operation = OperationalOperationId::new("divergent-control").expect("operation");
    let authority = crate::backup::export::current_authority("s10-divergent-control");
    let opened = OperationalControlRecord::workflow_opened(
        authority.authority_identity(),
        operation.clone(),
        OperationalTransitionId::new("divergent-control:opened").expect("transition"),
        OperationalWorkflowKind::OfflineInspection,
    );
    first.append(&opened).expect("first append");
    second.append(&opened).expect("second append");
    let selection = TestControlStoreFencingProvider::selected(
        &authority,
        &first,
        ControlStoreGeneration::initial(),
    );
    let fencing = ControlStoreFencingAuthority::for_current_store(&authority, &selection);
    assert!(matches!(
        inspect_control_store_copies(&[&first, &second], &fencing),
        ControlStoreTrustPosture::Selected(_)
    ));

    first
        .append(&OperationalControlRecord::workflow_opened(
            authority.authority_identity(),
            OperationalOperationId::new("divergent-control-extra").expect("second operation"),
            OperationalTransitionId::new("divergent-control:receipt").expect("transition"),
            OperationalWorkflowKind::OfflineInspection,
        ))
        .expect("advance only one copy");
    assert!(matches!(
        inspect_control_store_copies(&[&first, &second], &fencing),
        ControlStoreTrustPosture::Indeterminate(
            crate::ControlStoreSelectionIndeterminate::SelectedGenerationNotReadable { .. }
        )
    ));

    let selected_stale_copy = TestControlStoreFencingProvider::selected(
        &authority,
        &second,
        ControlStoreGeneration::initial(),
    );
    let stale_fencing =
        ControlStoreFencingAuthority::for_current_store(&authority, &selected_stale_copy);
    assert!(matches!(
        inspect_control_store_copies(&[&first, &second], &stale_fencing),
        ControlStoreTrustPosture::Selected(_)
    ));
}

#[test]
fn unavailable_or_unsupported_fencing_never_falls_back_to_a_readable_generation() {
    let directory = tempfile::tempdir().expect("temp directory");
    let control = control_store(directory.path().join("control.log"), "control");
    let authority = crate::backup::export::current_authority("s10-fencing-unavailable");

    for provider in [
        TestControlStoreFencingProvider::Unavailable,
        TestControlStoreFencingProvider::Unsupported,
    ] {
        let fencing = ControlStoreFencingAuthority::for_current_store(&authority, &provider);
        assert!(matches!(
            control.inspect_generations(&fencing),
            ControlStoreTrustPosture::Unavailable(
                crate::ControlStoreAvailabilityDenial::FencingUnavailable
                    | crate::ControlStoreAvailabilityDenial::FencingUnsupported
            )
        ));
    }
}

#[test]
fn a_selection_bound_to_another_current_authority_is_unavailable() {
    let directory = tempfile::tempdir().expect("temp directory");
    let control = control_store(directory.path().join("control.log"), "control");
    let first_authority = crate::backup::export::current_authority("s10-selection-authority-a");
    let other_authority = crate::backup::export::current_authority("s10-selection-authority-b");
    control
        .append(&OperationalControlRecord::workflow_opened(
            first_authority.authority_identity(),
            OperationalOperationId::new("selection-authority-a").expect("operation"),
            OperationalTransitionId::new("selection-authority-a:opened").expect("transition"),
            OperationalWorkflowKind::Backup,
        ))
        .expect("first authority history");
    let provider = TestControlStoreFencingProvider::selected(
        &first_authority,
        &control,
        ControlStoreGeneration::initial(),
    );
    let fencing = ControlStoreFencingAuthority::for_current_store(&other_authority, &provider);

    assert!(matches!(
        control.inspect_generations(&fencing),
        ControlStoreTrustPosture::Unavailable(
            crate::ControlStoreAvailabilityDenial::FencingUnavailable
        )
    ));
}

#[test]
fn selected_control_history_from_another_store_authority_is_indeterminate() {
    let directory = tempfile::tempdir().expect("temp directory");
    let control = control_store(directory.path().join("control.log"), "control");
    let source_authority = crate::backup::export::current_authority("s10-control-source");
    let selected_authority = crate::backup::export::current_authority("s10-control-selected");
    let record = OperationalControlRecord::workflow_opened(
        source_authority.authority_identity(),
        OperationalOperationId::new("cross-authority-control").expect("operation"),
        OperationalTransitionId::new("cross-authority-control:opened").expect("transition"),
        OperationalWorkflowKind::Backup,
    );
    control.append(&record).expect("source authority record");
    let provider = TestControlStoreFencingProvider::selected(
        &selected_authority,
        &control,
        ControlStoreGeneration::initial(),
    );
    let fencing = ControlStoreFencingAuthority::for_current_store(&selected_authority, &provider);

    assert!(matches!(
        control.inspect_generations(&fencing),
        ControlStoreTrustPosture::Indeterminate(
            crate::ControlStoreSelectionIndeterminate::SelectedAuthorityMismatch { .. }
        )
    ));
}

#[test]
fn selected_media_cannot_be_impersonated_by_a_valid_journal_replacement_at_the_same_path() {
    let directory = tempfile::tempdir().expect("temp directory");
    let path = directory.path().join("control.log");
    let displaced = directory.path().join("displaced.log");
    let control = control_store(path.clone(), "control");
    let authority = crate::backup::export::current_authority("s10-control-replacement");
    control
        .append(&OperationalControlRecord::workflow_opened(
            authority.authority_identity(),
            OperationalOperationId::new("control-replacement").expect("operation"),
            OperationalTransitionId::new("control-replacement:opened").expect("transition"),
            OperationalWorkflowKind::Backup,
        ))
        .expect("selected history");
    let provider = TestControlStoreFencingProvider::selected(
        &authority,
        &control,
        ControlStoreGeneration::initial(),
    );
    let fencing = ControlStoreFencingAuthority::for_current_store(&authority, &provider);
    let valid_history = std::fs::read(&path).expect("valid selected history");

    std::fs::rename(&path, displaced).expect("retain selected physical media");
    std::fs::write(&path, valid_history).expect("replace configured journal path");
    assert!(matches!(
        control.inspect_generations(&fencing),
        ControlStoreTrustPosture::Unavailable(crate::ControlStoreAvailabilityDenial::Media(
            worth_store_physical_backend::ControlMediaFault::ControlMediaIdentityChanged { .. }
        ))
    ));

    drop(control);
    let replacement = control_store(path, "control");
    assert!(matches!(
        replacement.inspect_generations(&fencing),
        ControlStoreTrustPosture::Indeterminate(
            crate::ControlStoreSelectionIndeterminate::SelectedMediaUnavailable { .. }
        )
    ));
}

#[test]
fn selected_generation_is_bound_to_the_exact_history_prefix_not_just_its_number() {
    let directory = tempfile::tempdir().expect("temp directory");
    let selected_path = directory.path().join("selected.log");
    let divergent_path = directory.path().join("divergent.log");
    let selected = control_store(selected_path.clone(), "selected-control");
    let divergent = control_store(divergent_path.clone(), "divergent-control");
    let authority = crate::backup::export::current_authority("s10-prefix-selection");
    selected
        .append(&OperationalControlRecord::workflow_opened(
            authority.authority_identity(),
            OperationalOperationId::new("selected-history").expect("operation"),
            OperationalTransitionId::new("selected-history:opened").expect("transition"),
            OperationalWorkflowKind::Backup,
        ))
        .expect("selected history");
    divergent
        .append(&OperationalControlRecord::workflow_opened(
            authority.authority_identity(),
            OperationalOperationId::new("divergent-history").expect("operation"),
            OperationalTransitionId::new("divergent-history:opened").expect("transition"),
            OperationalWorkflowKind::Backup,
        ))
        .expect("same-generation divergent history");
    let provider = TestControlStoreFencingProvider::selected(
        &authority,
        &selected,
        ControlStoreGeneration::initial(),
    );
    let fencing = ControlStoreFencingAuthority::for_current_store(&authority, &provider);

    let divergent_bytes = std::fs::read(divergent_path).expect("divergent journal bytes");
    std::fs::write(selected_path, divergent_bytes)
        .expect("replace content without replacing selected media identity");
    assert!(matches!(
        selected.inspect_generations(&fencing),
        ControlStoreTrustPosture::Indeterminate(
            crate::ControlStoreSelectionIndeterminate::SelectedPrefixDigestMismatch { .. }
        )
    ));
}

#[test]
fn byte_valid_materialization_before_workflow_open_is_indeterminate() {
    let scenario = BackupScenario::new("out-of-order-materialization-open");
    let control = scenario.control_store();
    let authority = crate::backup::export::current_authority("s10-invalid-history-plan");
    let operation = OperationalOperationId::new("invalid-history-plan").expect("operation");
    let plan =
        crate::BackupMaterializationRecoveryPlan::prepare([0x71; 32], &scenario.target, 4096)
            .expect("materialization plan");
    control
        .append(&OperationalControlRecord::backup_materialization_opened(
            authority.authority_identity(),
            operation.clone(),
            OperationalTransitionId::new("invalid-history-plan:materialization-opened")
                .expect("transition"),
            plan,
        ))
        .expect("byte-valid out-of-order record");
    let provider = TestControlStoreFencingProvider::selected(
        &authority,
        &control,
        ControlStoreGeneration::initial(),
    );
    let fencing = ControlStoreFencingAuthority::for_current_store(&authority, &provider);

    match control.inspect_generations(&fencing) {
        ControlStoreTrustPosture::Indeterminate(
            crate::ControlStoreSelectionIndeterminate::InvalidHistory(violation),
        ) => {
            assert_eq!(violation.record_index(), 0);
            assert_eq!(violation.operation_id(), &operation);
            assert_eq!(
                violation.kind(),
                &crate::OperationalControlHistoryViolationKind::RecordBeforeWorkflowOpen
            );
        }
        posture => panic!("out-of-order semantic history became usable: {posture:?}"),
    }
}

#[test]
fn byte_valid_materialization_receipt_without_a_source_lease_is_indeterminate() {
    let directory = tempfile::tempdir().expect("temp directory");
    let control = control_store(directory.path().join("control.log"), "control");
    let authority = crate::backup::export::current_authority("s10-invalid-history-receipt");
    let operation = OperationalOperationId::new("invalid-history-receipt").expect("operation");
    control
        .append(&OperationalControlRecord::workflow_opened(
            authority.authority_identity(),
            operation.clone(),
            OperationalTransitionId::new("invalid-history-receipt:opened").expect("transition"),
            OperationalWorkflowKind::Backup,
        ))
        .expect("workflow open");
    control
        .append(&OperationalControlRecord::from_persisted_parts(
            authority.authority_identity(),
            operation.clone(),
            OperationalTransitionId::new("invalid-history-receipt:materialized")
                .expect("transition"),
            crate::OperationalControlRecordKind::BackupMaterializationRecorded {
                manifest_digest: [0x81; 32],
            },
        ))
        .expect("byte-valid receipt");
    let provider = TestControlStoreFencingProvider::selected(
        &authority,
        &control,
        ControlStoreGeneration::from_raw(2).expect("second generation"),
    );
    let fencing = ControlStoreFencingAuthority::for_current_store(&authority, &provider);

    assert!(matches!(
        control.inspect_generations(&fencing),
        ControlStoreTrustPosture::Indeterminate(
            crate::ControlStoreSelectionIndeterminate::InvalidHistory(violation)
        ) if violation.operation_id() == &operation
            && violation.kind()
                == &crate::OperationalControlHistoryViolationKind::MaterializationReceiptBeforePlan
    ));
}

fn control_store(path: std::path::PathBuf, failure_domain_id: &str) -> OperationalControlStore {
    OperationalControlStore::open(
        OperationalControlLocation::new(path, failure_domain(failure_domain_id)),
        std::iter::empty::<ProtectedOperationalMediaLocation>(),
    )
    .expect("control store")
}
