use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::{
    ConflictBatchAdmissionInventory, ConflictBatchAdmissionScanPattern,
    ConflictBatchAdmissionSourceFirewallReport,
};

pub(super) fn temp_firewall_root(name: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "worth_conflict_batch_admission_inventory_{name}_{stamp}"
    ));
    fs::create_dir_all(&root).expect("temp root should be created");
    root
}

pub(super) fn write_source(root: &Path, name: &str, text: &str) -> PathBuf {
    let path = root.join(name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("source fixture parent should be created");
    }
    fs::write(&path, text).expect("source fixture should be written");
    path
}

pub(super) fn assert_violation_signatures(
    report: &ConflictBatchAdmissionSourceFirewallReport,
    expected: &[(&str, &str, ConflictBatchAdmissionScanPattern)],
) {
    let actual = report
        .violations()
        .iter()
        .map(|violation| {
            (
                normalized_path(violation.path()),
                violation.surface_name().to_owned(),
                violation.scan_pattern(),
            )
        })
        .collect::<Vec<_>>();
    let expected = expected
        .iter()
        .map(|(path, surface_name, pattern)| {
            (
                normalized_path(Path::new(path)),
                (*surface_name).to_owned(),
                *pattern,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.iter().zip(expected.iter()) {
        assert!(actual.0.ends_with(&expected.0));
        assert_eq!(actual.1, expected.1);
        assert_eq!(actual.2, expected.2);
    }
}

pub(super) fn assert_distinct_source_paths(
    inventory: &ConflictBatchAdmissionInventory,
    left: super::ConflictBatchAdmissionSurfaceIdentity,
    right: super::ConflictBatchAdmissionSurfaceIdentity,
) {
    let left_row = inventory
        .row_for_surface(left)
        .expect("left inventory row should exist");
    let right_row = inventory
        .row_for_surface(right)
        .expect("right inventory row should exist");
    assert_ne!(left_row.source_path(), right_row.source_path());
}

fn normalized_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
