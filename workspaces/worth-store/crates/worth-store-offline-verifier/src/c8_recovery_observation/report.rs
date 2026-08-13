use std::path::Path;

use sha2::{Digest, Sha256};

use super::artifact_walk;
use super::physical_format;
use super::report_protocol::{self, RecoveryObserverDecodeDenial};
use super::{RecoveryObserverLimits, RecoveryObserverObservationFailure};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryObserverReport {
    artifact_count: u64,
    bytes_read: u64,
    artifact_set_digest: [u8; 32],
}

pub fn observe_recovery_artifacts(
    store_root: &Path,
    limits: RecoveryObserverLimits,
) -> Result<RecoveryObserverReport, RecoveryObserverObservationFailure> {
    let walk = artifact_walk::walk(store_root, limits)?;
    let counters = walk.counters();
    let conclusion = physical_format::conclude(walk.artifacts());
    Ok(RecoveryObserverReport {
        artifact_count: counters.artifacts_observed(),
        bytes_read: counters.bytes_read(),
        artifact_set_digest: conclusion.artifact_set_digest(),
    })
}

impl RecoveryObserverReport {
    pub const fn artifact_count(self) -> u64 {
        self.artifact_count
    }

    pub const fn bytes_read(self) -> u64 {
        self.bytes_read
    }

    pub const fn artifact_set_digest(self) -> [u8; 32] {
        self.artifact_set_digest
    }

    pub fn encode(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(96);
        report_protocol::encode_header(&mut bytes);
        bytes.extend_from_slice(&self.artifact_count.to_le_bytes());
        bytes.extend_from_slice(&self.bytes_read.to_le_bytes());
        bytes.extend_from_slice(&self.artifact_set_digest);
        let digest: [u8; 32] = Sha256::digest(&bytes).into();
        bytes.extend_from_slice(&digest);
        bytes
    }

