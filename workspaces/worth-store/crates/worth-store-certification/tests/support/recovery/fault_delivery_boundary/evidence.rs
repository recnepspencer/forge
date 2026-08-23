use sha2::{Digest, Sha256};
use worth_store_offline_verifier::{observe_recovery_artifacts, RecoveryObserverLimits};
use worth_store_physical_certification::FreshRuntimeCrashRecoveryEvidence;
use worth_store_recovery_runtime::RecoveryReportEnvelope;

pub fn fresh_runtime_crash_evidence() -> FreshRuntimeCrashRecoveryEvidence {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("selector"), b"observed").unwrap();
    let observer = observe_recovery_artifacts(
        root.path(),
        RecoveryObserverLimits::new(1, 1, 1, 8).unwrap(),
    )
    .unwrap();
    let runtime_report = recovered_runtime_report();
    FreshRuntimeCrashRecoveryEvidence::from_reports(runtime_report, observer).unwrap()
}

fn recovered_runtime_report() -> RecoveryReportEnvelope {
    let family_text = worth_store_recovery_runtime::RECOVERY_REPORT_PROTOCOL
        .as_str()
        .to_owned();
    let family = family_text.as_bytes();
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&(family.len() as u64).to_le_bytes());
    bytes.extend_from_slice(family);
    bytes.extend_from_slice(&1_u32.to_le_bytes());
    bytes.push(1);
    bytes.push(1);
    bytes.extend_from_slice(&[7; 16]);
    bytes.push(1);
    bytes.extend_from_slice(&1_u64.to_le_bytes());
    bytes.extend_from_slice(&0_u64.to_le_bytes());
    bytes.extend_from_slice(&0_u64.to_le_bytes());
    bytes.extend_from_slice(&0_u64.to_le_bytes());
    let digest: [u8; 32] = Sha256::digest(&bytes).into();
    bytes.extend_from_slice(&digest);
    RecoveryReportEnvelope::decode(&bytes).unwrap()
}
