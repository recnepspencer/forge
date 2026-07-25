use sha2::{Digest, Sha256};

use super::{
    artifact_inventory, OfflineHostilePhysicalTruthBudget, OfflineHostilePhysicalTruthDenial,
};

#[test]
fn raw_inventory_is_sorted_exact_bounded_and_recovery_aware() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    std::fs::create_dir_all(root.join("families/physical-work")).unwrap();
    std::fs::create_dir_all(root.join("families/records")).unwrap();
    std::fs::write(root.join("families/records/z.data"), b"abcdef").unwrap();
    std::fs::write(
        root.join("families/physical-work/effect.pending"),
        b"journal",
    )
    .unwrap();
    let budget = OfflineHostilePhysicalTruthBudget::new(2, 32, 3).unwrap();

    let artifacts = artifact_inventory::inventory(&root, budget).unwrap();

    assert_eq!(artifacts.len(), 2);
    assert_eq!(artifacts[0].path(), "families/physical-work/effect.pending");
    assert_eq!(artifacts[0].prefix(), b"jou");
    assert!(artifacts[0].is_recovery_obligation());
    assert_eq!(artifacts[1].path(), "families/records/z.data");
    assert_eq!(artifacts[1].byte_length(), 6);
    let expected_digest: [u8; 32] = Sha256::digest(b"abcdef").into();
    assert_eq!(artifacts[1].digest(), expected_digest);
}

#[test]
fn raw_inventory_denies_before_crossing_file_or_byte_budget() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("a"), b"1234").unwrap();
    std::fs::write(root.join("b"), b"5678").unwrap();

    let files = OfflineHostilePhysicalTruthBudget::new(1, 16, 1).unwrap();
    assert_eq!(
        artifact_inventory::inventory(&root, files),
        Err(OfflineHostilePhysicalTruthDenial::FileBudgetExceeded)
    );
    let bytes = OfflineHostilePhysicalTruthBudget::new(2, 7, 1).unwrap();
    assert_eq!(
        artifact_inventory::inventory(&root, bytes),
        Err(OfflineHostilePhysicalTruthDenial::ByteBudgetExceeded)
    );
}

#[test]
fn bounded_inventory_retains_a_complete_mutation_observation() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    std::fs::create_dir_all(root.join("namespace")).unwrap();
    let observation = format!(
        "version=1\nprocess={:010}\nruntime={}\nattempt={}\n",
        41,
        "11".repeat(16),
        "22".repeat(16),
    );
    assert_eq!(observation.len(), 111);
    std::fs::write(root.join("namespace/mutation.lock"), observation.as_bytes()).unwrap();
    let budget = OfflineHostilePhysicalTruthBudget::new(1, 512, 512).unwrap();

    let artifacts = artifact_inventory::inventory(&root, budget).unwrap();
    let expected_digest: [u8; 32] = Sha256::digest(observation.as_bytes()).into();

    assert_eq!(artifacts.len(), 1);
    assert_eq!(artifacts[0].byte_length(), observation.len() as u64);
    assert_eq!(artifacts[0].prefix(), observation.as_bytes());
    assert_eq!(artifacts[0].digest(), expected_digest);
}