    pub fn decode(encoded: &[u8]) -> Result<Self, RecoveryObserverDecodeDenial> {
        if encoded.len() < 32 {
            return Err(RecoveryObserverDecodeDenial::Malformed);
        }
        let (payload, digest) = encoded.split_at(encoded.len() - 32);
        let expected: [u8; 32] = Sha256::digest(payload).into();
        if digest != expected {
            return Err(RecoveryObserverDecodeDenial::DigestMismatch);
        }
        let mut bytes = payload;
        report_protocol::admit_header(&mut bytes)?;
        let report = Self {
            artifact_count: report_protocol::u64_value(&mut bytes)?,
            bytes_read: report_protocol::u64_value(&mut bytes)?,
            artifact_set_digest: report_protocol::array(&mut bytes)?,
        };
        if !bytes.is_empty() {
            return Err(RecoveryObserverDecodeDenial::Malformed);
        }
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_foundational::facade::BoundaryProtocolUnsupportedVersionPosture;

    #[test]
    fn version_one_wire_and_artifact_set_domains_are_literal_contracts() {
        let report = RecoveryObserverReport {
            artifact_count: 3,
            bytes_read: 5,
            artifact_set_digest: [7; 32],
        };
        let family = b"store.physical.recovery-observer-report";
        let mut expected = Vec::new();
        expected.extend_from_slice(&(family.len() as u64).to_le_bytes());
        expected.extend_from_slice(family);
        expected.extend_from_slice(&1_u32.to_le_bytes());
        expected.extend_from_slice(&3_u64.to_le_bytes());
        expected.extend_from_slice(&5_u64.to_le_bytes());
        expected.extend_from_slice(&[7; 32]);
        let envelope_digest: [u8; 32] = Sha256::digest(&expected).into();
        expected.extend_from_slice(&envelope_digest);
        assert_eq!(report.encode(), expected);
        assert_eq!(RecoveryObserverReport::decode(&expected), Ok(report));

        let temporary = tempfile::tempdir().unwrap();
        std::fs::write(temporary.path().join("a"), b"abc").unwrap();
        let observed = observe_recovery_artifacts(
            temporary.path(),
            RecoveryObserverLimits::new(1, 1, 1, 3).unwrap(),
        )
        .unwrap();
        let artifact_digest: [u8; 32] = Sha256::digest(b"abc").into();
        let mut artifact_set = Sha256::new();
        artifact_set.update(b"worth.store.recovery-observer.artifact-set.v1");
        artifact_set.update(1_u64.to_le_bytes());
        artifact_set.update(1_u64.to_le_bytes());
        artifact_set.update(b"a");
        artifact_set.update(3_u64.to_le_bytes());
        artifact_set.update(artifact_digest);
        assert_eq!(
            observed.artifact_set_digest(),
            <[u8; 32]>::from(artifact_set.finalize())
        );
    }

    #[test]
    fn observer_is_bounded_and_protocol_denials_are_typed() {
        let temporary = tempfile::tempdir().unwrap();
        std::fs::write(temporary.path().join("a"), b"abc").unwrap();
        std::fs::create_dir(temporary.path().join("nested")).unwrap();
        std::fs::write(temporary.path().join("nested/b"), b"de").unwrap();

        let limits = RecoveryObserverLimits::new(3, 2, 2, 5).unwrap();
        let report = observe_recovery_artifacts(temporary.path(), limits).unwrap();
        assert_eq!(report.artifact_count(), 2);
        assert_eq!(report.bytes_read(), 5);
        let encoded = report.encode();
        assert_eq!(RecoveryObserverReport::decode(&encoded), Ok(report));

        let files = RecoveryObserverLimits::new(3, 2, 1, 5).unwrap();
        let failure = observe_recovery_artifacts(temporary.path(), files).unwrap_err();
        assert_eq!(
            failure.denial(),
            super::super::RecoveryObserverObservationDenial::ArtifactLimit {
                observed: 2,
                admitted: 1,
            }
        );
        assert_eq!(failure.counters().files_opened(), 1);
        let bytes = RecoveryObserverLimits::new(3, 2, 2, 4).unwrap();
        let failure = observe_recovery_artifacts(temporary.path(), bytes).unwrap_err();
        assert!(matches!(
            failure.denial(),
            super::super::RecoveryObserverObservationDenial::ByteLimit { admitted: 4, .. }
        ));

        let mut wrong_family = encoded.clone();
        wrong_family[8] = b'x';
        refresh_digest(&mut wrong_family);
        assert_eq!(
            RecoveryObserverReport::decode(&wrong_family),
            Err(RecoveryObserverDecodeDenial::WrongProtocolFamily)
        );
        let mut damaged = encoded.clone();
        let last = damaged.len() - 1;
        damaged[last] ^= 1;
        assert_eq!(
            RecoveryObserverReport::decode(&damaged),
            Err(RecoveryObserverDecodeDenial::DigestMismatch)
        );
        assert_eq!(
            RecoveryObserverReport::decode(&encoded[..31]),
            Err(RecoveryObserverDecodeDenial::Malformed)
        );
        let mut future = encoded.clone();
        let offset = 8 + report_protocol::RECOVERY_OBSERVER_REPORT_PROTOCOL
            .as_str()
            .len();
        future[offset..offset + 4].copy_from_slice(&2_u32.to_le_bytes());
        refresh_digest(&mut future);
        let Err(RecoveryObserverDecodeDenial::UnsupportedVersion(version)) =
            RecoveryObserverReport::decode(&future)
        else {
            panic!("future protocol must be typed");
        };
        assert_eq!(
            version.posture(),
            BoundaryProtocolUnsupportedVersionPosture::ExceedsWindow
        );
        assert_eq!(report_protocol::RECOVERY_OBSERVER_REPORT_VERSION.get(), 1);
        assert_eq!(
            report_protocol::RECOVERY_OBSERVER_REPORT_COMPATIBILITY_WINDOW
                .earliest()
                .get(),
            1
        );
        assert_eq!(
            report_protocol::RECOVERY_OBSERVER_REPORT_COMPATIBILITY_WINDOW
                .latest()
                .get(),
            1
        );
    }

    fn refresh_digest(bytes: &mut [u8]) {
        let split = bytes.len() - 32;
        let digest: [u8; 32] = Sha256::digest(&bytes[..split]).into();
        bytes[split..].copy_from_slice(&digest);
    }
}
