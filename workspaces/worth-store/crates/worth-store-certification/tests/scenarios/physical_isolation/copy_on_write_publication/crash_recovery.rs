use worth_store_physical_isolation::{
    PhysicalPublicationDenial, PhysicalPublicationIntent, PublicationCrashRecoveryOutcome,
};
use worth_store_recovery_physics::{PublicationCrashStage, RecoveredPublicationStructureKind};

use super::publication_support::{
    execute_mixed_tree_recovery_replay, execute_publication_recovery_replay, publication_inputs,
    publication_inputs_with_new_root_digest, publish_copy_on_write,
};

#[test]
fn crash_matrix_recovers_old_or_new_stable_structure_never_mixed_tree() {
    let inputs = publication_inputs();
    let receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            inputs.old_candidate,
            inputs.new_candidate,
            inputs.old_reachability,
        ),
        inputs.new_validation,
        None,
    );

    let before = PublicationCrashRecoveryOutcome::admit_recovery_receipt(
        &receipt,
        execute_publication_recovery_replay(PublicationCrashStage::BeforePublication),
    )
    .unwrap();
    assert_eq!(
        before.recovered().kind(),
        RecoveredPublicationStructureKind::OldStableStructure
    );
    assert_eq!(
        before.recovered().stable_root_epoch(),
        Some(inputs.old_root.epoch().get())
    );

    let during = PublicationCrashRecoveryOutcome::admit_recovery_receipt(
        &receipt,
        execute_publication_recovery_replay(PublicationCrashStage::DuringPublication),
    )
    .unwrap();
    assert_eq!(
        during.recovered().kind(),
        RecoveredPublicationStructureKind::NewStableStructure
    );
    assert_eq!(
        during.recovered().stable_root_epoch(),
        Some(inputs.new_root.epoch().get())
    );

    let after = PublicationCrashRecoveryOutcome::admit_recovery_receipt(
        &receipt,
        execute_publication_recovery_replay(PublicationCrashStage::AfterPublication),
    )
    .unwrap();
    assert_eq!(
        after.recovered().kind(),
        RecoveredPublicationStructureKind::NewStableStructure
    );
    assert_eq!(
        after.recovered().stable_root_epoch(),
        Some(inputs.new_root.epoch().get())
    );

    assert_eq!(
        PublicationCrashRecoveryOutcome::admit_recovery_receipt(
            &receipt,
            execute_mixed_tree_recovery_replay(),
        )
        .unwrap_err(),
        PhysicalPublicationDenial::MixedTreeAfterCrash
    );
}

#[test]
fn recovery_receipt_binds_to_each_publication_receipt_roots() {
    let first = publication_inputs_with_new_root_digest("s5-phase7-new-root-a", 721);
    let second = publication_inputs_with_new_root_digest("s5-phase7-new-root-b", 722);
    let first_receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            first.old_candidate,
            first.new_candidate,
            first.old_reachability,
        ),
        first.new_validation,
        None,
    );
    let second_receipt = publish_copy_on_write(
        PhysicalPublicationIntent::copy_on_write_root_manifest(
            second.old_candidate,
            second.new_candidate,
            second.old_reachability,
        ),
        second.new_validation,
        None,
    );
    let recovery_receipt =
        execute_publication_recovery_replay(PublicationCrashStage::AfterPublication);

    let first_outcome =
        PublicationCrashRecoveryOutcome::admit_recovery_receipt(&first_receipt, recovery_receipt)
            .unwrap();
    let second_outcome =
        PublicationCrashRecoveryOutcome::admit_recovery_receipt(&second_receipt, recovery_receipt)
            .unwrap();

    assert_eq!(
        first_outcome.recovered().stable_root_epoch(),
        Some(first.new_root.epoch().get())
    );
    assert_eq!(
        second_outcome.recovered().stable_root_epoch(),
        Some(second.new_root.epoch().get())
    );
    assert_ne!(
        first_outcome.recovered().stable_root_epoch(),
        second_outcome.recovered().stable_root_epoch()
    );
}
