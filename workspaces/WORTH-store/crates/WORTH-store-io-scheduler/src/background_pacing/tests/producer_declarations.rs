use super::test_support::World;

use crate::{
    admit_background_pacing, BackgroundIoPressureClass, BackgroundIoPressureShape,
    BackgroundPacingDenial, BackgroundPacingOutcome,
};

#[test]
fn producer_pressure_declarations_lower_into_scheduler_shapes() {
    let cases = [
        (
            physical_isolation_pressure(
                worth_store_physical_isolation::physical_isolation_compaction_background_pressure(),
            ),
            BackgroundIoPressureClass::CompactionRewrite,
        ),
        (
            physical_isolation_pressure(
                worth_store_physical_isolation::physical_isolation_checkpoint_background_pressure(),
            ),
            BackgroundIoPressureClass::CheckpointFlush,
        ),
        (
            physical_isolation_pressure(
                worth_store_physical_isolation::physical_isolation_scrub_background_pressure(),
            ),
            BackgroundIoPressureClass::ScrubScan,
        ),
        (
            worth_store_operations::replication_prep_background_pressure_shape(4),
            BackgroundIoPressureClass::ReplicationPrepRead,
        ),
        (
            worth_store_blob_chunks::blob_ingest_background_pressure_shape(4096),
            BackgroundIoPressureClass::IngestPressure,
        ),
        (
            worth_store_blob_chunks::blob_migration_background_pressure_shape(4096),
            BackgroundIoPressureClass::MigrationPressure,
        ),
        (
            worth_store_operations::backup_prep_background_pressure_shape(4096, 4),
            BackgroundIoPressureClass::BackupPrepRead,
        ),
        (
            worth_store_operations::repair_background_pressure_shape(4),
            BackgroundIoPressureClass::RepairScan,
        ),
        (
            worth_store_offline_verifier::offline_repair_scan_background_pressure_shape(4),
            BackgroundIoPressureClass::RepairScan,
        ),
        (
            worth_store_offline_verifier::offline_verification_pressure_background_pressure_shape(
                4,
            ),
            BackgroundIoPressureClass::VerificationPressure,
        ),
    ];

    for (declaration, expected) in cases {
        assert_eq!(
            BackgroundIoPressureShape::from_s6_background_pressure_declaration(declaration).class(),
            expected
        );
    }
}

#[test]
fn physical_isolation_declarations_lower_concrete_resource_units() {
    let compaction = BackgroundIoPressureShape::from_s6_background_pressure_declaration(
        physical_isolation_pressure(
            worth_store_physical_isolation::physical_isolation_compaction_background_pressure(),
        ),
    );
    assert!(compaction.requested_budget().queue_slots() > 0);
    assert!(compaction.requested_budget().bandwidth_tokens() > 0);
    assert!(compaction.requested_budget().write_back_window() > 0);
    assert!(compaction.requested_budget().dirty_page_budget() > 0);

    let checkpoint = BackgroundIoPressureShape::from_s6_background_pressure_declaration(
        physical_isolation_pressure(
            worth_store_physical_isolation::physical_isolation_checkpoint_background_pressure(),
        ),
    );
    assert!(checkpoint.requested_budget().flush_permits() > 0);
    assert!(checkpoint.requested_budget().sync_debt() > 0);
    assert!(checkpoint.requested_budget().write_back_window() > 0);

    let scrub = BackgroundIoPressureShape::from_s6_background_pressure_declaration(
        physical_isolation_pressure(
            worth_store_physical_isolation::physical_isolation_scrub_background_pressure(),
        ),
    );
    assert!(scrub.requested_budget().bandwidth_tokens() > 0);
    assert!(scrub.requested_budget().read_ahead_window() > 0);
}

