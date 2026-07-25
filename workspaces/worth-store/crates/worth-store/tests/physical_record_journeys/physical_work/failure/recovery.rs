use sha2::Digest;
use tempfile::tempdir;
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    PhysicalExecutorCommand, PhysicalStoreCloseOutcome, PhysicalWorkOperationFamily,
};
use worth_store_physical_backend::MediaFaultDirective;

use super::{
    admitted_write, serving_from_initialization_with_work_profile,
    serving_from_open_with_positioned_write_fault, work_fixture,
};

#[test]
fn crash_after_target_effect_leaves_a_durable_obligation_that_fences_reopen() {
    let root = tempdir().unwrap();
    let writer = child("physical_work_crash_writer", root.path());
    assert!(
        !writer.status.success(),
        "faulting writer must die before recovery-obligation retirement"
    );
    let reopener = child("physical_work_crash_reopener", root.path());
    assert!(
        reopener.status.success(),
        "reopener failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&reopener.stdout),
        String::from_utf8_lossy(&reopener.stderr),
    );
    assert!(String::from_utf8(reopener.stdout)
        .unwrap()
        .lines()
        .any(|line| line == "C5_PHYSICAL_RECOVERY_INSPECTION"));
}

#[test]
fn corrupted_recovery_evidence_is_fenced_without_forging_a_locator() {
    let root = tempdir().unwrap();
    let writer = child("physical_work_crash_writer", root.path());
    assert!(!writer.status.success());
    let journal = root.path().join("families/physical-work");
    let entry = std::fs::read_dir(&journal)
        .unwrap()
        .next()
        .expect("crash must retain one recovery record")
        .unwrap()
        .path();
    let mut bytes = std::fs::read(&entry).unwrap();
    bytes[56] ^= 0x80;
    std::fs::write(&entry, bytes).unwrap();
    let (profile, _, _) = work_fixture();
    let serving = super::super::fixture::serving_from_open_with_work_profile(root.path(), profile);

    assert!(serving.physical_recovery_obligations().is_empty());
    assert!(serving.physical_recovery_evidence_damaged());
    assert!(serving.close_plan().execute().requires_inspection());
}

#[test]
fn checksum_valid_noncanonical_recovery_evidence_cannot_forge_a_locator() {
    let root = tempdir().unwrap();
    let writer = child("physical_work_crash_writer", root.path());
    assert!(!writer.status.success());
    let journal = root.path().join("families/physical-work");
    let entry = std::fs::read_dir(&journal)
        .unwrap()
        .next()
        .expect("crash must retain one recovery record")
        .unwrap()
        .path();
    let mut bytes = std::fs::read(&entry).unwrap();
    bytes[10] = 1;
    let checksum = sha2::Sha256::digest(&bytes[..128]);
    bytes[128..].copy_from_slice(&checksum);
    std::fs::write(&entry, bytes).unwrap();
    let (profile, _, _) = work_fixture();
    let serving = super::super::fixture::serving_from_open_with_work_profile(root.path(), profile);

    assert!(serving.physical_recovery_obligations().is_empty());
    assert!(serving.physical_recovery_evidence_damaged());
    assert!(serving.close_plan().execute().requires_inspection());
}

pub(crate) fn crash_writer(root: &std::path::Path) {
    let (profile, _, mutation_request) = work_fixture();
    serving_from_initialization_with_work_profile(root, profile.clone()).close();
    let catalog = std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap();
    let payload = catalog[8..16].to_vec();
    let serving = serving_from_open_with_positioned_write_fault(
        root,
        profile,
        MediaFaultDirective::PanicAfter,
    );
    let admitted = admitted_write(&serving, mutation_request);
    let command = PhysicalExecutorCommand::exact_write(admitted, payload).unwrap();
    let _ = serving.execute_physical_work(command);
    panic!("fault schedule did not terminate the target effect");
}

pub(crate) fn crash_reopener(root: &std::path::Path) {
    let (profile, _, _) = work_fixture();
    let (format, _, access) = super::super::super::configuration();
    let serving = match super::super::super::media(root)
        .open_record_store(
            worth_store::physical_runtime::PhysicalRecordOpen::new(format, access)
                .with_physical_work_profile(profile),
        )
        .into_raw()
    {
        TransitionOutcome::Success(serving) => serving,
        TransitionOutcome::Denied(_) => panic!("recovery reopener admission denied"),
        TransitionOutcome::Deferred(_) => panic!("recovery reopener admission deferred"),
        TransitionOutcome::Stale(_) => panic!("recovery reopener admission stale"),
        TransitionOutcome::RebindRequired(_) => {
            panic!("recovery reopener admission requires rebind")
        }
        TransitionOutcome::Failed(_) => panic!("recovery reopener admission requires inspection"),
    };
    let obligations = serving.physical_recovery_obligations();
    assert_eq!(obligations.len(), 1);
    let locator = obligations[0];
    assert_eq!(locator.store(), serving.store_identity());
    assert_ne!(locator.runtime(), 0);
    assert_ne!(locator.generation(), 0);
    assert_ne!(locator.operation(), 0);
    assert_eq!(
        locator.family(),
        PhysicalWorkOperationFamily::ArtifactRangeWrite
    );
    assert_eq!(
        locator.coordinate(),
        Some(
            worth_store_physical_format::RecordFrameCoordinate::new(
                worth_store_physical_format::RecordArtifactFile::BootstrapCatalog,
                8,
                8,
            )
            .unwrap()
        )
    );
    assert!(matches!(
        locator.target(),
        worth_store::physical_runtime::PhysicalWorkRecoveryTarget::Range(_)
    ));
    let payload = &std::fs::read(root.join("families/records/bootstrap.catalog")).unwrap()[8..16];
    assert_eq!(
        locator.payload_digest(),
        Some(sha2::Sha256::digest(payload).into())
    );
    assert_eq!(
        locator.recovery_disposition(),
        worth_store::physical_runtime::PhysicalWorkRecoveryDisposition::InspectionRequired
    );
    assert!(!serving.physical_recovery_evidence_damaged());
    assert!(matches!(
        serving.close_plan().execute(),
        PhysicalStoreCloseOutcome::InspectionRequired { .. }
    ));
    println!("C5_PHYSICAL_RECOVERY_INSPECTION");
}

fn child(role: &str, root: &std::path::Path) -> std::process::Output {
    super::super::super::child_process::child_command(role, root)
        .output()
        .unwrap()
}
