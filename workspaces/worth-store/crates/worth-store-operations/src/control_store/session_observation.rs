use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};
use worth_store_authority::ControlStoreSelectionCoordinates;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationalControlSessionIdentity([u8; 32]);

impl OperationalControlSessionIdentity {
    pub(crate) const fn from_open_session(identity: [u8; 32]) -> Self {
        Self(identity)
    }

    pub const fn fingerprint(self) -> [u8; 32] {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationalControlProcessIdentity([u8; 32]);

impl OperationalControlProcessIdentity {
    pub(crate) const fn from_process_instance(identity: [u8; 32]) -> Self {
        Self(identity)
    }

    pub const fn fingerprint(self) -> [u8; 32] {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalControlSessionObservation {
    process: OperationalControlProcessIdentity,
    session: OperationalControlSessionIdentity,
    media_identity_fingerprint: [u8; 32],
    coordinates: Option<ControlStoreSelectionCoordinates>,
}

impl OperationalControlSessionObservation {
    pub(crate) const fn from_open_store(
        process: OperationalControlProcessIdentity,
        session: OperationalControlSessionIdentity,
        media_identity_fingerprint: [u8; 32],
        coordinates: Option<ControlStoreSelectionCoordinates>,
    ) -> Self {
        Self {
            process,
            session,
            media_identity_fingerprint,
            coordinates,
        }
    }

    /// Rehydrates an observation transported across a certification process
    /// boundary. This is deliberately only an observation: it carries no
    /// control-store authority and opens no operational door.
    pub const fn from_untrusted_certification_report(
        process_fingerprint: [u8; 32],
        session_fingerprint: [u8; 32],
        media_identity_fingerprint: [u8; 32],
        coordinates: Option<ControlStoreSelectionCoordinates>,
    ) -> Self {
        Self {
            process: OperationalControlProcessIdentity::from_process_instance(process_fingerprint),
            session: OperationalControlSessionIdentity::from_open_session(session_fingerprint),
            media_identity_fingerprint,
            coordinates,
        }
    }

    pub const fn process(self) -> OperationalControlProcessIdentity {
        self.process
    }

    pub const fn session(self) -> OperationalControlSessionIdentity {
        self.session
    }

    pub const fn media_identity_fingerprint(self) -> [u8; 32] {
        self.media_identity_fingerprint
    }

    pub const fn coordinates(self) -> Option<ControlStoreSelectionCoordinates> {
        self.coordinates
    }
}

pub(crate) fn next_control_session_identity(
    media_identity: [u8; 32],
) -> OperationalControlSessionIdentity {
    static NEXT_SESSION: AtomicU64 = AtomicU64::new(1);
    let sequence = NEXT_SESSION.fetch_add(1, Ordering::Relaxed);
    let mut digest = Sha256::new();
    digest.update(b"worth-store-operational-control-session-v1");
    digest.update(current_process_identity().fingerprint());
    digest.update(sequence.to_be_bytes());
    digest.update(media_identity);
    OperationalControlSessionIdentity::from_open_session(digest.finalize().into())
}

pub(crate) fn current_process_identity() -> OperationalControlProcessIdentity {
    static PROCESS_START_NONCE: OnceLock<[u8; 32]> = OnceLock::new();
    let nonce = PROCESS_START_NONCE.get_or_init(process_start_nonce);
    let mut digest = Sha256::new();
    digest.update(b"worth-store-operational-control-process-instance-v1");
    digest.update(nonce);
    digest.update(std::process::id().to_be_bytes());
    OperationalControlProcessIdentity::from_process_instance(digest.finalize().into())
}

fn process_start_nonce() -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-operational-control-process-start-v1");
    digest.update(std::process::id().to_be_bytes());
    digest.update(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .to_be_bytes(),
    );
    if let Ok(executable) = std::env::current_exe() {
        digest.update(executable.as_os_str().as_encoded_bytes());
    }
    digest.finalize().into()
}
