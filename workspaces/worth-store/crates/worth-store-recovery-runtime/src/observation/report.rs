use sha2::{Digest, Sha256};

use super::{protocol, RecoveryReportCounters, RecoveryReportDecodeDenial};
use crate::PhysicalRecoveryOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryReportOutcome {
    Recovered,
    Refused,
    Blocked,
    PublicationIndeterminate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryReportEnvelope {
    outcome: RecoveryReportOutcome,
    store: Option<[u8; 16]>,
    root_generation: Option<u64>,
    counters: RecoveryReportCounters,
}

impl RecoveryReportEnvelope {
    pub fn from_outcome(outcome: &PhysicalRecoveryOutcome) -> Self {
        match outcome {
            PhysicalRecoveryOutcome::Recovered(handoff) => {
                let cleanup = handoff.cleanup_posture().evidence().counters();
                Self {
                    outcome: RecoveryReportOutcome::Recovered,
                    store: Some(handoff.core().store_identity().bytes()),
                    root_generation: Some(handoff.core().root().generation()),
                    counters: RecoveryReportCounters::new(
                        cleanup.performed_effects,
                        cleanup.actions_completed,
                        cleanup.actions_deferred,
                    ),
                }
            }
            PhysicalRecoveryOutcome::Refused(refusal) => Self {
                outcome: RecoveryReportOutcome::Refused,
                store: None,
                root_generation: None,
                counters: RecoveryReportCounters::new(refusal.recovery_effects(), 0, 0),
            },
            PhysicalRecoveryOutcome::Blocked(block) => Self {
                outcome: RecoveryReportOutcome::Blocked,
                store: Some(block.store_identity().bytes()),
                root_generation: block.evidence().source_generation,
                counters: RecoveryReportCounters::new(block.recovery_effects(), 0, 0),
            },
            PhysicalRecoveryOutcome::PublicationIndeterminate(indeterminate) => Self {
                outcome: RecoveryReportOutcome::PublicationIndeterminate,
                store: Some(indeterminate.store_identity().bytes()),
                root_generation: None,
                counters: RecoveryReportCounters::new(indeterminate.recovery_effects(), 0, 0),
            },
        }
    }

    pub const fn outcome(&self) -> RecoveryReportOutcome {
        self.outcome
    }
    pub const fn store_identity(&self) -> Option<[u8; 16]> {
        self.store
    }
    pub const fn root_generation(&self) -> Option<u64> {
        self.root_generation
    }
    pub const fn counters(&self) -> RecoveryReportCounters {
        self.counters
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(96);
        protocol::encode_header(&mut bytes);
        bytes.push(outcome_byte(self.outcome));
        encode_optional(&mut bytes, self.store.as_ref());
        match self.root_generation {
            Some(generation) => {
                bytes.push(1);
                bytes.extend_from_slice(&generation.to_le_bytes());
            }
            None => bytes.push(0),
        }
        bytes.extend_from_slice(&self.counters.recovery_effects().to_le_bytes());
        bytes.extend_from_slice(&self.counters.cleanup_performed().to_le_bytes());
        bytes.extend_from_slice(&self.counters.cleanup_deferred().to_le_bytes());
        let digest: [u8; 32] = Sha256::digest(&bytes).into();
        bytes.extend_from_slice(&digest);
        bytes
    }

    pub fn decode(encoded: &[u8]) -> Result<Self, RecoveryReportDecodeDenial> {
        if encoded.len() < 32 {
            return Err(RecoveryReportDecodeDenial::Malformed);
        }
        let (payload, digest) = encoded.split_at(encoded.len() - 32);
        if <[u8; 32]>::from(Sha256::digest(payload)) != digest {
            return Err(RecoveryReportDecodeDenial::DigestMismatch);
        }
        let mut bytes = payload;
        protocol::admit_header(&mut bytes)?;
        let outcome = decode_outcome(protocol::byte(&mut bytes)?)?;
        let store = match protocol::byte(&mut bytes)? {
            0 => None,
            1 => Some(protocol::array::<16>(&mut bytes)?),
            _ => return Err(RecoveryReportDecodeDenial::Malformed),
        };
        let root_generation = match protocol::byte(&mut bytes)? {
            0 => None,
            1 => Some(protocol::u64_value(&mut bytes)?),
            _ => return Err(RecoveryReportDecodeDenial::Malformed),
        };
        let counters = RecoveryReportCounters::new(
            protocol::u64_value(&mut bytes)?,
            protocol::u64_value(&mut bytes)?,
            protocol::u64_value(&mut bytes)?,
        );
        if !bytes.is_empty() {
            return Err(RecoveryReportDecodeDenial::Malformed);
        }
        Ok(Self {
            outcome,
            store,
            root_generation,
            counters,
        })
    }
}

fn outcome_byte(outcome: RecoveryReportOutcome) -> u8 {
    match outcome {
        RecoveryReportOutcome::Recovered => 1,
        RecoveryReportOutcome::Refused => 2,
        RecoveryReportOutcome::Blocked => 3,
        RecoveryReportOutcome::PublicationIndeterminate => 4,
    }
}

fn decode_outcome(value: u8) -> Result<RecoveryReportOutcome, RecoveryReportDecodeDenial> {
    match value {
        1 => Ok(RecoveryReportOutcome::Recovered),
        2 => Ok(RecoveryReportOutcome::Refused),
        3 => Ok(RecoveryReportOutcome::Blocked),
        4 => Ok(RecoveryReportOutcome::PublicationIndeterminate),
        _ => Err(RecoveryReportDecodeDenial::Malformed),
    }
}

fn encode_optional(bytes: &mut Vec<u8>, value: Option<&[u8; 16]>) {
    match value {
        Some(value) => {
            bytes.push(1);
            bytes.extend_from_slice(value);
        }
        None => bytes.push(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PhysicalRecoveryOutcome, PhysicalRecoveryRefusal, PhysicalRecoveryRefusalKind};
    use worth_foundational::facade::BoundaryProtocolUnsupportedVersionPosture;

    #[test]
    fn report_round_trip_wrong_family_future_version_and_digest_are_distinct() {
        let outcome = PhysicalRecoveryOutcome::Refused(PhysicalRecoveryRefusal::new(
            PhysicalRecoveryRefusalKind::CoordinationUnavailable,
            0,
        ));
        let encoded = RecoveryReportEnvelope::from_outcome(&outcome).encode();
        assert_eq!(
            RecoveryReportEnvelope::decode(&encoded).unwrap().outcome(),
            RecoveryReportOutcome::Refused
        );

        let mut wrong_family = encoded.clone();
        wrong_family[8] = b'x';
        refresh_digest(&mut wrong_family);
        assert_eq!(
            RecoveryReportEnvelope::decode(&wrong_family),
            Err(RecoveryReportDecodeDenial::WrongProtocolFamily)
        );

        let mut future = encoded.clone();
        let version_offset = 8 + super::super::RECOVERY_REPORT_PROTOCOL.as_str().len();
        future[version_offset..version_offset + 4].copy_from_slice(&2_u32.to_le_bytes());
        refresh_digest(&mut future);
        let Err(RecoveryReportDecodeDenial::UnsupportedVersion(unsupported)) =
            RecoveryReportEnvelope::decode(&future)
        else {
            panic!("future version must be typed")
        };
        assert_eq!(
            unsupported.posture(),
            BoundaryProtocolUnsupportedVersionPosture::ExceedsWindow
        );

        let mut damaged = encoded;
        damaged[20] ^= 1;
        assert_eq!(
            RecoveryReportEnvelope::decode(&damaged),
            Err(RecoveryReportDecodeDenial::DigestMismatch)
        );
    }

    fn refresh_digest(bytes: &mut [u8]) {
        let split = bytes.len() - 32;
        let digest: [u8; 32] = Sha256::digest(&bytes[..split]).into();
        bytes[split..].copy_from_slice(&digest);
    }
}
