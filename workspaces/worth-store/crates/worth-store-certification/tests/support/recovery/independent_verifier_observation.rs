use sha2::{Digest, Sha256};
use worth_store_offline_verifier::{observe_recovery_artifacts, RecoveryObserverLimits};
use worth_store_physical_certification::IndependentVerifierObservation;
use worth_store_recovery_runtime::RecoveryReportEnvelope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum RuntimeComparisonFixture {
    Equivalent,
    ArtifactDigestMismatch,
}

pub fn observed_runtime_comparison(
    fixture: RuntimeComparisonFixture,
) -> IndependentVerifierObservation {
    let root = tempfile::tempdir().unwrap();
    if fixture == RuntimeComparisonFixture::Equivalent {
        std::fs::write(root.path().join("selector"), b"observed").unwrap();
    }
    let observer = observe_recovery_artifacts(
        root.path(),
        RecoveryObserverLimits::new(1, 8).unwrap(),
    )
    .unwrap();
    IndependentVerifierObservation::from_reports(&recovered_runtime_report(), observer)
}

fn recovered_runtime_report() -> RecoveryReportEnvelope {
    let family = worth_store_recovery_runtime::RECOVERY_REPORT_PROTOCOL
        .as_str()
        .as_bytes();
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