#[test]
fn quantity_bearing_producer_declarations_survive_scheduler_lowering() {
    let replication = BackgroundIoPressureShape::from_s6_background_pressure_declaration(
        worth_store_operations::replication_prep_background_pressure_shape(4),
    );
    assert_eq!(replication.requested_budget().read_ahead_window(), 4);
    assert!(replication.requested_budget().queue_slots() > 0);

    let blob = BackgroundIoPressureShape::from_s6_background_pressure_declaration(
        worth_store_blob_chunks::blob_ingest_background_pressure_shape(4096),
    );
    assert_eq!(blob.requested_budget().bandwidth_tokens(), 4096);
    assert!(blob.requested_budget().worker_permits() > 0);

    let backup = BackgroundIoPressureShape::from_s6_background_pressure_declaration(
        worth_store_operations::backup_prep_background_pressure_shape(4096, 4),
    );
    assert_eq!(backup.requested_budget().bandwidth_tokens(), 4096);
    assert_eq!(backup.requested_budget().read_ahead_window(), 4);

    let verification = BackgroundIoPressureShape::from_s6_background_pressure_declaration(
        worth_store_offline_verifier::offline_verification_pressure_background_pressure_shape(4),
    );
    assert_eq!(verification.requested_budget().read_ahead_window(), 4);
    assert!(verification.requested_budget().queue_slots() > 0);
}

#[test]
fn producer_pressure_declarations_are_scheduler_consumable() {
    let cases = [
        (
            World::new(),
            physical_isolation_pressure(
                worth_store_physical_isolation::physical_isolation_compaction_background_pressure(),
            ),
        ),
        (
            World::commit_wal(),
            physical_isolation_pressure(
                worth_store_physical_isolation::physical_isolation_checkpoint_background_pressure(),
            ),
        ),
        (
            World::new(),
            physical_isolation_pressure(
                worth_store_physical_isolation::physical_isolation_scrub_background_pressure(),
            ),
        ),
        (
            World::new(),
            worth_store_operations::replication_prep_background_pressure_shape(4),
        ),
        (
            World::new(),
            worth_store_blob_chunks::blob_migration_background_pressure_shape(4096),
        ),
        (
            World::new(),
            worth_store_operations::backup_prep_background_pressure_shape(4096, 4),
        ),
        (
            World::new(),
            worth_store_operations::repair_background_pressure_shape(4),
        ),
        (
            World::new(),
            worth_store_offline_verifier::offline_repair_scan_background_pressure_shape(4),
        ),
        (
            World::new(),
            worth_store_offline_verifier::offline_verification_pressure_background_pressure_shape(
                4,
            ),
        ),
    ];

    for (world, declaration) in cases {
        let shape = BackgroundIoPressureShape::from_s6_background_pressure_declaration(declaration);
        let outcome = admit_background_pacing(world.request(shape));
        assert!(matches!(
            outcome,
            BackgroundPacingOutcome::AdmittedWithDebt(_)
        ));
    }

    let blob_ingest = BackgroundIoPressureShape::from_s6_background_pressure_declaration(
        worth_store_blob_chunks::blob_ingest_background_pressure_shape(4096),
    );
    assert!(matches!(
        World::new().capacity_denial(blob_ingest),
        BackgroundPacingDenial::BackendRequirementMismatch { .. }
    ));
}

#[test]
fn equivalent_repair_declarations_have_parity_under_same_basis() {
    let operations = BackgroundIoPressureShape::from_s6_background_pressure_declaration(
        worth_store_operations::repair_background_pressure_shape(4),
    );
    let offline = BackgroundIoPressureShape::from_s6_background_pressure_declaration(
        worth_store_offline_verifier::offline_repair_scan_background_pressure_shape(4),
    );
    assert_eq!(operations.requested_budget(), offline.requested_budget());

    let operations_outcome = admit_background_pacing(World::new().request(operations));
    let offline_outcome = admit_background_pacing(World::new().request(offline));
    assert_eq!(operations_outcome, offline_outcome);
}

fn physical_isolation_pressure(
    pressure: worth_store_physical_isolation::PhysicalIsolationBackgroundPressureKind,
) -> worth_store_contracts::S6BackgroundPressureDeclaration {
    worth_store_physical_isolation::physical_isolation_s6_background_pressure_declaration(pressure)
}
