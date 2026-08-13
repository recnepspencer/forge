use std::process::Command;

use worth_store_offline_verifier::{RecoveryObserverReport, RECOVERY_OBSERVER_REPORT_VERSION};

#[test]
fn shipped_observer_process_emits_the_version_one_bounded_report() {
    let root = tempfile::tempdir().expect("observer input root");
    std::fs::write(root.path().join("selector"), b"observed").expect("observer input");
    let output = root.path().join("observer-report.bin");

    let process = Command::new(env!("CARGO_BIN_EXE_physical_store_offline_observer"))
        .arg("c8-recovery-observe")
        .arg(root.path())
        .arg(&output)
        .args(["2", "1", "1", "8"])
        .output()
        .expect("launch shipped offline observer");
    assert!(
        process.status.success(),
        "observer failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&process.stdout),
        String::from_utf8_lossy(&process.stderr),
    );

    let report =
        RecoveryObserverReport::decode(&std::fs::read(output).expect("observer report output"))
            .expect("version-one observer report");
    assert_eq!(RECOVERY_OBSERVER_REPORT_VERSION.get(), 1);
    assert_eq!(report.artifact_count(), 1);
    assert_eq!(report.bytes_read(), 8);
    assert_ne!(report.artifact_set_digest(), [0; 32]);
}
