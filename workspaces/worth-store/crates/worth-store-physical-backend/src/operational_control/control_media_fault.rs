use std::io;

use worth_store_authority::ControlStoreGeneration;

#[derive(Debug)]
pub enum ControlMediaFault {
    Io(io::Error),
    TornTail {
        offset: u64,
    },
    CorruptRecord {
        offset: u64,
        generation: Option<ControlStoreGeneration>,
    },
    GenerationMismatch {
        expected: Option<ControlStoreGeneration>,
        actual: Option<ControlStoreGeneration>,
    },
    DuplicateTransitionConflict,
    DerivedTransitionIndexCorrupt,
    RecordTooLarge {
        transition_bytes: u64,
        payload_bytes: u64,
    },
    MissingRecoveryObject {
        digest: [u8; 32],
    },
    RecoveryObjectLengthMismatch {
        digest: [u8; 32],
        expected: u64,
        actual: u64,
    },
    CorruptRecoveryObject {
        digest: [u8; 32],
    },
    EmptyRecoveryObject,
    MissingControlMediaIdentity,
    CorruptControlMediaIdentity,
    ControlMediaIdentityUnavailable,
    ControlMediaIdentityChanged {
        expected: [u8; 32],
        observed: [u8; 32],
    },
    ControlHistoryChanged,
    ControlHistoryRewound {
        expected_bytes: u64,
        observed_bytes: u64,
    },
    IdentityEntropyUnavailable,
    AllocationFailed,
    GenerationExhausted,
}

impl From<io::Error> for ControlMediaFault {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
